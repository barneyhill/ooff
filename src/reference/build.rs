use super::{BIOTYPES, Result};
use crate::{Block, Record, inputs};
use serde::Serialize;
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufRead, BufWriter, Write},
    path::Path,
};

#[derive(Debug, Clone, Serialize)]
pub struct Gene {
    pub id: String,
    pub name: String,
    pub contig: String,
    pub start: usize,
    pub end: usize,
    pub strand: String,
    pub biotype: String,
}
#[derive(Debug)]
struct Transcript {
    gene: String,
    contig: String,
    strand: String,
    exons: BTreeSet<(usize, usize)>,
}
struct RnaRecord {
    id: String,
    gene: String,
    transcript: Option<String>,
    blocks: Vec<Block>,
}
#[derive(Default, Serialize)]
pub struct Stats {
    pub records: usize,
    pub bases: usize,
    pub unknown_bases: usize,
}

fn attributes(text: &str) -> inputs::Result<BTreeMap<&str, &str>> {
    let mut fields = BTreeMap::new();
    for field in text.split(';').map(str::trim).filter(|f| !f.is_empty()) {
        let (key, value) = field
            .split_once(char::is_whitespace)
            .ok_or("invalid GTF attribute")?;
        let value = value.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .ok_or("GTF attributes must be quoted")?;
        if fields.insert(key, value).is_some()
            && matches!(
                key,
                "gene_id" | "transcript_id" | "gene_name" | "gene_biotype" | "gene_type"
            )
        {
            return Err(format!("duplicate GTF attribute: {key}").into());
        }
    }
    Ok(fields)
}
fn allowed(biotype: &str) -> bool {
    !biotype.contains("pseudogene")
        || biotype.starts_with("transcribed_")
        || biotype.starts_with("translated_")
}

