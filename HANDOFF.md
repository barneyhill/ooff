# RPi / EC2 implementation handoff

## User objective and current instruction

Build an ASO-specific off-target tool at `git@github.com:barneyhill/ooff.git`.
Prefer Rust. The next agent runs on a Raspberry Pi and should use AWS EC2 for
heavy computation; the Pi will have the AWS key. This handoff does not launch
EC2 or start another local scan.

Start with a correct wrapper/library integration around Sassy2. Establish what
is actually slow before inventing a new aligner. The user is exploring whether
this could become a useful standalone tool or methods project. A parameter
wrapper alone is probably not a methods contribution. A demonstrated speedup
with completeness guarantees, or a validated ASO-specific workflow, could be.
Do not promise publication or a speedup before measuring.

**User instruction: do not delete anything without explicit permission.** This
applies to local files and cleanup of existing data. A previous approval to
clean the Mac's uv cache has already been executed; it is not blanket deletion
permission. Do not touch unrelated research or cloud resources.

## Scientific basis (required reading)

This project is explicitly based on Andersson et al. (2025),
[Assessing Hybridization-Dependent Off-Target Risk for Therapeutic Oligonucleotides: Updated Industry Recommendations](https://doi.org/10.1089/nat.2024.0072).
Apply the RNase H1-dependent ASO discussion to this project; do not import siRNA
seed rules. These are industry recommendations, not a certification standard.

| Source location | Principle informing this implementation |
| --- | --- |
| RNase H1-dependent ASOs | Chemistry and mismatch/bulge positions matter. |
| Step 1, in silico prediction | Include mature RNA, pre-mRNA and ncRNA; balance sensitivity and specificity. |
| Figure 4 | Distinguish discovery screening from exhaustive shortlist assessment. |
| Step 1, transcriptomics | Computational matches need experimental follow-up. |
| Steps 2–3 | Consider productive uptake, expression and concentration-response margins. |
| Steps 4–5 | Evaluate biological consequences and manage remaining liabilities. |

**Our implementation choices:** Rust/Sassy2, fixed 20mers, initial unit-cost
k<=3, repeat retention, output schemas and performance targets. The paper does
not establish this edit budget as a universal safety boundary or endorse Sassy2.
Their RNase H1 discussion describes a practical convention of considering up
to three mismatches, usually omitting four or more without experimental
expression evidence. This is not a specified total-edit or bulge budget.
No ddG eligibility cutoff is a user/project requirement.

Expose separate maximum substitutions, inserted bases and deleted bases, plus
an optional total-edit cap. State all budgets in every run manifest. A k=3
baseline is a benchmark profile, not a complete translation of that convention:
three substitutions plus one inserted base requires k=4. Implement configurable
bulge budgets and compare sensitivity/runtime before choosing a production
profile. A proposed exploratory profile (our choice, not the paper's) is up to
three substitutions plus one inserted OR deleted base in total; retrieve at
k=4 then verify the constrained alignment. Do not filter solely on the single
minimum-distance CIGAR: an alternative alignment may satisfy the separate
budgets. Validate constrained existence/enumeration with the independent oracle.

The deliverable implements computational candidate-site discovery. Keep stable
gene/site IDs and optional evidence columns for subsequent expression, RNAseq
and experimental results; missing evidence stays unknown. Do not infer clinical
safety from a sequence hit count or claim to implement the entire assessment.

## Problem definition

- Inputs: many ASO sequences, initially exactly 20 nt, written 5' to 3'; query
  ID, intended gene ID(s), optional allele metadata. Accept DNA/RNA letters and
  normalise U/T explicitly. Chemistry is 5-10-5 MOE, full PS for the motivating
  project, but chemistry is metadata rather than an unvalidated search filter.
- Search: full-query approximate matching against annotated RNA sequence under
  **unit-cost Levenshtein distance**, initially k=0,1,2,3. Substitutions,
  insertions and deletions count; Hamming distance is insufficient.
- No thermodynamic delta-delta-G eligibility filter. Keep ddG as annotation if
  supplied. No GPU/model inference is needed for sequence searching.
- Output two distinct modes:
  1. `screen`: under an explicit user-selected rejection policy, find one
     qualifying off-target witness or exhaust the relevant
     reference; stop a query as soon as a witness suffices.
  2. `report`: enumerate defined distinct off-target sites, with gene mapping,
     edit counts, strand, coordinates and alignment operations; stream output.
- Use statuses such as `offtarget_found`, `none_found_within_scope`, and
  `incomplete`. Never call an unsearched/capped query clean or clinically safe.
- If screening stops on an arbitrary witness, report `witness_edit_distance`,
  not a claimed minimum. An exact minimum requires ordered distance strata or
  completing the necessary search. Capped counts are lower bounds.

A sequence match alone is not an automatic biological rejection. Keep the raw
search result separate from the configurable project filter. Preserve edit
positions and base identities, annotated as MOE wing (1–5, 16–20) or DNA gap
(6–15) in ASO coordinates. Do not turn those annotations into hard exclusions.
Optional later energy scoring must preserve the underlying candidate-site set
and document its chemistry assumptions.

## Reference and biological correctness

1. Retain lowercase/soft-masked repeat sequence. Lowercase is not evidence of
   absence from RNA. Do not hide repeats to improve timing.
2. Initial scope: annotated pre-mRNA gene bodies (introns plus exons) and ncRNA.
   State annotation release and biotype policy. The existing paper3 policy
   includes transcribed/translated pseudogenes but excludes other pseudogenes.
3. Spliced exon-exon junction sequences are **not** present in gene bodies.
   Mature-transcript/junction coverage is required for the intended release.
   A pre-mRNA-only milestone is acceptable only with explicit restricted scope.
   Preserve transcript-coordinate and genomic-block mappings for junction hits.
4. Preserve all gene/transcript associations when deduplicating reference
   sequence. Excluding the intended gene must not hide an overlapping or
   identical sequence belonging to another gene.
5. **Strand matters.** For oriented RNA records, search the ASO reverse
   complement against the transcript-sense sequence. If indexing genomic
   intervals, retain transcript strand and map hit orientation correctly.
   The historical benchmark searched both genomic strands and then used
   strand-agnostic gene-body membership. Reproduce it for timing comparisons,
   but do not inherit that simplification as biological truth.
6. Distinguish allele-specific binding to the spared allele from unrelated-gene
   off-targets. The old paper3 filter excludes the intended gene's entire
   locus (+/-20 bp); that is a legacy policy, not a general solution.
7. Make N/ambiguity handling explicit. Sassy IUPAC treats N as a wildcard.
   The benchmark excluded alignments containing N and split reference records
   at non-ACGT bases. Preserve lowercase but report excluded unknown sequence.
   An assembly gap cannot be declared free of off-targets.
8. No alignments crossing artificial joins, contig ends or transcript-record
   boundaries. Chunk with sufficient overlap: 20+k covers maximum target span
   for a full 20mer alignment within k edits. Deduplicate overlap hits.
9. Define site identity and alignment ties. Many edit alignments describe the
   same site. Raw alignment count is not gene count or necessarily site count.
   Label terminal deletions/partial pairing separately from internal bulges;
   do not silently change their treatment to improve performance.

## Existing software and evidence

- Sassy: https://github.com/RagnarGrootKoerkamp/sassy
- Sassy paper: https://doi.org/10.1093/bioinformatics/btag244
- Sassy2 batch-search preprint: https://doi.org/10.64898/2026.03.10.710811
- Rust API: https://docs.rs/sassy/latest/sassy/
- Alternative exact verifier: Rust-Bio Myers,
  https://docs.rs/bio/latest/bio/pattern_matching/myers/
- Other potential engines: SigAlign (Rust, affine gaps), SeqAn3 (C++, indexed
  approximate searching). These were researched but not benchmarked here.

Tested official Sassy binary: release **v0.2.6**, Apple Silicon asset
`sassy-aarch64-apple-darwin`, using `sassy search --v2`.
The repository README distinguishes Sassy2 from v1 via `--v2`; it is not a
separate executable named sassy2. Pin the version and verify current docs.
Source builds currently document Rust >=1.91 and SIMD CPU flags. Select an
EC2-compatible build, never copy the macOS executable to Linux.

Important Sassy semantics: the CLI's normal search emits representative local
minimum endpoints, not every possible alignment. Library exhaustive endpoint
and alignment APIs exist, with version-dependent names. Inspect the pinned
API and decide which semantics satisfy screen/report. Do not claim all-hit
completeness from a successful CLI run alone. CLI help also warns v1/v2 may
report slightly different reverse-complement matches.

## Completed benchmarks: measured, not extrapolated

Hardware: **Apple M4 Max, 16 physical cores, 64 GiB RAM**. Sassy used **4 CPU
threads**. No GPU.

### Correctness smoke fixture

The included fixtures were tested with both v1 and v2, k=1. Both detected exact
matches, substitutions, single-base insertions and deletions, reverse
complements and lowercase sequence; N-only targets were excluded with
`--max-n-frac 0`. This is a smoke test, not an exhaustive correctness proof.
The 16 fixture queries deliberately repeat one sequence; add distinct-query
cases before relying on batch identity correctness.

### 32 queries, bounded reference

- 32 evenly spaced unique ASOs from the current passing SCN2A candidate pool.
- Approximately 10.19 Mb: first 10 Mb of chromosome 2 plus the SCN2A locus.
- k=2, 4 threads, v2: **0.053731 seconds**, excluding reference preparation.
- 40 raw reported alignments. Three queries had gapped hits in other annotated
  transcribed loci under the historical strand-agnostic classification.
- Tiny, non-random and prefiltered sample. Do not extrapolate this timing or
  infer functional toxicity from these matches.

### 1,000 unfiltered tiles, full annotated gene-body union

- 1,000 unique reference SCN2A 20mers, evenly spaced along its gene body, **not
  prefiltered for off-targets and not allele-specific designs**.
- 412/1,000 overlap soft-masked reference bases.
- Ensembl 110 merged transcribed gene-body search space: **1,761,995,780 bases**
  including chunk overlaps, 36,612 FASTA records. Approximately 860 Mb remains
  lowercase. 969,241 unknown bases excluded; short known runs were also omitted.
- One-time Python reference preparation: **91.17 seconds**.
- **k=2: 189.4004 seconds (3 min 9 sec)** including process startup, reference
  reading, alignment, TSV streaming and Python aggregation.
- **5,604,616 raw reported alignments**, 878,605 containing I or D operations.
- Median 86 raw alignments/query. Top 10 queries account for **53.9%** of output;
  top 100 for **97.6%**. Repetitive queries dominate.
- **k=3 was stopped at the user's request.** Last recorded progress: 14 million
  alignments in 184 seconds. This is not a complete runtime or final hit count.
- No benchmark/search processes or OligoAI queues are running from this session.

Full per-query k=2 counts and original commands are in `benchmarks/`.
Historical off-locus counts use permissive strand-agnostic interval logic and
are raw counts: validate identities, strand and duplicate sites before using
them to classify actual candidates.

A whole-chromosome, unchunked pilot showed ~25 GiB resident memory and one busy
CPU in a sampled process snapshot. The cause was not isolated; do not claim
it was definitively N handling. Chunked reference processing was practical.
Writing a full raw k=2 TSV hit a deliberate **256 MiB file cap** and stopped;
streaming aggregation completed without storing the giant output.

## Performance targets and promising optimisations

**Under 30 seconds for 1,000-query pass/fail screening** was proposed as an
engineering target on the M4 Max, not achieved and not a theoretical limit.
Do not present the earlier 60-100 second multicore estimate as a measurement.

Prioritise measurable changes:

1. Separate rejection-only screening from full reporting. Retire a query after
   a qualifying off-target witness. Avoid CIGAR construction unless needed.
2. Deduplicate queries and shared reference sequence, retaining gene/strand
   metadata and appropriate exclusions. Reuse results across related designs.
3. Reuse a persistent reference/index across batches and genes; compare the
   amortised benefit with Sassy2's scan-based batching.
4. Exploit fixed 20mer length and small k for compact, batched verification.
   Sassy2 already does SIMD/batching; measure before reimplementing it.
5. Consider lossless seed discovery plus a fast edit-distance verifier. The
   existing Hamming seed scheme below does not retain its proof under indels.
6. Consider searching repetitive/high-hit reference sections first to reject
   queries sooner. This must reorder work, not omit it for surviving queries.
7. Assess k=0/1 screening before k=2/3 expansion. Benefits depend on rejection
   rate versus repeated reference traversal.
8. Stream count summaries and a bounded sample of witnesses. Exact full counts
   remain optional and potentially expensive. Never silently cap them.

Do not assume all central-DNA-gap mismatches/bulges prevent RNase H activity.
Such a hard biological rule would sacrifice sensitivity and needs validation.
Do not add ddG thresholds or remove repeats to manufacture a speedup.

## Suggested implementation sequence and acceptance tests

1. Inspect repo, available AWS credentials/region and EC2 resources on the Pi.
   Use EC2 for heavy jobs; keep the Pi as controller. No credentials in logs,
   commits or fixture files. Use existing configured authentication.
2. Establish a small Rust CLI with a pinned Sassy dependency and explicit
   screen/report semantics. It is acceptable to begin with the upstream CLI
   for performance experiments, then use the library for early termination.
3. Download/build a documented human reference on EC2. Keep raw downloads,
   derived indexes and provenance separate. The Mac reference is not in Git;
   use the original Ensembl 110 policy for comparable benchmarks or explicitly
   label any change of release.
4. Add an independent straightforward edit-distance oracle for short fixtures.
   Test distinct batched queries, k=0..3, all substitution positions, insertions
   and deletions in both sequences, combined edits, terminal edits, ties,
   reverse complements, lowercase, unknown bases, chunk boundaries and exon
   junctions (including short exons and multi-junction target windows).
5. Compare hit existence/defined site sets against the oracle and a suitable
   exhaustive Sassy/Rust-Bio configuration. For repetitive queries, verify no
   silent truncation and correct `incomplete` statuses under resource limits.
6. Test on-target exclusion with overlapping opposite-strand genes, shared
   isoform sequence, multiple intended genes and alleles at the same position.
7. Benchmark 1k and 10k queries: unfiltered tiles, low-complexity stress cases,
   and the actual allele-specific SCN2A candidate pool. Compare the same
   reference, k, strand policy, output semantics and thread count.
8. Report cold reference preparation/index construction, warm query runtime,
   peak RSS, threads, CPU model, number of queries/sites, bytes emitted, and
   whether the run completed. Include both screening and report mode.
9. Optimise only a measured bottleneck. Keep an exact baseline and demonstrate
   unchanged sensitivity under the stated model for each optimisation.
10. Produce a usable CLI, reproducible benchmarks, concise README and a clear
    result on whether custom indexing/early rejection materially improves
    throughput. An ASO hit is a potential liability, not validated knockdown.

## AWS execution boundaries

AWS EC2 is explicitly requested. Discover the configured region and available
instances/credentials on the RPi. Do not ask again for routine authorised work.
Use one appropriately sized CPU instance first, not a GPU or an uncontrolled
fleet. SIMD support differs between x86 and ARM; record the actual instruction
set and benchmark it. Check current pricing before selecting a paid instance.
Bound job duration, logs and output; record instance ID, region, launch time,
volumes and restart commands. Follow the user's no-deletion instruction for
any destructive cloud cleanup; stopping a dedicated completed instance is
preferable to leaving compute billing indefinitely. Do not affect pre-existing
instances or workloads. If cost/region choices require user input, ask narrowly.

## Related local work (Mac paths, not guaranteed on RPi)

- `/Users/barneyh/dphil/oligoai-offtarget/`: existing Rust Hamming <=2 scanner,
  transcript reference builders and SCN2A benchmarks. README documents BWA
  repetitive-hit undercounting and a 9mer seed scheme. Read it before porting.
- `/Users/barneyh/dphil/paper2/off-target-investigation/`: chemistry-position
  aware mismatch analysis. Its `gap_aware_analysis.py` means DNA-gap versus
  MOE-wing position, **not gapped sequence alignments**.
- `/Users/barneyh/dphil/paper3/analyses/logic/offtarget_filter.py`: production
  BWA substitutions-only <=1 filter. Existing cache flags come from this.
- `/Users/barneyh/dphil/paper3/analyses/benchmark_sassy*.py`: benchmark scripts.
  Snapshots included under `benchmarks/legacy_scripts/`; they require paper3
  modules and local reference files and are not standalone portable tools.
- `/Users/barneyh/dphil/paper3/data/exports/gene_handoff/`: CSVs for 86 genes,
  all cached-allele 20mers with filter statuses. SCN2A initially 33,223 rows,
  6,338 eligible cached ASOs without a ddG cutoff. Missing off-target status
  remains for some reconstructed uncached indel tiles; resolve before calling
  this a complete eligible pool. Preserve source caches.
- `/Users/barneyh/dphil/paper3/data/exports/gene_handoff/runpod_cache/`: saved
  OligoAI job IDs and predictions. Scoring was paused pending off-target review.
  Predictions remain useful. No model credentials are included in this repo.

The original manuscript latest edition removed the ddG prefilter. Several
older helper/export defaults still used >=4 or >=0, so inspect the actual
calling path rather than trusting stale defaults or comments.

## Definition of success

A tested, useful ASO off-target tool with explicit scope and exactness claims;
measured EC2 performance for realistic query sets; no lost work or silent
filtering changes. A novel core algorithm is optional, not the starting goal.
