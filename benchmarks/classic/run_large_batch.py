#!/usr/bin/env python3
"""Verify the shared 100000-query inputs, then run a bounded extension series."""
import argparse
import hashlib
import json
from pathlib import Path
from scale import benchmark

parser = argparse.ArgumentParser()
parser.add_argument('--tool', required=True, choices=['ooff', 'bwa', 'blast', 'minimap2'])
parser.add_argument('--run-timeout', type=float, default=600)
parser.add_argument('--repetitions', type=int, default=3)
args = parser.parse_args()
if args.run_timeout <= 0 or args.repetitions < 1: parser.error('timeout and repetitions must be positive')
root = Path('data/classic-v1')
manifest = json.loads((root/'large-batch-manifest.json').read_text())
assert manifest['queries'] == manifest['unique_sequences'] == 100000
for name, expected in manifest['files'].items():
    assert hashlib.sha256((root/name).read_bytes()).hexdigest() == expected, name
queries = (root/'queries-100000.fa').read_text().splitlines()
targets = (root/'targets-100000.fa').read_text().splitlines()
assert len(queries) == len(targets) == 200000
assert len(set(queries[1::2])) == 100000
for i, (aso, target) in enumerate(zip(queries[1::2], targets[1::2])):
    assert len(aso) == 20 and set(aso) <= set('ACGT')
    assert queries[2*i] == f'>q{i}|ENSG00000136531' and targets[2*i] == f'>q{i}'
    assert target == aso.translate(str.maketrans('ACGT', 'TGCA'))[::-1]
assert queries[:53286] == (root/'queries-26643.fa').read_text().splitlines()
attempts = []
for repetition in range(1, args.repetitions+1):
    row = benchmark(args.tool, 100000, repetition, args.run_timeout, 'measurement', run_timeout=args.run_timeout)
    attempts.append(row)
    if not row['complete']:
        assert any(json.loads(Path(stage).read_text())['timeout'] for stage in row['stages']), row
        break
Path(f'results/{args.tool}-100000-series.json').write_text(json.dumps(dict(complete=True, attempts=attempts, input_manifest=manifest), indent=2)+'\n')
