# Human reference

The benchmark reference contains RNA-sense **gene bodies, including introns,
and exon-concatenated mature transcripts**, derived from GRCh38 primary assembly
and Ensembl release 110. It covers annotated transcriptional sequence rather
than whole chromosomes: intergenic DNA and alternative assembly loci are excluded.

## Source data

| Input | Ensembl 110 archive |
| --- | --- |
| Soft-masked primary assembly | [Homo_sapiens.GRCh38.dna_sm.primary_assembly.fa.gz](https://ftp.ensembl.org/pub/release-110/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna_sm.primary_assembly.fa.gz) |
| Gene, transcript and exon annotation | [Homo_sapiens.GRCh38.110.gtf.gz](https://ftp.ensembl.org/pub/release-110/gtf/homo_sapiens/Homo_sapiens.GRCh38.110.gtf.gz) |

The derived reference has **49,145 gene-body records and 239,274 mature-transcript
records**, totaling **2,438,901,225 bases**, including **970,360 unknown bases**.
The FASTA SHA-256 is:

```text
59d8b76ea1a968b8b9688ca8804c79fdbcca9157163a580b35eed103d4d6ba85
```

## Selection and orientation

The [builder](../benchmarks/build_reference.py) retains gene biotypes except
pseudogenes without a `transcribed_` or `translated_` prefix. Transcript records
are retained when their parent gene is retained. The manifest records the
observed biotypes, record/base counts, missing contigs and sequence hash.

Gene-body sequences follow the annotated strand. Transcript sequences join all
annotated exon blocks in transcript order, preserving junctions across any
number of exons. Lowercase repeat sequence remains present; case is normalized
at search time. Unknown sequence is retained in the reference and excluded from
candidate matching, with incomplete coverage recorded in results.

Records keep their gene associations and genomic blocks. Default summaries
deduplicate identical genomic sites across records; detailed reports retain
record identities. An intended-only gene is excluded by `--policy other-gene`.

## Preparation

Download the two archives above into `data/raw/`, preserving their filenames.
Run from the repository root:

```sh
python3 benchmarks/build_reference.py --raw data/raw --out data/reference-v1
```

The output includes `reference.fa`, `records.jsonl` and `manifest.json`. The
builder refuses to overwrite sequence outputs and reports missing contigs as
an error. [Build an index](USAGE.md#build-and-reuse-an-index) from this FASTA;
use the corresponding records file for annotation.

The recorded preparation took 196.410 seconds. Input checksums and reference
manifests are retained with the [benchmark artifacts](ARTIFACTS.md). Large
references and indexes are not bundled with the package.

Historical reference comparisons and deployment details are preserved in
[the development archive](archive/reference-2026-09-08.md).
