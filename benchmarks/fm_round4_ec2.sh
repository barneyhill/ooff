#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
export PATH="/home/ubuntu/.cargo/bin:$PATH"
export OMP_NUM_THREADS=8
cargo build --release --locked --bin ooff-index --bin ooff
for dataset in scn2a-10000 random-1000 low-complexity-1000; do
    python3 benchmarks/run_iteration.py --label "fm-fastmemo-$dataset-screen" --reference data/reference-v1/reference.fa --queries "data/query-sets-v1/$dataset.fa" --native target/release/ooff-index --index data/fm-full-1300m --mmap --mode screen --n 10000 -k 3 --timeout 600
done
python3 benchmarks/run_iteration.py --label fm-fastmemo-full-sites32 --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-full-1300m --mmap --mode sites --n 32 -k 3 --repetitions 3 --timeout 600
python3 benchmarks/run_index_build.py --reference data/reference-v1/reference.fa --output data/fm-production-v1 --shard-bases 1300000000 --label fm-production-provenance-build
