use super::{
    BIOTYPES, Bundle, Current, PRESETS, RELEASE, Result, build_reference, cache_dir, download, key,
    preset, resolve,
};
use crate::{
    fm::Index,
    indexed::{AnnotationCache, ReferenceInfo},
    inputs,
};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
#[command(
    name = "oofft reference",
    bin_name = "oofft reference",
    about = "Download and prepare reusable RNA references"
)]
pub struct Cli {
    #[arg(
        long,
        global = true,
        help = "Reference cache directory (or OOFFT_CACHE_DIR)"
    )]
    cache_dir: Option<PathBuf>,
    #[command(subcommand)]
    command: Action,
}
#[derive(Subcommand)]
enum Action {
    /// List pinned species presets and their local preparation status.
    List,
    /// Download FASTA/GTF, extract RNA records, and build a compact search index.
    Prepare {
        /// hg38, mm39, rn7, cyno, rhesus; or a custom name with --fasta and --gtf.
        name: String,
        /// Custom genome FASTA (.fa/.fasta, optionally gzip).
        #[arg(long, requires = "gtf")]
        fasta: Option<PathBuf>,
        /// Matching custom GTF annotation, optionally gzip.
        #[arg(long, requires = "fasta")]
        gtf: Option<PathBuf>,
        /// Build workers (default: available logical CPUs).
        #[arg(short = 't', long, value_parser = clap::value_parser!(u16).range(1..))]
        threads: Option<u16>,
        /// Approximate reference bases per build shard; records are not split.
        #[arg(long, default_value_t = 200_000_000)]
        shard_bases: usize,
        /// Prepare a new generation, retaining the previous reference and downloads.
        #[arg(long)]
        rebuild: bool,
    },
}
pub(super) fn unique() -> String {
    format!(
        "{}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        std::process::id()
    )
}

