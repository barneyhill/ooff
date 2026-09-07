use clap::{Parser, Subcommand};
use ooff::fm::Index;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
    sync::Mutex,
    time::Instant,
};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Action,
}
#[derive(Subcommand)]
enum Action {
    /// Validate annotations once and save offsets for lazy, checked loading.
    CacheAnnotations {
        #[arg(long)]
        index: PathBuf,
        #[arg(long)]
        reference: PathBuf,
        #[arg(long)]
        annotations: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Build {
        #[arg(long)]
        reference: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value_t = 200_000_000)]
        shard_bases: usize,
        #[arg(long)]
        reverse_records: bool,
    },
    Search {
        #[arg(long)]
        index: PathBuf,
        // Accepted to keep paired runner commands identical; checked against manifest.
        #[arg(long)]
        reference: PathBuf,
        #[arg(long)]
        queries: PathBuf,
        #[arg(short = 'k', default_value_t = 3)]
        k: usize,
        #[arg(long,default_value="screen",value_parser=["screen","endpoints","sites"])]
        mode: String,
        #[arg(long, default_value_t = 1000)]
        queries_limit: usize,
        #[arg(long, default_value_t = 3)]
        repetitions: usize,
        /// Parallel query workers sharing immutable indexes.
        #[arg(long, default_value_t = 1)]
        threads: usize,
        #[arg(long, default_value = "ENSG00000136531")]
        intended_gene: String,
        #[arg(long)]
        distance_strata: bool,
        /// Index files must remain immutable throughout the run.
        #[arg(long)]
        mmap: bool,
        #[arg(long)]
        automaton: bool,
        /// Experiment: cache exact 10-mer ranges during full-site range union.
        #[arg(long)]
        word_cache: bool,
        #[arg(long)]
        reverse_index: Option<PathBuf>,
        #[arg(long)]
        site_tuples: Option<PathBuf>,
    },
}

#[derive(Clone, Default)]
struct QueryRun {
    count: u64,
    signature: u64,
    witness_cost: Option<usize>,
    witness_interval: Option<[usize; 4]>,
    seconds: f64,
}

#[derive(Serialize, Deserialize)]
struct Record {
    header: String,
    start: usize,
    length: usize,
    global_id: usize,
}
#[derive(Serialize, Deserialize)]
struct Shard {
    file: String,
    records: Vec<Record>,
    text_bases: usize,
    index_bytes: usize,
    build_seconds: f64,
}
#[derive(Serialize, Deserialize)]
struct Manifest {
    #[serde(default)]
    reverse_records: bool,
    format: String,
    reference: String,
    reference_bases: usize,
    reference_records: usize,
    shards: Vec<Shard>,
    preparation_seconds: f64,
    #[serde(default)]
    source_sha256: String,
}

fn fasta(path: &Path, mut consume: impl FnMut(String, Vec<u8>)) {
    let mut current = None;
    let mut sequence = Vec::new();
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        if let Some(header) = line.strip_prefix('>') {
            if let Some(header) = current.take() {
                consume(header, std::mem::take(&mut sequence));
            }
            current = Some(header.to_string());
        } else {
            assert!(current.is_some());
            sequence.extend(ooff::normalize(line.trim(), false).unwrap());
        }
    }
    if let Some(header) = current {
        consume(header, sequence);
    }
}

fn save_shard(output: &Path, text: &[u8], records: Vec<Record>, number: usize) -> Shard {
    let start = Instant::now();
    let index = Index::build(text);
    let file = format!("shard-{number:04}.fm");
    let mut out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(output.join(&file))
            .unwrap(),
    );
    index.write(&mut out).unwrap();
    out.flush().unwrap();
    let shard = Shard {
        file,
        records,
        text_bases: text.len(),
        index_bytes: index.bytes(),
        build_seconds: start.elapsed().as_secs_f64(),
    };
    eprintln!(
        "{}",
        json!({"shard":number,"bases":text.len(),"bytes":shard.index_bytes,"seconds":shard.build_seconds})
    );
    shard
}

