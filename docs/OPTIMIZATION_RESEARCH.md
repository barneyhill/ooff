# Further optimization research — 2026-09-07

## Relevant primary sources

- [Columba, Bioinformatics, December 2025](https://pmc.ncbi.nlm.nih.gov/articles/PMC12724072/): optimized search schemes constrain cumulative edit budgets across query pieces; dynamic partitioning chooses useful boundaries. Its bidirectional traversal can start inside a query. Our two independent half-limited passes are simpler and cannot yet implement that traversal directly.
- [b-move, Algorithms for Molecular Biology, 2025](https://link.springer.com/article/10.1186/s13015-025-00281-x): combines compressed bidirectional indexing, search schemes, and exact 10-mer lookup tables. Its reported experiments use 151-base reads; performance does not establish a speedup for our 20-base ASOs.

These are algorithm references, not native dependencies. No third-party implementation code was copied. Sassy remains an external EC2 comparator only.

## Experiments and priorities

1. **Lazy exact 10-mer cache (implemented, validation pending).** Cache only exact ranges actually needed while canonicalizing reverse-search leaves. Costs are charged to each query; no unmeasured index construction or persistent warm cache. The cache avoids ten repeated backward extensions on a hit and preserves all intervals/minimum distances. Optional `ooff-index search --word-cache`, full-site paired mode only. Baseline remains default until measured. Tests compare every interval against an independent scalar oracle, including mixed indels, repeats, unknown bases and rank-block boundaries.
2. **Search ordering / dynamic partitions (not implemented).** Use measured interval sizes to choose a more selective partition. Must coordinate forward and reverse cut positions so their union still covers every error distribution, including indels at the split. A single favorable seed without a complete search scheme would lose sensitivity.
3. **Persisted seed lookup (not implemented).** A dense ten-base range table takes 8 MiB per shard with two u32 bounds; computing it at index build time could reduce startup rank accesses. Include preparation time, bytes, and provenance in the benchmark. Assess lazy-cache reuse first.
4. **Compressed or sampled suffix arrays (not implemented).** Full suffix arrays dominate storage and cold paging. Compression may reduce memory, but locating millions of sites can become slower. Profile cold paging and locate costs before adopting a different representation.
5. **Batch sharing (not implemented).** Many allele-derived ASOs overlap. Sharing verified search states could help large batches, provided eligibility and per-query minimum distances remain separate.

Every executed attempt is recorded in `benchmarks/CLASSIC_ITERATIONS.md` and `benchmarks/classic/summary.csv` (component timing), with raw JSON/logs in unique iteration directories. Original optimization iterations remain in `BENCHMARKS.md` and `benchmarks/iterations/`. Research ideas without measurements are not benchmark results.


## Measured outcomes

- **Exact 10-mer cache:** identical full-site tuples on the human 32-query set for
  k=0..3; 45,113,535 sites on each 1,000-query run. Three-run median 27.1616 s
  uncached versus 27.3241 s cached (0.9941 baseline/cache ratio). The first
  uncached process took 85.73 s with page-cache effects; it is retained, not used
  as evidence of a speedup. Cache remains opt-in and disabled by default.
  [All observations](../benchmarks/WORD_CACHE_EXPERIMENT.md).
- **Reuse already computed exact extensions:** eight threads for every variant,
  explicit full-workload warmups, rotating order, retained-binary control plus
  disabled control. Correct tuples and 45,113,535 sites each run; retained median
  5.4238 s versus reuse 5.5137 s (0.9837 ratio). Even its disabled branch added
  slight overhead. Removed this experimental path from the working implementation;
  source archive and timings remain available.
  [All observations](../benchmarks/EXTENSION_EXPERIMENT.md),
  [experimental source](../benchmarks/experiments/extension-reuse-source.tar.gz).
- **Witness-distance correction:** the new independent classic audit caught a
  benchmark screening result reporting its first path's edit cost rather than
  the minimum for that interval. Native search now normalizes the matched word
  with the independently tested bit-distance routine before emitting a witness.
  The verifier remains strict. Corrected human screening recovered all 26,643
  ASOs and passed independent scalar verification. Full enumeration was already
  coalescing minimum distances, so the correction does not change its site set.

The timing evidence does not justify replacing the existing search with either
experimental optimization. Dynamic partitioning, dense lookup tables and index
compression remain hypotheses; the papers do not establish an ASO speedup.
