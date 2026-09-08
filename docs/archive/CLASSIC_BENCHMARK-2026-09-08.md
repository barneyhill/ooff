# Classic-aligner scaling benchmark

Status: the README has one line plot of median screening time, with index builds listed in its caption.
ooff and BWA have completed all eight sizes through 100,000 ASOs. BLAST finished
its series, with timeouts from 1,000 ASOs upward. Minimap2’s 100,000-ASO attempt exhausted 32 GiB of RAM after 421.16 seconds
(exit 137, confirmed by the kernel OOM log). This is a terminal resource failure,
not a completed timing or timeout lower bound. All planned sizes now have a
completed repeated measurement, timeout, or confirmed resource-failure outcome. The README now uses only
10, 100, 1,000, 10,000 and 100,000 ASOs, following the requested powers-of-ten
spacing. Previously measured 30, 5,000 and 26,643 batches remain in the ledger.
The remaining minimap2 26,643 repetitions were cancelled at the user’s request;
the recorded cancellation is neither a timeout nor a completed timing point.
Only completed three-repetition points are drawn as timing curves.

The figure uses the existing Ensembl 110 GRCh38-derived RNA reference:
RNA-sense gene bodies and mature transcripts, with SCN2A-only records excluded
under the same other-gene policy used by ooff. This is not the whole genomic
primary assembly including intergenic DNA. The prepared reference manifest
records exact hashes, bases, records, and exclusions. All tools receive the same
eligible sequences and orientation; ooff uses its original index and applies
the equivalent gene filter during search.

Nested batches contain 10, 30, 100, 1,000, 5,000, 10,000, and all 26,643 unique ASOs from
the historical SCN2A allele-query pool. No new energy or complexity filtering is
applied. Mapper queries are the reverse complements of the 5′→3′ ASOs. Headers
are simplified consistently; source query hashes and associations are retained.

The task is screening: find at least one valid other-gene interval per ASO with
full-query unit-cost Levenshtein distance ≤3. Every tool’s candidate witness
is checked against the actual reference bases by an independent scalar global
alignment, rejecting reverse-strand and ambiguous-base matches. Recovery is the
fraction of native-positive queries for which a comparator returns a verified
witness; it is not an all-sites recall measurement. Mapper heuristics may miss
valid witnesses; recovery values remain in the plot CSV and are disclosed in the README and result table.

Comparators are BWA-aln, BLASTN-short, minimap2 and Sassy2. Bowtie and Bowtie 2
are excluded following the user's request. Sassy remains an external EC2
comparator, including its new eight-thread screening series; it is not a native dependency.

- BWA-aln: edit threshold 3, three gap opens/extensions allowed, end-indel
  restriction removed, seed length above query length, unit scoring parameters,
  8 mapping threads. Normal iterative search and a 1,000-alternative SAM cap
  remain explicit limitations. Both mapping (`aln`) and SAM conversion (`samse`)
  are timed. BWA's options are described in its [official manual](https://bio-bwa.sourceforge.net/bwa.shtml).
