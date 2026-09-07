#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
predecessor="${1:?Classic setup PID required}"
while kill -0 "$predecessor" 2>/dev/null; do sleep 15; done
sha256sum -c results/classic-setup.success
python3 benchmarks/classic/test_verify.py
# Retain the exact native and driver sources used for the scaling experiment.
tar -czf data/classic-v1/source-before-pilot.tar.gz Cargo.toml Cargo.lock src tests benchmarks/classic
python3 benchmarks/classic/parallel_gate.py
python3 benchmarks/classic/scale.py --phase pilot --counts 100 --repetitions 1 --timeout 600
