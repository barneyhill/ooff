#!/usr/bin/env python3
"""Add nested 10/30-ASO points without changing any previously measured input."""
import hashlib,json
from pathlib import Path
root=Path('data/classic-v1');hashes={}
for kind in ['queries','targets']:
    original=root/f'{kind}-100.fa';lines=original.read_text().splitlines(keepends=True);assert len(lines)==200
    hashes[original.name]=hashlib.sha256(original.read_bytes()).hexdigest()
    for n in [10,30]:
        path=root/f'{kind}-{n}.fa';content=''.join(lines[:2*n])
        if path.exists():assert path.read_text()==content
        else:
            with path.open('x') as f:f.write(content)
        hashes[path.name]=hashlib.sha256(path.read_bytes()).hexdigest()
(root/'small-batches-manifest.json').write_text(json.dumps(hashes,indent=2)+'\n')
print(json.dumps(hashes))
