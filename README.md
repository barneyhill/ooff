# ooff

Build a fast, reproducible ASO off-target search tool in Rust, initially for
20mer gapmers with a default threshold of **three total edits** (substitutions
plus inserted/deleted bases; configurable). Use Sassy2 as the measured
baseline and a candidate search engine before developing a replacement.

Scientific basis: [Andersson et al. (2025), updated industry recommendations](https://doi.org/10.1089/nat.2024.0072). The handoff maps this framework to the computational scope and distinguishes project-specific choices.

**Start here: [HANDOFF.md](HANDOFF.md).** This repository currently contains the
implementation brief and benchmark evidence, not a finished search tool.

The next agent will run from a Raspberry Pi and use AWS EC2 for substantial
computation. AWS credentials will be available on the Pi. No credentials belong
in this repository.

Included fixtures are small sequence files and benchmark summaries. Large
reference genomes, indexes, raw alignment dumps and model credentials are not
included.