- BLASTN-short: 8 requested threads, plus strand, no dust or soft masking,
  expectation threshold 10⁹, match/mismatch scores 1/−1, gap open/extend 0/2,
  minimum 85% query coverage, up to 1,000 target sequences and one HSP each.
  The permissive expectation threshold avoids using statistical significance
  as a substitute for the fixed three-edit screening rule. The scoring pair is
  supported by [NCBI’s scoring table](https://www.ncbi.nlm.nih.gov/books/NBK279684/table/appendices.T.supported_rewardpenalty_val/).
  This is local seeded alignment; the witness validator enforces the full-query
  edit threshold afterward. Its short-query task uses a 7-base seed, documented
  in the [NCBI options table](https://www.ncbi.nlm.nih.gov/books/NBK279684/table/appendices.T.blastn_application_options/).
- minimap2: index k=7, window=5; reduced chain/alignment score thresholds and
  minimum two anchors, plus strand, up to 100 secondary mappings. This explicitly
  tuned 20mer configuration retains minimizer-frequency filtering and chaining
  heuristics. It has no completeness guarantee for this task. Parameter meanings
  are in the [official manual](https://github.com/lh3/minimap2/blob/master/minimap2.1).

Each tool runs on a separate EC2 worker with 8 logical CPUs (4 physical cores).
ooff, BWA and BLAST use c7i.2xlarge with 16 GiB; minimap2 uses m7i.2xlarge
with 32 GiB after its dense index build exhausted the 16 GiB worker. Both instance
families use Intel Sapphire Rapids, but the memory and instance-family exception
is explicit. ooff uses eight scoped query threads sharing immutable mapped
indexes; the production annotated CLI remains single-threaded. All supported
thread flags and the independent multiprocessing verifier receive all eight
CPUs. BWA samse, BWA index and makeblastdb expose no parallel-thread option.
Providing all CPUs does not imply each program can keep them all busy.

The README plots median elapsed process time through independently verified
screening results, including reference/index loading, mapper output and scalar
verification. Index builds are excluded and listed in the caption. Each completed
point has three fresh-process runs. The classic tools ran on separate workers;
Sassy later reused the idle native worker with the same eight-vCPU/16-GiB allocation.
Runs were sequential within each worker and never overlapped. OS page cache is
not flushed. Individual observations remain in the CSV, including native's slower
first 100,000-ASO run. Failed/time-limited runs are retained; timeout text labels
and the OOM note are not completed timing points.

Every preparation, build, pilot, search, conversion and verification invocation is
recorded in [CLASSIC_ITERATIONS.md](../../benchmarks/CLASSIC_ITERATIONS.md), with
[CSV](../../benchmarks/classic/summary.csv) and raw JSON artifacts. `screening.json`
links each composite measurement to its component commands and witness evidence.
The figure CSV additionally separates search, SAM conversion, independent
verification, and total tool time, so the audit overhead is visible and can be
plotted separately. `verification_seconds` in that CSV is elapsed verification
process time; the raw verifier JSON also records its internal timer.
Raw mapper output remains on the retained EC2 volume. Scientific plots will be
rendered from these measured artifacts using matplotlib.


## Index preparation

Build time is measured separately and excluded from the search curves. The build CSV retains peak process RSS,
CPU use and actual commands. These are single measured builds, not three-run
medians. BWA's existing build ran on the original 16-vCPU/32-GiB c7i.4xlarge;
its builder is single-threaded, and the resulting index was hash-verified on the
8-vCPU search worker. Native forward index construction was repeated on eight
vCPUs/16 GiB. Native construction includes 21 intended-only records subsequently
filtered from search (321,668 additional bases, about 0.013%); competitors index
the already eligible reference. The screening plot uses only the forward native
index, so its build component does not include the optional reverse index
used for exhaustive-site optimization experiments. Build cache state was not
normalized. The 16-GiB minimap2 OOM remains in the ledger alongside the successful
32-GiB retry.

## Reproduce independent EC2 jobs

Use `benchmarks/classic/launch_ec2.py` once per tool. Each command launches one
bounded worker; `--spot` is optional. The recorded launch JSON and AMI identify
the environment. Example for the benchmark account/network:

```bash
python3 benchmarks/classic/launch_ec2.py --tool bwa --spot --region eu-west-2 --ami ami-03cf5768bcc686a8c --subnet subnet-0c4ffdd3ab8b55846 --security-group sg-0d9c6e1a43e7eca29 --ssh-public-key data/ec2/controller.pub --instance-type c7i.2xlarge --hours 3
python3 benchmarks/classic/launch_ec2.py --tool blast --spot --region eu-west-2 --ami ami-03cf5768bcc686a8c --subnet subnet-0c4ffdd3ab8b55846 --security-group sg-0d9c6e1a43e7eca29 --ssh-public-key data/ec2/controller.pub --instance-type c7i.2xlarge --hours 3
python3 benchmarks/classic/launch_ec2.py --tool minimap2 --spot --region eu-west-2 --ami ami-03cf5768bcc686a8c --subnet subnet-0c4ffdd3ab8b55846 --security-group sg-0d9c6e1a43e7eca29 --ssh-public-key data/ec2/controller.pub --instance-type m7i.2xlarge --volume-gib 200 --hours 3
```

The source worker publishes only public benchmark inputs using `publish_inputs.py`
and a private HTTP server on port 8765, restricted to this security group.
Copy `benchmarks/classic/` to each worker (or retrieve the published scripts
archive); run from `/home/ubuntu/ooff`, substituting its tool name:

```bash
bash benchmarks/classic/isolated_job.sh bwa http://172.31.19.41:8765 pilot
bash benchmarks/classic/isolated_job.sh bwa http://172.31.19.41:8765 measurement
```

Run the analogous BLAST/minimap2 commands on their separate workers, concurrently.
Input fetches verify SHA-256 manifests. BWA reuses the completed, measured index;
to reproduce that preparation itself, run `bwa index -p data/classic-v1/bwa
data/classic-v1/eligible.fa` through `command.py`. Fresh BLAST/minimap2 jobs build
their indexes through the same timing wrapper. All exact search and verification
commands are retained in each iteration's `result.json`.

Stages have a 600-second limit. A timed-out size retains one censored attempt and
skips its remaining repeats; larger sizes are still attempted once each. Other
failures halt the series for inspection. Original failures are not overwritten.

For more On-Demand capacity, select **Amazon EC2 → Running On-Demand Standard
(A, C, D, H, I, M, R, T, Z) instances** in the eu-west-2 Service Quotas console,
then request an account-level increase. Forty-eight vCPUs would fit four eight-vCPU
workers plus the six unrelated vCPUs observed in this account. The current IAM
identity cannot read Service Quotas; no increase was submitted. Spot has a
separate quota. See [AWS quota instructions](https://docs.aws.amazon.com/servicequotas/latest/userguide/request-quota-increase.html).

The screening timing plot uses base-10 logarithmic axes for ASO count and elapsed
time. The README contains no recovery or memory panel; those measurements remain
in the linked CSVs and methods. Earlier separate figures are retained as artifacts.

The minimap2 disk was expanded online from 50 to 100 GiB during the first
26,643-ASO search to retain raw outputs; provisioned IOPS/throughput were unchanged.
The affected observation is retained and the intervention is recorded in
`benchmarks/classic/minimap2-storage-intervention.json`.

The expansion completed after that first largest-batch attempt had already failed
with ENOSPC. Its partial 11.9-GB SAM and empty timing files are retained, and the
recovered result records unavailable time/exit status rather than invented values.
A separate encrypted 100-GiB gp3 output volume was then attached for the retry.
New iteration directories link to that volume through `OOFF_CLASSIC_STORAGE`;
parameters, executable, CPU allocation and verification are unchanged. Follow
these links when collecting metadata (`rsync -L`). For a fresh reproduction,
allocate 200 GiB initially as in the minimap2 launch command above.


## 100,000-ASO extension and configurable timeout

The additional point preserves all 26,643 original allele-derived queries and
adds 73,357 distinct SCN2A gene-body-derived ASOs. These are unique 20mers, not
repeated copies of the smaller pool. Added candidates are selected deterministically
by target-sequence SHA-256 order. This is a mixed allele/reference-derived scale
workload, not 100,000 observed alleles or experimentally validated ASOs.
`large-batch-manifest.json` records input/output hashes and
`queries-100000-origins.csv` records origins and gene-body offsets.

Generate inputs with `python3 benchmarks/classic/add_large_batch.py`; transfer
both query FASTAs, the origins CSV and manifest identically to each worker.
The supplied public SCN2A reference FASTA is `data/ec2/SCN2A-reference-v1.fa`.
Run independently on each tool's worker, substituting `ooff`, `blast` or `minimap2`:

```bash
OOFF_BENCH_THREADS=8 python3 benchmarks/classic/run_large_batch.py --tool bwa --run-timeout 600 --repetitions 3
```

`--run-timeout` is a total wall-clock budget across search, conversion and
verification for each repetition. `scale.py` also exposes `--timeout` as a stage
cap. Both defaults are 600 seconds and reject nonpositive values. Cancellation
has a short termination grace period. A timeout retains partial output and skips
later repetitions at that size. The earlier ≤26,643 runs used separate 600-second
stage limits; their original observations are retained with that distinction.
A real-subprocess test on EC2 verifies that time spent in the first stage reduces
the second stage's remaining budget. Independent input hashing and preparation
are outside this run budget, as is index construction.

## Final 100,000-ASO observations

| Tool | Elapsed seconds, repetitions | Verified witnesses | Outcome |
| --- | --- | --- | --- |
| ooff | 51.6648, 3.1914, 3.0899 | 100,000 each | Complete; median 3.1914 s |
| BWA-aln | 282.7566, 246.4556, 246.8984 | 98,774 each | Complete; median 246.8984 s |
| BLASTN-short | 600.0165 | Unavailable | Complete-run timeout; later repetitions skipped |
| minimap2 | 421.1612 to failure | Unavailable | Kernel-confirmed OOM on 32 GiB; later repetitions not run |

The median BWA/ooff ratio is 77.36× for this verified screening workload, with
98.774% versus 100% witness recovery. The first native run after worker restart
is retained in the range; the median is not a universal cold-start claim.
The minimap2 OOM sidecar and kernel evidence accompany its raw result. The series
audit recognizes that resource failure without treating it as a fast completion.
All four dedicated workers are stopped; all eight benchmark volumes are retained.

## Matched Sassy2 screening comparison

`benchmarks/classic/sassy_screen.rs` is compiled only in the separate EC2 crate
`/home/ubuntu/comparators/sassy-classic-screen-0.2.6`. It uses pinned Sassy 0.2.6,
IUPAC batched forward search, and eight query workers sharing the same eligible
RNA reference. Queries are distributed round-robin. Workers retire a query after
finding a valid full-query witness; unknown bases split reference runs, with
65,536-base chunks and 20+k overlap. This is an ASO screening adapter around
Sassy's library, not a timing of the upstream CLI's default output mode.

Endpoint-to-interval reconstruction uses the same fixed-interval verifier as
previous external comparator experiments. Every returned witness is then checked
by the existing independent Python scalar verifier against the original eligible
reference. The adapter passed separate scalar-oracle fixtures for k=0..3 with
one/eight workers, including chunk boundaries, indels, unknowns and absent hits.
The tested source, external Cargo manifest/lockfile and every invocation are retained.
The library method used does not launch a nested Rayon pool; the adapter supplies
all eight query workers. No persistent reference index is built for Sassy.

On the retained c7i.2xlarge (8 vCPUs, 16 GiB), all five sizes completed three
repetitions with 100% verified witness recovery. Median elapsed seconds:

| ASOs | Sassy2 |
| ---: | ---: |
| 10 | 2.1344 |
| 100 | 2.3857 |
| 1,000 | 3.0869 |
| 10,000 | 4.2969 |
| 100,000 | 19.3230 |

At 100,000, ooff's 3.1914-second median is 6.05× faster. This matched eight-thread
screening comparison is distinct from the one-thread exhaustive-output 10.67×
comparison. Sassy's initial human pilot took 18.58 seconds for 100 queries after
worker restart and is retained separately from the subsequent measurement series;
no cold-cache performance claim is made. All Sassy measurements use a 600-second
total per-repetition budget across search and independent verification.

Reproduce on an otherwise idle eight-vCPU EC2 worker with the shared prepared inputs:

```bash
bash benchmarks/classic/build_sassy.sh
python3 benchmarks/classic/test_sassy.py --binary /home/ubuntu/comparators/sassy-classic-screen-0.2.6/target/release/sassy-classic-screen
OOFF_BENCH_THREADS=8 OMP_NUM_THREADS=8 python3 benchmarks/classic/scale.py --tools sassy --counts 10 100 1000 10000 100000 --repetitions 3 --run-timeout 600 --timeout 600
```

The build script uses `sassy-Cargo.lock`; native ooff Cargo.toml/Cargo.lock do not
include Sassy. Build time for the comparator executable is retained in the ledger
but is not a reference-index preparation cost.
