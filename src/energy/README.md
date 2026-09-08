# Independent Rust duplex energy implementation

`mod.rs` implements an antiparallel intermolecular minimum-energy dynamic program
in Rust. It does not call or link ViennaRNA. It optimizes over all duplexes within
the supplied two sequences, including stacks, bulges, internal loops up to 30
unpaired bases, terminal AU/GU effects, and exterior dangling bases. It reports
integer units of 0.01 kcal/mol; no edit cutoff or alignment constraint is used.

The current supported parameter model is RNA:RNA Turner 2004 at 37 C, ViennaRNA
2.7.0 defaults (default salt, GU pairing enabled). Custom temperature, salt,
chemistry and alternate parameter sets are not supported by this experimental
kernel. The existing whole-transcript RNAplex mode remains a separate engine.

Empirical tables in `turner2004_37c.json` were exported from installed ViennaRNA
2.7.0 using `benchmarks/ddg/export_parameters.c`; that exporter is a development
tool only. The table data retain attribution to the ViennaRNA authors and the
Institute for Theoretical Chemistry, University of Vienna. The upstream terms
are retained in `VIENNA_NOTICE`; these third-party tables are separate from the
project's MIT-licensed Rust implementation. The package license file and binary
release archives retain both sets of terms and this attribution.

Oracle sources inspected, pinned at v2.7.0:

- `src/ViennaRNA/duplex.c` (state transitions and exterior contributions)
- `src/ViennaRNA/eval/eval_internal.c` (nearest-neighbor loop model)
- `src/ViennaRNA/eval/eval_exterior.c` (dangling and terminal contributions)

These files are retained under `data/ddg-source`. Parameter dimensions and units
come from the corresponding installed `ViennaRNA/params/basic.h` header.

The optional optimization derives lower bounds from the complete energy tables.
Independent minima over loop types and neighboring bases are lower bounds on
every realizable loop energy. A predecessor can be skipped when its DP energy
plus that bound is no better than the current state. Row minima provide a second,
more optimistic bound for skipping a whole predecessor row. Equality may be
skipped because only energy, not a thermodynamic traceback, is returned.

`tests/energy.rs` compares exact integer energies against 1,825 pinned direct
RNAduplex outputs. `benchmarks/ddg/generate_energy_golden.py` regenerates the
oracle fixtures. End-to-end comparisons use `estimate_sites.py --energy-engine
rust`, including every original report row and component energy. These are
empirical checks in addition to the recurrence/bound reasoning. Current measured
performance and its workload scope are documented in `docs/DDG.md`.

The fast recurrence reuses a `DuplexWorkspace` per worker for encoded sequences
and DP buffers. It factors predecessor mismatch terms and specializes small
loops without changing the model. An experimental AArch64 NEON reduction is
available as `--features energy-neon`; it is disabled by default because the
1,825-case compute benchmark regressed by about 3.6% on the Pi. Reusing buffers
alone improved that benchmark by only about 0.4%.

Site annotation serializes typed energy metadata directly alongside the original
report fields and batches pairs across records. The work manifest reports cumulative worker time for parsing, annotation,
duplex scoring and serialization. Annotation includes off-target duplex work;
duplex scoring also includes intended targets. Setup, input, wave and spool
timers measure wall time; waves overlap input/spool work. These overlapping
measurements must not be summed.

A second disabled experiment, `--features energy-prune`, builds prefix/suffix
maximum-cardinality ordered matchings. If a duplex passes through a cell, its
number of transitions is at most the sum of those two matching sizes. Multiplying
that count by the non-positive global minimum loop cost and adding lower bounds
for initiation and both exterior contributions yields a lower bound on every
such duplex. An exact uninterrupted helix seeds a valid upper bound. Cells whose
lower bound is no better than that upper bound can be excluded without changing
the minimum energy. The relaxed matching permits unlimited gaps, so it includes
all permitted loop paths. On the tested Pi workloads the bookkeeping outweighed
pruning: the diverse kernel regressed about 21%, with slower real-site annotation
as well. The experiment remains opt-in and its binaries/timings are retained.

Native site batches reuse the score of identical full `(ASO, target interval
sequence)` pairs. The lookup borrows the current batch's sequences and is released
at the end of that duplex stage; it never grows across the report. Full string
equality resolves hash collisions. Each original site receives its own annotation,
including its original coordinates and supplied DDG. Different ASOs and different
interval lengths remain separate keys. The work profile records `native_pairs`
and `native_unique_pairs` across intended and off-target scoring stages so reuse
can be measured rather than inferred from record or ASO counts.

Intended-target energies are calculated when an ASO first appears in a site batch
and retained once per query. All query sequences and supplied target lengths are
still validated during input loading, including queries with no sites.

The report reader deserializes only site type, query/record IDs and interval bounds
on the common path. Other site content stays in its original JSON string. The
writer appends the typed energy fields to that validated object, reusing a small
serialization buffer. Existing root DDG or energy annotations take the complete
JSON object path, preserving supplied-DDG handling and rejection of already scored
reports. Unusual rows that do not fit the minimal envelope also use the full parser.
No site metadata, unknown nested fields, coordinates or discovery summaries are
removed. Tests cover escaped strings/braces, nested data, whitespace and null DDG.


Native batches use pulp runtime dispatch: x86 V4 (16 lanes), V3 (8 lanes),
AArch64 NEON (4 lanes), or scalar. All vector widths instantiate one generic
integer recurrence in `portable.rs`. Parameter lookups use per-lane reads because
pulp has no portable gather API. The earlier `energy-neon` experiment above is
a separate scalar-kernel loop reduction, not this default across-pair backend.
Site annotation runs bounded waves across the requested workers, overlapping
input and private-spool output with scoring. Each worker opens an independent
reference file handle and shares the immutable offset index. Intended energies
are cached across workers with locks acquired in query-ID order. Output remains
in original order and is only emitted after the complete report passes validation.
Shared-prefix and min-plus kernels remain benchmark experiments; current default
batching uses neither because their combined SIMD versions regressed on unique
real-site pairs. Their exact algorithms and timings are in `docs/DDG.md`.
