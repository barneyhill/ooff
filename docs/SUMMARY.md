# Default off-target summaries

`oofft` defaults to exhaustive per-ASO counts at edit distances 0, 1, 2 and
3. Each `query_summary` contains `edit_distance_counts` and `total_sites`.
It does not calculate ddG or emit individual hits, gene IDs or alignments.
Explicit `oofft summary` is equivalent to omitting the positional mode.
Summary supports mixed ASO lengths from 4 to 63 nt. Screen and report currently
require 20mers; the human-reference measurements here use 20mers.

```sh
oofft --queries queries.jsonl --reference reference.fa \
  --index index --reverse-index reverse-index \
  --annotations records.jsonl --annotation-cache annotation-cache.json \
  --policy other-gene --reference-release Ensembl110-GRCh38 \
  --scope 'Human gene-body and transcript records' \
  --biotype-policy 'retained reference manifest' --threads 8 > counts.jsonl
```

All flags describing the reference retain their existing meaning. The reverse
index and annotation cache are optional. Indexed summaries default to the
available logical CPUs, capped by the number of queries; `--threads` overrides
this. The reference and indexes are opened once and shared by the workers.

## What is counted

The default `--count-unit genomic-site` identifies a site by contig, strand and
ordered genomic blocks. Identical sites appearing in a gene-body record and
several transcripts count once per ASO. Adjacent blocks are coalesced; different
junction paths remain separate. If duplicate records give different distances,
the site belongs to its minimum-distance bin. Overlapping intervals with
different boundaries still count separately: these counts are neither numbers
of genes nor a clustering of nearby hits.

Only records associated with at least one gene outside the ASO's intended-gene
list contribute. Gene annotations are still required internally for exclusion
and genomic mapping, even when gene IDs are not included in output.

`--count-unit record-interval` counts distinct record/start/end intervals without
cross-record deduplication. It is faster and uses constant-sized counters, but
can count the same biological location several times. It is useful for comparing
counts with the existing detailed report.

## Optional detail

- `--genes` adds the sorted set of other-gene IDs to each summary.
- `--sites` emits the full annotated site report, equivalent to `oofft report`.
- `oofft screen` retains its existing first-witness behaviour; it is not exhaustive counting.

Detailed reports retain their established reference-record interval identity.
Their row count can therefore exceed the default distinct-genomic-site count.
Energy scoring is available through the separate [ΔΔG annotation tool](DDG.md).

When `-k` is below 3, bins above the searched distance are `null`, not misleading
zeroes. Unknown reference bases are excluded and set `counts_complete: false`.
A missing terminal `run_complete` means the output is incomplete. Summary mode
rejects `--max-sites`, because capped output would not establish exhaustive bins.

## Implementation and limits

Indexed counting uses the FM search's distinct minimum-distance intervals. It
avoids reference-sequence rereads, traceback, CIGAR/edit construction and per-hit
serialization. Exact genomic deduplication retains only the active query's keys;
its memory grows with that query's distinct sites. A fixed pool processes queries
through bounded work/output queues and writes summaries in input order. The
number of pending results is limited to twice the worker count.

The non-indexed JSONL fallback processes one ASO at a time and rereads the
reference for each ASO to bound deduplication memory. Use a prebuilt index for
human-reference workloads. Summary output reduces I/O substantially but does
not remove the work of locating and deduplicating matching sites, or the index
working set.

Tests compare bins against exhaustive site reports, with and without the reverse
index and annotation cache, at all supported edit budgets. They cover duplicate
records, adjacent blocks, junctions, opposite strands, unknown sequence,
minimum-distance assignment and ordered results across multiple worker windows.
Measured attempts and reproducible commands are in
[the summary benchmark ledger](../benchmarks/summary/ITERATIONS.md).

## Measured SCN2A run

The full repeat-filtered SCN2A gene-body design set (including introns) completed
in **438.953 seconds (7 min 19 s)** for **114,889 distinct 20mer ASOs** on EC2
m7i.2xlarge, eight logical CPU threads and 32 GiB RAM. One native default-summary
invocation searched the existing Ensembl 110 human gene-body/transcript reference
at up to three edits; no ddG or individual site output was requested.

