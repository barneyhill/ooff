#!/usr/bin/env python3
"""Bind a copied index to a byte-identical FASTA at its new location."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import tempfile

p = argparse.ArgumentParser(description=__doc__)
p.add_argument('--index', type=Path, required=True)
p.add_argument('--reference', type=Path, required=True)
a = p.parse_args()
info_path = a.index / 'reference-info.json'
info = json.loads(info_path.read_text())
reference = a.reference.resolve(strict=True)
before = reference.stat()
assert before.st_size == info['file_bytes'], 'Reference size differs'
with reference.open('rb') as f:
    digest = hashlib.sha256()
    for block in iter(lambda: f.read(1024 * 1024), b''):
        digest.update(block)
assert digest.hexdigest() == info['sha256'], 'Reference SHA-256 differs'
after = reference.stat()
assert (before.st_size, before.st_mtime_ns, before.st_ino) == (after.st_size, after.st_mtime_ns, after.st_ino), 'Reference changed during verification'
info.update(path=str(reference), modified_nanos=after.st_mtime_ns)
with tempfile.NamedTemporaryFile(mode='w', dir=a.index, delete=False) as f:
    temporary = Path(f.name)
    json.dump(info, f, separators=(',', ':'))
try:
    os.replace(temporary, info_path)
finally:
    temporary.unlink(missing_ok=True)
print(f'Verified {info["sha256"]}; bound index to {reference}')
print('Regenerate the annotation cache on this host before searching.')
