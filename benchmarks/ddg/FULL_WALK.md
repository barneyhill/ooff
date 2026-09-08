# SCN2A full gene-body walk — 2026-09-08

The complete timed run is in progress. No extrapolated final runtime is reported.

## Exact workload

- Ensembl110 SCN2A gene body: 197,318 nt, introns included.
- 20-nt windows, one-base step; exclude any window overlapping lowercase repeat
  masking or non-ACGT sequence. 114,926 positions /114,889 distinct ASOs.
- Search the retained human reference including repeat sequence: 288,419 RNA
  records (gene-body + transcript), 2,438,901,225 bases. Not intergenic chromosome
  sequence. Other-gene policy, exhaustive k≤3 intervals, no hit cap.
- Every eligible position retained in origins.jsonl; every distinct ASO searched
  once. Every hit row, alignment, annotation and Watt-sign ΔΔG retained in gzip.
- Eight worker slots; each slot runs existing discovery CLI, native DDG CLI and
  archival sequentially. ASOs belong to one chunk only, so chunking loses no
  full ASO/target cache reuse. Default 4-million-pair cache in each DDG invocation.
- Prebuilt indexes; build/EC2 boot time excluded. Initial OS caches were not
  explicitly warmed. Output archival and repeated per-process setup included.

## Retained iterations

1. Repeat-inclusive 196,063-ASO stress test stopped after about 6m38s to prioritize
   the clarified design pool. Counts-only discovery; incomplete, no DDG result.
2. Full JSONL report attempt on16GiB stopped after measured79–83%IO wait. Paired
   indexes total22GiB and do not fit. Raw partial outputs retained.
3. Forward-only/chunked16GiB baseline retained7 complete chunks before stopping
   dispatch to address whole-record rereading in DDG. Not a full walk result.
4. Direct FASTA interval-read optimization:15CLItests pass onARMandEC2, including
   actualViennaRNA. Entire527,956-site saved chunk byte-identical to baseline.
   Isolated check28.294506s; not a matched speedup comparison. Source/binaries
   and result hashes retained.
5. **Current full run:** m7i.2xlarge,8logical/4physical Xeon8488C,32GiB; paired
   indexes;225chunks of512ASOs. Dedicated200GiB output gp3,6000IOPS/500MiB/s.
   Artifact `/mnt/oofft-walk/20260908-full-scn2a-paired-direct` on project EC2.

See [iteration ledger](ITERATIONS.md), [CSV](iterations.csv), and the reproducible
[runner](walk_chunked.py). Interrupted attempts remain explicit, not treated as
complete timings. Stage durations overlap across workers; only total wall time
answers end-to-end latency. Historical seconds-scale screening finds one witness
per ASO; it is not this exhaustive workload.

## Independent verification

`check_full_walk.py` compares all198synthetic fixture site rows between monolithic
and chunked pipelines and verifies lossless archive hashes. `verify_walk_samples.py`
checks16distinct ASO/target pairs from the beginning of every finished chunk
against actualViennaRNA2.7.0, after timing completes. This is explicitly sample
verification across the gene walk, not an all-hit Vienna comparison. Its fixture
check passed48sampled sites/49distinct oracle pairs across3chunks.

## Runtime diagnosis at 179/225 completed chunks (2026-09-08)

91,648 queries produced 332,611,668 candidate intervals. Retained archive
metadata totals 710,783,954,296 uncompressed bytes across discovery and annotated
reports, compressed to 59,483,130,891 bytes. These are completed chunks only.
Cumulative worker-stage seconds: discovery 5,514.679; ddG process 16,950.306;
archive 7,679.503. Their shares are approximately 18.3%, 56.2%, 25.5%; these
are sums across concurrent slots, not additive wall-clock durations.
Within ddG, cumulative setup is 7,684.929 s, duplex stages 1,173.919 s,
input 2,405.399 s, spool 1,522.585 s. Internal timers overlap; do not sum them.
Duplex-stage time is only about 3.9% of summed top-level worker durations.
The wrapper repeatedly scans/hashes the reference and materializes then
compresses both full JSONL reports. Thus this is dominated by orchestration
and output handling, not the duplex recurrence alone. No pulp binaries were
used in this run. Full-walk completion still pending at this checkpoint.
