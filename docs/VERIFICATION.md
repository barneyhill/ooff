# Validation

Tests establish agreement with independent implementations on the tested inputs.
They do not establish biological activity or completeness beyond the supplied
known reference sequence and selected edit budget.

| Component | Validation |
| --- | --- |
| Endpoint and interval discovery | Independent scalar edit-distance oracles at k = 0–3; substitutions, indels, repeats and boundaries |
| FM indexes | Rank and locate checks; full and compact indexes compared with scalar search |
| Genomic-site counts | Bins compared with exhaustive reports; duplicate records, junctions, strands and minimum-distance assignment |
| Parallel counting | Equivalent ordered results across worker counts |
| RNA duplex energy | Pinned complete-duplex and loop-energy fixtures, plus live ViennaRNA 2.7.0 comparisons |
| Reference preparation | FASTA/GTF extraction, minus-strand splice junctions, biotype filtering, compact indexes, cache reuse and failed-rebuild isolation |
| FASTA and exclusions | FASTA/JSONL equivalence, gzip validation, gene symbols/IDs, ambiguity errors and preserved JSONL compatibility |
| CLI contracts | Provenance, partial results, unknown bases, subprocess failures and timeouts |

Run the standard suite from a checkout:

```sh
cargo test --locked
```

For the separately installed live energy oracle:

```sh
RNA_DUPLEX=/path/to/ViennaRNA-2.7.0/bin/RNAduplex \
  cargo test --release --test energy_vienna -- --ignored
```

GitHub Actions runs Rust tests, formatting, Clippy, library documentation checks, source-package verification
and packaged-binary smoke checks on Linux and macOS, x86-64 and ARM64. CI uses
pinned energy fixtures; the live command above requires a ViennaRNA installation.
See [release checks](RELEASING.md).

Human-reference benchmark correctness is assessed separately. Classic-tool
screening candidates are independently aligned against the reference; recovery
is a first-witness metric. Historical exhaustive comparisons check complete
interval sets. [Benchmark methods](CLASSIC_BENCHMARK.md) describe each workload
and its evidence; [the archived audit](archive/VERIFICATION-2026-09-08.md) retains
previous implementation checkpoints.

Reference preparation tests use small synthetic assemblies and GTFs, so CI
requires no mammalian download. The optional network smoke test verifies the
Rust HTTPS and checksum path against small files from each preset's Ensembl
directory, including cache reuse and corrupted-cache rejection:

```sh
cargo test --lib live_ensembl_https_and_checksums_for_all_presets -- --ignored
```

This checks downloader behavior, not a full build of every species assembly.
