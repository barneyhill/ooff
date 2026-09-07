# Completion audit — complete

Audit date: 2026-09-07 23:06 UTC (September 8 in Europe/London). The 10×
objective and requested benchmark deliverables are complete within the documented
workload and cache scope. Evidence is summarized in `benchmarks/completion-evidence.json`
and the classic series audit; cold-start limitations remain explicit.

| Requirement | Evidence inspected | Assessment |
| --- | --- | --- |
| Native Rust; Sassy external on EC2 only | `Cargo.toml`, `Cargo.lock`, separate comparator build script | Native dependency graph excludes Sassy. Comparator source/build lives separately. |
| 20mer ASOs, U/T normalization, explicit intended genes | `src/main.rs`, `src/lib.rs`, CLI tests | Implemented; invalid and ambiguous queries rejected. |
| Full-query unit-cost edits, k=0..3, no ddG/position filter | Scalar, FM and CLI tests; query validation | All 19 current Rust tests pass locally and on EC2, including parallel-query equality at 1/2/16 workers. The distance-only verifier is checked against global scalar distance. |
| Optional allele and energy annotations | `Query`, `SiteOutput`, metadata audit, energy test | Allele fields preserved for all 26,643 historical-pool witnesses. Optional `ddg` now retained without filtering; its no-filtering test also passed on EC2. |
| Correct biological orientation | Reverse complement at query preparation; strand/junction tests; paired index mapping tests | Reverse-record index changes traversal only. Oriented RNA matching and original ASO edit coordinates remain intact. |
| Mature transcripts and gene bodies | `build_reference.py`, reference manifest | All exon blocks concatenated in transcript order; 49,145 gene bodies plus 239,274 mature records. Short/multiple junctions retained. |
| Public GRCh38/Ensembl 110 provenance | `docs/reference.md`, downloaded checksums and derived manifest | Exact new primary-assembly files/hashes recorded. Historical Mac reference and later eligible pool are not falsely claimed reproduced. |
| Repeats, unknown bases, artificial joins | Reference builder; scalar/FM oracle cases; no-hit CLI statuses | Soft mask retained; ambiguity splits search and makes no-hit coverage incomplete. Record/chunk boundaries tested. |
| Shared associations and explicit rejection policy | CLI shared-gene fixture; indexed eligibility logic | `other-gene` preserves any off-target gene association. Intended-gene exclusion explicitly does not assess the spared allele. |
| Complete distinct sites and tie semantics | Scalar oracle; exact Sassy tuple audits; README output definition | The final range-union run matched all 135,340,605 literal unique tuples across three full 1k repetitions. Annotated report32 fields and k0..3 exact sites also matched. New parallel scheduling passes fixtures; the human eight-worker full-tuple gate passed at k=0..3. |
| Screen witnesses, caps, partial output | CLI tests and `run_complete` contract | Witness distance is the minimum for its reported interval, not a global minimum across all sites. The new classic audit caught and corrected first-path distance overstatement; the failed pilot is retained. Capped/unknown counts are lower bounds; absent completion marker means partial output. |
| Realistic 1k/10k, stress and actual variant queries | 107 completed ledger entries and raw JSON | Original reference tiles, generated/mixed/random/repetitive sets, actual shortlist32 and older allele-derived 1k/10k/26,643 pool measured. Later 6,338 eligible export unavailable and explicitly distinguished. |
| At least 10× performance | Fresh measurements and comparator source audit | Final full CSV-output repeats measured 325.31 s versus 30.48 s native median (10.67×), with literal equality; whole-process ratio 10.57×. No-CSV full report was 12.94×. All five final warm screen sets matched at 14–185×. Cold end-to-end was slower: 21.56 s native versus 18.71 s Sassy. |
| Fair timing and reproducibility | Iteration harness, source archives, binary hashes, time/resource files | Same reference, queries, mode, k and one search thread. Reused development baselines explicitly labelled. Cold residency separately recorded. The comparator emits engine-only time as diagnostic, while speedup still uses full benchmark search time. |
| Every iteration recorded for plotting | `BENCHMARKS.md`, `benchmarks/iterations/summary.csv` | Includes failures/regressions and optional AVX-512 compiler failure. Per-query results and full outputs retained. All original final gates complete; new classic benchmark has its own iteration ledger. |
| Deliver usable CLI and preserve work | README commands, fixture/EC2 CLI runs, retained volume | Implemented. Original final result metadata synced and worker inventory generated. Final native and classic artifacts were inventoried and collected locally; large raw outputs remain on retained volumes. |
| README image: increasing ASOs at eight threads versus classic tools | README SVG/PNG/CSV, exact witness audits | Measured figure added; ooff and BWA complete all eight sizes through 100,000; BLAST is terminal with recorded timeouts. Every plotted size now has three completed repetitions or a retained timeout/resource failure; minimap2 100,000 has kernel-confirmed OOM on 32 GiB. The plotted and required batch sizes are now powers of ten; the user cancelled further 26,643 repetitions and all earlier artifacts remain retained. Bowtie excluded. Separate index-time/RAM figure added; single-build and hardware caveats shown. |
| Configurable timeout and 100,000 unique ASOs | `test_timeout.py`, `large-batch-manifest.json`, completed native/BWA/BLAST attempt metadata | Cross-stage cancellation passed on EC2. All 100,000 sequences are unique; original 26,643 allele-derived prefix plus 73,357 reference-derived sequences. Minimap2 reached a recorded OOM outcome at 100,000; the timeout did not cause that failure. |
| Gene and genomic-location reporting | Executed fixture `ooff report`; `SiteOutput` | Ten annotated site rows plus `run_complete`; gene/transcript IDs, contig, strand, genomic blocks and edit alignment verified. |
| Stop dedicated EC2 workers; no deletions | AWS instance states and final artifact inventories | All four dedicated workers stopped, verified against AWS state; eight volumes retained. Final inventories and resource state copied locally. No file or volume deletion performed. |

The current reference contains unknown sequence and uses primary assembly only.
These are scope limitations, not evidence of absent biological off-targets.
Record-level site counts are not unique genomic loci or gene counts. No sequence
benchmark establishes knockdown, spared-allele discrimination or clinical safety.


Follow-up Sassy screening audit: all 25 combinations of five tools and five ASO
counts have three completed repetitions or a documented timeout/resource failure.
Sassy's new adapter passed independent scalar fixtures at k=0..3 with one/eight
workers and recovered every query in all 15 human measurements. README now uses
search time only, index-build costs in the caption, and timeout text instead of
triangles. The one-thread full-output 10.67× result remains explicitly distinct
from the new eight-thread 100,000-ASO screening ratio of 6.05×.
