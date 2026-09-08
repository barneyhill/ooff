# Usage

Install with `cargo install oofft --locked`, or use a [release archive](https://github.com/barneyhill/oofft/releases).
Linux release binaries require `libgomp1`; macOS archives include the OpenMP
runtime. Rust 1.94 or later is required to build from source.

## Inputs and output

Queries are JSONL, one object per ASO. Sequences are written 5′→3′:

```json
{"id":"aso1","sequence":"GCATCGTAGCTAGATCACGT","intended_genes":["intended"]}
```

Summary mode accepts lengths 4–63 nt, including mixed lengths. Detailed reports
and first-witness screening require 20 nt. Ambiguous query bases are rejected;
case and U/T are normalized. Optional `allele` metadata is preserved. Optional
`target` supplies the intended RNA sequence for subsequent energy annotation.

Small references can be JSONL with sequence and annotation together:

```json
{"id":"rna1","sequence":"ACGTGATCTAGCTACGATGC","genes":["other"],"transcripts":[],"contig":"chr1","strand":"+","blocks":[{"start":100,"end":120}]}
```

Reference sequences are RNA-sense, 5′→3′. The engine reverse-complements the ASO
for search. Genomic blocks are zero-based, half-open and ordered in transcript
order; their total length must equal sequence length. A gene body has one block;
a spliced transcript has exon blocks. See the complete [fixtures](../fixtures/).

Output streams as JSONL: a provenance manifest, optional site records,
`query_summary` records and a final `run_complete`. Require both a successful
process exit and the final marker before accepting a result. Input hashes,
reference labels, edit budget and coverage status accompany results.

The default counts distinct genomic sites in minimum edit-distance bins 0–3.
A site is identified by contig, strand and ordered genomic blocks. Identical
sites shared by transcripts count once per ASO; overlapping intervals with
different boundaries remain distinct. See [summary semantics](SUMMARY.md).

## Build and reuse an index

For large references, supply RNA-sense FASTA with headers
`record-id|gene-id[,gene-id]`, plus annotation JSONL in the same record order.
[Human reference preparation](reference.md) produces both files.

```sh
OMP_NUM_THREADS=8 oofft-index build \
  --reference reference.fa --output index --shard-bases 1300000000

oofft --queries queries.jsonl --reference reference.fa \
  --index index --annotations records.jsonl \
  --policy other-gene --reference-release Ensembl110-GRCh38 \
  --scope 'Gene bodies and mature transcripts' \
  --biotype-policy 'Exclude nontranscribed/nontranslated pseudogenes' \
  --threads 8 > counts.jsonl
```

The reference labels describe the inputs actually supplied; change them for
other releases or policies. `other-gene` retains sites associated with any gene
outside the query's intended-gene set. Intended-only records are excluded,
so this policy does not evaluate discrimination between alleles of one gene.

Indexed summary workers share one mapped index. With no `--threads` override,
summary mode uses available logical CPUs, capped by query count. Keep all
reference, annotation and index files immutable during a run.

A compact index trades additional locate work for lower memory requirements:

```sh
oofft-index compact --index index --output compact-index --sample-rate 16
```

Use `--index compact-index` in subsequent searches. The measured human forward
index occupies 2.90 GB in this format; a 1,000-ASO Pi 5 sample used about 3 GiB
peak RSS. Memory also depends on the number of matching sites and workers.
Compaction requires an existing full index; its construction has a larger
memory requirement than compact-index search. [Measurements and limits](SUMMARY.md).

To cache validated annotation offsets:

```sh
oofft-index cache-annotations --index compact-index \
  --reference reference.fa --annotations records.jsonl \
  --output annotation-cache.json
```

Add `--annotation-cache annotation-cache.json` when searching. Rebuild the cache
when its inputs change. If copying a reference/index to another machine, use
[relocate-index.py](../scripts/relocate-index.py) to verify sequence identity and
update the reference manifest; do not manually bypass identity checks.

An optional second index, built with `--reverse-records`, can accelerate
exhaustive search via `--reverse-index DIR`. This reverses records and patterns
together as a search optimization; it does not add a biological strand.

## Select an output mode

| Invocation | Result |
| --- | --- |
| `oofft` or `oofft summary` | Exhaustive distinct-genomic-site counts |
| Add `--genes` | Counts plus other-gene IDs |
| `oofft report` or add `--sites` | All annotated record intervals, coordinates and alignments |
| `oofft screen` | First qualifying witness per ASO |

All modes require the query, reference and provenance arguments shown above.
Use `-k 0` through `-k 3` to choose the edit budget (default 3). Each substituted,
inserted or deleted base costs one edit. For example, two substitutions plus a
one-base deletion cost three edits. A two-base insertion costs two edits.

Detailed reports use `(query, record, start, end)` identity, so their row count
can exceed a deduplicated genomic-site count. `--max-sites N` caps reports and
marks them incomplete; it is unavailable for exhaustive summaries. A screen
witness's distance is not necessarily the best distance in the reference.

Report alignments use RNA-sense order: `=` match, `X` substitution, `I` consumes
an ASO/query base and `D` consumes a reference base. Original ASO coordinates,
strand and splice-junction blocks are retained. One minimum-cost alignment is
reported per interval, with diagonal, insertion, deletion traceback tie order.

## Coverage and interpretation

Soft-masked sequence is retained. Search splits at ambiguous reference bases;
unknown sequence sets `counts_complete: false`. A completed run can therefore
have incomplete biological coverage. Unsearched edit bins are `null`, not zero.
No match means no candidate within the supplied known sequence and edit budget.

Count output does not calculate ΔΔG. To score reported intervals, use the
separate [energy annotation command](DDG.md). Candidate matches and modeled
energies support experimental prioritization; they do not predict knockdown
or account for tissue expression, delivery or chemistry-specific activity.
