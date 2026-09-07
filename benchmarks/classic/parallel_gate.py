#!/usr/bin/env python3
"""Fresh full human-reference tuple gate before publishing parallel timings."""
import datetime,json,os,sys
from pathlib import Path
from command import run
sys.path.insert(0,'benchmarks')
from compare_site_files import compare

from prepare_witnesses import main as prepare_witnesses
prepare_witnesses()

root=Path('data/classic-v1')/('parallel-gate-'+datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S'))
root.mkdir()
checks=[]
threads=int(os.environ.get('OOFF_BENCH_THREADS',len(os.sched_getaffinity(0))))
for k in range(4):
    files={}
    for tool in ['sassy','ooff']:
        tuples=root/f'{tool}-k{k}.csv'
        common=['--reference','data/reference-v1/reference.fa','--queries','benchmarks/sassy_benchmark/SCN2A_patterns.fa','--mode','sites','--queries-limit','32','-k',str(k),'--repetitions','1','--site-tuples',str(tuples)]
        if tool=='ooff':command=['target/release/ooff-index','search','--index',f'data/fm-classic-{threads}-v1','--reverse-index','data/fm-reverse-v1','--mmap','--threads',str(threads),*common]
        else:command=['/home/ubuntu/comparators/sassy-0.2.6-fast-interval/target/release/sassy-comparator',*common]
        directory,result=run(f'parallel-human-gate-{tool}-k{k}',command,300,dict(kind='correctness',threads=threads if tool=='ooff' else 1,k=k))
        assert result['returncode']==0,result
        output=json.loads((directory/'stdout').read_text());assert output['complete']
        files[tool]=tuples
    result=compare(files['ooff'],files['sassy'],root/f'buckets-k{k}')
    checks.append(dict(k=k,**result))
    (root/'results.json').write_text(json.dumps(checks,indent=2)+'\n')
    assert result['equal'] and result['unique'],result
Path('results/parallel-human-gate.success').write_text(json.dumps(dict(complete=True,artifact=str(root),checks=checks),indent=2)+'\n')