pub fn build_reference(fasta: &Path, gtf: &Path, output: &Path) -> Result<Stats> {
    let mut genes = BTreeMap::new();
    let mut excluded_genes = BTreeSet::new();
    let mut transcripts: BTreeMap<String, Transcript> = BTreeMap::new();
    for (i, line) in inputs::reader(gtf)?.lines().enumerate() {
        let line = line?;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 9 {
            return Err(format!("GTF line {}: expected 9 tab-separated columns", i + 1).into());
        }
        let kind = fields[2];
        if !matches!(kind, "gene" | "transcript" | "exon") {
            continue;
        }
        let start = fields[3]
            .parse::<usize>()?
            .checked_sub(1)
            .ok_or("GTF start must be positive")?;
        let end = fields[4].parse::<usize>()?;
        if start >= end || !matches!(fields[6], "+" | "-") {
            return Err(format!("invalid GTF coordinates/strand at line {}", i + 1).into());
        }
        let attrs = attributes(fields[8])?;
        let id = *attrs.get("gene_id").ok_or("GTF record missing gene_id")?;
        if id.is_empty() {
            return Err("empty GTF gene_id".into());
        }
        if kind == "gene" {
            if genes.contains_key(id) || excluded_genes.contains(id) {
                return Err(format!("duplicate GTF gene: {id}").into());
            }
            let biotype = *attrs
                .get("gene_biotype")
                .or_else(|| attrs.get("gene_type"))
                .ok_or("GTF gene missing gene_biotype/gene_type")?;
            if !allowed(biotype) {
                excluded_genes.insert(id.to_owned());
                continue;
            }
            genes.insert(
                id.to_owned(),
                Gene {
                    id: id.into(),
                    name: attrs.get("gene_name").copied().unwrap_or("").into(),
                    contig: fields[0].into(),
                    start,
                    end,
                    strand: fields[6].into(),
                    biotype: biotype.into(),
                },
            );
        } else {
            let tid = *attrs
                .get("transcript_id")
                .ok_or("GTF transcript/exon missing transcript_id")?;
            if tid.is_empty() {
                return Err("empty GTF transcript_id".into());
            }
            let tx = transcripts.entry(tid.into()).or_insert_with(|| Transcript {
                gene: id.into(),
                contig: fields[0].into(),
                strand: fields[6].into(),
                exons: BTreeSet::new(),
            });
            if tx.gene != id || tx.contig != fields[0] || tx.strand != fields[6] {
                return Err(format!("inconsistent transcript annotation: {tid}").into());
            }
            if kind == "exon" {
                tx.exons.insert((start, end));
            }
        }
    }
    if genes.is_empty() {
        return Err("GTF contains no eligible gene records".into());
    }
    let mut by_contig: BTreeMap<String, Vec<RnaRecord>> = BTreeMap::new();
    for gene in genes.values() {
        by_contig
            .entry(gene.contig.clone())
            .or_default()
            .push(RnaRecord {
                id: format!("gene_body:{}", gene.id),
                gene: gene.id.clone(),
                transcript: None,
                blocks: vec![Block {
                    start: gene.start,
                    end: gene.end,
                }],
            });
    }
    for (id, tx) in transcripts {
        let Some(gene) = genes.get(&tx.gene) else {
            if excluded_genes.contains(&tx.gene) {
                continue;
            }
            return Err(format!("transcript {id} has no gene record: {}", tx.gene).into());
        };
        if tx.exons.is_empty() {
            return Err(format!("transcript has no exons: {id}").into());
        }
        if gene.contig != tx.contig
            || gene.strand != tx.strand
            || tx
                .exons
                .iter()
                .any(|&(a, b)| a < gene.start || b > gene.end)
        {
            return Err(format!("transcript {id} lies outside its annotated gene").into());
        }
        let mut blocks: Vec<_> = tx
            .exons
            .into_iter()
            .map(|(start, end)| Block { start, end })
            .collect();
        if gene.strand == "-" {
            blocks.reverse();
        }
        by_contig
            .entry(gene.contig.clone())
            .or_default()
            .push(RnaRecord {
                id: format!("mature:{id}"),
                gene: gene.id.clone(),
                transcript: Some(id),
                blocks,
            });
    }
    let mut fa = BufWriter::new(File::create_new(output.join("reference.fa"))?);
    let mut annotations = BufWriter::new(File::create_new(output.join("records.jsonl"))?);
    let mut stats = Stats::default();
    let mut seen = BTreeSet::new();
    inputs::fasta(fasta, |header, sequence| {
        let contig = header.split_whitespace().next().unwrap();
        if !seen.insert(contig.to_owned()) {
            return Err(format!("duplicate FASTA contig: {contig}").into());
        }
        let Some(records) = by_contig.remove(contig) else {
            return Ok(());
        };
        if !sequence.is_ascii() {
            return Err(format!("non-ASCII genome sequence: {contig}").into());
        }
        eprintln!("Preparing RNA records from {contig}");
        for r in records {
            let gene = &genes[&r.gene];
            let mut seq = Vec::new();
            for block in &r.blocks {
                let piece = sequence
                    .as_bytes()
                    .get(block.start..block.end)
                    .ok_or_else(|| format!("gene {} exceeds FASTA contig bounds", gene.id))?;
                if gene.strand == "-" {
                    seq.extend(piece.iter().rev().copied().map(complement));
                } else {
                    seq.extend_from_slice(piece);
                }
            }
            let record = Record {
                id: r.id,
                sequence: String::from_utf8(seq)?,
                genes: vec![r.gene],
                transcripts: r.transcript.into_iter().collect(),
                contig: contig.into(),
                strand: gene.strand.clone(),
                blocks: r.blocks,
            };
            record.validate()?;
            stats.records += 1;
            stats.bases += record.sequence.len();
            stats.unknown_bases += crate::normalize(&record.sequence, false)?
                .iter()
                .filter(|&&b| b == b'N')
                .count();
            writeln!(fa, ">{}|{}\n{}", record.id, gene.id, record.sequence)?;
            serde_json::to_writer(
                &mut annotations,
                &json!({"id":record.id,"genes":record.genes,"transcripts":record.transcripts,
                "contig":record.contig,"strand":record.strand,"blocks":record.blocks,"length":record.sequence.len(),"biotype":gene.biotype}),
            )?;
            writeln!(annotations)?;
        }
        Ok(())
    })?;
    if !by_contig.is_empty() {
        return Err(format!(
            "GTF contigs absent from FASTA: {}",
            by_contig.keys().cloned().collect::<Vec<_>>().join(", ")
        )
        .into());
    }
    fa.flush()?;
    annotations.flush()?;
    let symbols: BTreeMap<_, _> = genes.iter().map(|(id, g)| (id, &g.name)).collect();
    serde_json::to_writer_pretty(File::create_new(output.join("genes.json"))?, &symbols)?;
    serde_json::to_writer_pretty(
        File::create_new(output.join("reference-manifest.json"))?,
        &json!({"counts":stats,"biotype_policy":BIOTYPES}),
    )?;
    Ok(stats)
}

fn complement(base: u8) -> u8 {
    let result = match base.to_ascii_uppercase() {
        b'A' => b'T',
        b'T' | b'U' => b'A',
        b'C' => b'G',
        b'G' => b'C',
        b'R' => b'Y',
        b'Y' => b'R',
        b'K' => b'M',
        b'M' => b'K',
        b'B' => b'V',
        b'V' => b'B',
        b'D' => b'H',
        b'H' => b'D',
        other => other,
    };
    if base.is_ascii_lowercase() {
        result.to_ascii_lowercase()
    } else {
        result
    }
}
