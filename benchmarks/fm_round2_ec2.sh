#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
export PATH="/home/ubuntu/.cargo/bin:$PATH"
export OMP_NUM_THREADS=8
cargo build --release --locked --bin ooff-index --bin ooff-bench
bash benchmarks/build_comparator_ec2.sh
python3 benchmarks/make_query_sets.py
python3 benchmarks/run_iteration.py --label fm-mmap-full-screen --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-full-v1 --mmap --mode screen --n 1000 -k 3 --repetitions 5 --timeout 600
for dataset in scn2a-10000 mixed-genes-1000 random-1000 low-complexity-1000; do
    python3 benchmarks/run_iteration.py --label "fm-mmap-$dataset-screen" --reference data/reference-v1/reference.fa --queries "data/query-sets-v1/$dataset.fa" --native target/release/ooff-index --index data/fm-full-v1 --mmap --mode screen --n 10000 -k 3 --timeout 600
done
python3 benchmarks/run_iteration.py --label fm-mmap-bounded-sites --reference data/reference-v1/bounded-10mb.fa --native target/release/ooff-index --index data/fm-bounded-v1 --mmap --mode sites --n 32 -k 3 --timeout 300
python3 benchmarks/run_iteration.py --label fm-mmap-full-sites32 --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-full-v1 --mmap --mode sites --n 32 -k 3 --repetitions 1 --timeout 600
