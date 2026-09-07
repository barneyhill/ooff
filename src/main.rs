use clap::{Parser, ValueEnum};
use ooff::{Query, Record, chunks, normalize, reverse_complement, search_chunk};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    time::Instant,
};

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Mode {
    Screen,
    Report,
}

#[derive(Parser)]
#[command(
    version,
    about = "Native Rust ASO candidate-site discovery; supplied oriented RNA records only"
)]
struct Args {
    #[arg(value_enum)]
    mode: Mode,
    /// JSONL: id, sequence (20 nt ASO, 5' to 3'), intended_genes, optional allele.
    #[arg(long)]
    queries: PathBuf,
    /// JSONL: id, sequence (RNA-sense), genes, transcripts, contig, strand, blocks.
    #[arg(long)]
    reference: PathBuf,
    #[arg(short = 'k', long, default_value_t = 3, value_parser = clap::value_parser!(u8).range(0..=3))]
    max_edits: u8,
    /// Explicit project policy: any candidate associated with another gene.
    #[arg(long, value_parser = ["other-gene"])]
    policy: String,
    #[arg(long)]
    reference_release: String,
    /// Describe included RNA classes and missing coverage.
    #[arg(long)]
    scope: String,
    #[arg(long)]
    biotype_policy: String,
    /// Per-query report cap; capped queries are incomplete and counts lower bounds.
    #[arg(long)]
    max_sites: Option<usize>,
    #[arg(long, default_value_t = 65536)]
    chunk_bases: usize,
    /// Reuse a prebuilt FM index; --reference then names its source FASTA.
    #[arg(long, requires = "annotations")]
    index: Option<PathBuf>,
    /// JSONL record annotations in the same order as the indexed FASTA.
    #[arg(long, requires = "index")]
    annotations: Option<PathBuf>,
    /// Optional reversed-record index: accelerates paired-direction search.
    #[arg(long, requires = "index")]
    reverse_index: Option<PathBuf>,
    /// Prevalidated annotation offsets, created by ooff-index cache-annotations.
    #[arg(long, requires = "index")]
    annotation_cache: Option<PathBuf>,
}

fn records<T: DeserializeOwned>(
    path: &Path,
) -> Result<impl Iterator<Item = Result<T, Box<dyn Error>>>, Box<dyn Error>> {
    Ok(BufReader::new(File::open(path)?)
        .lines()
        .enumerate()
        .map(|(n, line)| {
            let line = line?;
            serde_json::from_str(&line).map_err(|e| format!("JSONL line {}: {e}", n + 1).into())
        }))
}

fn hash(path: &Path) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hash.update(&buf[..n]);
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn emit(out: &mut impl Write, value: &Value) -> Result<(), Box<dyn Error>> {
    serde_json::to_writer(&mut *out, value)?;
    out.write_all(b"\n")?;
    Ok(())
}

#[derive(Serialize)]
struct SiteOutput<'a> {
    #[serde(rename = "type")]
    kind: &'static str,
    query_id: &'a str,
    allele: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ddg: Option<f64>,
    record_id: &'a str,
    site_id: [&'a str; 4],
    genes: &'a [String],
    offtarget_genes: Vec<&'a String>,
    transcripts: &'a [String],
    contig: &'a str,
    strand: &'a str,
    genomic_blocks: Vec<ooff::Block>,
    alignment: &'a ooff::Site,
    expression_evidence: Option<()>,
    experimental_evidence: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    witness_edit_distance: Option<usize>,
}

