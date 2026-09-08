#!/usr/bin/env python3
"""Retain repeated compute-only measurements and optional matched native baseline."""
import argparse, datetime, hashlib, json, statistics, subprocess, platform
from pathlib import Path
p=argparse.ArgumentParser()
p.add_argument('--label',required=True)
p.add_argument('--binary',default='target/release/examples/energy-bench')
p.add_argument('--baseline')
p.add_argument('--baseline-cases')
p.add_argument('--baseline-name',default='native-baseline')
p.add_argument('--cases',default='fixtures/energy_golden.json')
p.add_argument('--repetitions',type=int,default=10)
p.add_argument('--runs',type=int,default=3)
p.add_argument('--threads',type=int,default=1)
p.add_argument('--batch',action='store_true')
p.add_argument('--prefix',action='store_true')
p.add_argument('--shared',action='store_true')
p.add_argument('--paths',action='store_true')
p.add_argument('--minplus',action='store_true')
p.add_argument('--max-lanes',type=int,default=16)
p.add_argument('--baseline-batch',action='store_true')
p.add_argument('--baseline-shared',action='store_true')
p.add_argument('--baseline-max-lanes',type=int,default=8)
p.add_argument('--platform-label',default=platform.machine()+' '+platform.system())
a=p.parse_args()
root=Path('benchmarks/ddg/iterations')/(datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%f')+'-'+a.label)
root.mkdir(parents=True)
cases=json.loads(Path(a.cases).read_text())
checksum=sum(c['cents'] for c in cases)*a.repetitions
binaries={'rust-kernel':a.binary}
if a.baseline:binaries[a.baseline_name]=a.baseline
record=dict(label=a.label,platform=a.platform_label,cases=len(cases),batch_requested=a.batch,threads=a.threads,complete=False,workload='Compute-only pairs, includes pair encoding and DP, excludes IO/startup',
            attempts=[],sha256={str(f):hashlib.sha256(f.read_bytes()).hexdigest() for f in [Path(a.cases),*map(Path,binaries.values())]})
try:
    for rep in range(a.runs):
        names=list(binaries) if rep%2==0 else list(reversed(binaries))
        for name in names:
            command=[binaries[name],a.baseline_cases if name!='rust-kernel' and a.baseline_cases else a.cases,str(a.repetitions)]
            if a.threads != 1 or a.batch or a.baseline_batch or a.prefix or a.shared or a.minplus or a.paths or a.baseline_shared: command.append(str(a.threads))
            if a.paths and name=='rust-kernel': command.extend(['paths',str(a.max_lanes)])
            elif a.minplus and name=='rust-kernel': command.extend(['minplus',str(a.max_lanes)])
            elif a.shared and name=='rust-kernel': command.extend(['shared',str(a.max_lanes)])
            elif a.prefix and name=='rust-kernel': command.append('prefix')
            elif a.batch and name=='rust-kernel': command.extend(['batch',str(a.max_lanes)])
            elif a.baseline_shared and name!='rust-kernel': command.extend(['shared',str(a.baseline_max_lanes)])
            elif a.baseline_batch and name!='rust-kernel': command.extend(['batch',str(a.baseline_max_lanes)])
            result=subprocess.run(command,capture_output=True,text=True,timeout=600)
            (root/f'{name}-{rep}.stdout').write_text(result.stdout)
            (root/f'{name}-{rep}.stderr').write_text(result.stderr)
            row=json.loads(result.stdout) if result.returncode==0 else {}
            record['attempts'].append(dict(engine=name,repetition=rep,seconds=row.get('seconds',0),returncode=result.returncode,measurement_kind='kernel_compute',command=command))
            assert result.returncode==0 and row['checksum']==checksum and row['cases']==len(cases)
            print(name,rep,row['seconds'],flush=True)
    record.update(complete=True,all_fields_match=True)
    medians={name:statistics.median(r['seconds'] for r in record['attempts'] if r['engine']==name) for name in binaries}
    record['medians']=medians
    if a.baseline:record['speedup_vs_native_baseline']=medians[a.baseline_name]/medians['rust-kernel']
except Exception as error:
    record['error']=str(error)
    raise
finally:
    (root/'result.json').write_text(json.dumps(record,indent=2)+'\n')
print(root)
