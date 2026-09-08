//! Annotated, reusable reference indexes for the normal CLI.
use crate::{Block, Record, Site, align_site, fm::Index, normalize};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
    sync::OnceLock,
    time::UNIX_EPOCH,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize)]
pub struct FastaRecord {
    pub header: String,
    pub offset: usize,
    pub length: usize,
    pub line_bases: usize,
    pub line_bytes: usize,
}
#[derive(Serialize, Deserialize)]
pub struct ReferenceInfo {
    pub path: String,
    pub file_bytes: u64,
    pub modified_nanos: u128,
    pub sha256: String,
    pub unknown_bases: usize,
    pub records: Vec<FastaRecord>,
}

impl ReferenceInfo {
    pub fn create(reference: &Path, output: &Path) -> Result<Self> {
        let file = File::open(reference)?;
        let metadata = file.metadata()?;
        let modified_nanos = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_nanos();
        let mut reader = BufReader::new(file);
        let mut hash = Sha256::new();
        let mut line = Vec::new();
        let mut position = 0;
        let mut records: Vec<FastaRecord> = Vec::new();
        let mut unknown_bases = 0;
        let mut short = false;
        loop {
            line.clear();
            let bytes = reader.read_until(b'\n', &mut line)?;
            if bytes == 0 {
                break;
            }
            hash.update(&line);
            let mut bases = bytes;
            if line.last() == Some(&b'\n') {
                bases -= 1;
            }
            if bases > 0 && line[bases - 1] == b'\r' {
                bases -= 1;
            }
            if line.first() == Some(&b'>') {
                if records.last().is_some_and(|r| r.length == 0) {
                    return Err("empty FASTA record".into());
                }
                records.push(FastaRecord {
                    header: String::from_utf8(line[1..bases].to_vec())?,
                    offset: position + bytes,
                    length: 0,
                    line_bases: 0,
                    line_bytes: 0,
                });
                short = false;
            } else {
                if bases == 0 || short {
                    return Err("invalid or inconsistently wrapped FASTA".into());
                }
                let record = records.last_mut().ok_or("FASTA sequence before header")?;
                if record.line_bases == 0 {
                    record.line_bases = bases;
                    record.line_bytes = bytes;
                } else if bases > record.line_bases
                    || (bases == record.line_bases && bytes != record.line_bytes && bytes != bases)
                {
                    return Err("inconsistent FASTA wrapping".into());
                }
                short = bases < record.line_bases;
                record.length += bases;
                unknown_bases += normalize(std::str::from_utf8(&line[..bases])?, false)?
                    .iter()
                    .filter(|&&b| b == b'N')
                    .count();
            }
            position += bytes;
        }
        if records.is_empty() || records.last().is_some_and(|r| r.length == 0) {
            return Err("empty FASTA reference".into());
        }
        let info = Self {
            path: fs::canonicalize(reference)?.display().to_string(),
            file_bytes: metadata.len(),
            modified_nanos,
            sha256: format!("{:x}", hash.finalize()),
            unknown_bases,
            records,
        };
        info.verify_file(reference)?;
        let mut writer = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(output)?,
        );
        serde_json::to_writer(&mut writer, &info)?;
        writer.flush()?;
        Ok(info)
    }

    pub fn verify_file(&self, path: &Path) -> Result<()> {
        let metadata = fs::metadata(path)?;
        if fs::canonicalize(path)?.display().to_string() != self.path
            || metadata.len() != self.file_bytes
            || metadata.modified()?.duration_since(UNIX_EPOCH)?.as_nanos() != self.modified_nanos
        {
            return Err("reference differs from indexed source; rebuild the index".into());
        }
        Ok(())
    }
}

#[derive(Deserialize)]
struct IndexedRecord {
    header: String,
    start: usize,
    length: usize,
    global_id: usize,
}
#[derive(Deserialize)]
struct Shard {
    file: String,
    records: Vec<IndexedRecord>,
}
#[derive(Deserialize)]
struct Manifest {
    #[serde(default)]
    reverse_records: bool,
    reference_bases: usize,
    reference_records: usize,
    shards: Vec<Shard>,
    source_sha256: String,
}

