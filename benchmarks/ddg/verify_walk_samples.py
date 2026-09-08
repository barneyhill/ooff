#!/usr/bin/env python3
"""Independently check deterministic samples from every completed full-walk chunk.
This is sample verification, not a claim of a Vienna comparison for every hit.
"""
import argparse,gzip,json,re,subprocess,time
from decimal import Decimal
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--run',type=Path,required=True);p.add_argument('--index',type=Path,required=True);p.add_argument('--reference',type=Path,required=True);p.add_argument('--rnaduplex',type=Path,required=True);a=p.parse_args()
root=a.run/'vienna-sample-verification';root.mkdir(exist_ok=False)
run=json.load((a.run/'result.json').open());assert run['complete'],'Full walk must complete first'
version=subprocess.check_output([str(a.rnaduplex),'--version'],text=True).strip();assert '2.7.0' in version
info=json.load((a.index/'reference-info.json').open());records={r['header'].split('|')[0]:r for r in info['records']}
pairs={};samples=[]
def pair_index(aso,target):
 key=(aso.replace('T','U'),target.replace('T','U'))
 if key not in pairs:pairs[key]=len(pairs)
 return pairs[key]
with a.reference.open('rb') as reference:
 for chunk in sorted(run['chunks'],key=lambda c:c['chunk']):
  folder=a.run/f"chunk-{chunk['chunk']:04d}"
  queries={q['id']:q['sequence'] for q in map(json.loads,(folder/'queries.jsonl').open())}
  chosen=set()
  with gzip.open(folder/'ddg-work/annotated.jsonl.gz','rt') as stream:
   for line_number,line in enumerate(stream):
    if line_number>4096 or len(chosen)==16:break
    row=json.loads(line)
    if row['type']!='site':continue
    r=records[row['record_id']];start=row['alignment']['start'];end=row['alignment']['end']
    assert 0<=start<end<=r['length']
    target=[]
    for i in range(start,end):
     reference.seek(r['offset']+(i//r['line_bases'])*r['line_bytes']+i%r['line_bases'])
     target.append(reference.read(1).decode().upper())
    target=''.join(target);aso=queries[row['query_id']]
    if (aso,target) in chosen:continue
    chosen.add((aso,target))
    intended=aso.translate(str.maketrans('ACGT','TGCA'))[::-1]
    samples.append(dict(chunk=chunk['chunk'],site_id=row['site_id'],on_pair=pair_index(aso,intended),off_pair=pair_index(aso,target),ddg=row['ddg'],annotation=row['energy_annotation']))
  assert chosen or chunk['sites']==0, f"Chunk {chunk['chunk']} contains no sampled sites"
(root/'samples.json').write_text(json.dumps(samples,indent=2)+'\n')
(root/'RNAduplex.input').write_text(''.join(a+'\n'+t+'\n' for a,t in pairs))
t=time.perf_counter()
with (root/'RNAduplex.input').open('rb') as stdin,(root/'RNAduplex.stdout').open('wb') as out,(root/'RNAduplex.stderr').open('wb') as err:
 subprocess.run([str(a.rnaduplex),'--noconv'],stdin=stdin,stdout=out,stderr=err,check=True,timeout=300)
energies=[int(Decimal(re.search(r'\(\s*(-?\d+\.\d+)\s*\)\s*$',line)[1])*100) for line in (root/'RNAduplex.stdout').read_text().splitlines() if '&' in line]
assert len(energies)==len(pairs)
for s in samples:
 on,off=energies[s['on_pair']],energies[s['off_pair']]
 assert Decimal(str(s['annotation']['dg_target']))*100==on,s
 assert Decimal(str(s['annotation']['dg_other']))*100==off,s
 assert Decimal(str(s['ddg']))*100==off-on,s
 assert s['annotation']['ddg_sign']=='dg_other - dg_target'
record=dict(complete=True,all_sample_energies_match=True,all_hit_energies_compared=False,sampled_sites=len(samples),distinct_pairs=len(pairs),chunks=len(run['chunks']),vienna_version=version,seconds=time.perf_counter()-t,
            selection='First16 distinct ASO/target pairs among first4096 lines of every chunk; deterministic coverage across full gene walk, not random/unbiased sampling')
(root/'result.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record),flush=True)
