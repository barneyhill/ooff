#!/usr/bin/env python3
"""Inventory retained benchmark artifacts without reading large file contents."""
import argparse
import datetime as dt
import json
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--root', type=Path, default=Path('.'))
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
root = args.root.resolve()
selected = [root / p for p in ('benchmarks/iterations', 'results', 'data/raw',
                              'data/reference-v1', 'data/query-sets-v1',
                              'data/cached-allele-queries-v1',
                              'benchmarks/classic/iterations', 'data/classic-v1',
                              'data/classic-stage', 'data/word-cache-stage',
                              'data/extension-stage', 'data/classic-extra')]
selected.extend(root.glob('data/fm-*'))
selected.extend(root.glob('data/round*-stage'))
selected.append(root.parent / 'comparators')
files = {}
for directory in selected:
    if not directory.exists():
        continue
    for path in directory.rglob('*'):
        if path.is_file() and path.resolve() != args.output.resolve():
            stat = path.stat()
            name = str(path.relative_to(root)) if path.is_relative_to(root) else str(path)
            files[name] = dict(bytes=stat.st_size, mtime_ns=stat.st_mtime_ns)
result = dict(utc=dt.datetime.now(dt.timezone.utc).isoformat(), root=str(root),
              note='Size/mtime inventory only; content hashes are recorded in individual run and reference manifests.',
              files=files, file_count=len(files), total_bytes=sum(x['bytes'] for x in files.values()))
args.output.write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({k: v for k, v in result.items() if k != 'files'}))