pub fn run(cli: Cli) -> Result<()> {
    let cache = cache_dir(cli.cache_dir.as_deref())?;
    match cli.command {
        Action::List => {
            println!("PRESET\tSPECIES\tASSEMBLY\tANNOTATION\tSTATUS");
            for p in PRESETS {
                let status = if resolve(p.name, &cache).is_ok() {
                    "ready"
                } else {
                    "not prepared"
                };
                println!(
                    "{}\t{}\t{} ({})\tEnsembl {}\t{}",
                    p.name, p.label, p.assembly, p.assembly_scope, RELEASE, status
                );
            }
        }
        Action::Prepare {
            name,
            fasta,
            gtf,
            threads,
            shard_bases,
            rebuild,
        } => {
            if shard_bases == 0 || shard_bases >= i32::MAX as usize - 1 {
                return Err("--shard-bases must be between 1 and 2147483645".into());
            }
            let p = preset(&name);
            if p.is_some() && fasta.is_some() {
                return Err("use a custom reference name with --fasta/--gtf; preset names identify pinned Ensembl downloads".into());
            }
            if p.is_none() && fasta.is_none() {
                return Err("unknown preset; run 'oofft reference list', or supply --fasta and --gtf for a custom reference".into());
            }
            let root = cache.join(key(&name)?);
            fs::create_dir_all(&root)?;
            let lock = OpenOptions::new()
                .create(true)
                .truncate(false)
                .read(true)
                .write(true)
                .open(root.join("prepare.lock"))?;
            lock.try_lock().map_err(|error| {
                format!(
                    "cannot lock reference '{name}': {error}; another process may be preparing it"
                )
            })?;
            if root.join("current.json").exists() && !rebuild {
                let (directory, bundle) = resolve(&name, &cache)?;
                if let (Some(fasta), Some(gtf)) = (&fasta, &gtf) {
                    let hashes = [download::local(fasta)?.sha256, download::local(gtf)?.sha256];
                    if bundle
                        .source_files
                        .iter()
                        .map(|s| &s.sha256)
                        .ne(hashes.iter())
                    {
                        return Err("custom inputs changed; use --rebuild to prepare a new reference generation".into());
                    }
                }
                eprintln!("Reference ready: {}", directory.display());
                return Ok(());
            }
            let threads = threads.unwrap_or_else(|| {
                std::thread::available_parallelism()
                    .map_or(1, |n| n.get().min(u16::MAX as usize) as u16)
            });
            let downloads = root.join("downloads");
            fs::create_dir_all(&downloads)?;
            let (fasta, gtf, sources) = if let Some(p) = p {
                let urls = p.urls();
                let dna = download::fetch(&urls[0], &downloads)?;
                let annotation = download::fetch(&urls[1], &downloads)?;
                (
                    downloads.join(urls[0].rsplit('/').next().unwrap()),
                    downloads.join(urls[1].rsplit('/').next().unwrap()),
                    vec![dna, annotation],
                )
            } else {
                let fasta = fasta.unwrap();
                let gtf = gtf.unwrap();
                let sources = vec![download::local(&fasta)?, download::local(&gtf)?];
                (fasta, gtf, sources)
            };
            let directory_name = format!("build-{}", unique());
            let directory = root.join(&directory_name);
            fs::create_dir(&directory)?;
            let started = Instant::now();
            let stats = build_reference(&fasta, &gtf, &directory)?;
            eprintln!(
                "Building compact index: {} records, {} bases, {threads} workers",
                stats.records, stats.bases
            );
            build_index(
                &directory.join("reference.fa"),
                &directory.join("index"),
                shard_bases,
                threads,
            )?;
            AnnotationCache::create(
                &directory.join("index"),
                &directory.join("reference.fa"),
                &directory.join("records.jsonl"),
                &directory.join("annotation-cache.json"),
            )?;
            // Source identity must remain stable through preparation.
            for (path, source) in [fasta, gtf].iter().zip(&sources) {
                if download::fingerprint(path)?.0 != source.sha256 {
                    return Err("source changed during reference preparation".into());
                }
            }
            let info: ReferenceInfo =
                serde_json::from_reader(File::open(directory.join("index/reference-info.json"))?)?;
            let bundle = Bundle { format: "oofft-reference-v1".into(), name: p.map_or(name.clone(), |p| p.name.into()),
                reference_release: p.map_or_else(|| format!("custom-{name}"), |p| format!("Ensembl{RELEASE}-{}", p.assembly)),
                scope: p.map_or_else(|| "Gene bodies and mature transcripts from supplied FASTA/GTF".into(), |p|
                    format!("{} {} {}; annotated gene bodies including introns and mature transcripts; intergenic DNA excluded", p.label, p.assembly, p.assembly_scope)),
                biotype_policy: BIOTYPES.into(), source_files: sources, reference_sha256: info.sha256,
                genes_sha256: download::fingerprint(&directory.join("genes.json"))?.0,
                annotations_sha256: download::fingerprint(&directory.join("records.jsonl"))?.0,
                records: stats.records, bases: stats.bases, unknown_bases: stats.unknown_bases };
            write_json(&directory.join("bundle.json"), &bundle)?;
            let next = root.join(format!("current-{}.json", unique()));
            write_json(
                &next,
                &Current {
                    directory: directory_name,
                },
            )?;
            fs::rename(next, root.join("current.json"))?;
            eprintln!(
                "Reference '{}' ready in {:.1} s: {}",
                bundle.name,
                started.elapsed().as_secs_f64(),
                directory.display()
            );
            println!(
                "{}",
                serde_json::json!({"complete":true,"reference":bundle.name,"directory":directory,"records":stats.records,"bases":stats.bases,"seconds":started.elapsed().as_secs_f64()})
            );
        }
    }
    Ok(())
}
fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut out = BufWriter::new(File::create_new(path)?);
    serde_json::to_writer_pretty(&mut out, value)?;
    out.flush()?;
    out.get_ref().sync_all()?;
    Ok(())
}
#[derive(Serialize)]
struct IndexRecord {
    header: String,
    start: usize,
    length: usize,
    global_id: usize,
}
#[derive(Serialize)]
struct Shard {
    file: String,
    records: Vec<IndexRecord>,
    text_bases: usize,
    index_bytes: usize,
    build_seconds: f64,
}
#[derive(Serialize)]
struct Manifest {
    reverse_records: bool,
    format: String,
    reference: String,
    reference_bases: usize,
    reference_records: usize,
    shards: Vec<Shard>,
    preparation_seconds: f64,
    source_sha256: String,
}
fn build_index(reference: &Path, output: &Path, shard_bases: usize, threads: u16) -> Result<()> {
    fs::create_dir(output)?;
    let start = Instant::now();
    let info = ReferenceInfo::create(reference, &output.join("reference-info.json"))?;
    let mut manifest = Manifest {
        reverse_records: false,
        format: "ooff-fm-compact-v1".into(),
        reference: fs::canonicalize(reference)?.display().to_string(),
        reference_bases: 0,
        reference_records: 0,
        shards: Vec::new(),
        preparation_seconds: 0.0,
        source_sha256: info.sha256.clone(),
    };
    let mut text = Vec::new();
    let mut records = Vec::new();
    inputs::fasta(reference, |header, sequence| {
        let seq = crate::normalize(&sequence, false)?;
        if seq.len() >= i32::MAX as usize - 2 {
            return Err("reference record exceeds index size limit".into());
        }
        if !text.is_empty() && text.len() + seq.len() + 1 > shard_bases {
            manifest.shards.push(save_shard(
                output,
                &text,
                std::mem::take(&mut records),
                manifest.shards.len(),
                threads,
            )?);
            text.clear();
        }
        records.push(IndexRecord {
            header,
            start: text.len(),
            length: seq.len(),
            global_id: manifest.reference_records,
        });
        manifest.reference_bases += seq.len();
        manifest.reference_records += 1;
        text.extend(seq);
        text.push(b'N');
        Ok(())
    })?;
    if !text.is_empty() {
        manifest.shards.push(save_shard(
            output,
            &text,
            records,
            manifest.shards.len(),
            threads,
        )?);
    }
    manifest.preparation_seconds = start.elapsed().as_secs_f64();
    info.verify_file(reference)?;
    write_json(&output.join("manifest.json"), &manifest)
}
fn save_shard(
    output: &Path,
    text: &[u8],
    records: Vec<IndexRecord>,
    number: usize,
    threads: u16,
) -> inputs::Result<Shard> {
    let started = Instant::now();
    let index = Index::build_with_threads(text, libsais::ThreadCount::fixed(threads));
    let file = format!("shard-{number:04}.fm");
    let mut writer = BufWriter::new(File::create_new(output.join(&file))?);
    index.write_compact(&mut writer, 16)?;
    writer.flush()?;
    let index_bytes = writer.get_ref().metadata()?.len() as usize;
    eprintln!(
        "Index shard {}: {} bases, {:.1} s",
        number + 1,
        text.len(),
        started.elapsed().as_secs_f64()
    );
    Ok(Shard {
        file,
        records,
        text_bases: text.len(),
        index_bytes,
        build_seconds: started.elapsed().as_secs_f64(),
    })
}