#[derive(Deserialize)]
struct Annotation {
    id: String,
    genes: Vec<String>,
    #[serde(default)]
    transcripts: Vec<String>,
    contig: String,
    strand: String,
    blocks: Vec<Block>,
}

fn decode_annotation(bytes: &[u8], source: &FastaRecord) -> Result<Record> {
    let a: Annotation = serde_json::from_slice(bytes)?;
    let r = Record {
        id: a.id,
        sequence: String::new(),
        genes: a.genes,
        transcripts: a.transcripts,
        contig: a.contig,
        strand: a.strand,
        blocks: a.blocks,
    };
    let (id, genes) = source
        .header
        .split_once('|')
        .ok_or("FASTA header lacks genes")?;
    if r.id != id
        || r.genes.iter().map(String::as_str).collect::<BTreeSet<_>>() != genes.split(',').collect()
    {
        return Err("annotation IDs/genes differ from indexed FASTA; order must match".into());
    }
    r.validate_metadata(source.length)?;
    Ok(r)
}

/// Validated offsets let later runs decode only annotations for emitted sites.
#[derive(Serialize, Deserialize)]
pub struct AnnotationCache {
    format: String,
    path: String,
    file_bytes: u64,
    modified_nanos: u128,
    sha256: String,
    reference_sha256: String,
    offsets: Vec<(usize, usize)>,
}

impl AnnotationCache {
    pub fn create(
        directory: &Path,
        reference: &Path,
        annotations: &Path,
        output: &Path,
    ) -> Result<Self> {
        let info: ReferenceInfo = serde_json::from_reader(BufReader::new(File::open(
            directory.join("reference-info.json"),
        )?))?;
        info.verify_file(reference)?;
        let file = File::open(annotations)?;
        let metadata = file.metadata()?;
        let mut cache = Self {
            format: "ooff-annotation-offsets-v1".into(),
            path: fs::canonicalize(annotations)?.display().to_string(),
            file_bytes: metadata.len(),
            modified_nanos: metadata.modified()?.duration_since(UNIX_EPOCH)?.as_nanos(),
            sha256: String::new(),
            reference_sha256: info.sha256.clone(),
            offsets: Vec::new(),
        };
        let mut reader = BufReader::new(file);
        let (mut position, mut line, mut ids) = (0, Vec::new(), BTreeSet::new());
        let mut hash = Sha256::new();
        loop {
            line.clear();
            let bytes = reader.read_until(b'\n', &mut line)?;
            if bytes == 0 {
                break;
            }
            let source = info
                .records
                .get(cache.offsets.len())
                .ok_or("too many annotation records")?;
            let r = decode_annotation(&line, source)?;
            if !ids.insert(r.id) {
                return Err("duplicate annotation record ID".into());
            }
            hash.update(&line);
            cache.offsets.push((position, position + bytes));
            position += bytes;
        }
        if cache.offsets.len() != info.records.len() {
            return Err("missing annotation records".into());
        }
        cache.sha256 = format!("{:x}", hash.finalize());
        cache.verify(annotations)?;
        info.verify_file(reference)?;
        let mut out = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(output)?,
        );
        serde_json::to_writer(&mut out, &cache)?;
        out.flush()?;
        Ok(cache)
    }

    fn verify(&self, annotations: &Path) -> Result<()> {
        if self.format != "ooff-annotation-offsets-v1"
            || self.offsets.first().is_none_or(|r| r.0 != 0)
            || self
                .offsets
                .last()
                .is_none_or(|r| r.1 as u64 != self.file_bytes)
            || self.offsets.iter().any(|&(a, b)| a >= b)
            || self.offsets.windows(2).any(|w| w[0].1 != w[1].0)
        {
            return Err("invalid annotation cache format or offsets".into());
        }
        let metadata = fs::metadata(annotations)?;
        if self.path != fs::canonicalize(annotations)?.display().to_string()
            || self.file_bytes != metadata.len()
            || self.modified_nanos != metadata.modified()?.duration_since(UNIX_EPOCH)?.as_nanos()
        {
            return Err("annotations differ from validated cache; rebuild annotation cache".into());
        }
        Ok(())
    }
}

