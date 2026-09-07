# Reusing exact index extensions

Eight CPUs available to every variant. Full 1,000-query warmup per variant is recorded separately. Subsequent runs rotate order.

| Variant | Phase | Rep | Search seconds | Process seconds |
| --- | --- | ---: | ---: | ---: |
| retained | warmup | 0 | 5.439846 | 5.881743 |
| disabled | warmup | 0 | 5.471217 | 5.931991 |
| reuse | warmup | 0 | 5.508878 | 5.981304 |
| reuse | optimization | 1 | 5.516412 | 5.983647 |
| retained | optimization | 1 | 5.423792 | 5.830719 |
| disabled | optimization | 1 | 5.469792 | 5.932048 |
| disabled | optimization | 2 | 5.434597 | 5.882459 |
| reuse | optimization | 2 | 5.513668 | 5.983299 |
| retained | optimization | 2 | 5.348891 | 5.781118 |
| retained | optimization | 3 | 5.460896 | 5.881444 |
| disabled | optimization | 3 | 5.486529 | 5.931482 |
| reuse | optimization | 3 | 5.467826 | 5.932666 |

Retained / reuse median ratio: 0.9837. Disabled / reuse: 0.9920. Below 1 means regression.
Artifacts: `data/classic-stage/extension-reuse-20260907T220425`.
