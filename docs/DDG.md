# Relative hybridization energy (ΔΔG)

## Annotate discovered off-target sites (default report workflow)

Use `--energy-engine rust` for independent native site scoring without calling
or linking ViennaRNA. It evaluates the pinned RNA:RNA Turner 2004 model at 37 C.
The Vienna backend remains available and is still selected when no engine flag
is supplied. Whole-transcript mode uses ViennaRNA RNAplex and requires
`--energy-engine vienna`.

The native engine matches 1,825 pinned complete duplex energies and 11,160,000
loop-energy evaluations, with additional live ViennaRNA 2.7.0 checks and complete
real-site comparisons. The following recorded benchmark used the earlier
intrinsic SIMD backend; the current runtime backend is described below. On EC2 c7i.2xlarge (Xeon Platinum 8488C; four physical
cores, eight hardware threads), annotating **273,825 sites from 100,000 distinct
SCN2A ASOs against SCN1A pre-mRNA** measured:

| Workers for each engine | Rust median | Vienna median | End-to-end speedup |
| ---: | ---: | ---: | ---: |
| 4 | 1.106856 s | 11.809776 s | **10.67×** |
| 8 | 1.001300 s | 9.758840 s | **9.75×** |

Each row is three alternating runs of the same executable with different engine
flags. Every output field in every report row matched. Eight workers gives the
lowest native runtime; four workers gives the larger relative speedup because
Vienna benefits more from the extra hardware threads. The separate eight-worker
compute-only comparison against direct Vienna `duplexfold` measured **28.14×**.

End-to-end times include startup, provenance hashing, reference indexing, intended
and off-target energies, parsing, serialization, the private output spool and
stdout emission to regular files. They use warm-cache buffered I/O; earlier dirty
writes are drained before each timed run. Discovery and durable-storage flushes
are excluded. This is a real transcript-coordinate workload, not a measurement
of all human-reference off-targets. The [ledger](../benchmarks/ddg/ITERATIONS.md)
and [CSV](../benchmarks/ddg/iterations.csv) retain all iterations and regressions.

Reproduce with the retained input on the EC2 checkout:

```sh
cargo build --release --locked --bin oofft-ddg
python3 benchmarks/ddg/run_real_sites.py \
  --input data/ddg-real-sites/20260908T103353.260461-scn2a100000-scn1a \
  --binary target/release/oofft-ddg --baseline target/release/oofft-ddg \
  --baseline-engine vienna --rnaduplex vienna/bin/RNAduplex \
  --threads 4 --runs 3
python3 benchmarks/ddg/refresh.py
```

Use `--threads 8` for the other row. The input-generation script is
`benchmarks/ddg/prepare_real_sites.py --queries 100000`; it uses the pinned SCN2A
and SCN1A FASTAs prepared by `prepare_rpi.py`. Input and executable hashes,
Vienna version, commands and raw profiles accompany the results. Full annotated
outputs remain on the retained EC2 volume; result records and profiles are also
stored locally. [Optimization notes](../benchmarks/ddg/OPTIMIZATION_NOTES.md)
map each algorithm change to its retained iteration.

Native implementation and empirical table provenance: [energy module](../src/energy/README.md).

```sh
cargo build --release --locked --bin oofft-ddg
./target/release/oofft-ddg --sites hits.jsonl --queries queries.jsonl \
  --reference reference.jsonl --energy-engine rust --threads 4 --timeout 600 > annotated.jsonl
```

`--sites` consumes an `oofft report` JSONL file, including its manifest and final
`run_complete` marker. Supply the **same** FASTA/JSONL queries and JSONL/FASTA reference as
discovery; SHA256 checks prevent annotating coordinates against another reference.
Capped reports retain their incomplete/count-lower-bound status. Every supplied
site is annotated; none is filtered, merged or moved. Processing uses bounded
row batches (`--batch-size`, default 1024), a reference byte-offset index, and a
retained output spool. Site mode processes one task per worker, prefetching the
next wave and spooling the previous wave concurrently. Buffering is limited to
two input/processing waves plus one output wave. Intended energies are shared
across workers and computed once per query. No stdout is emitted until calculation and input validation
succeed. The reference is indexed on each annotation invocation; no new persistent
search index is built.

For each query, `target` (alias `targetDnaSequence`) supplies the intended RNA
sequence in 5′→3′ order. If omitted, the ASO's perfect complement is the reference
duplex and `on_target_basis` is `perfect_complement`. Supply `target` when the
intended allele is not perfectly complementary. This field can be present in
the original discovery queries. Intended-target energy is computed once per
query in site mode, and reused across its hits.

Site scoring uses **ΔG(ASO, hit interval) minus
ΔG(ASO, intended target)**, in kcal/mol. The hit interval is taken directly
from the report's zero-based half-open RNA-sense coordinates. Its length can
differ from ASO length because of indels. No flank is added and no exact-match
shortcut is applied. RNAduplex optimizes a duplex within these sequences;
it is **not constrained to the discovery CIGAR or to pairing every base**.
This is an interval-specific energy, not the energy of a prescribed edit alignment.

