# Site ΔΔG runtime estimate — 2026-09-08

User requests an estimate for 100,000 ASOs across the genome and wants ΔΔG to
be the default annotation. No default CLI behavior was changed in this estimate.

Measured the current `oofft-ddg --sites` command on Raspberry Pi 5, four workers,
using 1,000 distinct synthetic ASOs and 10,000 near-match site intervals of
length 17–23. Each interval differs from its intended target by one substitution
or 1–3 inserted/deleted bases. Every component energy and ΔΔG was compared with
direct RNAduplex; all sites and their coordinates matched. One repetition per
layout, suitable for a rough estimate rather than a published scaling claim.

| Layout | Complete annotation time | Approximate sites/s |
| --- | ---: | ---: |
| 10,000 hits in one RNA record | 0.830 s | 12,000 |
| 10 hits each in 1,000 RNA records | 5.554 s | 1,800 |

Timings include query/reference input, intended-target energies, subprocesses,
output spooling and serialization. They do not measure genome discovery,
human-reference indexing/reading, production annotation payloads or large-output
disk behavior. Record distribution affects current process-launch overhead;
these two layouts are not lower/upper guarantees. No EC2 extrapolation measured.

Rough projections for **100,000 ASOs**, allowing for intended-target scoring:

| Mean reported intervals per ASO | Total site energies | Pi ΔΔG annotation estimate |
| ---: | ---: | --- |
| 1 | 100,000 | 15–60 seconds |
| 10 | 1,000,000 | 2–10 minutes |
| 100 | 10,000,000 | 15–90 minutes |
| 1,000 | 100,000,000 | 2–16 hours |

These are conditional projections, not measured end-to-end human-genome times.
The earlier unfiltered SCN2A 1,000-query human RNA-reference benchmark at k=3
produced **45,113,535 intervals per repetition**, median 2,342 per query and
maximum 3,941,769. The 135,340,605 tuple comparison total includes three
repetitions. Repeats and multiple overlapping qualifying intervals matter.
The source is `benchmarks/iterations/20260907T200643-fm-word-union-full-tuples1000/result.json`.
That query distribution is not necessarily representative of another 100k pool;
if its mean did carry over, multi-billion-site annotation would take days on the
Pi, before additional large-scale IO costs.

Reproduce the calibration with:

```sh
python3 benchmarks/ddg/estimate_sites.py --records 1
python3 benchmarks/ddg/estimate_sites.py --records 1000
python3 benchmarks/ddg/refresh.py
```

Raw results, input hashes and commands are retained in `iterations/` and
summarized in `ITERATIONS.md` / `iterations.csv`.
