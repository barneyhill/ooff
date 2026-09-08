# Native summary benchmark iterations

Whole-process timings include index/reference loading, search, deduplication and summary output. Index construction is excluded. Completion checks are structural; independent equivalence tests are separate.

| Iteration | Complete | Seconds | Queries | Count unit | Sites | Output bytes |
| --- | --- | ---: | ---: | --- | ---: | ---: |
| [20260908T161016-fixture-default](iterations/20260908T161016-fixture-default/result.json) | False | 0.000517 |  |  |  |  |
| [20260908T161043-fixture-default](iterations/20260908T161043-fixture-default/result.json) | True | 0.001503 | 2 | genomic-site | 10 | 1896 |
| [20260908T161125-scn2a-genomic-default](iterations/20260908T161125-scn2a-genomic-default/result.json) | True | 438.953322 | 114889 | genomic-site | 359039833 | 28812047 |
| [20260908T164639-compact16-forward-spread1000](iterations/20260908T164639-compact16-forward-spread1000/result.json) | True | 16.862644 | 1000 | genomic-site | 3353931 | 252603 |
| [20260908T165555-pi5-compact16-forward-spread1000](iterations/20260908T165555-pi5-compact16-forward-spread1000/result.json) | True | 75.214347 | 1000 | genomic-site | 3353931 | 252591 |
| [20260908T170618-pi5-compact16-forward-spread1000-warm](iterations/20260908T170618-pi5-compact16-forward-spread1000-warm/result.json) | True | 61.157407 | 1000 | genomic-site | 3353931 | 252591 |
