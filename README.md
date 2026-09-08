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
for Linux and macOS, on Intel/AMD and ARM.

```sh
# Download and index the human reference once.
oofft reference prepare hg38

# Search ASOs in FASTA format; exclude the intended gene.
oofft --queries asos.fa --exclude SCN2A > counts.jsonl
```

ASOs are written **5′→3′** in standard FASTA (`.fa`, `.fasta`, or gzip-compressed):

```fasta
>aso1
GCATCGTAGCTAGATCACGT
```

**hg38 is the default reference.** Presets include human (`hg38`), mouse (`mm39`),
rat (`rn7`), cynomolgus macaque (`cyno`) and rhesus macaque (`rhesus`), with pinned
Ensembl 110 annotations. For example, prepare `mm39`, then search with
`--reference mm39 --exclude Scn2a`. `oofft reference list` shows assemblies and
which references are ready. [Preset details](docs/reference.md).

`--exclude` accepts gene symbols or Ensembl IDs, repeated or comma-separated.
Omit it to count sites in all genes. The default output contains one summary per
ASO, with distinct genomic-site counts at edit distances **0, 1, 2 and 3**.
Sites shared by gene-body and transcript records count once. `--genes` adds gene
IDs; `--sites` emits coordinates and alignments. Summary mode accepts 4–63 nt
ASOs; screening and site reports require 20 nt.

[Manual references and JSONL](docs/USAGE.md) · [Count semantics](docs/SUMMARY.md) ·
[Optional ΔΔG](docs/DDG.md)

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
assembly loci. Unknown bases mark counts incomplete. Gene exclusion does not assess
discrimination between alleles of the same gene.

Search tests use independent scalar oracles; energy tests compare against
ViennaRNA. GitHub Actions tests Linux and macOS on x86-64 and ARM64.
[Reference provenance](docs/reference.md) · [Algorithm](docs/ALGORITHM.md) ·
[Documentation](docs/README.md) · [License and parameter attribution](LICENSES.md)
