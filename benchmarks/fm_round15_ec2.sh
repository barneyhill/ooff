#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
predecessor="${1:?Pass the range-validation process ID}"
while kill -0 "$predecessor" 2>/dev/null; do sleep 15; done
sha256sum -c results/fm-round14.success
comparator=/home/ubuntu/comparators/sassy-0.2.6-fast-interval/target/release/sassy-comparator
python3 benchmarks/run_iteration.py --label fm-word-union-full-tuples1000 --reference data/reference-v1/reference.fa --native target/release/ooff-index --sassy "$comparator" --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 1000 -k 3 --repetitions 3 --site-tuples --timeout 1200
for name in scn2a-10000 mixed-genes-1000 random-1000 low-complexity-1000; do
    python3 benchmarks/run_iteration.py --label "fm-word-final-$name-screen" --reference data/reference-v1/reference.fa --queries "data/query-sets-v1/$name.fa" --native target/release/ooff-index --sassy "$comparator" --index data/fm-production-v1 --mmap --mode screen --n 10000 -k 3 --repetitions 3 --timeout 300
done
python3 benchmarks/run_iteration.py --label fm-word-final-alleles-all-screen --reference data/reference-v1/reference.fa --queries data/cached-allele-queries-v1/scn2a-cached-alleles-all.fa --native target/release/ooff-index --sassy "$comparator" --index data/fm-production-v1 --mmap --mode screen --n 30000 -k 3 --repetitions 3 --timeout 300
