use clap::{Parser, ValueEnum};
use oofft::{Query, Record, chunks, normalize, reverse_complement, search_chunk};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufWriter, Read, Write},
    path::{Path, PathBuf},
    time::Instant,
};

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Mode {
    Summary,
    Screen,
    Report,
}

#[derive(Parser)]
#[command(
    version,
    about = "ASO off-target search against human and preclinical RNA references",
    after_help = "References:\n  oofft reference list\n  oofft reference prepare hg38\n\nExample:\n  oofft --queries asos.fa --exclude SCN2A"
)]
struct Args {
    #[arg(value_enum, default_value = "summary")]
    mode: Mode,
    /// Emit every annotated site (equivalent to explicit report mode).
    #[arg(long, display_order = 70)]
    sites: bool,
    /// Include the sorted other-gene IDs in each count summary.
    #[arg(long, display_order = 60)]
    genes: bool,
    /// Summary identity; overlapping intervals remain separate sites.
    #[arg(long, value_enum, help_heading = "Advanced options")]
    count_unit: Option<oofft::summary::CountUnit>,
    /// Indexed summary workers; default uses available logical CPUs.
    #[arg(short = 't', long, display_order = 50)]
    threads: Option<usize>,
    /// ASOs written 5' to 3': FASTA (.fa/.fasta, optionally gzip) or JSONL.
    #[arg(short = 'q', long, display_order = 10)]
    queries: PathBuf,
    /// Prepared reference name, or manual RNA FASTA/JSONL file.
    #[arg(short = 'r', long, default_value = "hg38", display_order = 20)]
    reference: PathBuf,
    #[arg(short = 'k', long, display_order = 40, default_value_t = 3, value_parser = clap::value_parser!(u8).range(0..=3))]
    max_edits: u8,
    /// Compatibility option; gene exclusions are specified with --exclude.
    #[arg(long, value_parser = ["other-gene"], default_value = "other-gene", hide = true)]
    policy: String,
    /// Manual references only; prepared presets supply provenance automatically.
    #[arg(long, help_heading = "Manual references")]
    reference_release: Option<String>,
    /// Describe included RNA classes and missing coverage.
    #[arg(long, help_heading = "Manual references")]
    scope: Option<String>,
    /// Optional biotype description for a manually supplied reference.
    #[arg(long, help_heading = "Manual references")]
    biotype_policy: Option<String>,
    /// Per-query report cap; capped queries are incomplete and counts lower bounds.
    #[arg(long, help_heading = "Advanced options")]
    max_sites: Option<usize>,
    #[arg(long, default_value_t = 65536, hide = true)]
    chunk_bases: usize,
    /// Reuse a prebuilt FM index; --reference then names its source FASTA.
    #[arg(long, requires = "annotations", help_heading = "Manual references")]
    index: Option<PathBuf>,
    /// JSONL record annotations in the same order as the indexed FASTA.
    #[arg(long, requires = "index", help_heading = "Manual references")]
    annotations: Option<PathBuf>,
    /// Optional reversed-record index: accelerates paired-direction search.
    #[arg(long, requires = "index", help_heading = "Manual references")]
    reverse_index: Option<PathBuf>,
    /// Prevalidated annotation offsets, created by oofft-index cache-annotations.
    #[arg(long, requires = "index", help_heading = "Manual references")]
    annotation_cache: Option<PathBuf>,
    /// Exclude genes by symbol or ID; repeat or separate with commas.
    #[arg(long, value_delimiter = ',', display_order = 30)]
    exclude: Vec<String>,
    /// Reference cache directory (or OOFFT_CACHE_DIR).
    #[arg(long, display_order = 80)]
    cache_dir: Option<PathBuf>,
    #[arg(skip)]
    record_coordinates: bool,
    #[arg(skip)]
    preset_name: Option<String>,
}

impl Args {
    fn count_unit(&self) -> oofft::summary::CountUnit {
        self.count_unit.unwrap_or(if self.record_coordinates {
            oofft::summary::CountUnit::RecordInterval
        } else {
            oofft::summary::CountUnit::GenomicSite
        })
    }
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
    genomic_blocks: Vec<oofft::Block>,
    alignment: &'a oofft::Site,
    expression_evidence: Option<()>,
    experimental_evidence: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none")]
    witness_edit_distance: Option<usize>,
}

