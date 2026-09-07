#!/usr/bin/env python3
"""Wait for the current gate, then baseline scaling and the isolated ablation."""
import json
import argparse
import runpy
import time
from pathlib import Path
from scale import benchmark

parser = argparse.ArgumentParser()
parser.add_argument('--skip-experiment', action='store_true')
args = parser.parse_args()

deadline = time.monotonic() + 2400
while not Path('results/parallel-human-gate.success').exists():
    assert time.monotonic() < deadline, 'Native gate did not finish within 40 minutes'
    time.sleep(5)
gate = json.loads(Path('results/parallel-human-gate.success').read_text())
assert gate['complete']
runpy.run_path('benchmarks/classic/add_small_batches.py', run_name='__main__')
pilot = benchmark('ooff', 100, 1, 600, 'pilot')
assert pilot['complete'] and pilot['recovered'] == 100, pilot
for n in [10, 30, 100, 1000, 5000, 10000, 26643]:
    for rep in range(1, 4):
        result = benchmark('ooff', n, rep, 600, 'measurement')
        assert result['complete'], result
Path('results/native-classic-measurement.success').write_text('complete\n')
if not args.skip_experiment:
    from word_cache_experiment import main
    main()
