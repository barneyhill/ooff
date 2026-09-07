#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
while kill -0 25984 2>/dev/null; do sleep 15; done
bash benchmarks/build_comparator_ec2.sh /home/ubuntu/comparators/sassy-0.2.6-avx512 /home/ubuntu/ooff/benchmarks/sassy_engine.rs avx512
comparator=/home/ubuntu/comparators/sassy-0.2.6-avx512/target/release/sassy-comparator
python3 benchmarks/run_iteration.py --label fm-avx512-sites32 --reference data/reference-v1/reference.fa --native target/release/ooff-index --sassy "$comparator" --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 32 -k 3 --repetitions 3 --site-tuples --timeout 300
python3 benchmarks/run_iteration.py --label fm-avx512-cached-alleles-all-screen --reference data/reference-v1/reference.fa --queries data/cached-allele-queries-v1/scn2a-cached-alleles-all.fa --native target/release/ooff-index --sassy "$comparator" --index data/fm-production-v1 --mmap --mode screen --n 30000 -k 3 --repetitions 3 --timeout 300
python3 benchmarks/run_iteration.py --label fm-avx512-exact-tuples-1000-repeated --reference data/reference-v1/reference.fa --native target/release/ooff-index --sassy "$comparator" --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 1000 -k 3 --repetitions 3 --site-tuples --timeout 1800
