#!/usr/bin/env python3
"""Produce exact pinned ViennaRNA energies for a supplied sequence-pair corpus."""
import argparse, concurrent.futures, hashlib, json, subprocess, time
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--input',type=Path,required=True);p.add_argument('--output',type=Path,required=True)
p.add_argument('--rnaduplex',type=Path,required=True);p.add_argument('--threads',type=int,default=8)
a=p.parse_args();assert a.threads>0
version=subprocess.check_output([str(a.rnaduplex),'--version'],text=True).strip();assert '2.7.0' in version
rows=[json.loads(line) for line in a.input.read_text().splitlines()]
a.output.mkdir(parents=True,exist_ok=False)
size=(len(rows)+a.threads-1)//a.threads
started=time.perf_counter()
def work(item):
    i,chunk=item
    source=a.output/f'input-{i}.txt';dest=a.output/f'vienna-{i}.stdout';err=a.output/f'vienna-{i}.stderr'
    source.write_text(''.join(r['aso']+'\n'+r['target']+'\n' for r in chunk))
    with source.open('rb') as stdin,dest.open('wb') as stdout,err.open('wb') as stderr:
        result=subprocess.run([str(a.rnaduplex),'--noconv'],stdin=stdin,stdout=stdout,stderr=stderr,timeout=600)
    assert result.returncode==0,err.read_text()[-1000:]
    from decimal import Decimal
    energies=[int(Decimal(line.rsplit('(',1)[1].rstrip(')\n '))*100) for line in dest.read_text().splitlines() if '&' in line]
    assert len(energies)==len(chunk)
    return [dict(aso=r['aso'],target=r['target'],cents=e) for r,e in zip(chunk,energies)]
with concurrent.futures.ThreadPoolExecutor(max_workers=a.threads) as pool:
    parts=list(pool.map(work,enumerate([rows[i:i+size] for i in range(0,len(rows),size)])))
cases=[r for part in parts for r in part]
(a.output/'cases.json').write_text(json.dumps(cases)+'\n')
(a.output/'cases.tsv').write_text(''.join(f"{r['aso']}\t{r['target']}\t{r['cents']}\n" for r in cases))
record=dict(complete=True,cases=len(cases),threads=a.threads,vienna_version=version,seconds=time.perf_counter()-started,input=str(a.input),input_sha256=hashlib.sha256(a.input.read_bytes()).hexdigest(),rnaduplex_sha256=hashlib.sha256(a.rnaduplex.read_bytes()).hexdigest(),output_sha256=hashlib.sha256((a.output/'cases.json').read_bytes()).hexdigest(),scope='Independent actual ViennaRNA oracle generation; not a native speed measurement')
(a.output/'provenance.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record))
