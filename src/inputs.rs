//! Streaming FASTA/JSONL inputs, with transparent gzip decompression.
use crate::{Block, Query, Record};
use flate2::read::MultiGzDecoder;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn reader(path: &Path) -> io::Result<Box<dyn BufRead>> {
    let mut input = BufReader::new(File::open(path)?);
    if input.fill_buf()?.starts_with(&[0x1f, 0x8b]) {
        Ok(Box::new(BufReader::new(MultiGzDecoder::new(input))))
    } else {
        Ok(Box::new(input))
    }
}

pub fn is_fasta(path: &Path) -> Result<bool> {
    for line in reader(path)?.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            return Ok(line.trim_start().starts_with('>'));
        }
    }
    Err("sequence file is empty".into())
}

/// Holds at most one complete FASTA record, not the entire reference.
pub fn fasta(path: &Path, mut consume: impl FnMut(String, String) -> Result<()>) -> Result<()> {
    let mut header: Option<String> = None;
    let mut sequence = String::new();
    for (line_no, line) in reader(path)?.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(next) = line.strip_prefix('>') {
            if let Some(previous) = header.take() {
                if sequence.is_empty() {
                    return Err(format!("empty FASTA record: {previous}").into());
                }
                consume(previous, std::mem::take(&mut sequence))?;
            }
            if next.trim().is_empty() {
                return Err(format!("empty FASTA header at line {}", line_no + 1).into());
            }
            header = Some(next.trim().into());
        } else {
            if header.is_none() {
                return Err(format!("FASTA sequence before header at line {}", line_no + 1).into());
            }
            // Wrapping is allowed; spaces embedded in sequences are invalid.
            sequence.push_str(line);
        }
    }
    let header = header.ok_or("FASTA file is empty")?;
    if sequence.is_empty() {
        return Err(format!("empty FASTA record: {header}").into());
    }
    consume(header, sequence)
}

pub fn queries(path: &Path) -> Result<Vec<Query>> {
    let mut queries = Vec::new();
    if is_fasta(path)? {
        fasta(path, |header, sequence| {
            queries.push(Query {
                id: header.split_whitespace().next().unwrap().into(),
                sequence,
                intended_genes: Vec::new(),
                allele: None,
                ddg: None,
                target: None,
            });
            Ok(())
        })?;
    } else {
        for (i, line) in reader(path)?.lines().enumerate() {
            let line = line?;
            if !line.trim().is_empty() {
                queries.push(
                    serde_json::from_str(&line)
                        .map_err(|e| format!("query JSONL line {}: {e}", i + 1))?,
                );
            }
        }
    }
    Ok(queries)
}

/// Bare FASTA has record-relative coordinates. Optional gene= / gene: header
/// fields provide associations; otherwise its record ID is the association.
pub fn fasta_record(header: String, sequence: String) -> Record {
    let mut fields = header.split_whitespace();
    let id = fields.next().unwrap().to_owned();
    let genes = fields
        .find_map(|f| f.strip_prefix("gene=").or_else(|| f.strip_prefix("gene:")))
        .map(|s| s.split(',').map(str::to_owned).collect())
        .unwrap_or_else(|| vec![id.clone()]);
    let length = sequence.len();
    Record {
        id: id.clone(),
        sequence,
        genes,
        transcripts: Vec::new(),
        contig: id,
        strand: "+".into(),
        blocks: vec![Block {
            start: 0,
            end: length,
        }],
    }
}

pub struct ReferenceRecords {
    lines: std::io::Lines<Box<dyn BufRead>>,
    fasta: bool,
    pending: Option<String>,
    finished: bool,
}
impl ReferenceRecords {
    pub fn open(path: &Path) -> Result<Self> {
        Ok(Self {
            lines: reader(path)?.lines(),
            fasta: is_fasta(path)?,
            pending: None,
            finished: false,
        })
    }
}
impl Iterator for ReferenceRecords {
    type Item = Result<Record>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let mut sequence = String::new();
        loop {
            match self.lines.next() {
                Some(Err(e)) => {
                    self.finished = true;
                    return Some(Err(e.into()));
                }
                Some(Ok(line)) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    if !self.fasta {
                        return Some(serde_json::from_str(line).map_err(Into::into));
                    }
                    if let Some(header) = line.strip_prefix('>') {
                        if header.trim().is_empty() {
                            self.finished = true;
                            return Some(Err("empty FASTA header".into()));
                        }
                        if let Some(previous) = self.pending.replace(header.to_owned()) {
                            if sequence.is_empty() {
                                self.finished = true;
                                return Some(Err("empty FASTA record".into()));
                            }
                            return Some(Ok(fasta_record(previous, sequence)));
                        }
                    } else if self.pending.is_none() {
                        self.finished = true;
                        return Some(Err("FASTA sequence before header".into()));
                    } else {
                        sequence.push_str(line);
                    }
                }
                None => {
                    self.finished = true;
                    return self.pending.take().map(|header| {
                        if sequence.is_empty() {
                            Err("empty FASTA record".into())
                        } else {
                            Ok(fasta_record(header, sequence))
                        }
                    });
                }
            }
        }
    }
}
