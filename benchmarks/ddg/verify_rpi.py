#!/usr/bin/env python3
"""Compare all output fields against OligoAI's actual TypeScript implementation."""
import argparse,datetime,hashlib,json,platform,random,subprocess,time
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--binary',default='target/release/oofft-ddg');p.add_argument('--label',default='fixtures');p.add_argument('--queries');p.add_argument('--off-target');p.add_argument('--repetitions',type=int,default=1);p.add_argument('--threads',type=int,default=4);p.add_argument('--length',type=int,default=20);a=p.parse_args()
stamp=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%f')
root=Path('benchmarks/ddg/iterations')/(stamp+'-'+a.label);root.mkdir(parents=True)
source=Path('/home/barneyh/oligoai-v2/src/specificity.ts')
if a.queries:
 queries=Path(a.queries);off=Path(a.off_target)
else:
 rng=random.Random(20260908);seqs=[''.join(rng.choice('ACGT') for _ in range(a.length)) for _ in range(24)]
 text='NNN'.join(s[:9]+rng.choice('ACGT')+s[10:] for s in seqs[:16])+seqs[16]+seqs[16]+'NNN'+seqs[17][:8]+seqs[17][9:]+'NNN'+seqs[18][:8]+'A'+seqs[18][8:]+'ACGT'*15
 rows=[dict(id=f'q{i}',target=s,aso=s.translate(str.maketrans('ACGT','TGCA'))[::-1]) for i,s in enumerate(seqs)]
 rows += [dict(rows[0],id='duplicate'),dict(rows[16],id='exact_duplicate')]
 rows += [dict(id='rna_lowercase',aso=rows[1]['aso'].lower().replace('t','u'),target=rows[1]['target'].lower().replace('t','u'))]
 queries=root/'queries.jsonl';queries.write_text(''.join(json.dumps(r)+'\n' for r in rows));off=root/'off.fa';off.write_text('>off\n'+text+'\n')
commands={
 'oligoai':['/home/barneyh/.bun/bin/bun','benchmarks/ddg/oligoai_oracle.ts',str(source),str(queries.resolve()),str(off.resolve()),str(a.length)],
 'ooff':[a.binary,'--queries',str(queries),'--off-target',str(off),'--rnaplex','/home/barneyh/.local/bin/RNAplex','--rnaduplex','/home/barneyh/.local/bin/RNAduplex','--threads',str(a.threads),'--timeout','600']}
record=dict(label=a.label,host=platform.uname()._asdict(),threads=a.threads,queries=str(queries),off_target=str(off),sha256={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [source,Path(a.binary),queries,off,Path('/home/barneyh/.local/bin/RNAplex'),Path('/home/barneyh/.local/bin/RNAduplex')]},attempts=[])
(root/'started.json').write_text(json.dumps(record,indent=2)+'\n')
try:
 for rep in range(a.repetitions):
  outputs={}
  for name in (['oligoai','ooff'] if rep%2==0 else ['ooff','oligoai']):
   command=commands[name];t=time.perf_counter()
   result=subprocess.run(command,capture_output=True,text=True,timeout=660)
   elapsed=time.perf_counter()-t
   (root/f'{name}-{rep}.stdout').write_text(result.stdout);(root/f'{name}-{rep}.stderr').write_text(result.stderr)
   record['attempts'].append(dict(engine=name,repetition=rep,command=command,seconds=elapsed,returncode=result.returncode))
   assert result.returncode==0,(name,result.stderr[-1500:])
   outputs[name]=[json.loads(l) for l in result.stdout.splitlines()]
  assert len(outputs['oligoai'])==len(outputs['ooff'])
  for expected,actual in zip(outputs['oligoai'],outputs['ooff']):
   if expected['ddg'] is not None: expected['ddg'] = -expected['ddg']  # Watt sign; OligoAI v2 is reversed.
   assert expected.keys()==actual.keys(),(expected,actual)
   for key in expected:
    if key in ['ddg','dg_target','dg_other'] and expected[key] is not None:
     assert actual[key] is not None and abs(expected[key]-actual[key])<=1e-12,(key,expected,actual)
    else:assert expected[key]==actual[key],(key,expected,actual)
  print(f'PASS repetition {rep}: all {len(outputs["ooff"])} rows and all fields match',flush=True)
 record['complete']=True;record['all_fields_match']=True
except Exception as error:
 record['complete']=False;record['error']=str(error)
 raise
finally:
 (root/'result.json').write_text(json.dumps(record,indent=2)+'\n')
print(root)
