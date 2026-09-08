# Usage

## Prepare a reference

```sh
oofft reference list
oofft reference prepare hg38
```

Preparation downloads the pinned assembly FASTA and annotation GTF, extracts
RNA-sense gene bodies and spliced transcripts, and builds a compact FM index.
The completed bundle includes gene names, coordinates and provenance; search
loads these automatically. Preparation is explicit because mammalian references
require substantial download and disk space. A missing preset produces a
preparation command, without starting a download during search.

The default cache is `$XDG_CACHE_HOME/oofft/references`, or
`~/.cache/oofft/references` when XDG_CACHE_HOME is unset. Set `OOFFT_CACHE_DIR` or
pass `--cache-dir DIR` to preparation and search to choose another location.
Completed references are reused. `--rebuild` prepares a new generation and
retains the old one. Failed preparation cannot replace a completed reference.

See [species, assemblies and download sources](reference.md).

## Search FASTA ASOs

```sh
oofft --queries asos.fa --exclude SCN2A > counts.jsonl
```

`--reference hg38` is implicit. Select another prepared reference explicitly:

```sh
oofft reference prepare mm39
oofft --queries asos.fa --reference mm39 --exclude Scn2a > mouse-counts.jsonl
```

Each FASTA header's first whitespace-delimited token is its unique query ID.
Sequences are ASOs written 5′→3′; the engine reverse-complements them to search
RNA-sense records. Wrapped lines and gzip compression are supported. Case and
U/T are normalized; ambiguous ASO bases are rejected. Summary mode supports
mixed lengths of 4–63 nt. Screening and detailed reports require 20 nt.

### Gene exclusions

`--exclude SCN2A` excludes that gene from every query. Use
`--exclude SCN2A,SCN3A` or repeat the option for multiple genes. Prepared references
resolve symbols and Ensembl IDs using their own annotation, accepting case
variations and numeric ID version suffixes. Unknown or ambiguous symbols are
errors; use an explicit ID to disambiguate. No orthologs are inferred across species.

Without exclusions, sites in all genes—including the intended gene—are counted.
On a record associated with several genes, a site is retained if any association
is outside the excluded set. Excluding a gene therefore does not hide another
gene's association, and does not assess allele selectivity within the excluded gene.

JSONL remains available for query-specific exclusions and metadata:

```json
{"id":"aso1","sequence":"GCATCGTAGCTAGATCACGT","intended_genes":["ENSG00000136531"]}
```

`intended_genes` is optional and is combined with `--exclude`. Optional `allele`
metadata is preserved. Optional `target` supplies the intended RNA sequence for
subsequent energy annotation. Existing `--policy other-gene` invocations remain
accepted for compatibility; new commands need no policy flag.

## Output

Output streams as JSONL: a provenance manifest, optional sites, per-query
summaries, and a final `run_complete`. Accept a result only after a successful
exit and the final marker. The manifest records input hashes, reference release,
edit budget, exclusions and coverage.

| Invocation | Result |
| --- | --- |
| `oofft` or `oofft summary` | Distinct-genomic-site counts in edit bins 0–3 |
| Add `--genes` | Counts plus gene IDs |
| `oofft report` or add `--sites` | Annotated intervals, coordinates and alignments |
| `oofft screen` | First qualifying witness per ASO |

Each substituted, inserted or deleted base costs one edit. `-k 0` through `-k 3`
selects the budget; default 3. Sites shared by transcripts and gene bodies count
once per ASO, in their minimum-distance bin. Overlapping intervals with different
boundaries remain separate. [Count semantics](SUMMARY.md).

Indexed summaries use available logical CPUs by default; `--threads N` overrides
this. Their workers share one mapped index. Detailed reports use record-interval
identity and may have more rows than genomic-site summaries. `--max-sites N`
caps reports and marks them incomplete; it cannot cap exhaustive summaries.
A screening witness need not be the best match in the reference.

## Manual references

Use your own assembly and GTF to prepare a named reference:

```sh
oofft reference prepare my-reference --fasta genome.fa.gz --gtf genes.gtf.gz
oofft --queries asos.fa --reference my-reference --exclude MYGENE
```

The GTF requires `gene` records and exon annotations with gene/transcript IDs;
genes must have `gene_biotype` or `gene_type`. Preparation applies the documented
[biotype filter](reference.md#rna-records-and-coverage). Custom names cannot
replace built-in preset identities.

For a small RNA sequence collection, search FASTA directly:

```sh
oofft --queries asos.fa --reference transcripts.fa
```

Bare FASTA has record-relative coordinates and defaults to `record-interval`
counts, since genomic equivalence cannot be inferred. Headers may include
`gene=ID` or Ensembl-style `gene:ID`; otherwise each record ID is its association.
`--exclude` accepts those IDs. For real genomic coordinates and symbol lookup,
prepare a FASTA/GTF bundle instead. Direct FASTA search is a sequential fallback;
use an index for large inputs.

Existing annotated JSONL references also remain supported:

```sh
oofft --queries fixtures/queries.jsonl --reference fixtures/reference.jsonl
```

Each reference record has `id`, `sequence`, `genes`, `contig`, `strand`, and
zero-based half-open `blocks` in transcript order; optional `transcripts` retain
isoform associations. Sequence lengths must equal the total block length.

### Existing manual indexes

```sh
oofft --queries asos.fa --reference reference.fa \
  --index index --annotations records.jsonl --exclude ENSG00000136531
```

The FASTA must be the index's original RNA-sense source, with headers
`record-id|gene-id[,gene-id]`. Annotations follow FASTA record order. Manual
`--reference-release`, `--scope` and `--biotype-policy` labels are optional;
without them, provenance identifies a custom supplied reference. They cannot
override prepared-preset metadata. Manual exclusions use annotated gene IDs.

`oofft-index build`, `compact` and `cache-annotations` remain available. A
second index built with `--reverse-records` can be supplied as `--reverse-index`
for paired-direction search. It reverses records and patterns together as an
algorithmic optimization, without adding another biological strand.

Keep mapped references and indexes immutable during searches. When relocating
manual indexes, use [relocate-index.py](../scripts/relocate-index.py) to verify
sequence identity and update the manifest.

## Interpretation

Soft-masked repeat sequence is retained. Unknown reference bases are excluded
from matching and mark `counts_complete: false`. A completed process can thus
have incomplete sequence coverage. Unsearched edit bins are `null`, not zero.
No match means none within the supplied known sequence and selected edit budget.

Count mode does not calculate ΔΔG. Use the separate [energy annotator](DDG.md)
for reported sites. Sequence similarity and modeled energies support experimental
prioritization; they do not establish knockdown or clinical safety.

## Build requirements

Prebuilt release archives contain `oofft`, `oofft-index` and `oofft-ddg`.
Linux archives require glibc 2.35 or later and the OpenMP runtime (`libgomp1`
on Debian/Ubuntu). macOS archives require macOS 13 or later and bundle OpenMP.

To build from source, install Rust 1.94 or later and a C compiler with OpenMP
support. On Debian/Ubuntu, install `build-essential`; on macOS, install the
Xcode command-line tools and Homebrew `libomp`:

```sh
# macOS source builds
brew install libomp
CFLAGS="-I$(brew --prefix libomp)/include" cargo install oofft --locked
```

The repository pins its development toolchain in `rust-toolchain.toml`.
Its local Cargo configuration enables native CPU optimization; use
`RUSTFLAGS='' cargo build --release --locked` for portable target defaults.
ViennaRNA is only needed for the external energy engines and live oracle tests;
the Rust energy engine runs independently. Sassy2 and the other comparator tools
are benchmark dependencies only.
