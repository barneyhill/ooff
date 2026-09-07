#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
export PATH="/home/ubuntu/.cargo/bin:$PATH"
export OMP_NUM_THREADS=8
cargo build --release --locked --bin ooff-index
python3 benchmarks/run_index_build.py --reference data/reference-v1/bounded-10mb.fa --output data/fm-bounded-v1 --label fm-v1-bounded-build
python3 benchmarks/run_iteration.py --label fm-v1-bounded-endpoints --reference data/reference-v1/bounded-10mb.fa --native target/release/ooff-index --index data/fm-bounded-v1 --mode endpoints --n 32 -k 3 --timeout 300
python3 benchmarks/run_iteration.py --label fm-v1-bounded-screen --reference data/reference-v1/bounded-10mb.fa --native target/release/ooff-index --index data/fm-bounded-v1 --mode screen --n 1000 -k 3 --timeout 300
python3 benchmarks/run_index_build.py --reference data/reference-v1/reference.fa --output data/fm-full-v1 --label fm-v1-full-build
python3 benchmarks/run_iteration.py --label fm-v1-full-screen --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-full-v1 --mode screen --n 1000 -k 3 --timeout 600
