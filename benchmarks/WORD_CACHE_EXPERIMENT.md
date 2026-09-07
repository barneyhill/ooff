# Exact 10-mer cache experiment

Native ablation, one search thread on 8-vCPU/16-GiB host; 32-query exact tuples, 1000-query count/signature checks; fresh processes, OS cache retained.

| Variant | Rep | Search seconds | Process seconds | Sites |
| --- | ---: | ---: | ---: | ---: |
| baseline | 1 | 85.729130 | 86.162009 | 45113535 |
| cached | 1 | 30.277533 | 30.720870 | 45113535 |
| cached | 2 | 27.324103 | 27.762052 | 45113535 |
| baseline | 2 | 27.076667 | 27.558057 | 45113535 |
| baseline | 3 | 27.161637 | 27.610007 | 45113535 |
| cached | 3 | 26.921471 | 27.360263 | 45113535 |

Median baseline/cached speed ratio: 0.9941. Values below 1 are regressions.
Artifacts: `data/classic-stage/word-cache-20260907T215751`.
