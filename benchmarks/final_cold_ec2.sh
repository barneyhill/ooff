#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
predecessor="${1:?Pass the final-validation process ID}"
while kill -0 "$predecessor" 2>/dev/null; do sleep 15; done
python3 - <<'PY'
from pathlib import Path
import json
labels=['fm-word-union-full-tuples1000','fm-word-final-scn2a-10000-screen',
        'fm-word-final-mixed-genes-1000-screen','fm-word-final-random-1000-screen',
        'fm-word-final-low-complexity-1000-screen','fm-word-final-alleles-all-screen']
for label in labels:
    path=sorted(Path('benchmarks/iterations').glob('*-'+label+'/result.json'))[-1]
    result=json.loads(path.read_text())
    assert result['complete'] and result['matching_counts_and_signatures'], path
    if label=='fm-word-union-full-tuples1000':
        assert result['exact_site_tuples_equal_and_unique'], path
PY
python3 benchmarks/run_iteration.py --label fm-word-final-cold-original1000-screen --reference data/reference-v1/reference.fa --native target/release/ooff-index --sassy /home/ubuntu/comparators/sassy-0.2.6-fast-interval/target/release/sassy-comparator --index data/fm-production-v1 --mmap --mode screen --n 1000 -k 3 --repetitions 1 --cold --timeout 300
