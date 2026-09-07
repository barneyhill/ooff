#!/usr/bin/env python3
"""Render measured 8-thread runtime/recovery curves, never substitute fabricated data."""
import argparse,csv,json,statistics
from collections import defaultdict
from pathlib import Path

p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path('benchmarks/classic/iterations'));p.add_argument('--output',type=Path,default=Path('docs/images/classic-scaling'));a=p.parse_args()
# README batches follow powers of ten; other measurements remain in the ledger.
plot_counts={10,100,1000,10000,100000}
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
tools=['ooff','bwa','blast','minimap2']
for tool in tools:
    points=[n for (t,n),rs in groups.items() if t==tool and len(rs)>=3]
    measured={r['queries'] for r in records if r['tool']==tool and (r['complete'] or r['timed_out'])}
    assert len(points)>=2 and len(measured)>=3,f'{tool}: need repeated completed points and at least three measured sizes'
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
plt.rcParams.update({'font.family':'DejaVu Sans','font.size':10,'axes.spines.top':False,'axes.spines.right':False,'svg.fonttype':'none'})
fig,axes=plt.subplots(1,2,figsize=(11,5.1),gridspec_kw={'width_ratios':[1.35,1]})
labels={'ooff':'ooff','bwa':'BWA-aln','blast':'BLASTN-short','minimap2':'minimap2 (20mer settings)'}
colors={'ooff':'#007C91','bwa':'#C85132','blast':'#7965AC','minimap2':'#88703A'}
for tool in tools:
    ns=sorted(n for (t,n),rs in groups.items() if t==tool and len(rs)>=3)
    ys=[];low=[];high=[];recovery=[]
    for n in ns:
        rs=sorted(groups[(tool,n)],key=lambda r:r['artifact'])[-3:]
        times=[r['wall_seconds'] for r in rs];median=statistics.median(times)
        ys.append(median);low.append(median-min(times));high.append(max(times)-median)
        recovery.append(statistics.median(r['recovery_percent'] for r in rs))
    axes[0].errorbar(ns,ys,yerr=[low,high],label=labels[tool],color=colors[tool],marker='o',linewidth=2 if tool=='ooff' else 1.5,capsize=3)
    axes[1].plot(ns,recovery,label=labels[tool],color=colors[tool],marker='o',linewidth=2 if tool=='ooff' else 1.5)
    censored=[r for r in records if r['tool']==tool and r['timed_out']]
    for n in sorted({r['queries'] for r in censored}):
        lower_bound=statistics.median(r['wall_seconds'] for r in censored if r['queries']==n)
        axes[0].scatter([n],[lower_bound],marker='^',facecolors='none',edgecolors=colors[tool],s=60,zorder=5)
for r in records:
    if r['failure_reason']=='out_of_memory':
        axes[1].annotate(f"{r['queries']:,} ASOs: {r['tool']} OOM (32 GiB)",
                         xy=(r['queries'], .82), xycoords=axes[1].get_xaxis_transform(),
                         ha='right', va='top', fontsize=8, color=colors[r['tool']])
for ax in axes:
    ax.set_xscale('log',base=10);ax.set_xlabel('Number of 20nt ASOs');ax.grid(alpha=.17)
    ticks=sorted({r['queries'] for r in records});ax.set_xticks(ticks,[f'{n:,}' for n in ticks]);ax.tick_params(axis='x',labelrotation=25)
axes[0].set_yscale('log',base=10);axes[0].set_ylabel('Elapsed seconds · lower is faster');axes[0].set_title('Time to verified screening results',loc='left',fontsize=12)
axes[1].set_ylim(-3,103);axes[1].set_ylabel('Verified witness recovery (%)');axes[1].set_title('Off-target witness recovery',loc='left',fontsize=12)
handles,names=axes[0].get_legend_handles_labels();fig.legend(handles,names,loc='lower center',bbox_to_anchor=(.5,.10),ncol=4,frameon=False,fontsize=9)
fig.suptitle('ASO screening against the human RNA reference · 8 threads',x=.07,ha='left',fontsize=15,fontweight='bold')
fig.text(.07,.895,'Ensembl 110 / GRCh38 gene bodies + transcripts · SCN2A-derived ASOs · ≤3 total edits',fontsize=10,color='#444444')
fig.text(.07,.027,'Medians and ranges of 3 fresh-process runs; OS cache retained. Loading, output and verification included; index build excluded.\nSeparate EC2 workers: 8 vCPUs / 4 physical cores; 16 GiB (minimap2: 32 GiB). Recovery is relative to native-positive queries.\nOpen triangles: timeout lower bounds. OOM is labelled without a timing/recovery estimate. Heuristic comparators may miss witnesses.',fontsize=8,color='#444444')
fig.subplots_adjust(left=.07,right=.98,top=.79,bottom=.32,wspace=.32)
for suffix in ['.svg','.png']:
    fig.savefig(a.output.with_suffix(suffix),dpi=180,facecolor='white')
print(a.output.with_suffix('.png'))
