#!/usr/bin/env python3
"""Retain one complete count-only CLI iteration, including failures and timeout."""
import argparse,datetime,hashlib,json,os,platform,signal,subprocess,time,resource
from pathlib import Path
p=argparse.ArgumentParser()
p.add_argument('--label',required=True)
p.add_argument('--timeout',type=float,default=1800)
p.add_argument('--platform-label',default=platform.platform())
p.add_argument('command',nargs=argparse.REMAINDER)
a=p.parse_args();cmd=a.command
if cmd and cmd[0]=='--':cmd=cmd[1:]
assert cmd and a.timeout>0
root=Path('benchmarks/summary/iterations')/(datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S')+'-'+a.label)
root.mkdir(parents=True)
r=dict(command=cmd,platform=a.platform_label,timeout=a.timeout,complete=False,measurement='Whole CLI process: reference/index loading + search + summary output; index builds excluded',sha256={})
def sha(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for block in iter(lambda:f.read(1024*1024),b''):h.update(block)
 return h.hexdigest()
for path in [cmd[0],__file__]+[cmd[cmd.index(flag)+1] for flag in ['--queries','--annotation-cache'] if flag in cmd]:r['sha256'][path]=sha(path)
for flag in ['--index','--reverse-index']:
 if flag in cmd:
  path=str(Path(cmd[cmd.index(flag)+1])/'manifest.json');r['sha256'][path]=sha(path)
(root/'result.json').write_text(json.dumps(r,indent=2)+'\n')
started=time.perf_counter()
try:
 with (root/'stdout.jsonl').open('w') as out,(root/'stderr').open('w') as err:
  timed = ['/usr/bin/time','-f','{"max_rss_kib":%M,"user_seconds":%U,"system_seconds":%S}', '-o',str(root/'resources.json'),*cmd] if Path('/usr/bin/time').exists() else cmd
  job=subprocess.Popen(timed,stdout=out,stderr=err,start_new_session=True)
  try:r['returncode']=job.wait(timeout=a.timeout)
  except subprocess.TimeoutExpired:
   os.killpg(job.pid,signal.SIGTERM)
   try:job.wait(timeout=10)
   except subprocess.TimeoutExpired:os.killpg(job.pid,signal.SIGKILL);job.wait()
   r.update(returncode=124,error='timeout')
 r['seconds']=time.perf_counter()-started
 if not (root/'resources.json').exists():
  usage=resource.getrusage(resource.RUSAGE_CHILDREN);(root/'resources.json').write_text(json.dumps(dict(max_rss_kib=usage.ru_maxrss,user_seconds=usage.ru_utime,system_seconds=usage.ru_stime))+'\n')
 count=0;total=0;manifest=None;complete=None;ids=set()
 with (root/'stdout.jsonl').open() as f:
  for line in f:
   row=json.loads(line)
   if row['type']=='manifest':manifest=row
   elif row['type']=='query_summary':
    assert row['query_id'] not in ids;ids.add(row['query_id']);count+=1;total+=row['total_sites']
    assert sum(v for v in row['edit_distance_counts'].values() if v is not None)==row['total_sites']
   elif row['type']=='run_complete':complete=row
   else:raise ValueError('Unexpected non-summary output')
 r.update(queries=count,total_sites=total,manifest=manifest,run_complete=complete,output_bytes=(root/'stdout.jsonl').stat().st_size)
 assert r['returncode']==0 and complete and count==complete['queries'] and total==complete['total_sites']
 r['complete']=True
except Exception as e:r['error']=str(e)
finally:
 r.setdefault('seconds',time.perf_counter()-started)
 (root/'result.json').write_text(json.dumps(r,indent=2)+'\n')
 lines=['# Summary CLI iterations','','Each row is a retained whole-process attempt. Completion checks are structural; independent count equivalence is covered separately by tests.','','| Iteration | Complete | Seconds | Queries | Sites | Output bytes |','| --- | --- | ---: | ---: | ---: | ---: |']
 for f in sorted(Path('benchmarks/summary/iterations').glob('*/result.json')):
  x=json.loads(f.read_text());lines.append(f"| [{f.parent.name}](iterations/{f.parent.name}/result.json) | {x['complete']} | {x.get('seconds',0):.6f} | {x.get('queries','')} | {x.get('total_sites','')} | {x.get('output_bytes','')} |")
 Path('benchmarks/summary/ITERATIONS.md').write_text('\n'.join(lines)+'\n')
print(root,flush=True)
print(json.dumps(r),flush=True)
raise SystemExit(0 if r['complete'] else 1)
