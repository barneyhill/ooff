# Runtime estimate correction — 2026-09-08

The earlier “few hours on eight threads” SCN2A estimate is withdrawn as
insufficiently supported. Inspection of the actual retained result confirms
`runs.ooff.output.threads = 1` for the 30.476569099-second 1,000-query full-site
benchmark. It was NOT a measurement using all cores of the larger instance.
Its command also writes every tuple to CSV. Linear scaling to 114,889 queries
would give ~58 minutes at that single-worker throughput, not an eight-thread
runtime; dividing by eight is also unjustified because output locking/I/O and
repetitive-query skew limit scaling.

Separately, the ~3.19-second 100,000-query README result used eight threads in
screen mode (one other-gene witness per ASO), not exhaustive site enumeration.
The ~1.92-second measurement is unique-pair energy computation only for the
1,000-query pilot. None measures the full SCN2A gene walk with all-site ΔΔG.
Full end-to-end latency remains unmeasured; do not repeat a confident hours
estimate or promise seconds based on these different workloads.

# Default global site-energy cache — 2026-09-08

Implemented invocation-wide exact normalized ASO/target energy caching for both
Rust and ViennaRNA, enabled by default with 4,000,000 entries. Capacity zero
opts out; full capacity clears a generation. Concurrent misses may repeat
computation. No persistent cross-process cache or whole-transcript change.

Validation: all 14 ddg_cli tests passed, including live ViennaRNA comparisons
with cache off/default/capacity-one, one/four workers and two-row batches.
The repeated-site fixture spans two reference records and two queries sharing
an ASO with different intended targets. All rows/energies agree with uncached
runs; native energies and ΔΔG also agree with actual ViennaRNA. Both
energy_vienna tests passed, including the 1,825-case live kernel oracle.
Targeted clippy passed. Tests ran locally on ARM; no new EC2 performance
benchmark was run for this cache change. Existing speedup records are historical.

SCN2A planning estimate: full gene-body reference length 197,318 nt, 197,299
20-mer positions before ambiguity filtering; 114,889 distinct valid 20-mers.
The retained full-reference 1,000-query discovery pilot takes around 30 seconds
on the larger historical worker. A linear extrapolation to 114,889 distinct
queries is around 57 minutes for discovery alone on that machine, not a measured
full-walk runtime. Budget a few hours on the eight-thread worker including
annotation/output, pending an actual full-walk benchmark. Mature-transcript
walks have a different, smaller query set. Existing index assumed; index build
excluded. Repetition skew, billions of potential hit rows and output format can
make extrapolation inaccurate. The 1.92-second Rust energy measurement covers
only 2,931,542 unique pairs from the 1,000-query pilot, not end-to-end annotation.
