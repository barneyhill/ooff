#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
while kill -0 30047 2>/dev/null; do sleep 15; done
cp target/release/ooff-index data/round13-stage/ooff-index-before-packed
cp -a data/round13-stage/src/. src/
cp -a data/round13-stage/tests/. tests/
cp data/round13-stage/engine_driver.rs benchmarks/engine_driver.rs
export PATH="/home/ubuntu/.cargo/bin:$PATH"
OMP_NUM_THREADS=2 cargo test --locked --all-targets
cargo build --release --locked --bin ooff-index --bin ooff
bash benchmarks/build_comparator_ec2.sh /home/ubuntu/comparators/sassy-0.2.6-fast-interval
comparator=/home/ubuntu/comparators/sassy-0.2.6-fast-interval/target/release/sassy-comparator
python3 benchmarks/run_iteration.py --label fm-packed-fast-interval-sites32 --reference data/reference-v1/reference.fa --native target/release/ooff-index --sassy "$comparator" --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 32 -k 3 --repetitions 3 --site-tuples --timeout 300
python3 benchmarks/run_iteration.py --label fm-packed-fast-interval-sites1000 --reference data/reference-v1/reference.fa --native target/release/ooff-index --sassy "$comparator" --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 1000 -k 3 --repetitions 3 --timeout 1200
