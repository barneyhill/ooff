# Human reference selection and provenance

The historical benchmark specifies GRCh38 and Ensembl 110. Its metadata records
only the renamed Mac file `/Users/barneyh/dphil/paper3/data/offtarget/GRCh38.dna_sm.fa`.
It does not establish whether that was primary assembly or toplevel, and provides
no source download hash. The legacy scripts import the interval/biotype filter
from `analyses.logic.offtarget_filter`, which is not included in this repository.
An exact reproduction of the old reference is therefore not yet possible.

For the new implementation, select these archive inputs (filenames verified in
the official Ensembl listings on 2026-09-07):

| Purpose | Ensembl 110 archive input |
| --- | --- |
| Soft-masked primary assembly | [Homo_sapiens.GRCh38.dna_sm.primary_assembly.fa.gz](https://ftp.ensembl.org/pub/release-110/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna_sm.primary_assembly.fa.gz) |
| Gene/transcript/exon annotation | [Homo_sapiens.GRCh38.110.gtf.gz](https://ftp.ensembl.org/pub/release-110/gtf/homo_sapiens/Homo_sapiens.GRCh38.110.gtf.gz) |

These inputs were downloaded on the dedicated EC2 worker on 2026-09-07.
Archive CHECKSUMS and compressed-input SHA-256 values are retained in the worker
raw-data directory and `results/reference-downloads.sha256`. The derived
`data/reference-v1/manifest.json` records the exact observed biotype list.
The RNA FASTA contains 49,145 gene bodies and 239,274 mature transcripts,
2,438,901,225 bases including 970,360 unknown bases. Its SHA-256 is
`59d8b76ea1a968b8b9688ca8804c79fdbcca9157163a580b35eed103d4d6ba85`.
All requested contigs were present. Reference preparation took 196.410 seconds.

Build strand-oriented gene-body records for pre-mRNA/ncRNA and exon-concatenated
mature-transcript records from the same assembly and GTF. Keep genomic blocks
in transcript order, including short exons and multiple junctions. Retain every
gene/transcript association if deduplicating. A junction window must not be
limited to just two exons. Missing contigs are an error or explicit incomplete
coverage, never silently omitted. Primary assembly excludes alternative loci;
record this restriction and do not claim the historical toplevel scope matches.

The handoff's intended biotype policy includes annotated transcribed genes and
ncRNA, including transcribed/translated pseudogenes but excluding other
pseudogenes. The original exact predicate is unavailable. Before building the
new reference, implement and record an explicit policy over the GTF biotypes
and label it as a new policy, not byte-identical paper3 reproduction. Do not
substitute all annotated pseudogenes or protein-coding-only coverage silently.

Retain lowercase repeat sequence. Normalize case only at search time. Count
unknown bases and split at ambiguity. Record excluded known runs too if a
builder removes any; the native CLI does not discard short known runs. The old
benchmark dropped known runs shorter than 17 and used 100,000-base cores with
23-base overlap. Its merged genomic intervals and strand-agnostic off-locus
classification are historical timing conventions, not the new biological model.

## EC2 state inspected on 2026-09-07

Configured region: `eu-west-2`. Two existing running workers were found, named
`drugset-origin` and `hutwatch`; both belong to other workloads and were left
unchanged. The dedicated ooff worker is `i-0b5bad3102a5345c5`, originally a
c7i.4xlarge and later resized to c7i.2xlarge for eight-vCPU classic comparisons.
Its retained encrypted volume is `vol-0138903fbd36c52bf`. Native work is complete
and its stopped state was verified on September 7 at 22:53 UTC. Current classic
worker states and retained volumes are recorded in [ARTIFACTS.md](../ARTIFACTS.md).
See BENCHMARKS.md for execution details; do not delete any retained data.
Stop a completed dedicated worker; do not delete instances, volumes, downloads
or existing data without the user's permission.

Sassy v0.2.6 is only an external EC2 comparator. Keep its binary/build artifacts
outside the ooff dependency graph. Compare equivalent query orientations,
reference coverage, edit budgets and policy. Its ordinary CLI reports local
minimum representatives, whereas ooff report enumerates distinct intervals:
their raw hit counts and full-report timings are not directly interchangeable.

## Allele-derived query provenance

The local historical cache `/home/barneyh/dphil/paper3/data/genes/SCN2A.npz`
was recovered during this run. Its SHA-256 is
`cf6a80cf42db7b47b5c6fd21770d0c37aec114ab7e9022a9f7da052350684be4`.
It contains 1,340 variant target rows (1,256 SNP and 84 indel rows), built with
the historical minimum allele frequency 0.01 and nearby-variant filter, without
a ddG filter. Every REF allele matched the selected public Ensembl sequence.

`benchmarks/make_cached_allele_queries.py` generates every ACGT 20mer overlapping
each target allele, accounting for indel length. The result contains 26,643
unique ASO sequences and retains all 26,940 variant/window associations in JSONL.
No sample or haplotype arrays are exported. Output hashes and the exact policy
are in `data/cached-allele-queries-v1/manifest.json`; 1k, 10k and full-pool FASTA
subsets are benchmarked against the same human reference.

This older variant-associated pool is not the later 6,338-ASO eligible export
described in the historical handoff, which has not been located. It establishes
neither spared-allele discrimination nor eligibility. The provided 32-query
allele-specific shortlist is also benchmarked separately; the original 1,000
SCN2A reference tiles are not described as allele-specific.
