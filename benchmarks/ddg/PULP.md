# Portable ddG SIMD with pulp — 2026-09-08

The default `energy-batch` feature now uses pinned pulp 0.22.3 on stable Rust.
One generic integer recurrence replaces the separate AVX2/AVX-512 operation
wrappers. Runtime dispatch selects x86 V4 (16 i32 lanes), V3 (8), AArch64 NEON
(4), or scalar. Incomplete groups use the existing scalar workspace. The
shared-prefix and min-plus experiments also use the generic recurrence and
remain disabled by default. Global energy caching and Watt-sign ddG are unchanged.

This is a kernel migration. It does not integrate discovery and annotation into
one streaming CLI or reduce the full-reference index working set.

## Measurements

Raspberry Pi 5, Cortex-A76, 64-bit ARM; one worker; three alternating repetitions
of each binary, with every energy checked against the retained oracle values.
The native build used the repository's `target-cpu=native` setting. The baseline
is the previous production batch path, which used scalar calculations on ARM.
Timers include encoding, grouping, DP, worker creation and result checks;
exclude file parsing/startup. No concurrent compilation or test workload ran
during these timings.

| Workload | Passes per run | Previous median s | pulp NEON median s | Speedup |
| --- | ---: | ---: | ---: | ---: |
| 1,825 diverse oracle pairs | 10 | 1.151240257 | 1.004052257 | 1.147× |
| 20,543 unique real-site pairs | 3 | 2.643162365 | 1.244699429 | 2.123× |

All individual timings, commands, input/binary SHA256s and stdout/stderr are in:

- [Diverse fixture iteration](iterations/20260908T154414.515307-pulp-neon-golden/result.json)
- [Real-site iteration](iterations/20260908T154421.822452-pulp-neon-real-unique/result.json)
- [Combined Markdown ledger](ITERATIONS.md) and [plotting CSV](iterations.csv)

The original timed binaries are retained under `data/ddg-pulp/20260908/`.
The raw commands refer to their original paths; use these retained copies when
reproducing that exact revision. To benchmark a newly built revision:

```sh
cargo build --locked --release --example energy-bench
python3 benchmarks/ddg/run_kernel.py --label pulp-neon-golden --batch \
  --baseline data/ddg-pulp/20260908/pre-pulp-energy-bench --baseline-batch \
  --repetitions 10 --runs 3
python3 benchmarks/ddg/run_kernel.py --label pulp-neon-real-unique --batch \
  --baseline data/ddg-pulp/20260908/pre-pulp-energy-bench --baseline-batch \
  --cases data/ddg-real-sites/20260908T103345.681632-scn2a10000-scn1a/energy-cases-unique.json \
  --repetitions 3 --runs 3
python3 benchmarks/ddg/refresh.py
```

The real-site input is a retained local benchmark artifact; the diverse fixture
ships with the repository. These timings do not imply full-human latency.
Previous published x86 timings describe the previous intrinsic backend. The
pulp backend's x86 throughput still needs a matched measurement: its portable
parameter gather currently performs per-lane reads because pulp has no generic
gather operation. Do not transfer the old AVX-512 speedups to this implementation.

## Validation

The generic ARM release build passed all 42 repository tests, including the
pinned energy fixtures, experimental recurrence checks, and all 15 ddG CLI
tests with live ViennaRNA 2.7.0 comparisons. Scalar fallback tests and strict
all-target clippy also passed. The energy module passed a generic x86-64
cross-compilation check; x86 execution/performance has not yet been tested.
Tests now exercise limits 1/4/8/16, scalar fallback, tails, mixed lengths, empty
and invalid sequences, and workspace reuse across experimental mode changes.
AArch64 tests assert that default dispatch actually selects four NEON lanes.

```sh
RNA_DUPLEX=/path/to/ViennaRNA-2.7.0/bin/RNAduplex \
  cargo test --release --test energy --test energy_vienna --test ddg_cli -- --include-ignored
RUSTFLAGS='-C target-cpu=generic' RNA_DUPLEX=/path/to/RNAduplex \
  cargo test --locked --release --all-targets -- --include-ignored
cargo test --locked --no-default-features --test energy --test energy_vienna
cargo clippy --locked --all-targets -- -D warnings
```

CI already tests x86 and ARM generic builds; it now also tests the explicit
no-default-features scalar fallback on each existing platform. Live Vienna tests
require the external pinned executable; pinned oracle fixtures run without it.