pub struct ReferenceIndex {
    manifest: Manifest,
    indexes: Vec<Index>,
    record_lookups: Vec<crate::fm::RecordLookup>,
    reverse_indexes: Option<Vec<Index>>,
    source: File,
    sequence: OnceLock<std::io::Result<memmap2::Mmap>>,
    records: Vec<Record>,
    annotation_cache: Option<(AnnotationCache, memmap2::Mmap)>,
    cached_records: Vec<OnceLock<Box<Record>>>,
    pub info: ReferenceInfo,
}

impl ReferenceIndex {
    /// # Safety
    /// Index and source files must remain immutable for this object's lifetime.
    pub unsafe fn open_immutable(
        directory: &Path,
        reference: &Path,
        annotations: &Path,
    ) -> Result<Self> {
        // SAFETY: forward the caller's immutable-artifact contract.
        unsafe { Self::open_cached_immutable(directory, reference, annotations, None) }
    }

    /// # Safety
    /// Reference, annotations, cache and index files must remain immutable for
    /// this object's lifetime. The cache must be produced by AnnotationCache.
    pub unsafe fn open_cached_immutable(
        directory: &Path,
        reference: &Path,
        annotations: &Path,
        cache_path: Option<&Path>,
    ) -> Result<Self> {
        let manifest: Manifest =
            serde_json::from_reader(BufReader::new(File::open(directory.join("manifest.json"))?))?;
        if manifest.reverse_records {
            return Err("reversed experimental index requires paired-direction search".into());
        }
        let info: ReferenceInfo = serde_json::from_reader(BufReader::new(File::open(
            directory.join("reference-info.json"),
        )?))?;
        info.verify_file(reference)?;
        if manifest.source_sha256 != info.sha256
            || manifest.reference_records != info.records.len()
            || manifest.reference_bases != info.records.iter().map(|r| r.length).sum::<usize>()
        {
            return Err("index/reference provenance mismatch".into());
        }
        let (records, annotation_cache, cached_records) = if let Some(path) = cache_path {
            let cache: AnnotationCache =
                serde_json::from_reader(BufReader::new(File::open(path)?))?;
            cache.verify(annotations)?;
            if cache.reference_sha256 != info.sha256 || cache.offsets.len() != info.records.len() {
                return Err("annotation cache/reference mismatch".into());
            }
            let file = File::open(annotations)?;
            // SAFETY: the open API requires immutable annotation artifacts too.
            let map = unsafe { memmap2::MmapOptions::new().map(&file) }?;
            let slots = (0..info.records.len()).map(|_| OnceLock::new()).collect();
            (Vec::new(), Some((cache, map)), slots)
        } else {
            let mut records = Vec::with_capacity(manifest.reference_records);
            let mut ids = BTreeSet::new();
            for line in BufReader::new(File::open(annotations)?).lines() {
                let source = info
                    .records
                    .get(records.len())
                    .ok_or("too many annotation records")?;
                let r = decode_annotation(line?.as_bytes(), source)?;
                if !ids.insert(r.id.clone()) {
                    return Err("duplicate annotation record ID".into());
                }
                records.push(r);
            }
            if records.len() != manifest.reference_records {
                return Err("missing annotation records".into());
            }
            (records, None, Vec::new())
        };
        for (i, r) in manifest.shards.iter().flat_map(|s| &s.records).enumerate() {
            if r.global_id != i
                || r.header != info.records[i].header
                || r.length != info.records[i].length
            {
                return Err("inconsistent index record metadata".into());
            }
        }
        let mut indexes = Vec::new();
        for s in &manifest.shards {
            let file = File::open(directory.join(&s.file))?;
            // SAFETY: index files are immutable artifacts; this API's documented
            // contract excludes concurrent writes/truncation by any process.
            indexes.push(unsafe { Index::map_immutable(&file) }?);
        }
        let record_lookups = manifest
            .shards
            .iter()
            .zip(&indexes)
            .map(|(shard, index)| {
                crate::fm::RecordLookup::new(
                    shard.records.iter().map(|r| r.start).collect(),
                    index.len(),
                )
            })
            .collect();
        let source = File::open(reference)?;
        Ok(Self {
            manifest,
            indexes,
            record_lookups,
            reverse_indexes: None,
            source,
            sequence: OnceLock::new(),
            records,
            annotation_cache,
            cached_records,
            info,
        })
    }

    pub fn record_count(&self) -> usize {
        self.manifest.reference_records
    }

