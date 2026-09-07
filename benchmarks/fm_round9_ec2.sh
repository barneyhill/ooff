#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
while kill -0 25306 2>/dev/null; do sleep 15; done
# Use the exact already-tested binaries from round8; no concurrent compilation.
for size in 1000 10000 all; do
    python3 benchmarks/run_iteration.py --label "fm-cached-alleles-$size-screen" --reference data/reference-v1/reference.fa --queries "data/cached-allele-queries-v1/scn2a-cached-alleles-$size.fa" --native target/release/ooff-index --index data/fm-production-v1 --mmap --mode screen --n 30000 -k 3 --repetitions 3 --timeout 900
done
python3 benchmarks/run_iteration.py --label fm-exact-tuples-all-1000 --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 1000 -k 3 --repetitions 1 --site-tuples --timeout 1200
for k in 0 1 2; do
    python3 benchmarks/run_iteration.py --label "fm-random1000-screen-k$k" --reference data/reference-v1/reference.fa --queries data/query-sets-v1/random-1000.fa --native target/release/ooff-index --index data/fm-production-v1 --mmap --mode screen --n 1000 -k "$k" --repetitions 1 --timeout 900
done
