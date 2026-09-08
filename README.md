# oofft

**Fast off-target search for antisense oligonucleotides.**

Count candidate binding sites across annotated pre-mRNA and mature RNA, including
substitutions, insertions and deletions. Written in Rust, with reusable FM indexes,
parallel counting and optional relative hybridization energy (ΔΔG) annotation.

Updated industry recommendations from
[Andersson et al. (2025)](https://doi.org/10.1089/nat.2024.0072) prioritize
RNase H1 ASO off-targets with
**≤3 mismatches** when experimental expression data are absent. oofft searches
**≤3 total edits** by default, counting each substituted, inserted or deleted
base once. This is an implementation choice extending mismatch-only search;
sequence similarity alone does not establish biological activity.

[Watt et al. (2020)](https://doi.org/10.1089/nat.2020.0847) evaluated **96 gapmer
ASOs and 832 nearly matched unintended transcripts**, finding an association
between relative hybridization energy and relative potency among affected
transcripts. Optional scoring uses **ΔΔG = ΔG(off-target) − ΔG(intended)**:
−15 − (−20) = **+5 kcal/mol**, meaning weaker predicted off-target binding.
The RNA:RNA model does not explicitly represent modified gapmer chemistry.

## Install and run

```sh
cargo install oofft --locked
```

[Prebuilt binaries](https://github.com/barneyhill/oofft/releases) are available
for Linux and macOS, on Intel/AMD and ARM. From a checkout, run the small fixture:

```sh
oofft --queries fixtures/queries.jsonl --reference fixtures/reference.jsonl \
  --policy other-gene --reference-release fixture-v1 \
  --scope synthetic --biotype-policy all-fixture-records > counts.jsonl
```

The fixture returns **1, 2, 3 and 4 sites** for `aso1` in the four edit bins.
The default output has one summary per ASO, with distinct genomic-site counts
at edit distances **0, 1, 2 and 3**. Sites shared by transcript and gene-body
records count once; overlapping intervals with different boundaries remain
separate. `--genes` adds gene IDs; `--sites` emits coordinates and alignments.
Summary mode accepts 4–63 nt ASOs; screening and site reports require 20 nt.

For human-scale inputs, [prepare and reuse an index](docs/USAGE.md).
[Input and output formats](docs/USAGE.md#inputs-and-output) ·
[Count semantics](docs/SUMMARY.md) · [Optional ΔΔG](docs/DDG.md)

## Performance

![ASO screening benchmark: median elapsed time versus batch size on log10 axes, comparing oofft, Sassy2, BWA-aln, BLASTN-short and minimap2](docs/images/classic-scaling.svg)

**Screening 100,000 ASOs: 3.19 s with oofft, 19.32 s with Sassy2, 246.90 s
with BWA-aln.** Screening finds one qualifying site per ASO; exhaustive counting
is a different workload. A full SCN2A walk of **114,889 ASOs** took **7 min 19 s**
for distinct-site counts at ≤3 edits on eight threads ([measurement](docs/SUMMARY.md#measured-scn2a-run)).

The plot shows medians of three runs, eight logical CPUs, retained OS cache,
and Ensembl 110 GRCh38 **gene bodies plus transcripts**. Loading, output and
independent witness verification are included; index construction is timed
separately. At 100,000 ASOs, witness recovery was 100% for oofft/Sassy2 and
98.774% for BWA. BLAST timed out at ≥1,000; minimap2 ran out of 32 GiB RAM at
100,000. These failures are not plotted as completed measurements.

[Benchmark methods and build costs](docs/CLASSIC_BENCHMARK.md) ·
[Plot data](docs/images/classic-scaling.csv) · [All benchmark records](BENCHMARKS.md)

## Reference scope and validation

Search uses supplied RNA-sense sequences and gene annotations. The human benchmark
includes introns and splice junctions, but excludes intergenic DNA and alternative
assembly loci. Unknown bases mark counts incomplete; the `other-gene` policy
excludes intended-only genes and does not assess allele selectivity.

Search tests use independent scalar oracles; energy tests compare against
ViennaRNA. GitHub Actions tests Linux and macOS on x86-64 and ARM64.
[Reference provenance](docs/reference.md) · [Algorithm](docs/ALGORITHM.md) ·
[Documentation](docs/README.md) · [License and parameter attribution](LICENSES.md)
