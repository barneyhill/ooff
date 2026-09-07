# Indexed search and range-level reporting

The input ASO is reverse-complemented once to obtain the target pattern in
RNA-sense orientation. The index contains only the supplied oriented RNA
records, with ambiguity and record separators preventing artificial joins.
Reversing both a pattern and a reference record is an algorithmic traversal
choice; it does not introduce a second biological binding orientation.

## Discovery

Backward FM extensions follow exact matches, substitutions, query-consuming
insertions and reference-consuming deletions. A state records the suffix-array
range, remaining query length, target span and remaining edit budget. State
memoization retains the greatest remaining budget seen for the same state;
a more expensive path cannot improve its future intervals. With no edits left,
the remaining exact tail has only one possible continuation.

The optional reversed-record index permits two limited passes. Each constrains
the first searched query half to at most `floor(k/2)` edits, then releases that
constraint for the other half. Any alignment with at most k edits has a half
within that allowance. The union therefore retains the full k-edit site set;
it is not valid to use only one limited pass as an exhaustive search.

## Reporting without a per-occurrence hash table

The earlier implementation located occurrences during traversal and merged
their `(record, start, end)` tuples in a hash table. Repeated sequence can cause
millions of entries for just one query. Compact entries reduce that cost, but
the main full-report path now moves the merge before occurrence expansion:

1. Collect each matching forward FM range with its target span and minimum cost.
2. Recover each reverse-pass matched word from the traversal's character buffer.
   Backward-search its reversal in the forward index to obtain a canonical
   forward range. Merge this range and span with the first pass, retaining the
   smallest cost.
3. Locate occurrences from every merged range once. Apply the explicit gene
   policy and emit each qualifying interval.

For a fixed span, two different words have disjoint suffix-array ranges: a
suffix cannot begin with two different words of the same length. Equal words
have equal ranges. Consequently `(forward range, span)` identifies all
occurrences of one word, and distinct final entries cannot emit the same
interval. Different spans remain distinct sites even if they share a start.
Taking the minimum cost over both passes preserves the minimum distance for
that word and hence for each of its occurrences.

This changes storage with repetition: the merge holds matching words rather
than every repeated occurrence. It still expands all required sites; it does
not remove repeats or collapse distinct transcript records into genomic loci.
The production CLI independently aligns every emitted source interval to
produce its CIGAR, ASO edit positions and genomic annotations.

Screening and capped reporting retain the early-stop traversal. Full range
collection would add work before a single witness when early termination is
sufficient. The compact occurrence table remains available for those paths.
Before returning a screening witness, the traversal reconstructs its matched
word and computes the minimum full-query distance for that interval. The first
successful edit path can have a higher cost than another alignment of the same
word; returning that path cost directly would overstate the witness distance.
The independent classic-benchmark verifier checks the reported distance against
the original reference bases using scalar global alignment.

## Evidence and limits

`tests/fm.rs` compares the ordinary, single-index unique and paired-index unique
paths against an independent scalar global-distance oracle at k=0..3. It also
asserts that range expansion never emits a duplicate. Fixtures cover repeats,
mixed edits, ambiguity, rank-block boundaries and coordinate remapping.
`tests/indexed_cli.rs` compares full annotated outputs across the scalar,
indexed, reversed-index and cached paths, including wrapped FASTA records.

The full human-reference audit completed in
`20260907T200643-fm-word-union-full-tuples1000`: three repetitions produced
135,340,605 unique tuples per engine, exactly equal between native and external
Sassy. The comparison partitions rows to bound memory, then compares complete
row multisets and checks uniqueness; it does not rely on hash equality.
The eight-worker human 32-query gate also passed at k=0..3. Each benchmark
retains its own binary hash and source snapshot, so these results remain tied
to the tested implementation and workload.

Later experiments with exact-word caching and extension reuse preserved the
tested site sets but did not improve timings. Word caching remains opt-in;
extension reuse was removed after measurement. See
[research and experiment outcomes](OPTIMIZATION_RESEARCH.md).
