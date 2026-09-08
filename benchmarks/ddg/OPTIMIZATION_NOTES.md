# DDG optimization notes — 2026-09-08

Full pipeline measurements use 100,000 distinct SCN2A ASOs and 273,825 discovered SCN1A pre-mRNA sites. Both engines use the same executable and requested worker count unless the baseline column explicitly says Rust. Every run checks all report fields after timing. This workload is not full-human-reference annotation.

| Change | Workers | Baseline | Baseline s | Native s | Ratio | Evidence |
| --- | ---: | --- | ---: | ---: | ---: | --- |
| Automatic SIMD in the original serial annotation pipeline | 8 | vienna-baseline | 12.075393 | 2.251184 | 5.36× | [raw record](iterations/20260908T120440.575550-real-site-annotation/result.json) |
| Parallel complete batches; shared intended-energy cache | 8 | vienna-baseline | 10.052384 | 1.316229 | 7.64× | [raw record](iterations/20260908T121058.529664-real-site-annotation/result.json) |
| Remove redundant inner workers; group up to 1024 pairs for SIMD | 8 | vienna-baseline | 10.047490 | 1.155116 | 8.70× | [raw record](iterations/20260908T121257.055425-real-site-annotation/result.json) |
| Larger I/O buffers; native A/B with delayed-writeback outlier | 8 | rust-baseline | 1.251042 | 1.185245 | 1.06× | [raw record](iterations/20260908T121453.889560-real-site-annotation/result.json) |
| Overlap provenance hashing; drain prior dirty writes before timing | 8 | vienna-baseline | 10.020493 | 1.118794 | 8.96× | [raw record](iterations/20260908T121721.230384-real-site-annotation/result.json) |
| Overlap next-input and previous-spool I/O with scoring | 8 | vienna-baseline | 9.818999 | 1.016481 | 9.66× | [raw record](iterations/20260908T122005.749442-real-site-annotation/result.json) |
| Reuse owned normalization allocations; selected version, eight workers | 8 | vienna-baseline | 9.758840 | 1.001300 | 9.75× | [raw record](iterations/20260908T122228.749370-real-site-annotation/result.json) |
| Selected version, default four workers | 4 | vienna-baseline | 11.809776 | 1.106856 | 10.67× | [raw record](iterations/20260908T122341.403813-real-site-annotation/result.json) |
| Descriptor-copy experiment, eight workers; reverted | 8 | vienna-baseline | 9.784350 | 1.020828 | 9.58× | [raw record](iterations/20260908T122554.400775-real-site-annotation/result.json) |
| Descriptor-copy experiment, four workers; reverted | 4 | vienna-baseline | 11.794865 | 1.116413 | 10.56× | [raw record](iterations/20260908T122707.290092-real-site-annotation/result.json) |

The buffered-I/O A/B run included a 4.268 s native outlier. Its profile places the extra time in private-spool and final-output writes. Subsequent runs call `os.sync()` before timing to drain earlier runs’ dirty pages; they still measure buffered output, not durable-storage latency. No outlier was deleted.

The selected native executable SHA256 is `6bcd3a75298c5e13ea07c662c142a078c454d5972df7afdfbc5092fca73979a3`. Rebuilding after reverting the copy experiment produced exactly this hash. Selected medians are 10.67× at four workers and 9.75× at eight. Eight workers has lower absolute native latency.

Kernel experiments precede this pipeline work: vector gathers improved AVX2 about 4.1× over native scalar; automatic AVX-512 improved another 1.4× over AVX2. Direct unmodified ViennaRNA 2.7.0 `duplexfold` versus native AVX-512 at eight workers measured 28.14× compute-only. Shared scalar prefix calculations improved 1.84× on 20,543 unique full pairs, but combined SIMD prefix reuse regressed about 4%. Exact min-plus range minima regressed about 52%. All remain in [ITERATIONS.md](ITERATIONS.md), including failed/slower attempts.

The first parallel-pipeline implementation computed intended energies twice when concurrent batches shared a query. An existing once-per-query test caught it before benchmarking. The corrected implementation shares its intended-energy cache and acquires locks in query-ID order; the test passes. A new integration test rejects misplaced control rows across batch and wave boundaries without emitting partial stdout.

Final validation: 12 CLI integration tests, three energy tests, the pinned min-plus test and an actual ViennaRNA 2.7.0 oracle run passed in a generic x86-64 release build. AVX2 and AVX-512 are exercised by the energy tests. ARM CLI/energy tests and all-target/all-feature Clippy passed; x86 all-target/all-feature Clippy passed too.

Raw multi-gigabyte annotated outputs and work spools remain on the retained EC2 volume, instance `i-0b5bad3102a5345c5`, under `/home/ubuntu/ddg-simd-20260908/benchmarks/ddg/iterations` and `/tmp/oofft-ddg-sites-*`. Result records and profiles are also local. No experimental output or source snapshot was deleted. Benchmark input generation and the exact comparison command are documented in [DDG.md](../../docs/DDG.md).

This meets the 10× target on the measured default-four-worker site workflow. It does not establish a universal 10× speedup, a 10× whole-transcript RNAplex speedup, or a runtime for all human-reference sites. Full-human distinct sequence-pair counting and better cross-SIMD prefix scheduling are separate remaining research opportunities.

A later [reuse investigation](REUSE_INVESTIGATION.md) measured the retained
1,000-ASO full-reference pilot: 45.1 million intervals reduce to 2.93 million
sequence pairs. The larger corpus measured 33.25× native-versus-Vienna scoring;
shared-path scheduling regressed. This follow-up does not change the earlier
100,000-ASO complete-annotation measurement above.
