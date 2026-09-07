#!/usr/bin/env python3
"""Run sequential, fresh-process screening comparisons on allocated CPUs; retain every attempt."""
import argparse,datetime,json,os,random,sys,time
from pathlib import Path
from command import run,refresh

ROOT=Path('data/classic-v1')
THREADS=int(os.environ.get('OOFF_BENCH_THREADS',len(os.sched_getaffinity(0))))
INDEX=f'data/fm-classic-{THREADS}-v1'

def benchmark(tool,n,repetition,timeout,phase,run_timeout=None):
    assert timeout>0 and (run_timeout is None or run_timeout>0)
    deadline=time.monotonic()+run_timeout if run_timeout is not None else None
    query=ROOT/f'targets-{n}.fa'
    meta=dict(tool=tool,queries=n,repetition=repetition,phase=phase,threads=THREADS,workload='one verified other-gene witness per ASO',scope='Ensembl110 human RNA records',cache='fresh process; OS cache not flushed; no index-build time')
    meta.update(stage_timeout_seconds=timeout,run_timeout_seconds=run_timeout)
    prefix=f'{phase}-{tool}-{n}-r{repetition}'
    stages=[]
    def execute(label,cmd):
        remaining=min(timeout,max(0,deadline-time.monotonic())) if deadline is not None else timeout
        directory,result=run(prefix+'-'+label,cmd,remaining,meta);stages.append((directory,result));return directory,result['returncode']==0 and not result['timeout']
    if tool=='ooff':
        directory,ok=execute('search',['target/release/ooff-index','search','--index',INDEX,'--reference','data/reference-v1/reference.fa','--queries',str(ROOT/f'queries-{n}.fa'),'--queries-limit',str(n),'--threads',str(THREADS),'--repetitions','1','--mode','screen','--mmap','-k','3'])
        if ok:
            output=json.loads((directory/'stdout').read_text());counts=output['runs'][0]['counts']
            assert len(counts)==n and output['complete']
            verification,ok=execute('verify',['python3','benchmarks/classic/verify.py','--input',str(directory/'stdout'),'--kind','native','--queries',str(query),'--threads',str(THREADS)])
            if ok:
                audit=json.loads((verification/'stdout').read_text())
                audit['witness_query_ids']=[f'q{i}' for i,c in enumerate(counts) if c]
                assert set(audit['witnesses'])==set(audit['witness_query_ids']),'Native witnesses failed independent verification'
    else:
        if tool=='sassy':
            executable=os.environ.get('OOFF_SASSY_BIN','/home/ubuntu/comparators/sassy-classic-screen-0.2.6/target/release/sassy-classic-screen')
            directory,ok=execute('search',[executable,'--reference',str(ROOT/'eligible.fa'),'--queries',str(query),'--threads',str(THREADS),'-k','3'])
            kind='blast' # Four-column query/record/start/end witness format.
        elif tool=='bwa':
            sai,ok=execute('aln',['bwa','aln','-t',str(THREADS),'-n','3','-o','3','-e','3','-i','0','-l','1024','-M','1','-O','1','-E','1',str(ROOT/'bwa'),str(query)])
            if ok:directory,ok=execute('samse',['bwa','samse','-n','1000',str(ROOT/'bwa'),str(sai/'stdout'),str(query)])
            kind='sam'
        elif tool=='blast':
            directory,ok=execute('search',['blastn','-task','blastn-short','-db',str(ROOT/'blast'),'-query',str(query),'-num_threads',str(THREADS),'-strand','plus','-dust','no','-soft_masking','false','-evalue','1000000000','-reward','1','-penalty','-1','-gapopen','0','-gapextend','2','-qcov_hsp_perc','85','-max_target_seqs','1000','-max_hsps','1','-outfmt','6 qseqid sseqid sstart send'])
            kind='blast'
        elif tool=='minimap2':
            directory,ok=execute('search',['minimap2','-a','-t',str(THREADS),'-n','2','-m','10','-s','10','-A','1','-B','1','-O','1,1','-E','1,1','-N','100','-p','0','--for-only',str(ROOT/'minimap2-k7w5.mmi'),str(query)])
            kind='sam'
        else:raise ValueError(tool)
        if ok:
            verification,ok=execute('verify',['python3','benchmarks/classic/verify.py','--input',str(directory/'stdout'),'--kind',kind,'--queries',str(query),'--threads',str(THREADS)])
            if ok:audit=json.loads((verification/'stdout').read_text())
    summary=dict(**meta,complete=ok,stages=[str(d/'result.json') for d,r in stages],wall_seconds=sum(r['wall_seconds'] for d,r in stages),utc=datetime.datetime.now(datetime.timezone.utc).isoformat())
    if ok:summary.update(audit)
    path=stages[0][0]/'screening.json';path.write_text(json.dumps(summary,indent=2)+'\n')
    refresh()
    print(json.dumps({k:v for k,v in summary.items() if k not in ['witnesses','witness_query_ids']}),flush=True)
    return summary

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--tools',nargs='+',default=['ooff','bwa','blast','minimap2']);p.add_argument('--counts',nargs='+',type=int,default=[10,100,1000,10000,100000]);p.add_argument('--repetitions',type=int,default=3);p.add_argument('--timeout',type=float,default=600,help='Maximum seconds per stage (default: 600)');p.add_argument('--run-timeout',type=float,default=600,help='Total seconds across search, conversion and verification per run (default: 600)');p.add_argument('--phase',default='measurement');a=p.parse_args()
    if a.timeout<=0 or a.run_timeout<=0 or a.repetitions<1 or any(n<1 for n in a.counts):p.error('timeouts, repetitions and counts must be positive')
    for n in a.counts:
        timed_out=set()
        for rep in range(1,a.repetitions+1):
            tools=list(a.tools);random.Random(20260907+n+rep).shuffle(tools)
            for tool in tools:
                if tool in timed_out:continue
                result=benchmark(tool,n,rep,a.timeout,a.phase,run_timeout=a.run_timeout)
                if not result['complete']:
                    if any(json.loads(Path(stage).read_text()).get('timeout') for stage in result['stages']):
                        timed_out.add(tool)
                    else:raise RuntimeError(f"{tool} failed; inspect retained logs before continuing")
