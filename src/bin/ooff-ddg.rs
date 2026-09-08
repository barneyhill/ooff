//! Exact OligoAI-compatible orchestration of ViennaRNA, without changing its energy model.
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[path = "ddg/sites.rs"]
mod sites;
#[derive(Clone, Copy, ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
enum EnergyEngine {
    Vienna,
    Rust,
}
#[derive(Clone, Parser)]
#[command(about = "Batched, deduplicated OligoAI-compatible relative hybridization energies")]
struct Args {
    #[arg(long)]
    queries: PathBuf,
    /// Single RNA FASTA for the standalone whole-transcript workflow.
    #[arg(long, required_unless_present = "sites", conflicts_with = "sites")]
    off_target: Option<PathBuf>,
    /// Annotate an existing report JSONL, preserving every site and its coordinates.
    #[arg(long, requires = "reference")]
    sites: Option<PathBuf>,
    /// The same JSONL or FASTA reference used for discovery.
    #[arg(long, requires = "sites")]
    reference: Option<PathBuf>,
    /// Score the best interaction across the full RNA record instead of each hit interval.
    #[arg(long)]
    whole_transcript: bool,
    /// Rows per task; buffers two waves of --threads tasks plus one output wave.
    #[arg(long, default_value_t = 1024)]
    batch_size: usize,
    /// Maximum globally cached site-energy sequence pairs; 0 disables caching.
    #[arg(long, default_value_t = 4_000_000)]
    energy_cache_pairs: usize,
    /// Experimental independent Rust duplex implementation, or ViennaRNA oracle.
    #[arg(long, value_enum, default_value = "vienna")]
    energy_engine: EnergyEngine,
    #[arg(long, default_value = "RNAplex")]
    rnaplex: PathBuf,
    #[arg(long, default_value = "RNAduplex")]
    rnaduplex: PathBuf,
    #[arg(long)]
    work_dir: Option<PathBuf>,
    #[arg(long, default_value_t = 4)]
    threads: usize,
    #[arg(long, default_value_t = 600.0)]
    timeout: f64,
}
#[derive(Deserialize)]
struct Input {
    id: String,
    #[serde(alias = "asoDnaSequence")]
    aso: String,
    #[serde(alias = "targetDnaSequence")]
    target: String,
}
#[derive(Clone, Serialize)]
struct Output {
    id: String,
    ddg: Option<f64>,
    dg_target: Option<f64>,
    dg_other: Option<f64>,
    exact_off_target_match: bool,
    dg_other_position: Option<usize>,
}
fn normalize(s: &str) -> Result<String> {
    normalize_owned(s.to_owned())
}
fn normalize_owned(mut s: String) -> Result<String> {
    if s.trim().len() != s.len() {
        s = s.trim().to_owned();
    }
    let mut bytes = s.into_bytes();
    if bytes.is_empty() {
        return Err("Sequences must contain A/C/G/T/U/N and be nonempty".into());
    }
    for b in &mut bytes {
        *b = b.to_ascii_uppercase();
        if *b == b'T' {
            *b = b'U';
        }
        if !b"ACGUN".contains(b) {
            return Err("Sequences must contain A/C/G/T/U/N and be nonempty".into());
        }
    }
    Ok(String::from_utf8(bytes)?)
}

fn run(binary: &Path, args: &[&str], input: Vec<u8>, deadline: Instant) -> Result<String> {
    let mut child = Command::new(binary)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let writer = std::thread::spawn(move || stdin.write_all(&input));
    let reader = std::thread::spawn(move || {
        let mut b = Vec::new();
        stdout.read_to_end(&mut b).map(|_| b)
    });
    let errors = std::thread::spawn(move || {
        let mut b = Vec::new();
        stderr.read_to_end(&mut b).map(|_| b)
    });
    let mut timed_out = false;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if Instant::now() >= deadline {
            timed_out = true;
            child.kill()?;
            break child.wait()?;
        }
        std::thread::sleep(Duration::from_millis(2));
    };
    let wrote = writer.join().map_err(|_| "stdin writer panicked")?;
    let output = reader.join().map_err(|_| "stdout reader panicked")??;
    let errors = errors.join().map_err(|_| "stderr reader panicked")??;
    if timed_out {
        return Err(format!("{} timed out", binary.display()).into());
    }
    if !status.success() {
        return Err(format!(
            "{} failed: {}",
            binary.display(),
            String::from_utf8_lossy(&errors)
        )
        .into());
    }
    wrote?;
    Ok(String::from_utf8(output)?)
}
// Preserve the two-decimal CLI contract and avoid subtracting binary floats.
fn cents(line: &str) -> Result<i64> {
    let value = line
        .rsplit_once('(')
        .ok_or("Missing duplex energy")?
        .1
        .split(')')
        .next()
        .unwrap()
        .trim()
        .parse::<f64>()?;
    if !value.is_finite() {
        return Err("Nonfinite duplex energy".into());
    }
    Ok((value * 100.0).round() as i64)
}
fn main() -> Result<()> {
    let args = Args::parse();
    if args.threads == 0 || !args.timeout.is_finite() || args.timeout <= 0.0 {
        return Err("threads and timeout must be positive".into());
    }
    if args.whole_transcript && matches!(args.energy_engine, EnergyEngine::Rust) {
        return Err("The experimental Rust kernel is for site scoring; whole-transcript scoring currently requires ViennaRNA".into());
    }
    if args.sites.is_some() {
        return sites::annotate(&args);
    }
    if matches!(args.energy_engine, EnergyEngine::Rust) {
        return Err("The experimental Rust kernel currently supports --sites; whole-transcript scoring uses ViennaRNA".into());
    }
    for output in compute_transcript(&args)? {
        println!("{}", serde_json::to_string(&output)?);
    }
    Ok(())
}