Output was **28,812,047 bytes (28.8 MB)**. Maximum resident size was
13,789,412 KiB (about 13.15 GiB), including mapped reference/index pages. Index
loading took 0.361 s; search and output took 437.895 s. The OS cache was retained
from earlier work; this is not a controlled cold-cache run. Index construction
and compilation are excluded. This is a single complete measurement, not a median.

Counts summed across ASOs were 16,189 at distance 0; 341,556 at 1; 12,851,473 at
2; and 345,830,615 at 3. These are ASO/site pairs, not unique loci across all ASOs.
The reference contains 970,360 unknown bases, so summaries accurately mark coverage
incomplete there. Reference scope is gene bodies and transcripts, not intergenic
whole-chromosome sequence.

[Raw timing, command and provenance](../benchmarks/summary/iterations/20260908T161125-scn2a-genomic-default/result.json),
[resource usage](../benchmarks/summary/iterations/20260908T161125-scn2a-genomic-default/resources.json),
and [plotting CSV](../benchmarks/summary/iterations.csv) are retained. Full output
is retained locally and on the worker volume; its SHA256 and aggregate-bin check
are recorded alongside the timing. Independent count-equivalence tests passed
on ARM and x86 fixtures; this was not an independent re-enumeration of every
SCN2A site.

## Compact index for smaller machines

Convert a full index once on the build machine; this preserves exact search
results and trades additional suffix-position lookup work for less memory:

```sh
oofft-index compact --index index --output compact-index --sample-rate 16
# Use --index compact-index when searching. A reverse index is optional.
```

The Ensembl 110 forward index occupies **2,896,537,944 bytes (2.90 GB)** in the
sampled format. Conversion took **97.061 s** on the EC2 worker, separately from
search; the original suffix-array build is still required and is not included
in that conversion time. Both index formats remain readable.

On an **8 GiB Raspberry Pi 5**, four threads processed **1,000 evenly spaced
SCN2A 20mers in 75.214 s**, with peak RSS **3,147,152 KiB (3.00 GiB)**. All 1,000
summaries exactly matched the full-index EC2 run. OligoAI remained running.
This was the first search after transferring the reference; the OS cache was
not explicitly cleared. A repeat with the same query set took **61.157 s**,
with **3,141,776 KiB** peak RSS and identical results. These sample timings
project roughly 2–2.4 hours for 114,889 ASOs, but that is not a measured full run.
It is a sample measurement, not a full-gene memory or
latency guarantee. Per-query genomic deduplication can use more RAM on unusually
repetitive ASOs, and multiple simultaneous CLI processes add memory pressure.

A diagnostic EC2 compact-index sample took 16.863 s and 3,184,268 KiB RSS, but
ran concurrently with a reference download. Do not treat that as a clean
performance comparison. The ledger retains this caveat and both raw outputs.

Copied indexes bind to the original FASTA path and modification time. After
moving an index and its byte-identical FASTA, verify the entire reference hash
and bind its new location once, then regenerate the annotation cache:

```sh
python3 scripts/relocate-index.py --index compact-index --reference reference.fa
oofft-index cache-annotations --index compact-index --reference reference.fa \
  --annotations records.jsonl --output annotation-cache.json
```

Keep the indexed FASTA, index shards and annotations immutable during searches.
The relocation script changes only reference metadata after SHA-256 verification;
it does not rebuild or modify the index shards.

The final local integration validation repeated the same 1,000 queries with a
**5 GiB address-space limit** and lazy FASTA mapping: **45.816 s**, **3,137,920 KiB
(2.99 GiB) peak RSS**, all summaries identical. This followed earlier runs with
retained OS cache; do not attribute the entire timing difference to lazy mapping.
The limit is an OligoAI child-process setting, not a universal oofft memory promise.
Counts do not map FASTA sequence; screen/report map it lazily when needed.
