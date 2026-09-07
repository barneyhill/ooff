# ooff benchmark iteration log

Goal: at least **10× faster than Sassy**, verified on EC2 with matching query,
reference, edit-distance, strand, policy and output requirements. Warm indexed
screening and site enumeration exceed 10× in completed comparisons; first-run
and output-inclusive measurements can remain below 10×. The optional upstream
AVX-512 build failed; its compiler log is retained. Sassy is an external
comparator only, not an ooff dependency.

Every attempted timing run will be recorded here, including failures, timeouts
and regressions. Machine-readable run records will accompany the table. Separate
cold preparation/index construction from warm searches; do not label unlike
output modes as a like-for-like speedup. Correctness gates must pass before a
timing supports a claim. Keep raw artifacts and record their paths/checksums.

## Execution setup — 2026-09-07

- Controller: Raspberry Pi, aarch64. Small correctness tests only.
- Worker: dedicated `c7i.4xlarge` in `eu-west-2` (16 vCPUs, 32 GiB RAM),
  Ubuntu 24.04; actual CPU flags will be captured before timing.
- Storage: 100 GiB encrypted gp3, retained on termination.
- Automatic stop: four hours after boot. Other EC2 workloads untouched.
- Listed on-demand compute price: $0.8484/hour, excluding storage/public IPv4;
  [price listing checked 2026-09-07](https://aws-pricing.com/c7i.4xlarge.html).
  The account denied `pricing:GetProducts`; this price is from a secondary
  listing, not an account-specific quote.
- Instance metadata and restart material: ignored `data/ec2/` on controller.
- Starting implementation: native Myers endpoint scan plus scalar interval
  verification, one thread. Nine tests pass against a scalar oracle and CLI
  fixtures. These are not throughput measurements.

The additional 16-thread classic-aligner experiment has its own append-only
[iteration ledger](benchmarks/CLASSIC_ITERATIONS.md) and [CSV](benchmarks/classic/summary.csv).

## Iterations

| ID | Change | Dataset | Queries | k | Mode / threads | Build s | Load s | ooff search s | Sassy search s | Search speedup | First-batch speedup | Peak RSS MiB (max of pair) | Result | Artifacts |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 20260907T160055-baseline-endpoints | baseline-endpoints | data/reference-v1/bounded-10mb.fa | 32 | 3 | endpoints / 1 | — | 0.029022 | 1.038403 | — | — | — | 13.214844 | FAILED/TIMEOUT | [JSON](benchmarks/iterations/20260907T160055-baseline-endpoints/result.json) |
| 20260907T160300-baseline-iupac-endpoints | baseline-iupac-endpoints | data/reference-v1/bounded-10mb.fa | 32 | 3 | endpoints / 1 | — | 0.029845 | 1.038593 | 0.050779 | 0.048892 | 0.073692 | 13.128906 | matched; complete | [JSON](benchmarks/iterations/20260907T160300-baseline-iupac-endpoints/result.json) |
| 20260907T160431-baseline-full-screen | baseline-full-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | — | 7.067463 | 10.116535 | 0.826106 | 0.081659 | 0.425576 | 2363.582031 | matched; complete | [JSON](benchmarks/iterations/20260907T160431-baseline-full-screen/result.json) |
| 20260907T161046-fm-v1-bounded-build | fm-v1-bounded-build | data/reference-v1/bounded-10mb.fa | — | — | build / 8 | 0.227151 | — | — | — | — | — | 68.789062 | complete | [JSON](benchmarks/iterations/20260907T161046-fm-v1-bounded-build/result.json) |
| 20260907T161046-fm-v1-bounded-endpoints | fm-v1-bounded-endpoints | data/reference-v1/bounded-10mb.fa | 32 | 3 | endpoints / 1 | 0.224278 | 0.062132 | 0.946403 | 0.050819 | 0.053697 | 0.076955 | 65.082031 | matched; complete | [JSON](benchmarks/iterations/20260907T161046-fm-v1-bounded-endpoints/result.json) |
| 20260907T161049-fm-v1-bounded-screen | fm-v1-bounded-screen | data/reference-v1/bounded-10mb.fa | 1000 | 3 | screen / 1 | 0.224278 | 0.063068 | 2.721809 | 0.391811 | 0.143953 | 0.149998 | 65.386719 | matched; complete | [JSON](benchmarks/iterations/20260907T161049-fm-v1-bounded-screen/result.json) |
| 20260907T161059-fm-v1-full-build | fm-v1-full-build | data/reference-v1/reference.fa | — | — | build / 8 | 88.905476 | — | — | — | — | — | 1326.253906 | complete | [JSON](benchmarks/iterations/20260907T161059-fm-v1-full-build/result.json) |
| 20260907T161228-fm-v1-full-screen | fm-v1-full-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 88.854251 | 14.795040 | 0.047301 | 0.825788 | 17.458060 | 0.491872 | 11084.261719 | matched; complete | [JSON](benchmarks/iterations/20260907T161228-fm-v1-full-screen/result.json) |
| 20260907T161457-fm-v1-strata-full-screen | fm-v1-strata-full-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 88.854251 | 15.010091 | 1.643213 | 0.824548 | 0.501790 | 0.437732 | 11082.347656 | matched; complete | [JSON](benchmarks/iterations/20260907T161457-fm-v1-strata-full-screen/result.json) |
| 20260907T162047-fm-mmap-full-screen | fm-mmap-full-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 88.854251 | 0.102114 | 0.042576 | 0.828283 | 19.454375 | 44.878200 | 2363.574219 | matched; complete | [JSON](benchmarks/iterations/20260907T162047-fm-mmap-full-screen/result.json) |
| 20260907T162058-fm-mmap-scn2a-10000-screen | fm-mmap-scn2a-10000-screen | data/reference-v1/reference.fa | 10000 | 3 | screen / 1 | 88.854251 | 0.107185 | 0.532901 | 5.643590 | 10.590321 | 17.403227 | 2381.445312 | matched; complete | [JSON](benchmarks/iterations/20260907T162058-fm-mmap-scn2a-10000-screen/result.json) |
| 20260907T162123-fm-mmap-mixed-genes-1000-screen | fm-mmap-mixed-genes-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 88.854251 | 0.101727 | 0.143704 | 1.581618 | 11.006071 | 29.760350 | 2363.621094 | matched; complete | [JSON](benchmarks/iterations/20260907T162123-fm-mmap-mixed-genes-1000-screen/result.json) |
| 20260907T162135-fm-mmap-random-1000-screen | fm-mmap-random-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 88.854251 | 0.102004 | 1.529737 | 4.358239 | 2.849012 | 6.477214 | 2363.710938 | matched; complete | [JSON](benchmarks/iterations/20260907T162135-fm-mmap-random-1000-screen/result.json) |
| 20260907T162200-fm-mmap-low-complexity-1000-screen | fm-mmap-low-complexity-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 88.854251 | 0.102304 | 0.002325 | 0.016149 | 6.946477 | 60.937506 | 2363.839844 | matched; complete | [JSON](benchmarks/iterations/20260907T162200-fm-mmap-low-complexity-1000-screen/result.json) |
| 20260907T162207-fm-mmap-bounded-sites | fm-mmap-bounded-sites | data/reference-v1/bounded-10mb.fa | 32 | 3 | sites / 1 | 0.224278 | 0.000345 | 0.925377 | 0.052987 | 0.057260 | 0.085069 | 34.976562 | matched; complete | [JSON](benchmarks/iterations/20260907T162207-fm-mmap-bounded-sites/result.json) |
| 20260907T162210-fm-mmap-full-sites32 | fm-mmap-full-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 88.854251 | 0.101490 | 28.875124 | 13.368778 | 0.462986 | 0.685266 | 3170.082031 | matched; complete | [JSON](benchmarks/iterations/20260907T162210-fm-mmap-full-sites32/result.json) |
| 20260907T162619-fm-tail-scn2a-10000-screen | fm-tail-scn2a-10000-screen | data/reference-v1/reference.fa | 10000 | 3 | screen / 1 | 88.854251 | 0.100730 | 0.235478 | 5.650895 | 23.997531 | 31.148023 | 2381.503906 | matched; complete | [JSON](benchmarks/iterations/20260907T162619-fm-tail-scn2a-10000-screen/result.json) |
| 20260907T162643-fm-tail-random-1000-screen | fm-tail-random-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 88.854251 | 0.094678 | 0.449662 | 4.349306 | 9.672398 | 18.394748 | 2363.679688 | matched; complete | [JSON](benchmarks/iterations/20260907T162643-fm-tail-random-1000-screen/result.json) |
| 20260907T162705-fm-tail-low-complexity-1000-screen | fm-tail-low-complexity-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 88.854251 | 0.094547 | 0.002292 | 0.015372 | 6.706758 | 65.860767 | 2363.742188 | matched; complete | [JSON](benchmarks/iterations/20260907T162705-fm-tail-low-complexity-1000-screen/result.json) |
| 20260907T162712-fm-tail-full-sites32 | fm-tail-full-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 88.854251 | 0.095882 | 7.375734 | 13.238755 | 1.794907 | 2.640724 | 3139.093750 | matched; complete | [JSON](benchmarks/iterations/20260907T162712-fm-tail-full-sites32/result.json) |
| 20260907T162739-fm-1300m-build | fm-1300m-build | data/reference-v1/reference.fa | — | — | build / 8 | 128.915635 | — | — | — | — | — | 8393.476562 | complete | [JSON](benchmarks/iterations/20260907T162739-fm-1300m-build/result.json) |
| 20260907T162948-fm-1300m-scn2a-10000-screen | fm-1300m-scn2a-10000-screen | data/reference-v1/reference.fa | 10000 | 3 | screen / 1 | 128.825596 | 0.143640 | 0.142097 | 5.644053 | 39.719815 | 0.636668 | 3505.953125 | matched; complete | [JSON](benchmarks/iterations/20260907T162948-fm-1300m-scn2a-10000-screen/result.json) |
| 20260907T163036-fm-1300m-mixed-genes-1000-screen | fm-1300m-mixed-genes-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 128.825596 | 0.094670 | 0.012322 | 1.572344 | 127.607480 | 27.226810 | 2363.628906 | matched; complete | [JSON](benchmarks/iterations/20260907T163036-fm-1300m-mixed-genes-1000-screen/result.json) |
| 20260907T163048-fm-1300m-random-1000-screen | fm-1300m-random-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 128.825596 | 0.094473 | 0.074960 | 4.420532 | 58.971774 | 42.386355 | 2363.687500 | matched; complete | [JSON](benchmarks/iterations/20260907T163048-fm-1300m-random-1000-screen/result.json) |
| 20260907T163108-fm-1300m-low-complexity-1000-screen | fm-1300m-low-complexity-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 128.825596 | 0.095000 | 0.002739 | 0.015330 | 5.597009 | 64.115503 | 2363.828125 | matched; complete | [JSON](benchmarks/iterations/20260907T163108-fm-1300m-low-complexity-1000-screen/result.json) |
| 20260907T163115-fm-1300m-full-sites32 | fm-1300m-full-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 128.825596 | 0.094153 | 2.330669 | 13.339145 | 5.723312 | 7.192800 | 2729.843750 | matched; complete | [JSON](benchmarks/iterations/20260907T163115-fm-1300m-full-sites32/result.json) |
| 20260907T164220-fm-fastmemo-scn2a-10000-screen | fm-fastmemo-scn2a-10000-screen | data/reference-v1/reference.fa | 10000 | 3 | screen / 1 | 128.825596 | 0.105738 | 0.088108 | 5.631544 | 63.916403 | 36.530636 | 3515.605469 | matched; complete | [JSON](benchmarks/iterations/20260907T164220-fm-fastmemo-scn2a-10000-screen/result.json) |
| 20260907T164244-fm-fastmemo-random-1000-screen | fm-fastmemo-random-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 128.825596 | 0.100402 | 0.054903 | 4.360386 | 79.419349 | 54.523169 | 2363.703125 | matched; complete | [JSON](benchmarks/iterations/20260907T164244-fm-fastmemo-random-1000-screen/result.json) |
| 20260907T164304-fm-fastmemo-low-complexity-1000-screen | fm-fastmemo-low-complexity-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 128.825596 | 0.100352 | 0.001078 | 0.016118 | 14.953604 | 61.993697 | 2363.734375 | matched; complete | [JSON](benchmarks/iterations/20260907T164304-fm-fastmemo-low-complexity-1000-screen/result.json) |
| 20260907T164311-fm-fastmemo-full-sites32 | fm-fastmemo-full-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 128.825596 | 0.101009 | 2.172303 | 13.261961 | 6.105022 | 8.385324 | 2729.339844 | matched; complete | [JSON](benchmarks/iterations/20260907T164311-fm-fastmemo-full-sites32/result.json) |
| 20260907T164404-fm-production-provenance-build | fm-production-provenance-build | data/reference-v1/reference.fa | — | — | build / 8 | 148.827775 | — | — | — | — | — | 8422.816406 | complete | [JSON](benchmarks/iterations/20260907T164404-fm-production-provenance-build/result.json) |
| 20260907T164959-fm-automaton-full-sites32 | fm-automaton-full-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 148.739452 | 0.107397 | 2.854068 | 13.241304 | 4.639450 | 5.510063 | 2725.972656 | matched; complete | [JSON](benchmarks/iterations/20260907T164959-fm-automaton-full-sites32/result.json) |
| 20260907T165213-fm-automaton-prune-first-sites32 | fm-automaton-prune-first-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 148.739452 | 0.098192 | 2.671050 | 13.385965 | 5.011498 | 6.902164 | 2726.742188 | matched; complete | [JSON](benchmarks/iterations/20260907T165213-fm-automaton-prune-first-sites32/result.json) |
| 20260907T165308-cli-fm-screen-1000 | cli-fm-screen-1000 | data/reference-v1/reference.fa | 1000 | 3 | screen annotated CLI / 1 | — | 1.057080 | 1.066998 | — | — | — | 1471.027344 | complete | [JSON](benchmarks/iterations/20260907T165308-cli-fm-screen-1000/result.json) |
| 20260907T165310-cli-fm-report32 | cli-fm-report32 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.761559 | 3.043697 | — | — | — | 4970.734375 | complete | [JSON](benchmarks/iterations/20260907T165310-cli-fm-report32/result.json) |
| 20260907T165454-fm-automaton-band-sites32 | fm-automaton-band-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 148.739452 | 0.103361 | 2.570144 | 13.273427 | 5.164469 | 7.150941 | 2726.929688 | matched; complete | [JSON](benchmarks/iterations/20260907T165454-fm-automaton-band-sites32/result.json) |
| 20260907T165755-fm-reverse-1300m-build | fm-reverse-1300m-build | data/reference-v1/reference.fa | — | — | build / 8 | 136.946489 | — | — | — | — | — | 8422.968750 | complete | [JSON](benchmarks/iterations/20260907T165755-fm-reverse-1300m-build/result.json) |
| 20260907T170107-fm-two-directions-sites32 | fm-two-directions-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 285.601782 | 0.227051 | 0.649348 | 13.274224 | 20.442389 | 15.799406 | 5269.746094 | matched; complete | [JSON](benchmarks/iterations/20260907T170107-fm-two-directions-sites32/result.json) |
| 20260907T170202-fm-two-directions-screen10k | fm-two-directions-screen10k | data/reference-v1/reference.fa | 10000 | 3 | screen / 1 | 285.601782 | 0.228985 | 0.103559 | 5.597173 | 54.048004 | 0.392853 | 3376.343750 | matched; complete | [JSON](benchmarks/iterations/20260907T170202-fm-two-directions-screen10k/result.json) |
| 20260907T170424-fm-two-directions-sites1000 | fm-two-directions-sites1000 | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | 285.601782 | 0.224100 | 120.494013 | 522.236650 | 4.334129 | 4.379537 | 20698.050781 | matched; complete | [JSON](benchmarks/iterations/20260907T170424-fm-two-directions-sites1000/result.json) |
| 20260907T171542-fm-paired-exact-tuples32 | fm-paired-exact-tuples32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 285.601782 | 0.201828 | 1.644125 | 13.424245 | 8.164976 | 13.548319 | 5271.839844 | matched; complete | [JSON](benchmarks/iterations/20260907T171542-fm-paired-exact-tuples32/result.json) |
| 20260907T171610-cli-paired-report32 | cli-paired-report32 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 1.276750 | 1.847646 | — | — | — | 7489.875000 | complete | [JSON](benchmarks/iterations/20260907T171610-cli-paired-report32/result.json) |
| 20260907T171615-fm-paired-exact-tuples32-k0 | fm-paired-exact-tuples32-k0 | data/reference-v1/reference.fa | 32 | 0 | sites / 1 | 285.601782 | 0.202431 | 0.006512 | 14.887350 | 2286.175376 | 102.662557 | 2361.648438 | matched; complete | [JSON](benchmarks/iterations/20260907T171615-fm-paired-exact-tuples32-k0/result.json) |
| 20260907T171637-fm-paired-exact-tuples32-k1 | fm-paired-exact-tuples32-k1 | data/reference-v1/reference.fa | 32 | 1 | sites / 1 | 285.601782 | 0.202385 | 0.029183 | 11.963698 | 409.960208 | 79.709538 | 2361.769531 | matched; complete | [JSON](benchmarks/iterations/20260907T171637-fm-paired-exact-tuples32-k1/result.json) |
| 20260907T171656-fm-paired-exact-tuples32-k2 | fm-paired-exact-tuples32-k2 | data/reference-v1/reference.fa | 32 | 2 | sites / 1 | 285.601782 | 0.202116 | 0.203403 | 11.986609 | 58.930225 | 45.506475 | 3444.753906 | matched; complete | [JSON](benchmarks/iterations/20260907T171656-fm-paired-exact-tuples32-k2/result.json) |
| 20260907T171841-fm-hash-dedup-sites1000 | fm-hash-dedup-sites1000 [reused Sassy baseline] | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | 285.601782 | 0.201656 | 86.923309 | 522.236650 | 6.008016 | 4.535157 | 20693.390625 | matched; complete | [JSON](benchmarks/iterations/20260907T171841-fm-hash-dedup-sites1000/result.json) |
| 20260907T172540-sassy-dna-notrace-bounded-sites32 | sassy-dna-notrace-bounded-sites32 | data/reference-v1/bounded-10mb.fa | 32 | 3 | sites / 1 | 0.224278 | 0.003456 | 0.346867 | — | — | — | 18.835938 | FAILED/TIMEOUT | [JSON](benchmarks/iterations/20260907T172540-sassy-dna-notrace-bounded-sites32/result.json) |
| 20260907T172541-sassy-dna-notrace-full-sites32 | sassy-dna-notrace-full-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 285.601782 | 0.203113 | 0.815003 | — | — | — | 5274.167969 | FAILED/TIMEOUT | [JSON](benchmarks/iterations/20260907T172541-sassy-dna-notrace-full-sites32/result.json) |
| 20260907T173019-sassy-v1-notrace-bounded-sites32 | sassy-v1-notrace-bounded-sites32 | data/reference-v1/bounded-10mb.fa | 32 | 3 | sites / 1 | 0.224278 | 0.000357 | 0.124387 | 0.178453 | 1.434656 | 1.644159 | 18.894531 | matched; complete | [JSON](benchmarks/iterations/20260907T173019-sassy-v1-notrace-bounded-sites32/result.json) |
| 20260907T173019-sassy-v1-notrace-full-sites32 | sassy-v1-notrace-full-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 285.601782 | 0.190608 | 0.816166 | 45.332387 | 55.543113 | 53.235557 | 5274.167969 | matched; complete | [JSON](benchmarks/iterations/20260907T173019-sassy-v1-notrace-full-sites32/result.json) |
| 20260907T173114-fm-range-dedup-sites1000 | fm-range-dedup-sites1000 [reused Sassy baseline] | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | 285.601782 | 0.188765 | 33.650224 | 522.236650 | 15.519559 | 10.146369 | 20802.285156 | matched; complete | [JSON](benchmarks/iterations/20260907T173114-fm-range-dedup-sites1000/result.json) |
| 20260907T173543-fm-range-dedup-fresh-sites1000 | fm-range-dedup-fresh-sites1000 | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | 285.601782 | 0.191358 | 52.163913 | 525.742076 | 10.078655 | 10.203406 | 20622.781250 | matched; complete | [JSON](benchmarks/iterations/20260907T173543-fm-range-dedup-fresh-sites1000/result.json) |
| 20260907T174543-annotation-cache-build | annotation-cache-build | data/reference-v1/reference.fa | — | — | build / 1 | 1.125539 | — | — | — | — | — | 58.281250 | complete | [JSON](benchmarks/iterations/20260907T174543-annotation-cache-build/result.json) |
| 20260907T174544-cli-cached-screen1000-r1 | cli-cached-screen1000-r1 | data/reference-v1/reference.fa | 1000 | 3 | screen annotated CLI / 1 | — | 0.238204 | 0.351755 | — | — | — | 1262.019531 | complete | [JSON](benchmarks/iterations/20260907T174544-cli-cached-screen1000-r1/result.json) |
| 20260907T174545-cli-cached-paired-report32-r1 | cli-cached-paired-report32-r1 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.341194 | 4.884454 | — | — | — | 7326.210938 | complete | [JSON](benchmarks/iterations/20260907T174545-cli-cached-paired-report32-r1/result.json) |
| 20260907T174551-cli-cached-screen1000-r2 | cli-cached-screen1000-r2 | data/reference-v1/reference.fa | 1000 | 3 | screen annotated CLI / 1 | — | 0.237581 | 0.062932 | — | — | — | 1262.171875 | complete | [JSON](benchmarks/iterations/20260907T174551-cli-cached-screen1000-r2/result.json) |
| 20260907T174552-cli-cached-paired-report32-r2 | cli-cached-paired-report32-r2 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.340275 | 1.540341 | — | — | — | 7329.789062 | complete | [JSON](benchmarks/iterations/20260907T174552-cli-cached-paired-report32-r2/result.json) |
| 20260907T174555-cli-cached-paired-report32-r3 | cli-cached-paired-report32-r3 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.340181 | 1.533427 | — | — | — | 7329.777344 | complete | [JSON](benchmarks/iterations/20260907T174555-cli-cached-paired-report32-r3/result.json) |
| 20260907T174555-cli-cached-screen1000-r3 | cli-cached-screen1000-r3 | data/reference-v1/reference.fa | 1000 | 3 | screen annotated CLI / 1 | — | 0.237971 | 0.063005 | — | — | — | 1262.242188 | complete | [JSON](benchmarks/iterations/20260907T174555-cli-cached-screen1000-r3/result.json) |
| 20260907T174558-cli-cached-screen10000 | cli-cached-screen10000 | data/reference-v1/reference.fa | 10000 | 3 | screen annotated CLI / 1 | — | 0.236532 | 0.552346 | — | — | — | 4166.921875 | complete | [JSON](benchmarks/iterations/20260907T174558-cli-cached-screen10000/result.json) |
| 20260907T174602-fm-range-final-scn2a-10000-screen | fm-range-final-scn2a-10000-screen | data/reference-v1/reference.fa | 10000 | 3 | screen / 1 | 148.739452 | 0.099123 | 0.080911 | 5.669733 | 70.073367 | 46.005840 | 3487.789062 | matched; complete | [JSON](benchmarks/iterations/20260907T174602-fm-range-final-scn2a-10000-screen/result.json) |
| 20260907T174629-fm-range-final-mixed-genes-1000-screen | fm-range-final-mixed-genes-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.093397 | 0.009298 | 1.571601 | 169.022335 | 21.525913 | 2363.308594 | matched; complete | [JSON](benchmarks/iterations/20260907T174629-fm-range-final-mixed-genes-1000-screen/result.json) |
| 20260907T174644-fm-range-final-random-1000-screen | fm-range-final-random-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.092823 | 0.056332 | 4.382242 | 77.792572 | 37.290091 | 2363.609375 | matched; complete | [JSON](benchmarks/iterations/20260907T174644-fm-range-final-random-1000-screen/result.json) |
| 20260907T174705-fm-range-final-low-complexity-1000-screen | fm-range-final-low-complexity-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.091967 | 0.001180 | 0.015398 | 13.048003 | 55.097737 | 2363.851562 | matched; complete | [JSON](benchmarks/iterations/20260907T174705-fm-range-final-low-complexity-1000-screen/result.json) |
| 20260907T174712-fm-range-allele-shortlist32-sites | fm-range-allele-shortlist32-sites | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 285.601782 | 0.194025 | 0.517875 | 12.913092 | 24.934781 | 8.221739 | 4580.699219 | matched; complete | [JSON](benchmarks/iterations/20260907T174712-fm-range-allele-shortlist32-sites/result.json) |
| 20260907T174801-fm-range-v1-low-complexity-screen | fm-range-v1-low-complexity-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.092046 | 0.001156 | — | — | — | 2361.964844 | FAILED/TIMEOUT | [JSON](benchmarks/iterations/20260907T174801-fm-range-v1-low-complexity-screen/result.json) |
| 20260907T174808-fm-range-v1-original1000-screen | fm-range-v1-original1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.092545 | 0.006760 | — | — | — | 2361.992188 | FAILED/TIMEOUT | [JSON](benchmarks/iterations/20260907T174808-fm-range-v1-original1000-screen/result.json) |
| 20260907T174815-fm-range-cold-original1000-screen | fm-range-cold-original1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.169768 | 0.011617 | 0.829842 | 71.434337 | 1.230433 | 2363.539062 | matched; complete | [JSON](benchmarks/iterations/20260907T174815-fm-range-cold-original1000-screen/result.json) |
| 20260907T175700-fm-coarse-flat-verifier-sites32 | fm-coarse-flat-verifier-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 285.601782 | 0.227849 | 0.632162 | 13.005150 | 20.572511 | 1.240255 | 5197.093750 | matched; complete | [JSON](benchmarks/iterations/20260907T175700-fm-coarse-flat-verifier-sites32/result.json) |
| 20260907T175824-cli-coarse-flat-report32-r1 | cli-coarse-flat-report32-r1 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.496176 | 1.386513 | — | — | — | 7354.605469 | complete | [JSON](benchmarks/iterations/20260907T175824-cli-coarse-flat-report32-r1/result.json) |
| 20260907T175827-cli-coarse-flat-report32-r2 | cli-coarse-flat-report32-r2 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.363682 | 1.381373 | — | — | — | 7354.617188 | complete | [JSON](benchmarks/iterations/20260907T175827-cli-coarse-flat-report32-r2/result.json) |
| 20260907T175829-cli-coarse-flat-report32-r3 | cli-coarse-flat-report32-r3 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.365231 | 1.380531 | — | — | — | 7354.628906 | complete | [JSON](benchmarks/iterations/20260907T175829-cli-coarse-flat-report32-r3/result.json) |
| 20260907T175835-fm-v1-iupac-low-complexity-screen | fm-v1-iupac-low-complexity-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.112136 | 0.001051 | 0.029788 | 28.338536 | 34.075105 | 2362.175781 | matched; complete | [JSON](benchmarks/iterations/20260907T175835-fm-v1-iupac-low-complexity-screen/result.json) |
| 20260907T175843-fm-v1-iupac-original1000-screen | fm-v1-iupac-original1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.111642 | 0.006374 | 1.845034 | 289.478741 | 48.420406 | 2362.265625 | matched; complete | [JSON](benchmarks/iterations/20260907T175843-fm-v1-iupac-original1000-screen/result.json) |
| 20260907T175855-fm-coarse-flat-verifier-fresh-sites1000 | fm-coarse-flat-verifier-fresh-sites1000 | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | 285.601782 | 0.229790 | 29.103331 | 469.822673 | 16.143261 | 3.738373 | 20671.253906 | matched; complete | [JSON](benchmarks/iterations/20260907T175855-fm-coarse-flat-verifier-fresh-sites1000/result.json) |
| 20260907T183020-fm-cached-alleles-1000-screen | fm-cached-alleles-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.188883 | 0.009618 | 0.661407 | 68.769027 | 5.922917 | 2363.656250 | matched; complete | [JSON](benchmarks/iterations/20260907T183020-fm-cached-alleles-1000-screen/result.json) |
| 20260907T183030-fm-cached-alleles-10000-screen | fm-cached-alleles-10000-screen | data/reference-v1/reference.fa | 10000 | 3 | screen / 1 | 148.739452 | 0.115824 | 0.076216 | 6.850648 | 89.884186 | 3.612053 | 2381.433594 | matched; complete | [JSON](benchmarks/iterations/20260907T183030-fm-cached-alleles-10000-screen/result.json) |
| 20260907T183102-fm-cached-alleles-all-screen | fm-cached-alleles-all-screen | data/reference-v1/reference.fa | 26643 | 3 | screen / 1 | 148.739452 | 0.124120 | 0.150358 | 17.991701 | 119.659116 | 14.361242 | 2602.949219 | matched; complete | [JSON](benchmarks/iterations/20260907T183102-fm-cached-alleles-all-screen/result.json) |
| 20260907T183205-fm-exact-tuples-all-1000 | fm-exact-tuples-all-1000 | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | 285.601782 | 0.228510 | 81.638625 | 451.664227 | 5.532482 | 5.596453 | 20453.906250 | matched; complete | [JSON](benchmarks/iterations/20260907T183205-fm-exact-tuples-all-1000/result.json) |
| 20260907T184208-fm-random1000-screen-k0 | fm-random1000-screen-k0 | data/reference-v1/reference.fa | 1000 | 0 | screen / 1 | 148.739452 | 0.168210 | 0.060233 | 174.122178 | 2890.812091 | 839.579716 | 2363.191406 | matched; complete | [JSON](benchmarks/iterations/20260907T184208-fm-random1000-screen-k0/result.json) |
| 20260907T184521-fm-random1000-screen-k1 | fm-random1000-screen-k1 | data/reference-v1/reference.fa | 1000 | 1 | screen / 1 | 148.739452 | 0.112455 | 0.659536 | 249.817379 | 378.777299 | 332.061333 | 2363.406250 | matched; complete | [JSON](benchmarks/iterations/20260907T184521-fm-random1000-screen-k1/result.json) |
| 20260907T184938-fm-random1000-screen-k2 | fm-random1000-screen-k2 | data/reference-v1/reference.fa | 1000 | 2 | screen / 1 | 148.739452 | 0.112081 | 0.968150 | 89.708837 | 92.660080 | 89.084702 | 2363.359375 | matched; complete | [JSON](benchmarks/iterations/20260907T184938-fm-random1000-screen-k2/result.json) |
| 20260907T185138-sassy-avx512-build-failed | sassy-0.2.6-avx512-build | — | — | — | comparator build / 4 | — | — | — | — | — | — | — | BUILD FAILED | [JSON](benchmarks/iterations/20260907T185138-sassy-avx512-build-failed/result.json) |
| 20260907T185209-cli-borrowed-json-report32-r1 | cli-borrowed-json-report32-r1 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.582075 | 2.409689 | — | — | — | 7358.339844 | complete | [JSON](benchmarks/iterations/20260907T185209-cli-borrowed-json-report32-r1/result.json) |
| 20260907T185213-cli-borrowed-json-report32-r2 | cli-borrowed-json-report32-r2 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.350361 | 1.174011 | — | — | — | 7359.546875 | complete | [JSON](benchmarks/iterations/20260907T185213-cli-borrowed-json-report32-r2/result.json) |
| 20260907T185216-cli-borrowed-json-report32-r3 | cli-borrowed-json-report32-r3 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.351177 | 1.164522 | — | — | — | 7359.496094 | complete | [JSON](benchmarks/iterations/20260907T185216-cli-borrowed-json-report32-r3/result.json) |
| 20260907T185227-cli-borrowed-json-alleles-all-screen | cli-borrowed-json-alleles-all-screen | data/reference-v1/reference.fa | 26643 | 3 | screen annotated CLI / 1 | — | 0.247162 | 1.925668 | — | — | — | 3596.707031 | complete | [JSON](benchmarks/iterations/20260907T185227-cli-borrowed-json-alleles-all-screen/result.json) |
| 20260907T185255-fm-final-exact-tuples-1000-repeated | fm-final-exact-tuples-1000-repeated | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | 285.601782 | 0.326419 | 34.379688 | 444.330500 | 12.924216 | 7.422676 | 20624.433594 | matched; complete | [JSON](benchmarks/iterations/20260907T185255-fm-final-exact-tuples-1000-repeated/result.json) |
| 20260907T192251-fm-packed-fast-interval-sites32 | fm-packed-fast-interval-sites32 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 285.601782 | 0.361761 | 0.621927 | 12.755640 | 20.509861 | 27.823682 | 5264.253906 | matched; complete | [JSON](benchmarks/iterations/20260907T192251-fm-packed-fast-interval-sites32/result.json) |
| 20260907T192353-fm-packed-fast-interval-sites1000 | fm-packed-fast-interval-sites1000 | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | 285.601782 | 0.206275 | 28.026787 | 320.551896 | 11.437340 | 8.860669 | 20520.851562 | matched; complete | [JSON](benchmarks/iterations/20260907T192353-fm-packed-fast-interval-sites1000/result.json) |
| 20260907T194149-fm-word-union-full-tuples1000 | fm-word-union-full-tuples1000 | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | — | — | — | — | — | — | 0.000000 | ABORTED: prerequisite failed | [JSON](benchmarks/iterations/20260907T194149-fm-word-union-full-tuples1000/result.json) |
| 20260907T194149-native-range-build-failed | native-range-build-stale-artifact | — | — | — | native build / 2 | — | — | — | — | — | — | — | BUILD FAILED | [JSON](benchmarks/iterations/20260907T194149-native-range-build-failed/result.json) |
| 20260907T194659-fm-word-union-sites32-k0 | fm-word-union-sites32-k0 | data/reference-v1/reference.fa | 32 | 0 | sites / 1 | 285.601782 | 0.197979 | 0.007442 | 14.882699 | 1999.808767 | 111.176873 | 2361.882812 | matched; complete | [JSON](benchmarks/iterations/20260907T194659-fm-word-union-sites32-k0/result.json) |
| 20260907T194722-fm-word-union-sites32-k1 | fm-word-union-sites32-k1 | data/reference-v1/reference.fa | 32 | 1 | sites / 1 | 285.601782 | 0.395880 | 0.034271 | 11.919390 | 347.802705 | 45.620051 | 2361.859375 | matched; complete | [JSON](benchmarks/iterations/20260907T194722-fm-word-union-sites32-k1/result.json) |
| 20260907T194743-fm-word-union-sites32-k2 | fm-word-union-sites32-k2 | data/reference-v1/reference.fa | 32 | 2 | sites / 1 | 285.601782 | 0.198007 | 0.221507 | 11.959010 | 53.989190 | 44.049229 | 3385.109375 | matched; complete | [JSON](benchmarks/iterations/20260907T194743-fm-word-union-sites32-k2/result.json) |
| 20260907T194802-fm-word-union-sites32-k3 | fm-word-union-sites32-k3 | data/reference-v1/reference.fa | 32 | 3 | sites / 1 | 285.601782 | 0.198067 | 0.881344 | 12.731913 | 14.446023 | 18.211937 | 4462.089844 | matched; complete | [JSON](benchmarks/iterations/20260907T194802-fm-word-union-sites32-k3/result.json) |
| 20260907T194824-fm-word-union-sites1000 | fm-word-union-sites1000 | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | 285.601782 | 0.198990 | 24.790994 | 320.705999 | 12.936391 | 12.783770 | 11971.753906 | matched; complete | [JSON](benchmarks/iterations/20260907T194824-fm-word-union-sites1000/result.json) |
| 20260907T200618-cli-word-union-report32-r1 | cli-word-union-report32-r1 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.495852 | 2.433543 | — | — | — | 6576.531250 | complete | [JSON](benchmarks/iterations/20260907T200618-cli-word-union-report32-r1/result.json) |
| 20260907T200621-cli-word-union-report32-r2 | cli-word-union-report32-r2 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.364764 | 1.172058 | — | — | — | 6577.750000 | complete | [JSON](benchmarks/iterations/20260907T200621-cli-word-union-report32-r2/result.json) |
| 20260907T200624-cli-word-union-report32-r3 | cli-word-union-report32-r3 | data/reference-v1/reference.fa | 32 | 3 | report annotated CLI / 1 | — | 0.364647 | 1.174574 | — | — | — | 6577.804688 | complete | [JSON](benchmarks/iterations/20260907T200624-cli-word-union-report32-r3/result.json) |
| 20260907T200643-fm-word-union-full-tuples1000 | fm-word-union-full-tuples1000 | data/reference-v1/reference.fa | 1000 | 3 | sites / 1 | 285.601782 | 0.398118 | 30.476569 | 325.311813 | 10.674161 | 10.441111 | 11973.902344 | matched; complete | [JSON](benchmarks/iterations/20260907T200643-fm-word-union-full-tuples1000/result.json) |
| 20260907T202834-fm-word-final-scn2a-10000-screen | fm-word-final-scn2a-10000-screen | data/reference-v1/reference.fa | 10000 | 3 | screen / 1 | 148.739452 | 0.238364 | 0.092601 | 5.638512 | 60.890234 | 1.364429 | 3419.324219 | matched; complete | [JSON](benchmarks/iterations/20260907T202834-fm-word-final-scn2a-10000-screen/result.json) |
| 20260907T202927-fm-word-final-mixed-genes-1000-screen | fm-word-final-mixed-genes-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.097172 | 0.008563 | 1.583487 | 184.923358 | 9.564167 | 2365.656250 | matched; complete | [JSON](benchmarks/iterations/20260907T202927-fm-word-final-mixed-genes-1000-screen/result.json) |
| 20260907T202940-fm-word-final-random-1000-screen | fm-word-final-random-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.096635 | 0.058253 | 4.403181 | 75.587269 | 25.312151 | 2365.597656 | matched; complete | [JSON](benchmarks/iterations/20260907T202940-fm-word-final-random-1000-screen/result.json) |
| 20260907T203000-fm-word-final-low-complexity-1000-screen | fm-word-final-low-complexity-1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.095882 | 0.001076 | 0.015171 | 14.094931 | 40.143576 | 2365.824219 | matched; complete | [JSON](benchmarks/iterations/20260907T203000-fm-word-final-low-complexity-1000-screen/result.json) |
| 20260907T203008-fm-word-final-alleles-all-screen | fm-word-final-alleles-all-screen | data/reference-v1/reference.fa | 26643 | 3 | screen / 1 | 148.739452 | 0.133909 | 0.162925 | 18.208131 | 111.757965 | 11.094579 | 2628.367188 | matched; complete | [JSON](benchmarks/iterations/20260907T203008-fm-word-final-alleles-all-screen/result.json) |
| 20260907T203114-fm-word-final-cold-original1000-screen | fm-word-final-cold-original1000-screen | data/reference-v1/reference.fa | 1000 | 3 | screen / 1 | 148.739452 | 0.238881 | 21.272581 | 0.826918 | 0.038872 | 0.861038 | 2365.121094 | matched; complete | [JSON](benchmarks/iterations/20260907T203114-fm-word-final-cold-original1000-screen/result.json) |

## Interpretation

**19:49 UTC:** the stronger distance-only comparator measured 320.551896 s
versus compact native storage at 28.026787 s (11.437× median, three full-report
repetitions). A staged range-union build initially reused an older Cargo library
because source timestamps were preserved. That compiler failure and the
interrupted downstream attempt are retained. Refreshing source timestamps fixed
the build; all 18 tests and exact report32 comparisons at k=0..3 passed. The
harness now exits unsuccessfully after a failed comparison, and downstream
validation checks a successful build's binary hashes. Full range-union timing
and large-output validation remain active.

**19:04 UTC audit update:** the reporting comparator still reconstructed
tracebacks when the benchmark only needed interval distance. A tested global
bit-vector distance verifier is now staged for both benchmark drivers; it must
be measured before accepting any final reporting speedup. Earlier ratios remain
measurements of their archived configurations, not proof against this stronger
comparator. Compact native site storage is staged alongside it. Screening does
not use this interval-verification path.

At 18:42 UTC, 79 iterations are retained. Fresh full-report search on 1,000
queries measured 29.103331 s native versus 469.822673 s Sassy (median of three,
16.143×), but the native first repetition took 118.727 s and the corresponding
load-plus-first-search ratio was 3.738×. A separate run wrote and compared all
45,113,535 literal site tuples: exact equality, no duplicates, 81.639 s native
versus 451.664 s Sassy (5.532×). It is not valid to attribute that difference
solely to serialization because the first-run page-cache state also differed.

The recovered historical allele-derived pool contains 26,643 unique sequences;
its screening median was 0.150358 s native versus 17.991701 s Sassy (119.659×).
The original 1,000-query set contains reference tiles, not allele-specific ASOs.
The production annotated CLI writes 64.9 MB for report32 in roughly 1.72 s wall
after borrowed-field serialization (previously 1.97 s). All fields of all 78,733
site records match the previous output exactly. The complete 26,643-query
allele-derived pool finishes the annotated screening CLI in 2.37 s wall,
writing 30.1 MB; all query summaries and allele metadata were audited. These
figures include ordinary fresh-process runs with uncontrolled OS page cache;
this is a separate output workload and has no equivalent annotated Sassy timing.

Controlled cold screening measured only 1.230× for load plus the first search.
The table's median can reflect warm repetitions even for a `cold`-labelled run;
use the per-repetition JSON and cache-residency evidence for cold-start analysis.
Both directions require approximately 23 GB of reusable indexes and about
286 seconds of index construction, separate from reference preparation.

**Comparator API audit:** the pinned Sassy 0.2.6 batched API does not propagate
`without_trace()` into its pattern-tiling engine. Attempts named
`sassy-dna-notrace-*` still traced and crashed; those failures are retained.
The working IUPAC batched comparator is restored. The separate official v1 DNA
API honors no-trace for single-pattern reporting but is slower; its DNA batch
screening API is unimplemented. The supported v1 IUPAC batch screening build
also measured slower than v2. The optional v2 `avx512` feature failed to compile
with incompatible `u64x8`/`u64x4` types; the failed build and locked source are
recorded as an iteration. Default features only enable CLI/diagnostic support,
so disabling them does not force scalar search. The supported comparator uses
`target-cpu=native`. No upstream Sassy code has been modified.

Development iterations marked **reused Sassy baseline** retain a real,
previously measured identical-workload Sassy result to avoid repeating a
nine-minute scan for every implementation change. Their JSON links that
baseline explicitly. Final acceptance requires a fresh paired comparison.
Cold-cache iterations request eviction of only the benchmark input files and
record actual remaining page residency before each engine; this is separate
from ordinary fresh-process runs, whose OS cache is uncontrolled.

Production CLI runs, which include JSON serialization and full annotation
loading, are logged separately in [CLI iterations](benchmarks/CLI_ITERATIONS.md).
Matcher-only ratios must not be presented as production CLI speedups.

The first automaton benchmark attempt failed to compile because a closure
needed explicit types; no timing was produced. The EC2 log is retained at
`results/fm-automaton.log`. The corrected attempt and its regression are in
the measured table.


The historical M4 benchmarks in `benchmarks/` are prior evidence, not EC2
iterations and not a matched baseline for the new tool. Human-reference choice
and historical provenance gaps are documented in `docs/reference.md`.