fn emit_site(
    out: &mut impl Write,
    query: &Query,
    record: &Record,
    site: &oofft::Site,
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

fn configure_reference(
    args: &mut Args,
) -> Result<Option<BTreeMap<String, String>>, Box<dyn Error>> {
    if args.reference.is_file() {
        if args.index.is_none() && oofft::inputs::is_fasta(&args.reference)? {
            args.record_coordinates = true;
            if args.count_unit == Some(oofft::summary::CountUnit::GenomicSite) {
                return Err("genomic-site counts require genomic annotations; use a prepared reference or --count-unit record-interval for bare RNA FASTA".into());
            }
        }
        args.reference_release
            .get_or_insert_with(|| "custom".into());
        args.scope.get_or_insert_with(|| {
            if args.record_coordinates {
                "Supplied RNA FASTA; record-relative coordinates".into()
            } else {
                "Supplied annotated RNA records".into()
            }
        });
        args.biotype_policy
            .get_or_insert_with(|| "as supplied".into());
        return Ok(None);
    }
    if args.index.is_some()
        || args.annotations.is_some()
        || args.reverse_index.is_some()
        || args.annotation_cache.is_some()
    {
        return Err("manual index options require an existing --reference FASTA file".into());
    }
    if args.reference_release.is_some() || args.scope.is_some() || args.biotype_policy.is_some() {
        return Err("prepared references supply provenance automatically; manual labels require a --reference file".into());
    }
    let name = args.reference.to_str().ok_or("invalid reference name")?;
    let cache = oofft::reference::cache_dir(args.cache_dir.as_deref())?;
    let (directory, bundle) = oofft::reference::resolve(name, &cache)?;
    let genes_path = directory.join("genes.json");
    if hash(&genes_path)? != bundle.genes_sha256 {
        return Err("reference gene map changed; rebuild the reference".into());
    }
    let info: oofft::indexed::ReferenceInfo =
        serde_json::from_reader(File::open(directory.join("index/reference-info.json"))?)?;
    if info.sha256 != bundle.reference_sha256 {
        return Err("reference bundle/index sequence identity mismatch".into());
    }
    let genes = serde_json::from_reader(File::open(genes_path)?)?;
    args.reference = directory.join("reference.fa");
    args.index = Some(directory.join("index"));
    args.annotations = Some(directory.join("records.jsonl"));
    args.annotation_cache = Some(directory.join("annotation-cache.json"));
    args.reference_release = Some(bundle.reference_release);
    args.scope = Some(bundle.scope);
    args.biotype_policy = Some(bundle.biotype_policy);
    args.preset_name = Some(bundle.name);
    Ok(Some(genes))
}

fn apply_exclusions(
    args: &mut Args,
    queries: &mut [Query],
    symbols: Option<BTreeMap<String, String>>,
) -> Result<(), Box<dyn Error>> {
    // Existing per-query JSONL IDs remain compatible. Presets also resolve their
    // symbols against the selected species, never via cross-species guessing.
    let resolve_per_query = symbols.is_some();
    let mut genes = symbols.unwrap_or_default();
    if !args.exclude.is_empty() && genes.is_empty() {
        if let Some(annotations) = &args.annotations {
            for line in oofft::inputs::reader(annotations)?.lines() {
                let line = line?;
                if line.trim().is_empty() {
                    continue;
                }
                let record: Value = serde_json::from_str(&line)?;
                let ids = record["genes"]
                    .as_array()
                    .ok_or("annotation missing genes")?;
                for id in ids {
                    genes.insert(
                        id.as_str().ok_or("gene ID must be a string")?.into(),
                        String::new(),
                    );
                }
            }
        } else {
            for record in oofft::inputs::ReferenceRecords::open(&args.reference)? {
                for id in record?.genes {
                    genes.insert(id, String::new());
                }
            }
        }
    }
    let resolved = oofft::reference::resolve_exclusions(&args.exclude, &genes)?;
    let mut query_ids = BTreeMap::new();
    if resolve_per_query {
        let unique: BTreeSet<_> = queries
            .iter()
            .flat_map(|q| q.intended_genes.iter())
            .collect();
        for name in unique {
            query_ids.insert(
                name.clone(),
                oofft::reference::resolve_exclusions(std::slice::from_ref(name), &genes)?,
            );
        }
    }
    for query in queries {
        let mut exclusions: BTreeSet<_> = resolved.iter().cloned().collect();
        for gene in &query.intended_genes {
            if resolve_per_query {
                exclusions.extend(query_ids[gene].iter().cloned());
            } else {
                exclusions.insert(gene.clone());
            }
        }
        query.intended_genes = exclusions.into_iter().collect();
    }
    args.exclude = resolved;
    Ok(())
}

fn run(mut args: Args) -> Result<(), Box<dyn Error>> {
    let started = Instant::now();
    let gene_symbols = configure_reference(&mut args)?;
    if args.sites {
        if matches!(args.mode, Mode::Screen) {
            return Err("--sites cannot be combined with screen".into());
        }
        args.mode = Mode::Report;
    }
    if args.threads == Some(0) {
        return Err("--threads must be positive".into());
    }
    if args.threads.is_some() && (!matches!(args.mode, Mode::Summary) || args.index.is_none()) {
        return Err("--threads currently applies to indexed summary searches".into());
    }
    if matches!(args.mode, Mode::Summary) && args.max_sites.is_some() {
        return Err("--max-sites applies to report; summary counts must be exhaustive".into());
    }
    if args.chunk_bases == 0 || args.max_sites == Some(0) {
        return Err("limits must be positive".into());
    }
    if matches!(args.mode, Mode::Screen) && args.max_sites.is_some() {
        return Err("--max-sites applies only to report".into());
    }
    let mut queries = oofft::inputs::queries(&args.queries)?;
    apply_exclusions(&mut args, &mut queries, gene_symbols)?;
    if queries.is_empty() {
        return Err("query file is empty".into());
    }
    let mut ids = HashSet::new();
    let mut patterns = Vec::new();
    for q in &queries {
        if q.id.is_empty() || !ids.insert(&q.id) || q.intended_genes.iter().any(String::is_empty) {
            return Err(
                "queries require unique nonempty IDs; excluded gene IDs must be nonempty".into(),
            );
        }
        let seq = normalize(&q.sequence, true)?;
        if matches!(args.mode, Mode::Summary) && !(4..=63).contains(&seq.len()) {
            return Err(format!("query {} must be 4..63 nt for summary", q.id).into());
        }
        if !matches!(args.mode, Mode::Summary) && seq.len() != 20 {
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
    for r in oofft::inputs::ReferenceRecords::open(&args.reference)? {
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
            "count_unit": if matches!(args.mode, Mode::Summary) { Some(args.count_unit().name()) } else { None },
            "max_edits": args.max_edits, "policy": args.policy, "exclude": args.exclude, "reference_preset": args.preset_name, "coordinate_scope": if args.record_coordinates { "reference-record" } else { "genomic" }, "reference_release": args.reference_release,
            "scope": args.scope, "biotype_policy": args.biotype_policy,
            "queries_sha256": hash(&args.queries)?, "reference_sha256": hash(&args.reference)?,
            "reference_records": record_ids.len(), "reference_bases": reference_bases,
            "unknown_bases_excluded": unknown_bases, "normalization": "uppercase; U to T; retain soft-masked sequence",
            "orientation": "reverse-complement ASO against RNA-sense record", "coordinates": "zero-based half-open",
            "site_identity": if matches!(args.mode, Mode::Summary) && args.count_unit() == oofft::summary::CountUnit::GenomicSite {"query ID, contig, strand, ordered genomic blocks; minimum edit distance across records"} else {"query ID, record ID, start, end; one minimum-cost alignment per interval"},
            "tie_policy": "traceback prefers diagonal, I, D", "cigar_direction": "RNA-sense (reverse ASO order)",
            "chemistry_annotations": if matches!(args.mode, Mode::Summary) {"chemistry not inferred; no positional or thermodynamic exclusion"} else {"5-10-5 MOE wings/DNA gap; no positional or thermodynamic exclusion"},
            "allele_policy": "metadata only; intended-gene exclusion does not assess spared alleles",
            "unknown_policy": "split records at ambiguity; no-hit status incomplete when unknown bases exist",
            "max_sites": args.max_sites, "chunk_bases": args.chunk_bases, "threads": 1,
            "arch": std::env::consts::ARCH, "os": std::env::consts::OS,
            "completion_contract": "output is incomplete unless run_complete is present"
        }),
    )?;
    if matches!(args.mode, Mode::Summary) {
        return run_summary_plain(&args, &queries, &patterns, unknown_bases, &mut out, started);
    }
    let mut counts = vec![0usize; queries.len()];
    let mut retired = vec![false; queries.len()];
    let mut capped = vec![false; queries.len()];
    let k = args.max_edits as usize;
    for r in oofft::inputs::ReferenceRecords::open(&args.reference)? {
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
    let result = if std::env::args_os()
        .nth(1)
        .is_some_and(|arg| arg == "reference")
    {
        let argv = std::env::args_os()
            .take(1)
            .chain(std::env::args_os().skip(2));
        oofft::reference::run(oofft::reference::Cli::parse_from(argv))
    } else {
        run(Args::parse())
    };
    if let Err(e) = result {
        eprintln!("oofft: {e}");
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
        oofft::indexed::ReferenceIndex::open_cached_immutable(
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
    let threads = if matches!(args.mode, Mode::Summary) {
        args.threads
            .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, usize::from))
            .min(queries.len())
    } else {
        1
    };
    let mut out = BufWriter::new(std::io::stdout().lock());
    emit(
        &mut out,
        &json!({"type":"manifest","schema_version":1,"ooff_version":env!("CARGO_PKG_VERSION"),
        "engine":if matches!(args.mode, Mode::Summary) {"native-rust-fm-minimum-distance-counts"} else {"native-rust-fm-with-independent-interval-verification"},"mode":format!("{:?}",args.mode).to_lowercase(),
        "count_unit":if matches!(args.mode, Mode::Summary) {Some(args.count_unit().name())} else {None},
        "max_edits":args.max_edits,"policy":args.policy,"exclude":args.exclude,"reference_preset":args.preset_name,"coordinate_scope":"genomic","reference_release":args.reference_release,"scope":args.scope,
        "biotype_policy":args.biotype_policy,"queries_sha256":hash(&args.queries)?,"reference_sha256":index.info.sha256,
        "annotations_sha256":if let Some(hash) = index.cached_annotation_sha256() { hash.to_owned() } else { hash(annotations)? },"index_manifest_sha256":hash(&directory.join("manifest.json"))?,
        "reference_records":index.record_count(),"reference_bases":index.reference_bases(),"unknown_bases_excluded":index.info.unknown_bases,
        "normalization":"uppercase; U to T; retain soft-masked sequence","orientation":"reverse-complement ASO against RNA-sense record",
        "coordinates":"zero-based half-open","site_identity":if matches!(args.mode, Mode::Summary) && args.count_unit() == oofft::summary::CountUnit::GenomicSite {"query ID, contig, strand, ordered genomic blocks; minimum edit distance across records"} else {"query ID, record ID, start, end; one minimum-cost alignment per interval"},
        "tie_policy":"traceback prefers diagonal, I, D","cigar_direction":"RNA-sense (reverse ASO order)",
        "chemistry_annotations":if matches!(args.mode, Mode::Summary) {"chemistry not inferred; no positional or thermodynamic exclusion"} else {"5-10-5 MOE wings/DNA gap; no positional or thermodynamic exclusion"},
        "allele_policy":"metadata only; intended-gene exclusion does not assess spared alleles",
        "unknown_policy":"split records at ambiguity; no-hit status incomplete when unknown bases exist",
        "annotation_cache":args.annotation_cache,"paired_directions":args.reverse_index.is_some(),
        "reverse_index_manifest_sha256":args.reverse_index.as_ref().map(|p| hash(&p.join("manifest.json"))).transpose()?,"max_sites":args.max_sites,"threads":threads,"index_load_seconds":load_seconds,
        "arch":std::env::consts::ARCH,"os":std::env::consts::OS,"completion_contract":"output is incomplete unless run_complete is present"}),
    )?;
    if matches!(args.mode, Mode::Summary) {
        return run_summary_indexed(&args, queries, patterns, &index, threads, &mut out, started);
    }
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

fn count_summary(
    args: &Args,
    query: &Query,
    counts: &oofft::summary::Counts,
    unknown: usize,
) -> Value {
    let bins: serde_json::Map<String, Value> = (0..4)
        .map(|d| {
            (
                d.to_string(),
                if d <= args.max_edits as usize {
                    json!(counts.by_edit_distance[d])
                } else {
                    Value::Null
                },
            )
        })
        .collect();
    let mut row = json!({"type":"query_summary", "query_id":query.id,
        "status":if unknown > 0 {"incomplete"} else if counts.total() > 0 {"offtarget_found"} else {"none_found_within_scope"},
        "count_unit":args.count_unit().name(), "edit_distance_counts":bins,
        "total_sites":counts.total(), "candidate_found":counts.total()>0,
        "counts_complete":unknown==0, "unknown_bases_excluded":unknown});
    if args.genes {
        row["offtarget_genes"] = json!(counts.genes);
    }
    row
}

fn run_summary_plain(
    args: &Args,
    queries: &[Query],
    patterns: &[Vec<u8>],
    unknown: usize,
    out: &mut impl Write,
    started: Instant,
) -> Result<(), Box<dyn Error>> {
    let mut total = 0u64;
    // One query's deduplication state at a time, even without an index.
    for (query, pattern) in queries.iter().zip(patterns) {
        let mut counts = oofft::summary::Counts::new(args.count_unit(), args.genes);
        for r in oofft::inputs::ReferenceRecords::open(&args.reference)? {
            let r = r?;
            if r.genes.iter().all(|g| query.intended_genes.contains(g)) {
                continue;
            }
            let text = normalize(&r.sequence, false)?;
            for (a, b, owned_end) in chunks(
                &text,
                args.chunk_bases,
                pattern.len() + args.max_edits as usize,
            ) {
                oofft::search_chunk_intervals(
                    std::slice::from_ref(pattern),
                    &text[a..b],
                    args.max_edits as usize,
                    |_, start, end, d| {
                        if a + start < owned_end {
                            counts.add(&r, &query.intended_genes, a + start, a + end, d);
                        }
                        true
                    },
                );
            }
        }
        total += counts.total();
        emit(out, &count_summary(args, query, &counts, unknown))?;
    }
    emit(
        out,
        &json!({"type":"run_complete", "seconds":started.elapsed().as_secs_f64(), "total_sites":total,"queries":queries.len()}),
    )?;
    out.flush()?;
    Ok(())
}

fn run_summary_indexed(
    args: &Args,
    queries: &[Query],
    patterns: &[Vec<u8>],
    index: &oofft::indexed::ReferenceIndex,
    threads: usize,
    out: &mut impl Write,
    started: Instant,
) -> Result<(), Box<dyn Error>> {
    use std::sync::{Arc, Mutex, mpsc::sync_channel};
    let search_started = Instant::now();
    let total = std::thread::scope(|scope| -> Result<u64, Box<dyn Error>> {
        let window = threads * 2;
        let (tasks, jobs) = sync_channel::<usize>(window);
        let jobs = Arc::new(Mutex::new(jobs));
        let (sender, receiver) = sync_channel(window);
        let mut scheduled = queries.len().min(window);
        for q in 0..scheduled {
            tasks.send(q)?;
        }
        for _ in 0..threads {
            let sender = sender.clone();
            let jobs = jobs.clone();
            scope.spawn(move || {
                loop {
                    let job = { jobs.lock().unwrap().recv() };
                    let Ok(q) = job else {
                        break;
                    };
                    let result = (|| {
                        let mut counts = oofft::summary::Counts::new(args.count_unit(), args.genes);
                        index
                            .count(
                                &patterns[q],
                                &queries[q].intended_genes,
                                args.max_edits as usize,
                                &mut counts,
                                None,
                            )
                            .map_err(|e| e.to_string())?;
                        Ok::<_, String>(count_summary(
                            args,
                            &queries[q],
                            &counts,
                            index.info.unknown_bases,
                        ))
                    })();
                    let failed = result.is_err();
                    if sender.send((q, result)).is_err() || failed {
                        break;
                    }
                }
            });
        }
        drop(sender);
        let mut pending = std::collections::BTreeMap::new();
        let (mut expected, mut total) = (0usize, 0u64);
        while expected < queries.len() {
            let (q, result) = receiver.recv()?;
            pending.insert(q, result?);
            while let Some(row) = pending.remove(&expected) {
                total += row["total_sites"].as_u64().unwrap();
                emit(out, &row)?;
                expected += 1;
                if scheduled < queries.len() {
                    tasks.send(scheduled)?;
                    scheduled += 1;
                }
            }
        }
        drop(tasks);
        Ok(total)
    })?;
    emit(
        out,
        &json!({"type":"run_complete", "seconds":started.elapsed().as_secs_f64(),
        "search_and_output_seconds":search_started.elapsed().as_secs_f64(),"queries":queries.len(),"total_sites":total}),
    )?;
    out.flush()?;
    Ok(())
}