    pub fn cached_annotation_sha256(&self) -> Option<&str> {
        self.annotation_cache
            .as_ref()
            .map(|(cache, _)| cache.sha256.as_str())
    }

    fn record(&self, rid: usize) -> Result<&Record> {
        if let Some((cache, bytes)) = &self.annotation_cache {
            let slot = &self.cached_records[rid];
            if slot.get().is_none() {
                let (start, end) = cache.offsets[rid];
                let line = bytes
                    .get(start..end)
                    .ok_or("invalid annotation cache offset")?;
                let r = decode_annotation(line, &self.info.records[rid])?;
                let _ = slot.set(Box::new(r));
            }
            Ok(slot.get().unwrap())
        } else {
            Ok(&self.records[rid])
        }
    }

    pub fn reference_bases(&self) -> usize {
        self.manifest.reference_bases
    }

    fn interval(&self, rid: usize, start: usize, end: usize) -> Result<Vec<u8>> {
        let r = &self.info.records[rid];
        if start >= end || end > r.length {
            return Err("indexed hit outside reference record".into());
        }
        // Counts never read reference sequence. Map it only when a report or
        // witness needs verification, avoiding an unused full-FASTA address map.
        let sequence = self
            .sequence
            .get_or_init(|| {
                // SAFETY: open_immutable requires the source to remain immutable.
                unsafe { memmap2::MmapOptions::new().map(&self.source) }
            })
            .as_ref()
            .map_err(|error| std::io::Error::new(error.kind(), error.to_string()))?;
        let bytes: Vec<_> = (start..end)
            .map(|p| sequence[r.offset + (p / r.line_bases) * r.line_bytes + p % r.line_bases])
            .collect();
        normalize(std::str::from_utf8(&bytes)?, false).map_err(Into::into)
    }

    /// Stream distinct sites one shard at a time. Report-mode memory is bounded
    /// by sites in a shard for one query; an explicit cap stops discovery.
    /// Attach an optional reversed-record index for exhaustive paired search.
    ///
    /// # Safety
    /// All index files must remain immutable for this object's lifetime.
    pub unsafe fn attach_reverse_immutable(&mut self, directory: &Path) -> Result<()> {
        let other: Manifest =
            serde_json::from_reader(BufReader::new(File::open(directory.join("manifest.json"))?))?;
        if !other.reverse_records
            || other.source_sha256 != self.manifest.source_sha256
            || other.shards.len() != self.manifest.shards.len()
        {
            return Err("reverse index provenance mismatch".into());
        }
        let mut indexes = Vec::new();
        for (a, b) in other.shards.iter().zip(&self.manifest.shards) {
            if a.records.len() != b.records.len() {
                return Err("reverse shard mismatch".into());
            }
            for (x, y) in a.records.iter().zip(&b.records) {
                if (&x.header, x.start, x.length, x.global_id)
                    != (&y.header, y.start, y.length, y.global_id)
                {
                    return Err("reverse record mismatch".into());
                }
            }
            let file = File::open(directory.join(&a.file))?;
            // SAFETY: caller guarantees immutable reverse-index artifacts.
            indexes.push(unsafe { Index::map_immutable(&file) }?);
        }
        self.reverse_indexes = Some(indexes);
        Ok(())
    }

