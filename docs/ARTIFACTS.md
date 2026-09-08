# Benchmark artifacts

Small measurement records, hashes and plotting inputs are versioned. Large
reference files, indexes and raw alignment outputs are retained separately and
are not distributed in the crate.

| Location | Contents |
| --- | --- |
| `benchmarks/NATIVE_ITERATIONS.md` | Native search iterations, including unsuccessful attempts |
| `benchmarks/iterations/` | Native timing, validation and resource records |
| `benchmarks/classic/iterations/` | Comparator stages and verified screening records |
| `benchmarks/ddg/iterations/` | Energy timing, oracle results and profiles |
| `benchmarks/summary/iterations/` | Exhaustive-count timings and validation |
| `docs/images/*.csv` | Figure observations, including failed and censored attempts |

Paths in result JSON files identify the original input, executable and output
artifacts. `screening.json` connects a composite measurement to its timed stages.
SHA-256 hashes identify reference and query content where recorded. Inventory
files record names and sizes but do not replace content hashes.

The [benchmark overview](../BENCHMARKS.md) links methods and regeneration commands.
Historical storage locations and infrastructure inventories are preserved in
[the archive](archive/ARTIFACTS-2026-09-08.md).