Each site receives a `ddg` plus an `energy_annotation` object containing component
energies, calculation scope, model, sign, scored interval and on-target basis.
An earlier caller-supplied `ddg` is preserved as `supplied_ddg`.

Add **`--whole-transcript --energy-engine vienna`** for full-record scoring:
RNAduplex for the intended pair, RNAplex's best interaction anywhere in the
supplied full RNA record, and an exact-match shortcut. The reported site
coordinates remain unchanged; the best RNAplex position is stored separately.
Multiple sites for an ASO in one record can consequently have the same score.
The flag does not reconstruct mature transcripts from genomic gene bodies:
the input record defines the scan scope, including pre-mRNA when supplied.

Both modes use ViennaRNA's RNA:RNA defaults, not a modified-gapmer energy model.
Site-mode workers stay within `--threads`; whole-transcript mode retains up to
that many RNAplex processes plus one RNAduplex process.

## Scientific context and sign

[Watt et al. (2020)](https://doi.org/10.1089/nat.2020.0847) studied 96 gapmer ASOs
and 832 nearly matched unintended transcripts. Relative hybridization energy was
associated with off-target activity and relative potency. Their RNA:RNA
predictions used RNAstructure; RNA:RNA destabilization was a proxy for changes
in modified ASO:RNA binding, not an absolute chemistry-specific energy model.
They define destabilization as mismatched minus matched duplex energy.

oofft reports **ΔΔG = off-target ΔG − intended-target ΔG**, following Watt et al.
Positive values mean weaker off-target binding; negative values mean stronger
off-target binding. For example, −15 − (−20) = +5 kcal/mol. This applies to site
annotation and whole-transcript mode, with both energy engines.

The `ddg_sign` metadata records `dg_other - dg_target`. ViennaRNA
energies are not the paper's RNAstructure values.
The default installed ViennaRNA 2.7.0 model is RNA:RNA at 37°C; MOE/cEt/PS chemistry
is not explicitly modeled. No energy cutoff is applied to oofft candidate search.

Whole-transcript mode uses an exact-match shortcut: if the intended target k-mer
occurs anywhere in the off-target sequence, `ddg` is zero, both component energies
are null, and the first 1-based target position is returned. This is an explicit
application rule, not a claim that context-dependent duplex energies are equal.
A too-short off-target returns unknown values. RNAplex coordinates use a 1-based target-start convention; they do not imply a fixed
physical duplex span equal to the ASO length.

## Standalone full-record scoring

With ViennaRNA RNAplex and RNAduplex on PATH:

```sh
oofft-ddg --queries queries.jsonl --off-target other-transcript.fa \
  --threads 4 --timeout 600 > energies.jsonl
```

Each query has `id`, `aso` and `target`, both sequences written 5′→3′. The aliases
`asoDnaSequence` and `targetDnaSequence` are accepted. Intended target and ASO
must have the same length; different queries can have different lengths. Supply
one off-target RNA record. `--rnaplex` and `--rnaduplex` override executable paths.

Results include `ddg`, `dg_target`, `dg_other`, `exact_off_target_match` and
`dg_other_position`, in input order. Energies use kcal/mol. Failed subprocesses
or timeouts produce nonzero exits. The completion manifest is written to stderr
and the work directory. `--work-dir` selects a new directory for retained inputs
and raw engine output.

## Validation

Pinned fixtures cover 1,825 complete duplex energies and 11,160,000 loop-energy
evaluations. Live tests compare against ViennaRNA 2.7.0:

```sh
RNA_DUPLEX=/path/to/ViennaRNA-2.7.0/bin/RNAduplex \
  cargo test --release --test energy_vienna -- --ignored
```

The site oracle (`benchmarks/ddg/verify_sites.py`) independently scores discovered
intervals with RNAduplex and checks energies and preserved report metadata,
including 17–23 nt intervals for 20 nt ASOs. CI also checks subprocess contracts,
missing values and timeouts. Historical full-record compatibility comparisons
and every benchmark attempt remain in the [energy ledger](../benchmarks/ddg/ITERATIONS.md).

## Parallel execution and caching

The native engine uses `pulp` for runtime SIMD dispatch: AVX-512/AVX2 on supported
x86 CPUs, NEON on AArch64, and scalar fallback. It groups independent duplexes by
sequence length and distributes scoring across `--threads` workers. The same
thermodynamic recurrence and parameter tables apply to all backends.

Site scoring enables a shared cache by default. `--energy-cache-pairs 4000000`
sets its entry limit; `--energy-cache-pairs 0` disables it. Keys contain complete
normalized ASO and target sequences. The cache spans records, batches and workers
within one invocation, and resets when full. It reuses energies while preserving
every output site and query-specific intended-target score. The entry limit is
not a memory limit. Whole-transcript RNAplex scoring is separate.

Backend-specific timing results and experimental optimizations are documented
in the [SIMD measurements](../benchmarks/ddg/PULP.md) and
[optimization notes](../benchmarks/ddg/OPTIMIZATION_NOTES.md). Earlier intrinsic
backend timings should not be interpreted as fresh measurements of the pulp
backend. Full-reference discovery and output costs are excluded from kernel-only
comparisons.
