#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
predecessor="${1:?Pass the final-validation process ID}"
while kill -0 "$predecessor" 2>/dev/null; do sleep 15; done
python3 benchmarks/inventory_artifacts.py --output results/artifact-inventory.json
python3 benchmarks/refresh_ledger.py
# The controller inspects the final evidence and stops the worker separately.
