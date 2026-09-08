//! Streaming annotation of discovered intervals; discovery scope is never changed.
use super::{Args, Result, cents, normalize, run};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

static DUPLEX_NANOS: AtomicU64 = AtomicU64::new(0);
static NATIVE_PAIRS: AtomicU64 = AtomicU64::new(0);
static NATIVE_UNIQUE_PAIRS: AtomicU64 = AtomicU64::new(0);

// Shared for one annotation invocation, so engine/model settings cannot mix.
// Compute outside the lock: simultaneous misses may repeat work, never serialize
// the numerical kernels. A full cache starts a new bounded generation.
#[derive(Default)]
struct EnergyCache {
    values: rustc_hash::FxHashMap<String, rustc_hash::FxHashMap<String, i64>>,
    entries: usize,
    hits: u64,
    misses: u64,
    resets: u64,
}

fn cached_duplex(
    pairs: &[(String, String)],
    args: &Args,
    work: &Path,
    deadline: Instant,
    cache: &Mutex<EnergyCache>,
) -> Result<Vec<i64>> {
    if matches!(args.energy_engine, super::EnergyEngine::Rust) {
        NATIVE_PAIRS.fetch_add(pairs.len() as u64, Ordering::Relaxed);
    }
    if args.energy_cache_pairs == 0 {
        return duplex(pairs, args, work, deadline);
    }
    if Instant::now() >= deadline {
        return Err("Annotation timed out".into());
    }
    let mut results = vec![0; pairs.len()];
    let mut missing = Vec::new();
    let mut positions = Vec::new();
    let mut seen = rustc_hash::FxHashMap::default();
    {
        let mut cache = cache.lock().map_err(|_| "Energy cache poisoned")?;
        for (i, (aso, target)) in pairs.iter().enumerate() {
            if let Some(&energy) = cache.values.get(aso).and_then(|m| m.get(target)) {
                results[i] = energy;
                cache.hits += 1;
            } else {
                cache.misses += 1;
                let index = *seen
                    .entry((aso.as_str(), target.as_str()))
                    .or_insert_with(|| {
                        let index = missing.len();
                        missing.push((aso.clone(), target.clone()));
                        index
                    });
                positions.push((i, index));
            }
        }
    }
    let energies = duplex(&missing, args, work, deadline)?;
    for (i, index) in positions {
        results[i] = energies[index];
    }
    if !missing.is_empty() {
        let mut cache = cache.lock().map_err(|_| "Energy cache poisoned")?;
        for ((aso, target), energy) in missing.into_iter().zip(energies) {
            if cache
                .values
                .get(&aso)
                .is_some_and(|m| m.contains_key(&target))
            {
                continue;
            }
            if cache.entries == args.energy_cache_pairs {
                cache.values.clear();
                cache.entries = 0;
                cache.resets += 1;
            }
            cache.values.entry(aso).or_default().insert(target, energy);
            cache.entries += 1;
        }
    }
    Ok(results)
}

#[derive(Deserialize)]
struct Query {
    id: String,
    #[serde(alias = "aso", alias = "asoDnaSequence")]
    sequence: String,
    #[serde(default, alias = "targetDnaSequence")]
    target: Option<String>,
}
#[derive(Clone)]
struct Design {
    aso: String,
    target: String,
    basis: &'static str,
    energy: Option<i64>,
    shared_energy: Arc<Mutex<Option<i64>>>,
}

#[derive(Clone, Serialize)]
struct EnergyAnnotation {
    scope: &'static str,
    model: &'static str,
    units: &'static str,
    ddg_sign: &'static str,
    on_target_basis: &'static str,
    dg_target: Option<f64>,
    dg_other: Option<f64>,
    engine: &'static str,
    scored_record_interval: (usize, usize),
    coordinates: &'static str,
    alignment_constrained: bool,
    whole_transcript_best_position_1based: Option<u64>,
    whole_transcript_exact_match_shortcut: Option<bool>,
}
#[derive(Clone, Serialize)]
struct ComputedEnergy {
    ddg: Option<f64>,
    energy_annotation: EnergyAnnotation,
}
#[derive(Serialize)]
struct AnnotatedSite<'a> {
    #[serde(flatten)]
    original: &'a Value,
    #[serde(flatten)]
    computed: &'a ComputedEnergy,
}

