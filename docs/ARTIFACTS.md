# Retained benchmark artifacts

Local review and plotting files:

- `BENCHMARKS.md`: every completed timing/build attempt, including failures.
- `benchmarks/iterations/summary.csv`: plot-friendly timings, workload and cache
  labels, output counts, resource use and equality results.
- Each local iteration directory contains result JSON, process/time records,
  query hashes and source archives. Large site CSVs, comparison buckets and
  annotated JSONL outputs remain on the dedicated EC2 volume.

The retained worker is `i-0b5bad3102a5345c5` in `eu-west-2`, volume
`vol-0138903fbd36c52bf`. Artifact paths are rooted at `/home/ubuntu/ooff`:

| Path | Contents |
| --- | --- |
| `benchmarks/iterations/<ID>/` | Full timing inputs/outputs and source snapshots; `*.sites.csv`, exact-comparison buckets, annotated `output.jsonl`, audits where requested |
| `results/` | Build/reference/resource logs and serial benchmark-round logs |
| `data/raw/` | Original public Ensembl FASTA/GTF downloads and checksum listings |
| `data/reference-v1/` | Derived RNA FASTA, annotations and provenance manifest |
| `data/fm-production-v1/`, `data/fm-reverse-v1/` | Reusable forward/reversed-record indexes and annotation cache |
| Other `data/fm-*` directories | Earlier retained index experiments |
| `data/round*-stage/` | Staged experiment sources and saved prior native executables |
| `data/query-sets-v1/`, `data/cached-allele-queries-v1/` | Generated controls and historical allele-derived queries with provenance |
| `/home/ubuntu/comparators/` | External Sassy builds, including unsupported/failed configurations |

`benchmarks/inventory_artifacts.py` records names, sizes and modification times
without scanning large data files. Final inventories were refreshed after
all jobs ended; they supplement the existing content hashes rather than
replacing them. Native final inventory is local at `data/ec2/artifact-inventory-native-100000.json`
(6,104 files, 105,333,552,190 bytes); its stopped state was verified at 22:53 UTC.
All four dedicated workers are now stopped, verified at 23:06 UTC. Final state
records are `data/ec2/final-worker-states.json` and `final-volume-states.json`.
Minimap2 inventory `artifact-inventory-minimap2-100000.json` records 290 files
and 78,843,699,372 bytes; its final search failed with kernel-confirmed OOM.

Stopping this worker preserves its volume. Do not delete files, indexes,
downloads, the instance or its volume without explicit user permission.


## Independent classic benchmark workers

All paths below are rooted at `/home/ubuntu/ooff`; every root volume is retained.
The source/native worker now has eight vCPUs and 16 GiB and a 200-GiB disk.

| Tool | Instance | Retained volume | State at 23:06 UTC Sept 7 |
| --- | --- | --- | --- |
| Native/source | i-0b5bad3102a5345c5 | vol-0138903fbd36c52bf | Stopped, verified; all eight sizes complete and final inventory collected |
| BWA | i-0eed25aed0ad7dd2c | vol-0fb6f267d2dbd55d9 | Stopped, verified; all eight sizes ×3 complete and final inventory collected |
| BLAST | i-043c11d9e3597df11 | vol-08fb4d24762e1d2c4 | Stopped, verified; all eight sizes completed or timed out, final inventory collected |
| minimap2, 32 GiB | i-0b880ceee50e0c3fa | vol-022afdbf62d1db6cc and vol-0cb196a72cc0da8de | Stopped, verified; 100,000 OOM retained, final inventory collected |
| minimap2, failed 16 GiB | i-0b00bcd23188ec5be | vol-0070954e222f18344 | Instance terminated after earlier Spot cancellation; volume retained, failure logs recovered locally through a read-only mount |
| Earlier unused BWA worker | i-0637cce3f6894de45 | vol-01d47c2593b105cb1 | Earlier request cancelled; volume retained |
| Earlier unused BLAST worker | i-0f6f4db034c61b233 | vol-04240a40cb1554904 | Earlier request cancelled; volume retained |

Do not cancel a stopped persistent Spot request as a cleanup shortcut: earlier
cancellations led to terminated instances. Stop completed workers and retain
requests/volumes unless deletion is explicitly authorized. No volumes were deleted.

`benchmarks/classic/iterations/` contains every component attempt, including
original failed verification and OOM records; `screening.json` links verified
witnesses with timing stages. Small JSON/log/resource artifacts are copied locally.
Raw BWA/minimap2 SAM and BLAST tabular files remain on their respective volumes.
Native full-tuple comparisons and experimental sources are under
`data/classic-stage/word-cache-*` and `data/classic-stage/extension-reuse-*`.
The rejected extension implementation is also archived locally at
`benchmarks/experiments/extension-reuse-source.tar.gz`. No regression was erased.


Minimap2 storage update: root `vol-022afdbf62d1db6cc` is now 100 GiB. Additional
retained output disk `vol-0cb196a72cc0da8de` (100 GiB) is mounted at
`/home/ubuntu/ooff/data/classic-extra`, ext4 UUID
`139cca5c-9449-4df4-9fd1-7d982da97521`. Remount that UUID at the same path after
restarting the worker; it has no automatic mount entry. Final retry iteration
directories under `benchmarks/classic/iterations/` are symlinks to this disk.
Collect with `rsync -L`; raw SAM remains on the disk. Preserve both volumes.


The source worker was subsequently resumed for the matched Sassy2 screening
series on eight CPUs. Final inventory `data/ec2/artifact-inventory-with-sassy.json`
records 6,976 files and 105,620,185,074 bytes at 23:39:51 UTC. The series exit code
was zero. New external comparator sources, dependency lockfile, binary, scalar
fixture checks and all 15 verified measurement attempts remain retained.
