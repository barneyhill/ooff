# Benchmarks

Measurements cover three distinct tasks. Compare results only within the same
reference scope, output requirements and hardware configuration.

| Task | Representative result | Methods and data |
| --- | --- | --- |
| First-witness screening | 100,000 ASOs: oofft 3.19 s, Sassy2 19.32 s; eight threads, medians of three runs | [Classic-tool comparison](docs/CLASSIC_BENCHMARK.md), [CSV](docs/images/classic-scaling.csv) |
| Exhaustive genomic-site counts | 114,889 SCN2A ASOs: 438.95 s; eight threads, one full run | [Summary methods](docs/SUMMARY.md), [iteration ledger](benchmarks/summary/ITERATIONS.md) |
| Site ΔΔG annotation | 273,825 sites: Rust 1.107 s, ViennaRNA 11.810 s; four workers, medians of three runs | [Energy methods](docs/DDG.md), [iteration ledger](benchmarks/ddg/ITERATIONS.md) |

Screening and counting use Ensembl 110 human gene-body and transcript records.
The energy comparison uses SCN2A ASOs and SCN1A pre-mRNA sites; discovery is
excluded. The methods describe cache state, memory, reference exclusions and
correctness checks. Index builds are reported separately from search.

## Reproduce the figure

From the repository root, with Python and Matplotlib installed:

```sh
python3 benchmarks/classic/plot.py
```

This regenerates the README's SVG, PNG and CSV from retained measurement records.
Completed points require at least three repetitions. Failed and censored runs
remain in the exported data; timings are never interpolated or fabricated.

## Experiment records

Every recorded attempt is retained, including regressions, interruptions,
timeouts and resource failures. Historical artifacts may use the former `ooff`
name and describe earlier implementations.

- [Native search development ledger](benchmarks/NATIVE_ITERATIONS.md)
- [Classic-tool ledger](benchmarks/CLASSIC_ITERATIONS.md)
- [Annotated CLI ledger](benchmarks/CLI_ITERATIONS.md)
- [Summary ledger](benchmarks/summary/ITERATIONS.md)
- [Energy ledger](benchmarks/ddg/ITERATIONS.md)
- [Index build time and memory](docs/images/classic-index-builds.csv)