#[derive(Default)]
struct Present(bool);
impl<'de> Deserialize<'de> for Present {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        let _ = serde::de::IgnoredAny::deserialize(deserializer)?;
        Ok(Self(true))
    }
}
#[derive(Default, Deserialize)]
struct IntervalFields {
    #[serde(default)]
    start: Value,
    #[serde(default)]
    end: Value,
}
#[derive(Deserialize)]
struct ReportFields {
    #[serde(rename = "type", default)]
    kind: Value,
    #[serde(default)]
    query_id: Value,
    #[serde(default)]
    record_id: Value,
    #[serde(default)]
    alignment: Option<IntervalFields>,
    #[serde(default)]
    ddg: Present,
    #[serde(default)]
    energy_annotation: Present,
}

// Preserve the original site object as bytes, parsing only fields needed for
// scoring. Existing energy fields use the full object path so supplied DDG and
// rejection of already annotated reports retain their established semantics.
fn parse_report_row(line: String) -> Result<(Value, Option<String>)> {
    let Ok(fields) = serde_json::from_str::<ReportFields>(&line) else {
        return Ok((serde_json::from_str(&line)?, None));
    };
    if fields.kind == "site" && !fields.ddg.0 && !fields.energy_annotation.0 {
        let interval = fields.alignment.unwrap_or_default();
        let value = json!({"type":"site", "query_id":fields.query_id,
            "record_id":fields.record_id, "alignment":{"start":interval.start,"end":interval.end}});
        Ok((value, Some(line)))
    } else {
        Ok((serde_json::from_str(&line)?, None))
    }
}

