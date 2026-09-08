#!/usr/bin/env python3
"""Read-only monitor: mirror timing records locally while the EC2 run proceeds."""
import json,subprocess,time,shlex
from pathlib import Path
remote='/mnt/oofft-walk/20260908-full-scn2a-paired-direct'
local=Path('benchmarks/ddg/iterations/20260908-full-scn2a-paired-direct');local.mkdir(parents=True,exist_ok=True)
script=f'''import json,pathlib
p=pathlib.Path({remote!r})
r=json.load((p/'result.json').open())
v=p/'vienna-sample-verification/result.json'
print(json.dumps(dict(run=r,markdown=(p/'iterations.md').read_text(),verification=json.load(v.open()) if v.exists() else None)))'''
cmd=['ssh','-i','data/ec2/controller.key','-o','UserKnownHostsFile=data/ec2/known_hosts','-o','ConnectTimeout=15','ubuntu@18.175.151.21','python3 -c '+shlex.quote(script)]
previous=-1
while True:
 try:
  result=subprocess.run(cmd,capture_output=True,text=True,timeout=30)
  result.check_returncode();payload=json.loads(result.stdout);r=payload['run']
  if payload['verification'] is not None:r['sample_verification']=payload['verification']
  (local/'result.json').write_text(json.dumps(r,indent=2)+'\n')
  (local/'iterations.md').write_text(payload['markdown'])
  subprocess.run(['python3','benchmarks/ddg/refresh.py'],check=True,stdout=subprocess.DEVNULL)
  count=len(r['chunks'])
  if count!=previous or r['complete']:
   print(json.dumps(dict(chunks=count,queries=sum(c['queries'] for c in r['chunks']),sites=sum(c['sites'] for c in r['chunks']),complete=r['complete'],seconds=r.get('total_wall_seconds'),verification=payload['verification'])),flush=True);previous=count
  if r['complete'] and payload['verification'] is not None:break
  if r.get('error'):print('BENCHMARK ERROR',r['error'],flush=True);break
 except (OSError,subprocess.SubprocessError,ValueError) as e:print('Monitor retry:',str(e)[:300],flush=True)
 time.sleep(40)
