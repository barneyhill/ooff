#!/usr/bin/env python3
"""Compare complete real-site annotations against the retained Vienna baseline."""
import argparse, datetime, hashlib, json, statistics, subprocess, time, platform, os
from pathlib import Path
from itertools import zip_longest
p=argparse.ArgumentParser();p.add_argument('--input',type=Path,required=True)
p.add_argument('--baseline-engine',choices=['vienna','rust'],default='vienna')
p.add_argument('--baseline',default='target/release/oofft-ddg')
p.add_argument('--binary',default='target/release/oofft-ddg');p.add_argument('--runs',type=int,default=3)
p.add_argument('--threads',type=int,default=4)
p.add_argument('--rnaduplex',default='/home/barneyh/.local/bin/RNAduplex')
p.add_argument('--platform-label',default=platform.machine()+' '+platform.system())
a=p.parse_args()
assert a.threads > 0
baseline_name='vienna-baseline' if a.baseline_engine=='vienna' else 'rust-baseline'
stamp=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%f')
root=Path('benchmarks/ddg/iterations')/(stamp+'-real-site-annotation');root.mkdir(parents=True)
record=dict(label=root.name,threads=a.threads,platform=a.platform_label,complete=False,workload='Complete real SCN2A/SCN1A site annotation; same files and all report rows',
            queries=sum(bool(line.strip()) for line in (a.input/'queries.jsonl').open()),input_directory=str(a.input),stdout_sink='regular file; comparison streamed after timing',pre_run_sync=True,attempts=[],sha256={str(f):hashlib.sha256(f.read_bytes()).hexdigest() for f in [Path(a.baseline),Path(a.binary),a.input/'queries.jsonl',a.input/'reference.jsonl',a.input/'report.jsonl']})
if a.baseline_engine=='vienna':
    comparator=Path(a.rnaduplex)
    record['sha256'][str(comparator)]=hashlib.sha256(comparator.read_bytes()).hexdigest()
    record['vienna_version']=subprocess.check_output([str(comparator),'--version'],text=True).strip()
try:
    for rep in range(a.runs):
        outputs={}
        for name in ([baseline_name,'ooff'] if rep%2==0 else ['ooff',baseline_name]):
            command=[a.baseline if name==baseline_name else a.binary,'--sites',str(a.input/'report.jsonl'),
                     '--queries',str(a.input/'queries.jsonl'),'--reference',str(a.input/'reference.jsonl'),
                     '--rnaduplex',a.rnaduplex,'--threads',str(a.threads),'--timeout','600']
            command.extend(['--energy-engine',a.baseline_engine if name==baseline_name else 'rust'])
            output_path=root/f'{name}-{rep}.stdout'
            # Drain prior repetitions' dirty pages outside the timed region;
            # otherwise delayed EBS writeback randomly lands in the next run.
            os.sync()
            start=time.perf_counter()
            with output_path.open('w') as output:
                result=subprocess.run(command,stdout=output,stderr=subprocess.PIPE,text=True,timeout=660)
            elapsed=time.perf_counter()-start
            (root/f'{name}-{rep}.stderr').write_text(result.stderr)
            profile={}
            if result.returncode==0:
                profile=json.loads(result.stderr.splitlines()[-1]).get('profile',{})
            record['attempts'].append(dict(engine=name,repetition=rep,seconds=elapsed,returncode=result.returncode,command=command,profile=profile))
            assert result.returncode==0,result.stderr[-1000:]
            outputs[name]=output_path
            print(name,rep,elapsed,flush=True)
        with outputs['ooff'].open() as candidate, outputs[baseline_name].open() as baseline:
            for line_number,(left,right) in enumerate(zip_longest(candidate,baseline),1):
                assert left is not None and right is not None and json.loads(left)==json.loads(right), f'Annotated outputs differ at row {line_number}'
    record.update(complete=True,all_fields_match=True,sites=sum(json.loads(line)['type']=='site' for line in outputs['ooff'].open()))
    record['medians']={name:statistics.median(r['seconds'] for r in record['attempts'] if r['engine']==name) for name in outputs}
    record['speedup']=record['medians'][baseline_name]/record['medians']['ooff']
    print('PASS',record['sites'],'real sites;',record['speedup'],'speedup',flush=True)
except Exception as error:
    record['error']=str(error);raise
finally:
    (root/'result.json').write_text(json.dumps(record,indent=2)+'\n')
print(root)