fn main() {
    match Args::parse().command {
        Action::CacheAnnotations {
            index,
            reference,
            annotations,
            output,
        } => {
            let start = Instant::now();
            ooff::indexed::AnnotationCache::create(&index, &reference, &annotations, &output)
                .unwrap();
            println!(
                "{}",
                json!({"complete":true,"seconds":start.elapsed().as_secs_f64(),"cache_bytes":fs::metadata(output).unwrap().len()})
            );
        }
        Action::Build {
            reference,
            output,
            shard_bases,
            reverse_records,
        } => {
            assert!(shard_bases > 0 && shard_bases < i32::MAX as usize - 1);
            fs::create_dir(&output).expect("index directory must not already exist");
            let start = Instant::now();
            let source_info = ooff::indexed::ReferenceInfo::create(
                &reference,
                &output.join("reference-info.json"),
            )
            .unwrap();
            let mut manifest = Manifest {
                reverse_records,
                format: "ooff-fm-v1".into(),
                reference: fs::canonicalize(&reference).unwrap().display().to_string(),
                reference_bases: 0,
                reference_records: 0,
                shards: Vec::new(),
                preparation_seconds: 0.0,
                source_sha256: source_info.sha256.clone(),
            };
            let mut text = Vec::new();
            let mut records = Vec::new();
            fasta(&reference, |header, mut seq| {
                if reverse_records {
                    seq.reverse();
                }
                assert!(
                    header.contains('|'),
                    "reference records require gene associations"
                );
                assert!(seq.len() < i32::MAX as usize - 1);
                if !text.is_empty() && text.len() + seq.len() + 1 > shard_bases {
                    manifest.shards.push(save_shard(
                        &output,
                        &text,
                        std::mem::take(&mut records),
                        manifest.shards.len(),
                    ));
                    text.clear();
                }
                records.push(Record {
                    header,
                    start: text.len(),
                    length: seq.len(),
                    global_id: manifest.reference_records,
                });
                manifest.reference_bases += seq.len();
                manifest.reference_records += 1;
                text.extend(seq);
                text.push(b'N');
            });
            if !text.is_empty() {
                manifest
                    .shards
                    .push(save_shard(&output, &text, records, manifest.shards.len()));
            }
            manifest.preparation_seconds = start.elapsed().as_secs_f64();
            source_info.verify_file(&reference).unwrap();
            serde_json::to_writer(
                BufWriter::new(
                    OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(output.join("manifest.json"))
                        .unwrap(),
                ),
                &manifest,
            )
            .unwrap();
            println!(
                "{}",
                json!({"complete":true,"shards":manifest.shards.len(),"seconds":manifest.preparation_seconds,"index_bytes":manifest.shards.iter().map(|s|s.index_bytes).sum::<usize>()})
            );
        }
        Action::Search {
            index,
            reference,
            queries,
            k,
            mode,
            queries_limit,
            repetitions,
            threads,
            intended_gene,
            distance_strata,
            mmap,
            automaton,
            word_cache,
            reverse_index,
            site_tuples,
        } => {
            assert!(k <= 3 && repetitions > 0 && threads > 0);
            assert!(!distance_strata || mode == "screen");
            assert!(!word_cache || (mode == "sites" && reverse_index.is_some() && !automaton));
            let start = Instant::now();
            let manifest: Manifest = serde_json::from_reader(BufReader::new(
                File::open(index.join("manifest.json")).unwrap(),
            ))
            .unwrap();
            assert_eq!(
                manifest.reference,
                fs::canonicalize(&reference).unwrap().display().to_string()
            );
            assert!(
                !manifest.reverse_records,
                "use forward index as primary index"
            );
            let mut patterns = Vec::new();
            let mut query_ids = Vec::new();
            fasta(&queries, |id, sequence| {
                if patterns.len() < queries_limit {
                    assert!(sequence.len() == 20 && !sequence.contains(&b'N'));
                    query_ids.push(id);
                    patterns.push(ooff::reverse_complement(&sequence));
                }
            });
            assert!(!patterns.is_empty());
            let intended: Vec<Vec<String>> = query_ids
                .iter()
                .map(|id| {
                    id.split_once('|')
                        .map(|(_, g)| g)
                        .unwrap_or(&intended_gene)
                        .split(',')
                        .map(str::to_owned)
                        .collect()
                })
                .collect();
            let indexes: Vec<_> = manifest
                .shards
                .iter()
                .map(|s| {
                    let file = File::open(index.join(&s.file)).unwrap();
                    if mmap {
                        // SAFETY: benchmark indexes are created exclusively in new
                        // directories and never modified during a search run.
                        unsafe { Index::map_immutable(&file) }.unwrap()
                    } else {
                        Index::read(BufReader::new(file)).unwrap()
                    }
                })
                .collect();
            let record_lookups: Vec<_> = manifest
                .shards
                .iter()
                .zip(&indexes)
                .map(|(shard, index)| {
                    ooff::fm::RecordLookup::new(
                        shard.records.iter().map(|r| r.start).collect(),
                        index.len(),
                    )
                })
                .collect();
            let mut reverse_build_seconds = 0.0;
            let reverse_indexes: Option<Vec<Index>> = reverse_index.as_ref().map(|directory| {
                let other: Manifest = serde_json::from_reader(BufReader::new(
                    File::open(directory.join("manifest.json")).unwrap(),
                ))
                .unwrap();
                reverse_build_seconds = other.preparation_seconds;
                assert!(other.reverse_records);
                assert_eq!(other.source_sha256, manifest.source_sha256);
                assert_eq!(other.shards.len(), manifest.shards.len());
                for (a, b) in other.shards.iter().zip(&manifest.shards) {
                    assert_eq!(a.records.len(), b.records.len());
                    for (x, y) in a.records.iter().zip(&b.records) {
                        assert_eq!(
                            (&x.header, x.start, x.length, x.global_id),
                            (&y.header, y.start, y.length, y.global_id)
                        );
                    }
                }
                other
                    .shards
                    .iter()
                    .map(|shard| {
                        let file = File::open(directory.join(&shard.file)).unwrap();
                        if mmap {
                            // SAFETY: benchmark inputs are immutable for the process lifetime.
                            unsafe { Index::map_immutable(&file) }.unwrap()
                        } else {
                            Index::read(BufReader::new(file)).unwrap()
                        }
                    })
                    .collect()
            });
            assert!(!(automaton && reverse_indexes.is_some()));
            let load_seconds = start.elapsed().as_secs_f64();
            let mut tuples = site_tuples.as_ref().map(|p| {
                Mutex::new(BufWriter::new(
                    OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(p)
                        .unwrap(),
                ))
            });
            let mut runs = Vec::new();
            for repetition in 0..repetitions {
                let start = Instant::now();
                let mut results = vec![QueryRun::default(); patterns.len()];
                let budgets: Vec<_> = if distance_strata {
                    (0..=k).collect()
                } else {
                    vec![k]
                };
                for budget in budgets {
                    for (shard_no, (shard, fm)) in manifest.shards.iter().zip(&indexes).enumerate()
                    {
                        let chunk_size = patterns.len().div_ceil(threads);
                        let patterns = &patterns;
                        let intended = &intended;
                        let mode = mode.as_str();
                        let reverse_indexes = &reverse_indexes;
                        let record_lookups = &record_lookups;
                        let tuples = &tuples;
                        std::thread::scope(|scope| {
                            for (chunk_no, chunk) in results.chunks_mut(chunk_size).enumerate() {
                                let mut worker = move || {
                                    for (offset, result) in chunk.iter_mut().enumerate() {
                                        let q = chunk_no * chunk_size + offset;
                                        let pattern = &patterns[q];
                                        if mode == "screen" && result.count > 0 {
                                            continue;
                                        }
                                        let query_started = (mode == "sites").then(Instant::now);
                                        if mode == "sites" && !automaton && !distance_strata {
                                            fm.search_unique_cached(
                                                pattern,
                                                budget,
                                                reverse_indexes.as_ref().map(|r| &r[shard_no]),
                                                word_cache,
                                                |a, b, d| {
                                                    let Some(record_no) =
                                                        record_lookups[shard_no].preceding(a)
                                                    else {
                                                        return false;
                                                    };
                                                    let record = &shard.records[record_no];
                                                    assert!(
                                                        b <= record.start + record.length,
                                                        "index crossed a record boundary"
                                                    );
                                                    if record
                                                        .header
                                                        .split_once('|')
                                                        .unwrap()
                                                        .1
                                                        .split(',')
                                                        .all(|g| intended[q].iter().any(|i| i == g))
                                                    {
                                                        return false;
                                                    }
                                                    let (rid, a, b) = (
                                                        record.global_id,
                                                        a - record.start,
                                                        b - record.start,
                                                    );
                                                    if let Some(out) = tuples {
                                                        let mut out = out.lock().unwrap();
                                                        writeln!(
                                                            out,
                                                            "{repetition},{q},{rid},{a},{b},{d}"
                                                        )
                                                        .unwrap();
                                                    }
                                                    result.count += 1;
                                                    result.signature =
                                                        result.signature.wrapping_add(
                                                            ((rid as u64) << 40)
                                                                ^ ((a as u64) << 8)
                                                                ^ (((b - a) as u64) << 3)
                                                                ^ d as u64,
                                                        );
                                                    false
                                                },
                                            );
                                            if let Some(started) = query_started {
                                                result.seconds += started.elapsed().as_secs_f64();
                                            }
                                            continue;
                                        }
                                        let mut sites = ooff::fm::SiteSet::default();
                                        let mut emit =
                                            |a: usize, b: usize, d: usize, reverse: bool| {
                                                let Some(record_no) =
                                                    record_lookups[shard_no].preceding(a)
                                                else {
                                                    return false;
                                                };
                                                let record = &shard.records[record_no];
                                                assert!(
                                                    b <= record.start + record.length,
                                                    "index crossed a record boundary"
                                                );
                                                let (a, b) = if reverse {
                                                    (
                                                        record.start + record.length
                                                            - (b - record.start),
                                                        record.start + record.length
                                                            - (a - record.start),
                                                    )
                                                } else {
                                                    (a, b)
                                                };
                                                if record
                                                    .header
                                                    .split_once('|')
                                                    .unwrap()
                                                    .1
                                                    .split(',')
                                                    .all(|g| intended[q].iter().any(|i| i == g))
                                                {
                                                    return false;
                                                }
                                                if mode == "screen" {
                                                    result.count = 1;
                                                    result.witness_cost = Some(d);
                                                    result.witness_interval = Some([
                                                        record.global_id,
                                                        a - record.start,
                                                        b - record.start,
                                                        d,
                                                    ]);
                                                    return true;
                                                }
                                                sites.insert(record_no, a, b, d);
                                                false
                                            };
                                        if let Some(reverse) = &reverse_indexes {
                                            let stopped = fm.search_limited(
                                                pattern,
                                                budget,
                                                true,
                                                |a, b, d| emit(a, b, d, false),
                                            );
                                            if !stopped {
                                                let reversed: Vec<_> =
                                                    pattern.iter().rev().copied().collect();
                                                reverse[shard_no].search_limited(
                                                    &reversed,
                                                    budget,
                                                    true,
                                                    |a, b, d| emit(a, b, d, true),
                                                );
                                            }
                                        } else if automaton {
                                            fm.search_automaton(pattern, budget, |a, b, d| {
                                                emit(a, b, d, false)
                                            });
                                        } else {
                                            fm.search(pattern, budget, |a, b, d| {
                                                emit(a, b, d, false)
                                            });
                                        }
                                        if mode == "endpoints" {
                                            let ends: BTreeSet<_> = sites
                                                .into_sites()
                                                .map(|(r, _, end, _)| {
                                                    let record = &shard.records[r];
                                                    (record.global_id, end - record.start)
                                                })
                                                .collect();
                                            result.count += ends.len() as u64;
                                            for (rid, end) in ends {
                                                result.signature = result.signature.wrapping_add(
                                                    ((rid as u64) << 32) ^ end as u64,
                                                );
                                            }
                                        } else if mode == "sites" {
                                            result.count += sites.len() as u64;
                                            for (r, a, b, d) in sites.into_sites() {
                                                let record = &shard.records[r];
                                                let (rid, a, b) = (
                                                    record.global_id,
                                                    a - record.start,
                                                    b - record.start,
                                                );
                                                if let Some(out) = tuples {
                                                    let mut out = out.lock().unwrap();
                                                    writeln!(
                                                        out,
                                                        "{repetition},{q},{rid},{a},{b},{d}"
                                                    )
                                                    .unwrap();
                                                }
                                                result.signature = result.signature.wrapping_add(
                                                    ((rid as u64) << 40)
                                                        ^ ((a as u64) << 8)
                                                        ^ (((b - a) as u64) << 3)
                                                        ^ d as u64,
                                                );
                                            }
                                        }
                                        if let Some(started) = query_started {
                                            result.seconds += started.elapsed().as_secs_f64();
                                        }
                                    }
                                };
                                if threads == 1 {
                                    worker();
                                } else {
                                    scope.spawn(worker);
                                }
                            }
                        });
                    }
                }
                if let Some(out) = &mut tuples {
                    out.get_mut().unwrap().flush().unwrap();
                }
                let counts: Vec<_> = results.iter().map(|r| r.count).collect();
                let signature: Vec<_> = results.iter().map(|r| r.signature).collect();
                let witness_costs: Vec<_> = results.iter().map(|r| r.witness_cost).collect();
                let witness_intervals: Vec<_> =
                    results.iter().map(|r| r.witness_interval).collect();
                let query_seconds: Vec<_> = results.iter().map(|r| r.seconds).collect();
                runs.push(json!({"repetition":repetition,"search_seconds":start.elapsed().as_secs_f64(),"query_seconds":query_seconds,"counts":counts,"signature":signature,"witness_costs":witness_costs,"witness_intervals":witness_intervals}));
            }
            println!(
                "{}",
                json!({"engine":"ooff-fm-v1","mmap":mmap,"automaton":automaton,"word_cache":word_cache,"paired_directions":reverse_indexes.is_some(),"distance_strata":distance_strata,"mode":mode,"k":k,"threads":threads,"query_ids":query_ids,"reference_bases":manifest.reference_bases,"reference_records":manifest.reference_records,"load_seconds":load_seconds,"index_preparation_seconds":manifest.preparation_seconds + reverse_build_seconds,"index_bytes":manifest.shards.iter().map(|s|s.index_bytes).sum::<usize>() * if reverse_indexes.is_some() {2} else {1},"runs":runs,"complete":true})
            );
        }
    }
}
