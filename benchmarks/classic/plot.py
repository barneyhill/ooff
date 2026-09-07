#!/usr/bin/env python3
"""Plot verified screening time across ASO counts, with index builds in the caption."""
import argparse,csv,json,statistics
from collections import defaultdict
from pathlib import Path

p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path('benchmarks/classic/iterations'));p.add_argument('--output',type=Path,default=Path('docs/images/classic-scaling'));a=p.parse_args()
# README batches follow powers of ten; other measurements remain in the ledger.
plot_counts={10,100,1000,10000,100000}
build_names={'ooff':'ooff','BWA':'bwa','BLAST':'blast','minimap2':'minimap2'}
with Path('docs/images/classic-index-builds.csv').open() as f:
    builds={build_names[r['tool']]:r for r in csv.DictReader(f) if int(r['returncode'])==0}
assert set(builds)==set(build_names.values()), 'Need all four successful measured builds'
records=[]
for path in sorted(a.root.glob('*/screening.json')):
    x=json.loads(path.read_text())
    if x.get('phase')!='measurement' or x['queries'] not in plot_counts:continue
    x['artifact']=str(path)
    stages=[json.loads(Path(stage).read_text()) for stage in x['stages']]
    x['timed_out']=any(stage.get('timeout',False) for stage in stages)
    failure=path.parent/'resource_failure.json'
    x['failure_reason']=json.loads(failure.read_text())['kind'] if failure.exists() else ''
    # Keep mapper execution separate from our independent audit in exported
    # data. The main figure still reports the complete verified screening task.
    x['search_seconds']=sum(s['wall_seconds'] for s in stages if s['label'].endswith(('-search','-aln')))
    x['conversion_seconds']=sum(s['wall_seconds'] for s in stages if s['label'].endswith('-samse'))
    x['verification_seconds']=sum(s['wall_seconds'] for s in stages if s['label'].endswith('-verify'))
    x['tool_seconds']=x['search_seconds']+x['conversion_seconds']
    records.append(x)
assert records,'No measured scaling results; refusing to create a placeholder chart'
truth={}
for x in records:
    if x['tool']=='ooff' and x['complete']:
        ids=set(x['witness_query_ids'])
        if x['queries'] in truth:assert truth[x['queries']]==ids,'Native screening differs between repetitions'
        truth[x['queries']]=ids
rows=[];groups=defaultdict(list)
for x in records:
    row={k:x[k] for k in ['tool','queries','repetition','phase','threads','complete','timed_out','wall_seconds','artifact','failure_reason']}
    row.update({k:x[k] for k in ['search_seconds','conversion_seconds','verification_seconds','tool_seconds']})
    row['index_build_seconds']=float(builds[x['tool']]['seconds']) if x['tool'] in builds else 0.0
    row['index_build_artifact']=builds[x['tool']]['artifact'] if x['tool'] in builds else 'No persistent index'
    row['build_plus_search_seconds']=row['index_build_seconds']+x['wall_seconds'] if x['complete'] or x['timed_out'] else ''
    row['recovered']=x.get('recovered','');row['recovery_percent']=''
    if x['complete'] and x['queries'] in truth:
        ids=set(x['witness_query_ids'] if x['tool']=='ooff' else x['witnesses'])
        assert ids<=truth[x['queries']],f"Unexpected verified comparator positive: {x['artifact']}"
        row['recovery_percent']=100*len(ids)/len(truth[x['queries']]) if truth[x['queries']] else 100
        groups[(x['tool'],x['queries'])].append(row)
    rows.append(row)
a.output.parent.mkdir(parents=True,exist_ok=True)
with a.output.with_suffix('.csv').open('w',newline='') as f:
    writer=csv.DictWriter(f,fieldnames=rows[0]);writer.writeheader();writer.writerows(rows)
tools=['ooff','bwa','blast','minimap2'] + (['sassy'] if any(r['tool']=='sassy' for r in records) else [])
for tool in tools:
    points=[n for (t,n),rs in groups.items() if t==tool and len(rs)>=3]
    measured={r['queries'] for r in records if r['tool']==tool and (r['complete'] or r['timed_out'])}
    assert len(points)>=2 and len(measured)>=3,f'{tool}: need repeated completed points and at least three measured sizes'
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':10,'axes.spines.top':False,'axes.spines.right':False,'svg.fonttype':'none'})
fig,ax=plt.subplots(figsize=(9,4.8))
labels={'ooff':'ooff','bwa':'BWA-aln','blast':'BLASTN-short','minimap2':'minimap2','sassy':'Sassy2'}
colors={'ooff':'#007C91','bwa':'#C85132','blast':'#7965AC','minimap2':'#88703A','sassy':'#4677B4'}
for tool in tools:
    ns=sorted(n for (t,n),rs in groups.items() if t==tool and len(rs)>=3)
    ys=[statistics.median(r['wall_seconds'] for r in groups[(tool,n)]) for n in ns]
    ax.plot(ns,ys,label=labels[tool],color=colors[tool],marker='o',linewidth=2)
    censored=[r for r in rows if r['tool']==tool and r['timed_out']]
    for n in sorted({r['queries'] for r in censored}):
        lower_bound=statistics.median(r['wall_seconds'] for r in censored if r['queries']==n)
        ax.annotate('timeout',xy=(n,lower_bound),xytext=(0,12 if tool=='sassy' else 0),textcoords='offset points',ha='center',fontsize=8,color=colors[tool])
for r in rows:
    if r['failure_reason']=='out_of_memory':
        ax.text(.98,.04,f"{labels[r['tool']]}: OOM at {r['queries']:,} ASOs (32 GiB)",
                transform=ax.transAxes,ha='right',fontsize=9,color=colors[r['tool']])
ax.set_xscale('log',base=10);ax.set_yscale('log',base=10)
ticks=sorted(plot_counts);ax.set_xticks(ticks,[f'{n:,}' for n in ticks])
ax.set_xlabel('Number of ASOs');ax.set_ylabel('Search time (seconds)')
ax.grid(axis='y',alpha=.17);ax.set_axisbelow(True)
ax.legend(ncol=len(tools),loc='upper left',frameon=False,fontsize=9)
ax.set_ylim(.3,2500)
fig.suptitle('ASO screening against the human RNA reference',x=.10,ha='left',fontsize=15,fontweight='bold')
fig.text(.10,.885,'Median screening time · 8 search threads · log10 axes',fontsize=10,color='#444444')
fig.text(.10,.035,'Index build: ooff 206 s · BWA 2,753 s · BLAST 16 s · minimap2 74 s · Sassy2: no persistent index.\nSearch includes loading, output and verification. Build costs excluded. Heuristic tools may miss hits; see methods.',fontsize=8,color='#444444')
fig.subplots_adjust(left=.10,right=.98,top=.83,bottom=.23)
for suffix in ['.svg','.png']:
    fig.savefig(a.output.with_suffix(suffix),dpi=180,facecolor='white')
print(a.output.with_suffix('.png'))
