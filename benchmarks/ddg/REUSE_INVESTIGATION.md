# Investigation beyond the 10× DDG result — 2026-09-08

The strongest measured opportunity is reuse across the full reference. The
current ordinary AVX-512 kernel remains the fastest tested kernel. The new
lane-path scheduler is experimental and disabled by default.

## Exact sequence reuse

The retained full-reference pilot contains **1,000 distinct 20-nt SCN2A ASOs** against
288,419 human RNA records (gene-body and transcript records, 2,438,901,225 bases),
using the existing k≤3 discovery scope. This is a census of that pilot, not all
possible SCN2A ASOs or a 100,000-ASO full-reference run.

| Quantity | Exact count |
| --- | ---: |
| Candidate intervals in the first repetition | 45,113,535 |
| Distinct complete ASO/target sequence pairs | 2,931,542 |
| Potential reduction in scoring calls with global exact caching | 15.389× |
| Intervals with ambiguous bases | 0 |

Keys contain the entire normalized ASO identity and target sequence, including
length. The packed representation is collision-free; the hash table also checks
key equality. Duplicate ASOs are canonicalized. The CSV contains three benchmark
repetitions; only the first is counted. Its interval count agrees with the
previously validated native/comparator discovery run. No Sassy code is used by
the census or energy kernel.

Reuse is skewed: the ten highest-hit ASOs contribute 24,480,208 intervals (54.26%)
but only 372,154 unique pairs. The median per-ASO duplicate factor is 1.457×,
and the largest is 202.812×. The 15.389× aggregate factor should not be projected
uniformly onto every ASO or a different design pool.

Global caching would reuse the duplex energy, then calculate each query's DDG
using its own intended-target energy. Every original interval and its metadata
would still be emitted. The production site pipeline currently deduplicates
within bounded batches; the census itself did not implement a global production
energy cache. A subsequent CLI change enables a bounded, invocation-wide cache
by default (see `docs/DDG.md`); the timings here predate that change. The call reduction could benefit ViennaRNA too, so it is separate
from the native-versus-Vienna kernel comparison.

[Census and per-ASO counts](reuse/full-reference-1000-20260908/census.json),
[input hashes](reuse/full-reference-1000-20260908/source.sha256).

## Measured scoring on all unique pairs

Actual ViennaRNA 2.7.0 generated an independent energy for **every one of the
2,931,542 unique pairs**, not just the 100,000-pair sample. Each timed run below
checked every returned energy against that oracle. Times are medians of three
alternating runs with eight workers on the same c7i.2xlarge.

| Engine / experiment | Seconds | Outcome |
| --- | ---: | --- |
| Ordinary native AVX-512 | 1.923879 | Selected kernel |
| Direct unmodified ViennaRNA `duplexfold` | 63.975903 | Native is **33.25×** faster |
| Lane-preserving shared paths | 2.613517 | About 36% slower than matched ordinary SIMD |
| Original adjacent-pack prefix reuse | 1.926264 | Essentially tied with matched ordinary SIMD |

The two experimental rows have their own alternating ordinary-SIMD baselines,
approximately 1.925 s. The 100,000-pair sampled experiment also regressed, by
about 19%; the full-corpus runs were added because thinning removes neighbouring
prefixes. Every repetition and its matching baseline is in the
[iteration ledger](ITERATIONS.md) and [CSV](iterations.csv).

These are scoring times: encoding, length bucketing and native result checks are
included; reading the reference, deduplicating 45 million intervals, JSON
annotation and output are excluded. The census itself took 32.57 s including
reference loading, CSV scanning, sorting and export. Oracle generation took
68.66 s including Python orchestration and file output; that is distinct from
the direct-library measurement above. No 100× end-to-end result is established.

[Independent full-corpus oracle provenance](reuse/full-reference-1000-20260908/oracle-all/provenance.json).

## Why the attempted SIMD sharing did not help

Sorting all unique targets within each ASO would allow the scalar algorithm to
reuse 37,524,536 of 57,492,634 columns (65.27%). It recomputes the final shared
column because dangling and mismatch terms inspect the following base.