fn compute_transcript(args: &Args) -> Result<Vec<Output>> {
    let started = Instant::now();
    let deadline = started
        .checked_add(Duration::try_from_secs_f64(args.timeout)?)
        .ok_or("Timeout overflow")?;
    let mut queries = Vec::new();
    let mut ids = std::collections::HashSet::new();
    for line in fs::read_to_string(&args.queries)?
        .lines()
        .filter(|l| !l.trim().is_empty())
    {
        let mut q: Input = serde_json::from_str(line)?;
        if !ids.insert(q.id.clone()) {
            return Err("Duplicate query ID".into());
        }
        q.aso = normalize(&q.aso)?;
        q.target = normalize(&q.target)?;
        if q.aso.len() != q.target.len() {
            return Err("ASO and intended target must have the same length".into());
        }
        queries.push(q);
    }
    let reference = fs::read_to_string(args.off_target.as_ref().unwrap())?;
    if reference.lines().filter(|l| l.starts_with('>')).count() > 1 {
        return Err("Supply a single off-target RNA record".into());
    }
    let raw_off = reference
        .lines()
        .filter(|l| !l.starts_with('>'))
        .collect::<String>();
    let off = if raw_off.trim().is_empty() {
        String::new()
    } else {
        normalize(&raw_off)?
    };
    let mut outputs: Vec<_> = queries
        .iter()
        .map(|q| Output {
            id: q.id.clone(),
            ddg: None,
            dg_target: None,
            dg_other: None,
            exact_off_target_match: false,
            dg_other_position: None,
        })
        .collect();
    // Index only requested k-mers; don't allocate a String for every transcript window.
    let mut wanted: HashMap<usize, HashMap<&str, Option<usize>>> = HashMap::new();
    for q in &queries {
        wanted
            .entry(q.target.len())
            .or_default()
            .insert(&q.target, None);
    }
    for (length, targets) in &mut wanted {
        for (i, window) in off.as_bytes().windows(*length).enumerate() {
            if let Some(pos) = targets.get_mut(std::str::from_utf8(window)?)
                && pos.is_none()
            {
                *pos = Some(i + 1);
            }
        }
    }
    let mut pairs = Vec::new();
    let mut pair_ids = HashMap::new();
    let mut asos = Vec::new();
    let mut aso_ids = HashMap::new();
    let mut pending = Vec::new();
    for (i, q) in queries.iter().enumerate() {
        if off.len() < q.aso.len() {
            continue;
        }
        if let Some(pos) = wanted[&q.target.len()][q.target.as_str()] {
            outputs[i].ddg = Some(0.0);
            outputs[i].exact_off_target_match = true;
            outputs[i].dg_other_position = Some(pos);
            continue;
        }
        let pair = (q.aso.clone(), q.target.clone());
        let pair_id = *pair_ids.entry(pair.clone()).or_insert_with(|| {
            let n = pairs.len();
            pairs.push(pair);
            n
        });
        let aso_id = *aso_ids.entry(q.aso.clone()).or_insert_with(|| {
            let n = asos.len();
            asos.push(q.aso.clone());
            n
        });
        pending.push((i, pair_id, aso_id));
    }
    let work = args.work_dir.clone().unwrap_or_else(|| {
        std::env::temp_dir().join(format!(
            "oofft-ddg-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    });
    fs::create_dir(&work)?;
    let target_path = work.join("offtarget.fa");
    fs::write(&target_path, format!(">offtarget\n{off}\n"))?;
    let mut target_energies = Vec::new();
    let mut other_energies = vec![None; asos.len()];
    if !pending.is_empty() {
        let worker_count = args.threads.min(asos.len());
        let mut partitions = vec![Vec::new(); worker_count];
        for i in 0..asos.len() {
            partitions[i % worker_count].push(i);
        }
        let counter = AtomicUsize::new(0);
        let results = std::thread::scope(|scope| -> Result<_> {
            let target_job = scope.spawn(|| -> Result<Vec<i64>> {
                let input = pairs
                    .iter()
                    .map(|(a, t)| format!("{a}\n{t}\n"))
                    .collect::<String>();
                let text = run(&args.rnaduplex, &["--noconv"], input.into_bytes(), deadline)?;
                let values = text
                    .lines()
                    .filter(|l| l.contains('&'))
                    .map(cents)
                    .collect::<Result<Vec<_>>>()?;
                if values.len() != pairs.len() {
                    return Err("RNAduplex result count mismatch".into());
                }
                Ok(values)
            });
            let jobs: Vec<_> = partitions
                .into_iter()
                .map(|indices| {
                    let work = &work;
                    let asos = &asos;
                    let args = &args;
                    let target_path = &target_path;
                    let counter = &counter;
                    scope.spawn(move || -> Result<_> {
                        let part = counter.fetch_add(1, Ordering::Relaxed);
                        let path = work.join(format!("queries-{part}.fa"));
                        fs::write(
                            &path,
                            indices
                                .iter()
                                .map(|&i| format!(">{i}\n{}\n", asos[i]))
                                .collect::<String>(),
                        )?;
                        let text = run(
                            &args.rnaplex,
                            &[
                                "-q",
                                path.to_str().ok_or("Non-UTF8 work path")?,
                                "-t",
                                target_path.to_str().ok_or("Non-UTF8 work path")?,
                            ],
                            Vec::new(),
                            deadline,
                        )?;
                        fs::write(work.join(format!("plex-{part}.out")), &text)?;
                        let assigned: std::collections::HashSet<_> =
                            indices.iter().copied().collect();
                        let mut hits = HashMap::new();
                        let mut current = None;
                        for line in text.lines() {
                            if let Some(header) = line.strip_prefix('>') {
                                if let Ok(i) = header.trim().parse::<usize>() {
                                    if !assigned.contains(&i) {
                                        return Err("Unexpected RNAplex query ID".into());
                                    }
                                    current = Some(i);
                                }
                                continue;
                            }
                            if !line.contains('&') {
                                continue;
                            }
                            let i = current.ok_or("RNAplex result lacks query ID")?;
                            let energy = cents(line)?;
                            let pos = line
                                .split_whitespace()
                                .nth(1)
                                .ok_or("Missing interval")?
                                .split(',')
                                .next()
                                .unwrap()
                                .parse::<usize>()?;
                            if hits.get(&i).is_none_or(|&(e, _)| energy < e) {
                                hits.insert(i, (energy, pos));
                            }
                        }
                        Ok(hits)
                    })
                })
                .collect();
            let energies = target_job
                .join()
                .map_err(|_| "RNAduplex worker panicked")??;
            let hits = jobs
                .into_iter()
                .map(|j| j.join().map_err(|_| "RNAplex worker panicked")?)
                .collect::<Result<Vec<_>>>()?;
            Ok((energies, hits))
        })?;
        target_energies = results.0;
        for hits in results.1 {
            for (i, value) in hits {
                other_energies[i] = Some(value);
            }
        }
    }
    for (i, pair, aso) in &pending {
        let on = target_energies[*pair];
        outputs[*i].dg_target = Some(on as f64 / 100.0);
        if let Some((other, pos)) = other_energies[*aso] {
            outputs[*i].dg_other = Some(other as f64 / 100.0);
            outputs[*i].ddg = Some((other - on) as f64 / 100.0);
            outputs[*i].dg_other_position = Some(pos);
        }
    }
    let manifest = serde_json::json!({"complete":true,"queries":queries.len(),"unique_scan_asos":asos.len(),"unique_on_target_pairs":pairs.len(),"threads":args.threads,"seconds":started.elapsed().as_secs_f64(),"model":"OligoAI-v2 RNAduplex/RNAplex defaults, RNA:RNA, 37C","ddg_sign":"dg_other - dg_target","work_dir":work});
    fs::write(
        work.join("manifest.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;
    eprintln!("{manifest}");
    Ok(outputs)
}