fn hash(path: &Path) -> Result<String> {
    let mut input = File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let n = input.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        digest.update(&buffer[..n]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

// Store byte ranges only, so human-scale references need not reside in RAM.
struct Reference {
    file: File,
    ranges: Arc<HashMap<String, (u64, u64)>>,
    single_lines: Arc<HashMap<String, (u64, usize)>>,
    fasta: bool,
}
impl Reference {
    fn open(path: &Path) -> Result<Self> {
        let mut reader = BufReader::new(File::open(path)?);
        let fasta = reader.fill_buf()?.first() == Some(&b'>');
        let mut ranges = HashMap::new();
        let mut current = None;
        let mut single_lines = HashMap::new();
        let mut data_lines = 0usize;
        let mut offset = 0;
        let mut line = String::new();
        loop {
            line.clear();
            let n = reader.read_line(&mut line)?;
            if n == 0 {
                break;
            }
            if fasta {
                if let Some(header) = line.strip_prefix('>') {
                    if let Some((id, start)) = current.take()
                        && ranges.insert(id, (start, offset)).is_some()
                    {
                        return Err("Duplicate reference ID".into());
                    }
                    let id = header.split(['|', ' ', '\t', '\r', '\n']).next().unwrap();
                    current = Some((id.to_owned(), offset + n as u64));
                    data_lines = 0;
                } else if let Some((id, _)) = &current {
                    data_lines += 1;
                    let sequence = line.trim();
                    if data_lines == 1
                        && !sequence.is_empty()
                        && sequence.bytes().all(|b| b"ACGTUNacgtun".contains(&b))
                    {
                        single_lines.insert(
                            id.clone(),
                            (
                                offset + (line.len() - line.trim_start().len()) as u64,
                                sequence.len(),
                            ),
                        );
                    } else {
                        single_lines.remove(id);
                    }
                }
            } else if !line.trim().is_empty() {
                let value: Value = serde_json::from_str(&line)?;
                let id = value["id"].as_str().ok_or("Reference lacks ID")?;
                if ranges
                    .insert(id.to_owned(), (offset, offset + n as u64))
                    .is_some()
                {
                    return Err("Duplicate reference ID".into());
                }
            }
            offset += n as u64;
        }
        if let Some((id, start)) = current
            && ranges.insert(id, (start, offset)).is_some()
        {
            return Err("Duplicate reference ID".into());
        }
        Ok(Self {
            file: File::open(path)?,
            ranges: Arc::new(ranges),
            single_lines: Arc::new(single_lines),
            fasta,
        })
    }
    fn fork(&self, path: &Path) -> Result<Self> {
        // Open independently: File::try_clone would share the seek offset.
        Ok(Self {
            file: File::open(path)?,
            ranges: Arc::clone(&self.ranges),
            single_lines: Arc::clone(&self.single_lines),
            fasta: self.fasta,
        })
    }
    // Single-line FASTA records need only the hit bytes. Wrapped FASTA and
    // JSONL keep the existing record reader; whole-transcript calls always do.
    fn short_interval(&mut self, id: &str, start: usize, end: usize) -> Result<String> {
        let &(offset, length) = self
            .single_lines
            .get(id)
            .ok_or("Missing direct interval layout")?;
        if start >= end || end > length {
            return Err("Site outside reference record".into());
        }
        self.file.seek(SeekFrom::Start(offset + start as u64))?;
        let mut bytes = vec![0; end - start];
        self.file.read_exact(&mut bytes)?;
        normalize(std::str::from_utf8(&bytes)?)
    }
    fn sequence(&mut self, id: &str) -> Result<String> {
        let &(start, end) = self
            .ranges
            .get(id)
            .ok_or("Site record absent from reference")?;
        self.file.seek(SeekFrom::Start(start))?;
        let mut bytes = vec![0; (end - start).try_into()?];
        self.file.read_exact(&mut bytes)?;
        if self.fasta {
            bytes.retain(|b| !b.is_ascii_whitespace());
            Ok(String::from_utf8(bytes)?
                .to_ascii_uppercase()
                .replace('T', "U"))
        } else {
            let value: Value = serde_json::from_slice(&bytes)?;
            Ok(value["sequence"]
                .as_str()
                .ok_or("Reference lacks sequence")?
                .to_ascii_uppercase()
                .replace('T', "U"))
        }
    }
}

fn duplex(
    pairs: &[(String, String)],
    args: &Args,
    work: &Path,
    deadline: Instant,
) -> Result<Vec<i64>> {
    if pairs.is_empty() {
        return Ok(Vec::new());
    }
    let stage_started = Instant::now();
    // Reuse only full sequence-pair matches within this bounded batch. Report
    // rows and coordinates remain distinct; hash collisions use key equality.
    let mut unique = Vec::with_capacity(pairs.len());
    let mut positions = Vec::with_capacity(pairs.len());
    if matches!(args.energy_engine, super::EnergyEngine::Rust) {
        let mut seen = rustc_hash::FxHashMap::default();
        for pair in pairs {
            let index = *seen
                .entry((pair.0.as_str(), pair.1.as_str()))
                .or_insert_with(|| {
                    let index = unique.len();
                    unique.push(pair);
                    index
                });
            positions.push(index);
        }
        NATIVE_UNIQUE_PAIRS.fetch_add(unique.len() as u64, Ordering::Relaxed);
    } else {
        unique.extend(pairs);
        positions.extend(0..pairs.len());
    }
    let workers = args.threads.min(unique.len());
    let chunk_size = unique.len().div_ceil(workers);
    let compute = |i: usize, chunk: &[&(String, String)]| -> Result<Vec<i64>> {
        if matches!(args.energy_engine, super::EnergyEngine::Rust) {
            #[cfg(feature = "energy-batch")]
            {
                let mut workspace = oofft::energy::BatchWorkspace::default();
                let mut values = Vec::with_capacity(chunk.len());
                for group in chunk.chunks(1024) {
                    if Instant::now() >= deadline {
                        return Err("Annotation timed out".into());
                    }
                    let pairs: Vec<_> = group
                        .iter()
                        .map(|(a, t)| (a.as_bytes(), t.as_bytes()))
                        .collect();
                    values.extend(workspace.energies(&pairs)?.into_iter().map(i64::from));
                }
                return Ok(values);
            }
            #[cfg(not(feature = "energy-batch"))]
            {
                let mut workspace = oofft::energy::DuplexWorkspace::default();
                return chunk
                    .iter()
                    .map(|(a, t)| {
                        if Instant::now() >= deadline {
                            return Err("Annotation timed out".into());
                        }
                        workspace
                            .energy(a.as_bytes(), t.as_bytes())
                            .map(i64::from)
                            .map_err(Into::into)
                    })
                    .collect();
            }
        }
        let input = chunk
            .iter()
            .map(|(a, t)| format!("{a}\n{t}\n"))
            .collect::<String>();
        fs::write(work.join(format!("duplex-{i}.input")), &input)?;
        let output = run(&args.rnaduplex, &["--noconv"], input.into_bytes(), deadline)?;
        fs::write(work.join(format!("duplex-{i}.out")), &output)?;
        let values = output
            .lines()
            .filter(|l| l.contains('&'))
            .map(cents)
            .collect::<Result<Vec<_>>>()?;
        if values.len() != chunk.len() {
            return Err("RNAduplex result count mismatch".into());
        }
        Ok(values)
    };
    let results = if workers == 1 {
        vec![compute(0, &unique)?]
    } else {
        std::thread::scope(|scope| {
            unique
                .chunks(chunk_size)
                .enumerate()
                .map(|(i, chunk)| {
                    let compute = &compute;
                    scope.spawn(move || compute(i, chunk))
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|job| job.join().map_err(|_| "Duplex worker panicked")?)
                .collect::<Result<Vec<_>>>()
        })?
    };
    DUPLEX_NANOS.fetch_add(stage_started.elapsed().as_nanos() as u64, Ordering::Relaxed);
    let values: Vec<_> = results.into_iter().flatten().collect();
    Ok(positions.into_iter().map(|i| values[i]).collect())
}

fn transcript(
    designs: &[&Design],
    sequence: &str,
    args: &Args,
    work: &Path,
    deadline: Instant,
) -> Result<Vec<Value>> {
    let queries = work.join("queries.jsonl");
    let target = work.join("target.fa");
    let rows = designs
        .iter()
        .enumerate()
        .map(|(i, q)| {
            json!({"id":i.to_string(), "aso":q.aso, "target":q.target}).to_string() + "\n"
        })
        .collect::<String>();
    fs::write(&queries, rows)?;
    fs::write(&target, format!(">offtarget\n{sequence}\n"))?;
    let remaining = deadline
        .saturating_duration_since(Instant::now())
        .as_secs_f64();
    if remaining <= 0.0 {
        return Err("Annotation timed out".into());
    }
    let options = Args {
        queries,
        off_target: Some(target),
        sites: None,
        reference: None,
        whole_transcript: true,
        energy_engine: args.energy_engine,
        batch_size: args.batch_size,
        energy_cache_pairs: args.energy_cache_pairs,
        rnaplex: args.rnaplex.clone(),
        rnaduplex: args.rnaduplex.clone(),
        threads: args.threads,
        timeout: remaining,
        work_dir: Some(work.join("engine")),
    };
    let values = super::compute_transcript(&options)?
        .into_iter()
        .map(serde_json::to_value)
        .collect::<std::result::Result<Vec<Value>, _>>()?;
    if values.len() != designs.len() {
        return Err("Whole-transcript result count mismatch".into());
    }
    for (i, value) in values.iter().enumerate() {
        if value["id"].as_str().and_then(|s| s.parse::<usize>().ok()) != Some(i) {
            return Err("Whole-transcript result order mismatch".into());
        }
    }
    Ok(values)
}

#[allow(clippy::too_many_arguments)]
fn annotate_batch(
    rows: &mut [Value],
    designs: &mut HashMap<String, Design>,
    reference: &mut Reference,
    args: &Args,
    work: &Path,
    deadline: Instant,
    cache: &Mutex<EnergyCache>,
) -> Result<Vec<Option<ComputedEnergy>>> {
    if !args.whole_transcript {
        let mut seen = rustc_hash::FxHashSet::default();
        let mut pending = Vec::new();
        let mut pairs = Vec::new();
        for row in rows.iter().filter(|row| row["type"] == "site") {
            let id = row["query_id"].as_str().ok_or("Site lacks query ID")?;
            let query = designs.get(id).ok_or("Site query absent from queries")?;
            if query.energy.is_none() && seen.insert(id) {
                pending.push(id);
            }
        }
        // Acquire intended-energy locks in a global order to avoid deadlocks
        // when concurrent batches contain overlapping query sets. Cache once
        // across all workers while retaining vector batches for unseen queries.
        pending.sort_unstable();
        let mut locked = pending
            .iter()
            .map(|&id| {
                Ok((
                    id,
                    designs[id]
                        .shared_energy
                        .lock()
                        .map_err(|_| "Intended energy cache poisoned")?,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let mut missing = Vec::new();
        pairs.clear();
        for (i, (id, energy)) in locked.iter().enumerate() {
            if energy.is_none() {
                let query = &designs[*id];
                missing.push(i);
                pairs.push((query.aso.clone(), query.target.clone()));
            }
        }
        if !pairs.is_empty() {
            let on_work = work.join("on-target");
            fs::create_dir(&on_work)?;
            for (i, energy) in missing
                .into_iter()
                .zip(cached_duplex(&pairs, args, &on_work, deadline, cache)?)
            {
                *locked[i].1 = Some(energy);
            }
        }
        let energies: Vec<_> = locked
            .into_iter()
            .map(|(id, energy)| (id, *energy))
            .collect();
        for (id, energy) in energies {
            designs.get_mut(id).unwrap().energy = energy;
        }
    }

    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, row) in rows.iter().enumerate() {
        if row["type"] == "site" {
            let record = row["record_id"].as_str().ok_or("Site lacks record ID")?;
            groups.entry(record.to_owned()).or_default().push(i);
        }
    }
    let mut computed = vec![None; rows.len()];
    let mut batch_pairs = Vec::new();
    let mut batch_sites = Vec::new();
    for (group, (record, indices)) in groups.into_iter().enumerate() {
        if Instant::now() >= deadline {
            return Err("Annotation timed out".into());
        }
        let direct = !args.whole_transcript && reference.single_lines.contains_key(&record);
        let sequence = if direct {
            String::new()
        } else {
            reference.sequence(&record)?
        };
        let group_work = work.join(group.to_string());
        if args.whole_transcript {
            fs::create_dir(&group_work)?;
        }
        let mut pairs = Vec::new();
        let mut qs = Vec::new();
        let mut intervals = Vec::new();
        for &i in &indices {
            let row = &rows[i];
            let id = row["query_id"].as_str().ok_or("Site lacks query ID")?;
            let q = designs.get(id).ok_or("Site query absent from queries")?;
            let start: usize = row["alignment"]["start"]
                .as_u64()
                .ok_or("Site lacks start")?
                .try_into()?;
            let end: usize = row["alignment"]["end"]
                .as_u64()
                .ok_or("Site lacks end")?
                .try_into()?;
            let site = if direct {
                reference.short_interval(&record, start, end)?
            } else {
                if start >= end || end > sequence.len() {
                    return Err("Site outside reference record".into());
                }
                sequence
                    .get(start..end)
                    .ok_or("Invalid sequence coordinates")?
                    .to_owned()
            };
            if !site.bytes().all(|b| b"ACGU".contains(&b)) {
                return Err("Site contains unknown bases".into());
            }
            pairs.push((q.aso.clone(), normalize(&site)?));
            qs.push(q);
            intervals.push((start, end));
        }
        if !args.whole_transcript {
            batch_pairs.extend(pairs);
            for (j, &i) in indices.iter().enumerate() {
                batch_sites.push((i, qs[j], intervals[j]));
            }
            continue;
        }
        let values = transcript(&qs, &sequence, args, &group_work, deadline)?;
        for (j, &i) in indices.iter().enumerate() {
            let row = rows[i].as_object_mut().ok_or("Site is not an object")?;
            if row.contains_key("energy_annotation") {
                return Err("Report already has computed energy annotations".into());
            }
            if let Some(previous) = row.remove("ddg") {
                row.insert("supplied_ddg".into(), previous);
            }
            computed[i] = Some(ComputedEnergy {
                ddg: values[j]["ddg"].as_f64(),
                energy_annotation: EnergyAnnotation {
                    scope: "whole_transcript",
                    model: "ViennaRNA defaults; RNA:RNA, 37C",
                    units: "kcal/mol",
                    ddg_sign: "dg_other - dg_target",
                    on_target_basis: qs[j].basis,
                    dg_target: values[j]["dg_target"].as_f64(),
                    dg_other: values[j]["dg_other"].as_f64(),
                    engine: "RNAduplex/RNAplex",
                    scored_record_interval: (0, sequence.len()),
                    coordinates: "zero-based half-open, RNA-sense",
                    alignment_constrained: false,
                    whole_transcript_best_position_1based: values[j]["dg_other_position"].as_u64(),
                    whole_transcript_exact_match_shortcut: values[j]["exact_off_target_match"]
                        .as_bool(),
                },
            });
        }
    }
    if !args.whole_transcript {
        let pair_work = work.join("site-pairs");
        fs::create_dir(&pair_work)?;
        let energies = cached_duplex(&batch_pairs, args, &pair_work, deadline, cache)?;
        for ((i, query, interval), other) in batch_sites.into_iter().zip(energies) {
            let row = rows[i].as_object_mut().ok_or("Site is not an object")?;
            if row.contains_key("energy_annotation") {
                return Err("Report already has computed energy annotations".into());
            }
            if let Some(previous) = row.remove("ddg") {
                row.insert("supplied_ddg".into(), previous);
            }
            computed[i] = Some(ComputedEnergy {
                ddg: Some((other - query.energy.expect("Intended energy prepared")) as f64 / 100.0),
                energy_annotation: EnergyAnnotation {
                    scope: "site",
                    model: "ViennaRNA defaults; RNA:RNA, 37C",
                    units: "kcal/mol",
                    ddg_sign: "dg_other - dg_target",
                    on_target_basis: query.basis,
                    dg_target: Some(query.energy.expect("Intended energy prepared") as f64 / 100.0),
                    dg_other: Some(other as f64 / 100.0),
                    engine: "RNAduplex/RNAduplex",
                    scored_record_interval: interval,
                    coordinates: "zero-based half-open, RNA-sense",
                    alignment_constrained: false,
                    whole_transcript_best_position_1based: None,
                    whole_transcript_exact_match_shortcut: None,
                },
            });
        }
    }
    Ok(computed)
}

struct AnnotationWorker {
    cache: Arc<Mutex<EnergyCache>>,
    designs: HashMap<String, Design>,
    reference: Reference,
}
struct AnnotationBatch {
    bytes: Vec<u8>,
    sites: usize,
    complete: bool,
    expected_count: Option<u64>,
    parse_seconds: f64,
    score_seconds: f64,
    serialization_seconds: f64,
}

#[allow(clippy::too_many_arguments)]
fn process_report_batch(
    lines: Vec<String>,
    batch_number: usize,
    worker: &mut AnnotationWorker,
    designs: &HashMap<String, Design>,
    args: &Args,
    work: &Path,
    deadline: Instant,
    query_hash: &str,
    reference_hash: &str,
) -> Result<AnnotationBatch> {
    let parse_started = Instant::now();
    let mut rows = Vec::with_capacity(lines.len());
    let mut original_rows = Vec::with_capacity(lines.len());
    let mut complete = false;
    let mut expected_count = None;
    for (i, line) in lines.into_iter().enumerate() {
        if complete {
            return Err("Unexpected rows after run_complete".into());
        }
        let (mut value, original) = parse_report_row(line)?;
        if batch_number == 0 && i == 0 {
            if value["type"] != "manifest" || value["mode"] != "report" {
                return Err("Annotation requires an ooff report with a manifest".into());
            }
            if value["queries_sha256"] != query_hash || value["reference_sha256"] != reference_hash
            {
                return Err("Queries/reference differ from discovery manifest".into());
            }
            value["energy_annotation"] = json!({"scope":if args.whole_transcript { "whole_transcript" } else { "site" },
                "ddg_sign":"dg_other - dg_target", "candidate_filter":false});
        } else if value["type"] == "manifest" {
            return Err("Duplicate report manifest".into());
        }
        if value["type"] == "run_complete" {
            complete = true;
            expected_count = value["reported_sites"].as_u64();
        }
        if value["type"] == "site" {
            let id = value["query_id"].as_str().ok_or("Site lacks query ID")?;
            if !worker.designs.contains_key(id) {
                let design = designs.get(id).ok_or("Site query absent from queries")?;
                worker.designs.insert(id.to_owned(), design.clone());
            }
        }
        rows.push(value);
        original_rows.push(original);
    }
    let parse_seconds = parse_started.elapsed().as_secs_f64();
    let batch_work = work.join(format!("batch-{batch_number}"));
    fs::create_dir(&batch_work)?;
    let score_started = Instant::now();
    let computed = annotate_batch(
        &mut rows,
        &mut worker.designs,
        &mut worker.reference,
        args,
        &batch_work,
        deadline,
        &worker.cache,
    )?;
    let score_seconds = score_started.elapsed().as_secs_f64();
    let serialization_started = Instant::now();
    let mut bytes = Vec::new();
    let mut energy_buffer = Vec::new();
    let mut sites = 0;
    for ((row, energy), original) in rows.into_iter().zip(computed).zip(original_rows) {
        if let (Some(computed), Some(original)) = (&energy, original) {
            let prefix = original
                .trim_end()
                .strip_suffix('}')
                .ok_or("Site is not an object")?;
            energy_buffer.clear();
            serde_json::to_writer(&mut energy_buffer, computed)?;
            bytes.write_all(prefix.as_bytes())?;
            bytes.write_all(b",")?;
            bytes.write_all(&energy_buffer[1..])?;
            sites += 1;
        } else if let Some(computed) = energy {
            serde_json::to_writer(
                &mut bytes,
                &AnnotatedSite {
                    original: &row,
                    computed: &computed,
                },
            )?;
            sites += 1;
        } else {
            serde_json::to_writer(&mut bytes, &row)?;
        }
        bytes.write_all(b"\n")?;
    }
    Ok(AnnotationBatch {
        bytes,
        sites,
        complete,
        expected_count,
        parse_seconds,
        score_seconds,
        serialization_seconds: serialization_started.elapsed().as_secs_f64(),
    })
}

pub(super) fn annotate(args: &Args) -> Result<()> {
    DUPLEX_NANOS.store(0, Ordering::Relaxed);
    NATIVE_PAIRS.store(0, Ordering::Relaxed);
    NATIVE_UNIQUE_PAIRS.store(0, Ordering::Relaxed);
    if args.batch_size == 0 {
        return Err("batch-size must be positive".into());
    }
    let started = Instant::now();
    let deadline = started
        .checked_add(Duration::try_from_secs_f64(args.timeout)?)
        .ok_or("Timeout overflow")?;
    let work: PathBuf = args.work_dir.clone().unwrap_or_else(|| {
        std::env::temp_dir().join(format!(
            "oofft-ddg-sites-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    });
    fs::create_dir(&work)?;
    let reference_path = args.reference.as_ref().unwrap();
    let report = args.sites.as_ref().unwrap();
    let (query_hash, reference_hash, designs, reference, report_hash) =
        std::thread::scope(|scope| -> Result<_> {
            // Hashing the immutable discovery report is independent of query parsing
            // and reference indexing. Join before accepting its provenance.
            let report_job = (args.threads > 1).then(|| scope.spawn(|| hash(report)));
            let query_hash = hash(&args.queries)?;
            let reference_hash = hash(reference_path)?;
            let mut designs = HashMap::new();
            let queries: Vec<Query> =
                if oofft::inputs::is_fasta(&args.queries).map_err(|e| e.to_string())? {
                    oofft::inputs::queries(&args.queries)
                        .map_err(|e| e.to_string())?
                        .into_iter()
                        .map(|q| Query {
                            id: q.id,
                            sequence: q.sequence,
                            target: q.target,
                        })
                        .collect()
                } else {
                    let mut queries = Vec::new();
                    for line in oofft::inputs::reader(&args.queries)?.lines() {
                        let line = line?;
                        if !line.trim().is_empty() {
                            queries.push(serde_json::from_str(&line)?);
                        }
                    }
                    queries
                };
            for q in queries {
                let aso = super::normalize_owned(q.sequence)?;
                let basis = if q.target.is_some() {
                    "supplied_target"
                } else {
                    "perfect_complement"
                };
                let target = match q.target {
                    Some(t) => super::normalize_owned(t)?,
                    None => aso
                        .bytes()
                        .rev()
                        .map(|b| match b {
                            b'A' => 'U',
                            b'U' => 'A',
                            b'C' => 'G',
                            b'G' => 'C',
                            _ => 'N',
                        })
                        .collect(),
                };
                if target.len() != aso.len() {
                    return Err("Intended target must have ASO length".into());
                }
                if q.id.is_empty()
                    || designs
                        .insert(
                            q.id,
                            Design {
                                aso,
                                target,
                                basis,
                                energy: None,
                                shared_energy: Arc::new(Mutex::new(None)),
                            },
                        )
                        .is_some()
                {
                    return Err("Query IDs must be unique and nonempty".into());
                }
            }
            let reference = Reference::open(reference_path)?;
            let report_hash = if let Some(job) = report_job {
                job.join().map_err(|_| "Report hashing worker panicked")??
            } else {
                hash(report)?
            };
            Ok((query_hash, reference_hash, designs, reference, report_hash))
        })?;
    let output_path = work.join("annotated.jsonl");
    let mut output = BufWriter::new(File::create(&output_path)?);
    // Parallelise complete bounded batches, including parsing and serialization.
    // Keep transcript orchestration's existing inner worker allocation.
    let worker_count = if args.whole_transcript {
        1
    } else {
        args.threads
    };
    let mut worker_args = args.clone();
    if !args.whole_transcript {
        worker_args.threads = 1;
    }
    let cache = Arc::new(Mutex::new(EnergyCache::default()));
    let mut workers = (0..worker_count)
        .map(|_| {
            Ok(AnnotationWorker {
                cache: Arc::clone(&cache),
                designs: HashMap::new(),
                reference: reference.fork(reference_path)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let mut input = BufReader::with_capacity(1024 * 1024, File::open(report)?).lines();
    let (mut batch_number, mut count) = (0usize, 0usize);
    let mut complete_seen = false;
    let mut expected_count = None;
    let mut parse_seconds = 0.0;
    let mut score_seconds = 0.0;
    let mut serialization_seconds = 0.0;
    let setup_seconds = started.elapsed().as_secs_f64();
    let mut input_seconds = 0.0;
    let mut waves_seconds = 0.0;
    let mut spool_seconds = 0.0;
    let mut read_tasks = || -> Result<Vec<Vec<String>>> {
        let input_started = Instant::now();
        let mut tasks = Vec::new();
        for _ in 0..worker_count {
            let mut lines = Vec::new();
            while lines.len() < args.batch_size {
                let Some(line) = input.next() else {
                    break;
                };
                let line = line?;
                if !line.trim().is_empty() {
                    lines.push(line);
                }
            }
            if lines.is_empty() {
                break;
            }
            tasks.push(lines);
        }
        input_seconds += input_started.elapsed().as_secs_f64();
        Ok(tasks)
    };
    let mut consume = |results: Vec<AnnotationBatch>, more: bool| -> Result<()> {
        let spool_started = Instant::now();
        for result in results {
            if complete_seen {
                return Err("Unexpected rows after run_complete".into());
            }
            complete_seen = result.complete;
            if complete_seen {
                expected_count = result.expected_count;
            }
            count += result.sites;
            parse_seconds += result.parse_seconds;
            score_seconds += result.score_seconds;
            serialization_seconds += result.serialization_seconds;
            output.write_all(&result.bytes)?;
        }
        spool_seconds += spool_started.elapsed().as_secs_f64();
        if complete_seen && more {
            return Err("Unexpected rows after run_complete".into());
        }
        Ok(())
    };
    let mut tasks = read_tasks()?;
    let mut pending = None;
    loop {
        if tasks.is_empty() {
            if let Some(results) = pending.take() {
                consume(results, false)?;
            }
            break;
        }
        let task_count = tasks.len();
        let wave_started = Instant::now();
        let (results, next_tasks) = std::thread::scope(|scope| -> Result<_> {
            let jobs = workers
                .iter_mut()
                .zip(tasks)
                .enumerate()
                .map(|(i, (worker, lines))| {
                    let args = &worker_args;
                    let designs = &designs;
                    let work = &work;
                    let query_hash = &query_hash;
                    let reference_hash = &reference_hash;
                    scope.spawn(move || {
                        process_report_batch(
                            lines,
                            batch_number + i,
                            worker,
                            designs,
                            args,
                            work,
                            deadline,
                            query_hash,
                            reference_hash,
                        )
                    })
                })
                .collect::<Vec<_>>();
            // While workers score this wave, emit the previous wave to the
            // private spool and prefetch the next one. Order stays deterministic.
            if let Some(results) = pending.take() {
                consume(results, true)?;
            }
            let next_tasks = read_tasks()?;
            let results = jobs
                .into_iter()
                .map(|job| job.join().map_err(|_| "Annotation worker panicked")?)
                .collect::<Result<Vec<_>>>()?;
            Ok((results, next_tasks))
        })?;
        waves_seconds += wave_started.elapsed().as_secs_f64();
        batch_number += task_count;
        pending = Some(results);
        tasks = next_tasks;
    }
    if !complete_seen || expected_count != Some(count as u64) {
        return Err("Report is truncated or reported site count differs".into());
    }
    if Instant::now() >= deadline {
        return Err("Annotation timed out".into());
    }
    output.flush()?;
    let cache = cache.lock().map_err(|_| "Energy cache poisoned")?;
    let manifest = json!({"complete":true,
        "energy_cache":{"capacity_pairs":args.energy_cache_pairs,"entries":cache.entries,"hits":cache.hits,"misses":cache.misses,"generation_resets":cache.resets},"annotated_sites":count,"seconds":started.elapsed().as_secs_f64(),
        "scope":if args.whole_transcript { "whole_transcript" } else { "site" },
        "queries_sha256":query_hash,"reference_sha256":reference_hash,"report_sha256":report_hash,
        "threads":args.threads,"batch_size":args.batch_size,"work_dir":work,"energy_implementation":args.energy_engine,
        "profile":{"setup_seconds":setup_seconds,"input_seconds":input_seconds,"waves_wall_seconds":waves_seconds,"spool_seconds":spool_seconds,"stage_times":"cumulative worker seconds; stages overlap across workers","native_pairs":NATIVE_PAIRS.load(Ordering::Relaxed),
        "native_unique_pairs":NATIVE_UNIQUE_PAIRS.load(Ordering::Relaxed),
        "duplex_stages_seconds":DUPLEX_NANOS.load(Ordering::Relaxed) as f64/1e9,
        "report_parse_seconds":parse_seconds,"annotation_batches_seconds":score_seconds,"serialization_seconds":serialization_seconds}});
    fs::write(
        work.join("manifest.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;
    std::io::copy(
        &mut BufReader::with_capacity(1024 * 1024, File::open(output_path)?),
        &mut std::io::stdout().lock(),
    )?;
    eprintln!("{manifest}");
    Ok(())
}
