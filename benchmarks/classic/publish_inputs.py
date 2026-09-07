#!/usr/bin/env python3
"""Publish an explicit, credential-free input set to the private worker network."""
import hashlib,json,os,sys,time
from pathlib import Path
source=Path('data/classic-v1').resolve();dest=Path('data/classic-transfer');dest.mkdir(exist_ok=True)
def sha(path):
    with path.open('rb') as f:return hashlib.file_digest(f,'sha256').hexdigest()
files=['eligible.fa','records.json','manifest.json']+[p.name for p in sorted(source.glob('queries-*.fa'))]+[p.name for p in sorted(source.glob('targets-*.fa'))]
files += [name for name in ['large-batch-manifest.json','queries-100000-origins.csv'] if (source/name).is_file()]
manifest={}
for name in files:
    target=dest/name
    if not target.is_symlink():target.symlink_to(source/name)
    manifest[name]=sha(source/name)
(dest/'transfer-manifest.json').write_text(json.dumps(dict(files=manifest),indent=2)+'\n')
print('Common inputs ready',flush=True)
while True:
    result=Path('benchmarks/classic/iterations/20260907T203303.291193-build-bwa-index/result.json')
    if result.exists():break
    time.sleep(10)
build=json.loads(result.read_text());assert build['returncode']==0,build
files={}
for suffix in ['amb','ann','bwt','pac','sa']:
    name='bwa.'+suffix;path=source/name;assert path.stat().st_size>0
    if not (dest/name).is_symlink():(dest/name).symlink_to(path)
    files[name]=sha(path)
(dest/'bwa-ready.json').write_text(json.dumps(dict(files=files,build=build),indent=2)+'\n')
print('Verified BWA index ready',flush=True)
