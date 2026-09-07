#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
while kill -0 28460 2>/dev/null; do sleep 15; done
# Optional upstream AVX-512 feature failed to compile. Keep the supported build.
python3 benchmarks/run_iteration.py --label fm-final-exact-tuples-1000-repeated --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 1000 -k 3 --repetitions 3 --site-tuples --timeout 1650