fn emit_site(
    out: &mut impl Write,
    query: &Query,
    record: &Record,
    site: &ooff::Site,
    screen: bool,
) -> Result<(), Box<dyn Error>> {
    let start = site.start.to_string();
    let end = site.end.to_string();
    let value = SiteOutput {
        kind: "site",
        query_id: &query.id,
        allele: &query.allele,
        ddg: query.ddg,
        record_id: &record.id,
        site_id: [&query.id, &record.id, &start, &end],
        genes: &record.genes,
        offtarget_genes: record
            .genes
            .iter()
            .filter(|g| !query.intended_genes.contains(g))
            .collect(),
        transcripts: &record.transcripts,
        contig: &record.contig,
        strand: &record.strand,
        genomic_blocks: record.map_interval(site.start, site.end),
        alignment: site,
        expression_evidence: None,
        experimental_evidence: None,
        witness_edit_distance: screen.then_some(site.edit_distance),
    };
    serde_json::to_writer(&mut *out, &value)?;
    out.write_all(b"\n")?;
    Ok(())
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let started = Instant::now();
    if args.chunk_bases == 0 || args.max_sites == Some(0) {
        return Err("limits must be positive".into());
    }
    if matches!(args.mode, Mode::Screen) && args.max_sites.is_some() {
        return Err("--max-sites applies only to report".into());
    }
    let queries = records::<Query>(&args.queries)?.collect::<Result<Vec<_>, _>>()?;
    if queries.is_empty() {
        return Err("query file is empty".into());
    }
    let mut ids = HashSet::new();
    let mut patterns = Vec::new();
    for q in &queries {
        if q.id.is_empty()
            || !ids.insert(&q.id)
            || q.intended_genes.is_empty()
            || q.intended_genes.iter().any(String::is_empty)
        {
            return Err("queries require unique nonempty IDs and intended gene IDs".into());
        }
        let seq = normalize(&q.sequence, true)?;
        if seq.len() != 20 {
            return Err(format!("query {} must be exactly 20 nt", q.id).into());
        }
        patterns.push(reverse_complement(&seq));
    }
    if args.index.is_some() {
        return run_indexed(args, &queries, &patterns, started);
    }
    // Validate all records before emitting search results. Only one sequence
    // record is held at a time; IDs remain resident to reject collisions.
    let mut record_ids = HashSet::new();
    let (mut unknown_bases, mut reference_bases) = (0usize, 0usize);
    for r in records::<Record>(&args.reference)? {
        let r = r?;
        r.validate()?;
        if !record_ids.insert(r.id.clone()) {
            return Err(format!("duplicate reference ID {}", r.id).into());
        }
        unknown_bases += normalize(&r.sequence, false)?
            .iter()
            .filter(|&&b| b == b'N')
            .count();
        reference_bases += r.sequence.len();
    }
    if record_ids.is_empty() {
        return Err("reference file is empty".into());
    }
    let mut out = BufWriter::new(std::io::stdout().lock());
    emit(
        &mut out,
        &json!({
            "type": "manifest", "schema_version": 1, "ooff_version": env!("CARGO_PKG_VERSION"),
            "engine": "native-rust-myers-with-fixed-interval-verification", "mode": format!("{:?}", args.mode).to_lowercase(),
            "max_edits": args.max_edits, "policy": args.policy, "reference_release": args.reference_release,
            "scope": args.scope, "biotype_policy": args.biotype_policy,
            "queries_sha256": hash(&args.queries)?, "reference_sha256": hash(&args.reference)?,
            "reference_records": record_ids.len(), "reference_bases": reference_bases,
            "unknown_bases_excluded": unknown_bases, "normalization": "uppercase; U to T; retain soft-masked sequence",
            "orientation": "reverse-complement ASO against RNA-sense record", "coordinates": "zero-based half-open",
            "site_identity": "query ID, record ID, start, end; one minimum-cost alignment per interval",
            "tie_policy": "traceback prefers diagonal, I, D", "cigar_direction": "RNA-sense (reverse ASO order)",
            "chemistry_annotations": "5-10-5 MOE wings/DNA gap; no positional or thermodynamic exclusion",
            "allele_policy": "metadata only; intended-gene exclusion does not assess spared alleles",
            "unknown_policy": "split records at ambiguity; no-hit status incomplete when unknown bases exist",
            "max_sites": args.max_sites, "chunk_bases": args.chunk_bases, "threads": 1,
            "arch": std::env::consts::ARCH, "os": std::env::consts::OS,
            "completion_contract": "output is incomplete unless run_complete is present"
        }),
    )?;
    let mut counts = vec![0usize; queries.len()];
    let mut retired = vec![false; queries.len()];
    let mut capped = vec![false; queries.len()];
    let k = args.max_edits as usize;
    for r in records::<Record>(&args.reference)? {
        let r = r?;
        let text = normalize(&r.sequence, false)?;
        for (a, b, owned_end) in chunks(&text, args.chunk_bases, 20 + k) {
            let active: Vec<usize> = queries
                .iter()
                .enumerate()
                .filter_map(|(i, q)| {
                    (!retired[i] && r.genes.iter().any(|g| !q.intended_genes.contains(g)))
                        .then_some(i)
                })
                .collect();
            let active_patterns: Vec<_> = active.iter().map(|&q| patterns[q].clone()).collect();
            let mut write_error = None;
            search_chunk(&active_patterns, &text[a..b], k, |index, mut site| {
                if write_error.is_some() {
                    return false;
                }
                let q = active[index];
                site.start += a;
                site.end += a;
                if site.start >= owned_end {
                    return true;
                }
                if let Err(e) = emit_site(
                    &mut out,
                    &queries[q],
                    &r,
                    &site,
                    matches!(args.mode, Mode::Screen),
                ) {
                    write_error = Some(e);
                    return false;
                }
                counts[q] += 1;
                if matches!(args.mode, Mode::Screen) {
                    retired[q] = true;
                }
                if args.max_sites.is_some_and(|max| counts[q] >= max) {
                    retired[q] = true;
                    capped[q] = true;
                }
                !retired[q]
            });
            if let Some(e) = write_error {
                return Err(e);
            }
            if retired.iter().all(|&x| x) {
                break;
            }
        }
        if retired.iter().all(|&x| x) {
            break;
        }
    }
    for (q, query) in queries.iter().enumerate() {
        let status = if matches!(args.mode, Mode::Screen) && counts[q] > 0 {
            "offtarget_found"
        } else if capped[q] || unknown_bases > 0 {
            "incomplete"
        } else if counts[q] > 0 {
            "offtarget_found"
        } else {
            "none_found_within_scope"
        };
        emit(
            &mut out,
            &json!({"type": "query_summary", "query_id": query.id, "status": status,
            "reported_sites": counts[q], "candidate_found": counts[q] > 0,
            "count_is_lower_bound": capped[q] || unknown_bases > 0 || matches!(args.mode, Mode::Screen),
            "capped": capped[q], "unknown_bases_excluded": unknown_bases}),
        )?;
    }
    emit(
        &mut out,
        &json!({"type": "run_complete", "seconds": started.elapsed().as_secs_f64(), "reported_sites": counts.iter().sum::<usize>()}),
    )?;
    out.flush()?;
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("ooff: {e}");
        std::process::exit(1);
    }
}

