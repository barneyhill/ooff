#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
mkdir -p data/raw results /home/ubuntu/comparators
curl -fsSL https://sh.rustup.rs -o /home/ubuntu/comparators/rustup.sh
sh /home/ubuntu/comparators/rustup.sh -y --no-modify-path --profile minimal --default-toolchain 1.94.0
export PATH="/home/ubuntu/.cargo/bin:$PATH"
rustup component add rustfmt clippy
cargo test --locked
cargo build --release --locked
base=https://ftp.ensembl.org/pub/release-110
curl -fL --retry 3 "$base/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna_sm.primary_assembly.fa.gz" -o data/raw/Homo_sapiens.GRCh38.dna_sm.primary_assembly.fa.gz
curl -fL --retry 3 "$base/gtf/homo_sapiens/Homo_sapiens.GRCh38.110.gtf.gz" -o data/raw/Homo_sapiens.GRCh38.110.gtf.gz
curl -fL --retry 3 "$base/fasta/homo_sapiens/dna/CHECKSUMS" -o data/raw/dna.CHECKSUMS
curl -fL --retry 3 "$base/gtf/homo_sapiens/CHECKSUMS" -o data/raw/gtf.CHECKSUMS
sha256sum data/raw/*.gz > results/reference-downloads.sha256
lscpu > results/cpu.txt
rustc -Vv > results/rustc.txt
printf 'bootstrap complete\n'
