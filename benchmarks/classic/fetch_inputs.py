#!/usr/bin/env python3
"""Download only the benchmark's published inputs and verify every SHA-256."""
import argparse,hashlib,json,time,urllib.request
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--source',required=True);p.add_argument('--tool',required=True);a=p.parse_args()
root=Path('data/classic-v1');root.mkdir(parents=True,exist_ok=True)
def get(name):
    with urllib.request.urlopen(a.source.rstrip('/')+'/'+name,timeout=60) as response:return response.read()
manifest=json.loads(get('transfer-manifest.json'))
for name,expected in manifest['files'].items():
    path=root/name
    if not path.exists():
        with urllib.request.urlopen(a.source.rstrip('/')+'/'+name,timeout=120) as response, path.open('xb') as out:
            while block:=response.read(4*1024*1024):out.write(block)
    with path.open('rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==expected,name
if a.tool=='bwa':
    for attempt in range(360):
        try:bwa=json.loads(get('bwa-ready.json'));break
        except urllib.error.HTTPError as e:
            if e.code!=404:raise
            time.sleep(10)
    else:raise RuntimeError('BWA index publication timed out')
    for name,expected in bwa['files'].items():
        path=root/name
        if not path.exists():
            with urllib.request.urlopen(a.source.rstrip('/')+'/'+name,timeout=120) as response, path.open('xb') as out:
                while block:=response.read(4*1024*1024):out.write(block)
        with path.open('rb') as f:assert hashlib.file_digest(f,'sha256').hexdigest()==expected,name
    (root/'bwa-build-provenance.json').write_text(json.dumps(bwa,indent=2)+'\n')
(root/f'inputs-ready-{a.tool}.json').write_text(json.dumps(dict(complete=True,source=a.source,manifest=manifest),indent=2)+'\n')
print('Input SHA-256 checks passed for',a.tool)