The new scheduler keeps each lane on an ASO's neighbouring target path, selecting
similarly sized paths and splitting long ones without padded calculations.
However, all lanes must resume from their shortest shared prefix. It reused
only 8,074,403 columns (~14%). The older same-ASO adjacent packs reused
12,319,064 (~21%). Path scheduling also changes which divergent cases share a
vector and adds bookkeeping. Neither version yielded a useful runtime gain.

## Next kernel design: a prefix-trie frontier

A genuine frontier kernel would calculate each distinct target-prefix node once,
batching nodes at the same depth rather than repeatedly advancing complete
sequence pairs. Keeping the core DP column independent of the following base
would allow its children to share the entire prefix; child edges would supply
the dangling/mismatch lookahead terms. Terminal energies would remain specific
to each complete target, including targets that end at an internal trie node.

The exact work-count model gives:

| Work model | Column slots |
| --- | ---: |
| Independent pairs | 57,492,634 |
| Distinct trie nodes | 17,040,555 |
| Trie frontier padded to 16 lanes per ASO/depth | 17,214,096 |

That is **3.34× fewer column slots** even with padding. This is a work-count
model, **not an implemented energy kernel or a predicted 3.34× runtime gain**.
Ancestor-state gathers, table lookups, sorting and memory traffic remain. A
prototype should use bounded subtrees and compact ancestor storage so shared
state stays near the CPU; a large pointer-heavy global trie could easily lose
its arithmetic advantage. Each node needs exact interior-loop predecessors and
correct terminal/lookahead handling, not an approximate alignment band.

[Machine-readable frontier counts](reuse/full-reference-1000-20260908/frontier-stats.json).

100× raw scoring remains an ambitious research target: this state reduction
provides a plausible arithmetic route from the measured 33×, but the memory and
scheduling costs need an actual kernel and the same Vienna verification. Global
exact energy caching is the more direct workflow improvement. Neither its
15.4× call reduction nor the trie state factor should be multiplied into the
measured 33.25× and presented as achieved performance.

## Reproduce

On the retained EC2 checkout, with the existing public-reference inputs:

```sh
cargo build --release --example energy-reuse-census --example energy-bench
python3 benchmarks/ddg/check_reuse_census.py
# DDG_REUSE_OUT must name a new directory; experiments are never overwritten.
DDG_REUSE_OUT=data/reuse/my-census
./target/release/examples/energy-reuse-census \
  /home/ubuntu/ooff/data/reference-v1/reference.fa \
  /home/ubuntu/ooff/benchmarks/sassy_1000/SCN2A_unfiltered_1000.fa \
  /home/ubuntu/ooff/benchmarks/iterations/20260907T200643-fm-word-union-full-tuples1000/sassy.sites.csv \
  "$DDG_REUSE_OUT" 100000
./target/release/examples/energy-reuse-census expand "$DDG_REUSE_OUT" "$DDG_REUSE_OUT/all-pairs.jsonl"
./target/release/examples/energy-reuse-census frontier-stats "$DDG_REUSE_OUT"
python3 benchmarks/ddg/oracle_pairs.py --input "$DDG_REUSE_OUT/all-pairs.jsonl" \
  --output "$DDG_REUSE_OUT/oracle" --rnaduplex vienna/bin/RNAduplex --threads 8
python3 benchmarks/ddg/run_kernel.py --label full-reference-shared-paths \
  --cases "$DDG_REUSE_OUT/oracle/cases.json" --paths --threads 8 --repetitions 1 --runs 3 \
  --baseline target/release/examples/energy-bench --baseline-batch --baseline-max-lanes 16
python3 benchmarks/ddg/refresh.py
```

Use `--shared` for the original prefix scheduler or `--batch` for ordinary SIMD.
The direct Vienna comparator source is `vienna_kernel_bench.c`; its binary/input
paths and hashes are recorded in the full-reference timing artifact. Packed keys,
expanded pairs, actual Vienna output and source snapshots remain on the retained
worker volume; small census/provenance files and every timing repetition are
also local. No failed or slower experiment was deleted.
