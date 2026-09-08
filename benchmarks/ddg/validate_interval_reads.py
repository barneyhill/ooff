#!/usr/bin/env python3
import argparse,datetime,gzip,json,subprocess,time
from pathlib import Path
from full_walk import sha
p=argparse.ArgumentParser();p.add_argument('--chunk',type=Path,required=True);p.add_argument('--output',type=Path,required=True);a=p.parse_args()
a.output.mkdir(parents=True,exist_ok=False)
prior=json.load((a.chunk/'result.json').open());report=a.output/'report.jsonl'
with gzip.open(a.chunk/'report.jsonl.gz','rb') as src,report.open('wb') as out:
 import shutil;shutil.copyfileobj(src,out)
cmd=json.load((a.chunk/'commands.json').open())['ddg']
cmd[cmd.index('--sites')+1]=str(report.resolve());cmd[cmd.index('--work-dir')+1]=str((a.output/'work').resolve());cmd[cmd.index('--timeout')+1]='600'
record=dict(label=a.output.name,threads=1,platform='EC2 c7i.2xlarge',complete=False,all_fields_match=False,sites=prior['sites'],workload='Exact full-output regression check for direct FASTA interval reads; isolated validation, not matched speedup',attempts=[],command=cmd)
try:
 start=time.perf_counter()
 with open('/dev/null','wb') as out,(a.output/'stderr').open('wb') as err:r=subprocess.run(cmd,stdout=out,stderr=err,timeout=610)
 record['attempts'].append(dict(engine='ooff',repetition=0,seconds=time.perf_counter()-start,returncode=r.returncode,measurement_kind='interval_read_validation'))
 assert r.returncode==0
 expected=next(r['uncompressed_sha256'] for r in prior['archives'] if 'annotated' in r['path'])
 actual=sha(a.output/'work/annotated.jsonl');assert actual==expected,(actual,expected)
 record.update(complete=True,all_fields_match=True,output_sha256=actual,binary_sha256=sha(cmd[0]),manifest=json.load((a.output/'work/manifest.json').open()))
 print('PASS',prior['sites'],'sites, byte-identical output;',record['attempts'][0]['seconds'],'seconds',flush=True)
finally:(a.output/'result.json').write_text(json.dumps(record,indent=2)+'\n')