    /// Count exact edit-distance bins without traceback, reference rereads or site output.
    /// Genomic deduplication holds only this query's distinct genomic sites.
    pub fn count(
        &self,
        pattern: &[u8],
        intended: &[String],
        k: usize,
        counts: &mut crate::summary::Counts,
        cap: Option<u64>,
    ) -> Result<bool> {
        if k > 3 {
            return Err("count supports at most three edits".into());
        }
        for (shard_no, (shard, index)) in self.manifest.shards.iter().zip(&self.indexes).enumerate()
        {
            let mut error = None;
            let stopped = index.search_unique(
                pattern,
                k,
                self.reverse_indexes.as_ref().map(|r| &r[shard_no]),
                |a, b, d| {
                    let Some(record_no) = self.record_lookups[shard_no].preceding(a) else {
                        return false;
                    };
                    let r = &shard.records[record_no];
                    assert!(b <= r.start + r.length, "index crossed a record boundary");
                    if r.header
                        .split_once('|')
                        .unwrap()
                        .1
                        .split(',')
                        .all(|g| intended.iter().any(|i| i == g))
                    {
                        return false;
                    }
                    if counts.needs_annotation() {
                        match self.record(r.global_id) {
                            Ok(record) => counts.add(record, intended, a - r.start, b - r.start, d),
                            Err(e) => {
                                error = Some(e);
                                return true;
                            }
                        }
                    } else {
                        counts.add_interval(d);
                    }
                    cap.is_some_and(|limit| counts.total() >= limit)
                },
            );
            if let Some(error) = error {
                return Err(error);
            }
            if stopped {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn search(
        &self,
        pattern: &[u8],
        intended: &[String],
        k: usize,
        screen: bool,
        cap: Option<usize>,
        mut emit: impl FnMut(&Record, Site) -> Result<()>,
    ) -> Result<(usize, bool)> {
        let mut count = 0;
        for (shard_no, (shard, index)) in self.manifest.shards.iter().zip(&self.indexes).enumerate()
        {
            if !screen && cap.is_none() {
                let mut error = None;
                index.search_unique(
                    pattern,
                    k,
                    self.reverse_indexes.as_ref().map(|r| &r[shard_no]),
                    |a, b, _| {
                        let Some(record_no) = self.record_lookups[shard_no].preceding(a) else {
                            return false;
                        };
                        let r = &shard.records[record_no];
                        assert!(b <= r.start + r.length, "index crossed a record boundary");
                        if r.header
                            .split_once('|')
                            .unwrap()
                            .1
                            .split(',')
                            .all(|g| intended.iter().any(|i| i == g))
                        {
                            return false;
                        }
                        let result = (|| -> Result<()> {
                            let a = a - r.start;
                            let b = b - r.start;
                            let target = self.interval(r.global_id, a, b)?;
                            let site = align_site(pattern, &target, a);
                            if target.contains(&b'N') || site.edit_distance > k {
                                return Err(
                                    "indexed witness failed independent interval verification"
                                        .into(),
                                );
                            }
                            emit(self.record(r.global_id)?, site)?;
                            count += 1;
                            Ok(())
                        })();
                        if let Err(e) = result {
                            error = Some(e);
                            true
                        } else {
                            false
                        }
                    },
                );
                if let Some(error) = error {
                    return Err(error);
                }
                continue;
            }
            let mut sites = crate::fm::SiteSet::default();
            let mut collect = |a: usize, b: usize, d: usize, reverse: bool| {
                let Some(record_no) = self.record_lookups[shard_no].preceding(a) else {
                    return false;
                };
                let r = &shard.records[record_no];
                assert!(b <= r.start + r.length, "index crossed a record boundary");
                if r.header
                    .split_once('|')
                    .unwrap()
                    .1
                    .split(',')
                    .all(|g| intended.iter().any(|i| i == g))
                {
                    return false;
                }
                let (a, b) = if reverse {
                    (
                        r.start + r.length - (b - r.start),
                        r.start + r.length - (a - r.start),
                    )
                } else {
                    (a, b)
                };
                sites.insert(record_no, a, b, d);
                screen || cap.is_some_and(|n| count + sites.len() >= n)
            };
            let stopped = if let Some(reverse) = &self.reverse_indexes {
                let first =
                    index.search_limited(pattern, k, true, |a, b, d| collect(a, b, d, false));
                if first {
                    true
                } else {
                    let reversed: Vec<_> = pattern.iter().rev().copied().collect();
                    reverse[shard_no]
                        .search_limited(&reversed, k, true, |a, b, d| collect(a, b, d, true))
                }
            } else {
                index.search(pattern, k, |a, b, d| collect(a, b, d, false))
            };
            for (record_no, a, b, _) in sites.into_sites() {
                let record = &shard.records[record_no];
                let (rid, a, b) = (record.global_id, a - record.start, b - record.start);
                let target = self.interval(rid, a, b)?;
                let site = align_site(pattern, &target, a);
                if target.contains(&b'N') || site.edit_distance > k {
                    return Err("indexed witness failed independent interval verification".into());
                }
                emit(self.record(rid)?, site)?;
                count += 1;
            }
            if stopped {
                return Ok((count, !screen));
            }
        }
        Ok((count, false))
    }
}