fn run_indexed(
    args: Args,
    queries: &[Query],
    patterns: &[Vec<u8>],
    started: Instant,
) -> Result<(), Box<dyn Error>> {
    let load_started = Instant::now();
    let directory = args.index.as_ref().unwrap();
    let annotations = args.annotations.as_ref().unwrap();
    // SAFETY: the CLI requires immutable, prebuilt index/reference artifacts.
    // It never modifies these files, and records their provenance in output.
    let mut index = unsafe {
        ooff::indexed::ReferenceIndex::open_cached_immutable(
            directory,
            &args.reference,
            annotations,
            args.annotation_cache.as_deref(),
        )
    }?;
    if let Some(reverse) = &args.reverse_index {
        // SAFETY: the same immutable artifact contract applies to both indexes.
        unsafe { index.attach_reverse_immutable(reverse) }?;
    }
    let load_seconds = load_started.elapsed().as_secs_f64();
    let mut out = BufWriter::new(std::io::stdout().lock());
    emit(
        &mut out,
        &json!({"type":"manifest","schema_version":1,"ooff_version":env!("CARGO_PKG_VERSION"),
        "engine":"native-rust-fm-with-independent-interval-verification","mode":format!("{:?}",args.mode).to_lowercase(),
        "max_edits":args.max_edits,"policy":args.policy,"reference_release":args.reference_release,"scope":args.scope,
        "biotype_policy":args.biotype_policy,"queries_sha256":hash(&args.queries)?,"reference_sha256":index.info.sha256,
        "annotations_sha256":if let Some(hash) = index.cached_annotation_sha256() { hash.to_owned() } else { hash(annotations)? },"index_manifest_sha256":hash(&directory.join("manifest.json"))?,
        "reference_records":index.record_count(),"reference_bases":index.reference_bases(),"unknown_bases_excluded":index.info.unknown_bases,
        "normalization":"uppercase; U to T; retain soft-masked sequence","orientation":"reverse-complement ASO against RNA-sense record",
        "coordinates":"zero-based half-open","site_identity":"query ID, record ID, start, end; one minimum-cost alignment per interval",
        "tie_policy":"traceback prefers diagonal, I, D","cigar_direction":"RNA-sense (reverse ASO order)",
        "chemistry_annotations":"5-10-5 MOE wings/DNA gap; no positional or thermodynamic exclusion",
        "allele_policy":"metadata only; intended-gene exclusion does not assess spared alleles",
        "unknown_policy":"split records at ambiguity; no-hit status incomplete when unknown bases exist",
        "annotation_cache":args.annotation_cache,"paired_directions":args.reverse_index.is_some(),
        "reverse_index_manifest_sha256":args.reverse_index.as_ref().map(|p| hash(&p.join("manifest.json"))).transpose()?,"max_sites":args.max_sites,"threads":1,"index_load_seconds":load_seconds,
        "arch":std::env::consts::ARCH,"os":std::env::consts::OS,"completion_contract":"output is incomplete unless run_complete is present"}),
    )?;
    let search_started = Instant::now();
    let mut total = 0;
    for (q, pattern) in queries.iter().zip(patterns) {
        let screen = matches!(args.mode, Mode::Screen);
        let (count, capped) = index.search(
            pattern,
            &q.intended_genes,
            args.max_edits as usize,
            screen,
            args.max_sites,
            |r, site| emit_site(&mut out, q, r, &site, screen),
        )?;
        let status = if screen && count > 0 {
            "offtarget_found"
        } else if capped || index.info.unknown_bases > 0 {
            "incomplete"
        } else if count > 0 {
            "offtarget_found"
        } else {
            "none_found_within_scope"
        };
        emit(
            &mut out,
            &json!({"type":"query_summary","query_id":q.id,"status":status,"reported_sites":count,"candidate_found":count>0,
            "count_is_lower_bound":screen||capped||index.info.unknown_bases>0,"capped":capped,"unknown_bases_excluded":index.info.unknown_bases}),
        )?;
        total += count;
    }
    emit(
        &mut out,
        &json!({"type":"run_complete","seconds":started.elapsed().as_secs_f64(),"search_and_output_seconds":search_started.elapsed().as_secs_f64(),"reported_sites":total}),
    )?;
    out.flush()?;
    Ok(())
}
