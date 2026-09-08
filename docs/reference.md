# Reference presets

```sh
oofft reference list
oofft reference prepare hg38
```

Presets combine an assembly and **Ensembl release 110** annotation. Versions are
pinned rather than following a changing “latest” download. **hg38 is the default**
for search; other species must be selected with `--reference`.

| Preset | Species | Assembly | Assembly sequence scope |
| --- | --- | --- | --- |
| `hg38` | Human | GRCh38 | Primary assembly |
| `mm39` | Mouse | GRCm39 | Primary assembly |
| `rn7` | Rat | mRatBN7.2 | Toplevel |
| `cyno` | Cynomolgus macaque (*Macaca fascicularis*) | Macaca_fascicularis_6.0 | Toplevel |
| `rhesus` | Rhesus macaque (*Macaca mulatta*) | Mmul_10 | Toplevel |

Aliases include `human`/`GRCh38`, `mouse`/`GRCm39`, `rat`/`mRatBN7.2`,
`cynomolgus`, and `Mmul_10`. Monkey species are explicit; no generic `monkey`
alias or automatic cross-species gene substitution is used.

## Downloads and integrity

The native preparer retrieves soft-masked DNA FASTA and the matching GTF over
HTTPS from the official Ensembl archive:

| Species | FASTA directory | GTF directory |
| --- | --- | --- |
| Human | [DNA](https://ftp.ensembl.org/pub/release-110/fasta/homo_sapiens/dna/) | [GTF](https://ftp.ensembl.org/pub/release-110/gtf/homo_sapiens/) |
| Mouse | [DNA](https://ftp.ensembl.org/pub/release-110/fasta/mus_musculus/dna/) | [GTF](https://ftp.ensembl.org/pub/release-110/gtf/mus_musculus/) |
| Rat | [DNA](https://ftp.ensembl.org/pub/release-110/fasta/rattus_norvegicus/dna/) | [GTF](https://ftp.ensembl.org/pub/release-110/gtf/rattus_norvegicus/) |
| Cynomolgus | [DNA](https://ftp.ensembl.org/pub/release-110/fasta/macaca_fascicularis/dna/) | [GTF](https://ftp.ensembl.org/pub/release-110/gtf/macaca_fascicularis/) |
| Rhesus | [DNA](https://ftp.ensembl.org/pub/release-110/fasta/macaca_mulatta/dna/) | [GTF](https://ftp.ensembl.org/pub/release-110/gtf/macaca_mulatta/) |

File names, release and assembly scope are encoded in the preset catalogue.
Provider `CHECKSUMS` entries verify the downloaded compressed bytes using the
Ensembl BSD checksum and 1-KiB block count. SHA-256 hashes are also recorded for
source files and derived sequences/annotations. Cached downloads are checked
before reuse. Gzip decoding validates compressed data, including concatenated
members. Ensembl's BSD checksum is an integrity check, not a cryptographic signature.

Each build occupies an immutable generation directory. Only a fully prepared
bundle is activated. Concurrent preparations of the same reference are blocked
by a filesystem lock; interrupted downloads/builds are retained but never used
as completed references. `--rebuild` activates a new generation after completion,
preserving the previous one.

## RNA records and coverage

Search covers **RNA-sense gene bodies, including introns, and exon-concatenated
mature transcripts**. It does not scan intergenic DNA. Primary-assembly presets
exclude alternative loci; toplevel presets retain the sequence regions present
in their Ensembl source and annotate only eligible genes/transcripts there.

The builder retains gene biotypes except pseudogenes without a `transcribed_`
or `translated_` prefix. Transcript records require an eligible parent gene.
The GTF must contain gene records, gene biotypes, transcript IDs on exons and
valid coordinates. Missing FASTA contigs, conflicting gene/transcript annotations,
empty transcripts and invalid block layouts fail preparation.

Minus-strand genes and exons are reverse-complemented into RNA-sense order.
Spliced records include all annotated exon junctions. Lowercase repeat sequence
is retained. Unknown bases remain in the reference, are excluded from matching,
and mark search coverage incomplete. Gene symbols are resolved within the
selected reference; ambiguous or unknown exclusions fail explicitly.

Prepared bundles include `reference.fa`, `records.jsonl`, `genes.json`, a compact
index, an annotation cache and `bundle.json`. The manifest records source URLs,
hashes, assembly/release, selection policy and record/base counts.

## Build resources

Preparation holds one assembly FASTA record at a time plus parsed GTF metadata.
It then constructs the index in shards, writing sampled suffix positions directly
to compact files. It does not require a full suffix-array index on disk first.
`--threads N` controls build workers; the default is available logical CPUs.
`--shard-bases N` controls the approximate build shard size (default 200 million
bases). Individual RNA records are not split, so this is not a hard memory limit.

Mammalian preparation needs several gigabytes of RAM and disk space for compressed
sources, RNA records and indexes. Actual usage depends on species and annotation.
The published performance measurements use their recorded index configuration;
they are not timings of every preset or this preparation pipeline.

## Human benchmark reference

The published human comparison uses the same assembly/release and biotype scope:
**49,145 gene-body records and 239,274 mature transcripts**, totaling
**2,438,901,225 bases**, including **970,360 unknown bases**. Its FASTA SHA-256 is:

```text
59d8b76ea1a968b8b9688ca8804c79fdbcca9157163a580b35eed103d4d6ba85
```

The benchmark builder is retained in [benchmarks/build_reference.py](../benchmarks/build_reference.py).
A newly prepared bundle records its own content hash; record ordering and index
configuration can differ. [Benchmark methods](CLASSIC_BENCHMARK.md) describe the
measured inputs and configuration.
