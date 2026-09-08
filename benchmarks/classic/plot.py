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
plt.rcParams.update({
    'font.family': 'DejaVu Sans', 'font.size': 10,
    'text.color': '#263343', 'axes.labelcolor': '#394657',
    'xtick.color': '#536171', 'ytick.color': '#536171',
    'axes.spines.top': False, 'axes.spines.right': False,
    'axes.spines.left': False, 'axes.spines.bottom': False,
    'svg.fonttype': 'none', 'svg.hashsalt': 'oofft-screening',
})
fig, ax = plt.subplots(figsize=(9.2, 4.9))
labels = {'ooff': 'oofft', 'bwa': 'BWA-aln', 'blast': 'BLASTN-short',
          'minimap2': 'minimap2', 'sassy': 'Sassy2'}
colors = {'ooff': '#087F8C', 'bwa': '#BA6541', 'blast': '#8A6BA8',
          'minimap2': '#63798B', 'sassy': '#5078B5'}
# Direct labels replace the legend; only completed repeated runs form curves.
# Thin vertical ranges show observed min/max, not confidence intervals.
for tool in tools:
    ns = sorted(n for (t, n), rs in groups.items() if t == tool and len(rs) >= 3)
    values = [[r['wall_seconds'] for r in groups[(tool, n)]] for n in ns]
    ys = [statistics.median(v) for v in values]
    color = colors[tool]
    ax.vlines(ns, [min(v) for v in values], [max(v) for v in values],
              color=color, linewidth=1.2, alpha=.45, zorder=2)
    ax.plot(ns, ys, color=color, marker='o', markersize=5 if tool == 'ooff' else 4,
            markeredgecolor='white', markeredgewidth=.7,
            linewidth=2.6 if tool == 'ooff' else 1.7, zorder=4 if tool == 'ooff' else 3)
    label = labels[tool]
    if ns[-1] == max(plot_counts):
        label += f"  {ys[-1]:.2f} s"
    ax.annotate(label, (ns[-1], ys[-1]), xytext=(10, 0),
                textcoords='offset points', color=color, fontsize=10,
                fontweight='bold' if tool == 'ooff' else 'normal', va='center')
ax.set_xscale('log', base=10)
ax.set_yscale('log', base=10)
ax.set_xticks(sorted(plot_counts), ['10', '100', '1,000', '10,000', '100,000'])
ax.set_yticks([.1, 1, 10, 100, 1000], ['0.1', '1', '10', '100', '1,000'])
ax.minorticks_off()
ax.tick_params(length=0, pad=9)
ax.set_xlim(7, 850000)
ax.set_ylim(.30, 850)
ax.set_xlabel('ASOs per batch', labelpad=14)
ax.set_ylabel('Elapsed time (s)', labelpad=12)
ax.grid(axis='y', color='#E5EAF0', linewidth=.8)
ax.set_axisbelow(True)
fig.text(.085, .945, 'Human RNA off-target screening', fontsize=16, fontweight='bold')
fig.text(.085, .892, '8 logical CPUs  ·  ≤3 edits  ·  median of 3 runs  ·  log₁₀ axes',
         fontsize=10, color='#647182')
fig.text(.085, .035,
         'Loading + search + output + verification; index build excluded. Ranges show observed min–max.',
         fontsize=8.5, color='#647182')
fig.subplots_adjust(left=.085, right=.98, top=.83, bottom=.20)
for suffix in ['.svg', '.png']:
    fig.savefig(a.output.with_suffix(suffix), dpi=200, facecolor='white',
                metadata={'Date': None} if suffix == '.svg' else None)
svg = a.output.with_suffix('.svg')
svg.write_text('\n'.join(line.rstrip() for line in svg.read_text().splitlines()) + '\n')
print(a.output.with_suffix('.png'))
