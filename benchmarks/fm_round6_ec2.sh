#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
export PATH="/home/ubuntu/.cargo/bin:$PATH"
cargo build --release --locked --bin ooff-index --bin ooff
bash benchmarks/build_comparator_ec2.sh
bash benchmarks/build_comparator_ec2.sh /home/ubuntu/comparators/sassy-0.2.6-v1 /home/ubuntu/ooff/benchmarks/sassy_v1_engine.rs
python3 benchmarks/run_iteration.py --label sassy-v1-notrace-bounded-sites32 --reference data/reference-v1/bounded-10mb.fa --native target/release/ooff-index --index data/fm-bounded-v1 --mmap --mode sites --n 32 -k 3 --repetitions 1 --site-tuples --timeout 120 --sassy /home/ubuntu/comparators/sassy-0.2.6-v1/target/release/sassy-comparator
python3 benchmarks/run_iteration.py --label sassy-v1-notrace-full-sites32 --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 32 -k 3 --repetitions 1 --site-tuples --timeout 180 --sassy /home/ubuntu/comparators/sassy-0.2.6-v1/target/release/sassy-comparator
python3 benchmarks/run_iteration.py --label fm-range-dedup-sites1000 --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 1000 -k 3 --repetitions 3 --timeout 600 --reuse-sassy benchmarks/iterations/20260907T170424-fm-two-directions-sites1000/result.json
