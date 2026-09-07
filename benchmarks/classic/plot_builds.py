#!/usr/bin/env python3
"""Plot actual single index builds, including memory and CPU allocation caveats."""
import csv
import json
from pathlib import Path

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

rows = []
labels = {'build-ooff-index-8': ('ooff', 8, 16), 'build-bwa-index': ('BWA', 1, 32), 'build-blast-index': ('BLAST', 1, 16), 'build-minimap2-index': ('minimap2', 8, 32)}
for path in sorted(Path('benchmarks/classic/iterations').glob('*/result.json')):
    result = json.loads(path.read_text())
    if result['label'] not in labels:
        continue
    tool, threads, ram = labels[result['label']]
    stats = {}
    for line in (path.parent / 'resources.txt').read_text().splitlines():
        if ': ' in line:
            key, value = line.strip().split(': ', 1); stats[key] = value
    rss = float(stats['Maximum resident set size (kbytes)']) / 1048576
    if result['returncode'] and tool == 'minimap2':
        ram = 16
    rows.append(dict(tool=tool, seconds=result['wall_seconds'], peak_rss_gib=rss, configured_threads=threads, host_ram_gib=ram, returncode=result['returncode'], cpu_percent=stats['Percent of CPU this job got'], artifact=str(path)))
out = Path('docs/images/classic-index-builds')
out.parent.mkdir(parents=True, exist_ok=True)
with out.with_suffix('.csv').open('w', newline='') as f:
    writer = csv.DictWriter(f, fieldnames=rows[0]); writer.writeheader(); writer.writerows(rows)
successful = {r['tool']: r for r in rows if r['returncode'] == 0}
order = ['ooff', 'BWA', 'BLAST', 'minimap2']
assert set(successful) == set(order), 'Need all four actual successful index builds'
plt.rcParams.update({'font.family': 'DejaVu Sans', 'font.size': 10, 'axes.spines.top': False, 'axes.spines.right': False, 'svg.fonttype': 'none'})
fig, axes = plt.subplots(1, 2, figsize=(11, 4.2))
colors = ['#007C91', '#C85132', '#7965AC', '#88703A']
for ax, key, title in zip(axes, ['seconds', 'peak_rss_gib'], ['Index build elapsed seconds', 'Peak process memory (GiB)']):
    values = [successful[t][key] for t in order]
    ax.bar(order, values, color=colors, width=.6)
    ax.set_title(title, loc='left', fontsize=12)
    ax.set_yscale('log',base=10); ax.grid(axis='y', alpha=.17); ax.set_axisbelow(True)
    ax.set_ylim(min(values) / 2, max(values) * 2)
    for i, value in enumerate(values):
        ax.text(i, value * 1.12, f'{value:,.2f}', ha='center', fontsize=10)
fig.suptitle('Human RNA reference · index preparation measured separately', x=.07, ha='left', fontsize=15, fontweight='bold')
fig.text(.07, .86, 'Single builds · native/minimap2 configured for 8 threads · BWA/makeblastdb are single-threaded', fontsize=10)
fig.text(.07, .03, 'ooff and BLAST: 8 vCPUs / 16 GiB. BWA build: original 16-vCPU / 32-GiB host; index reused for 8-vCPU search.\nminimap2: 8 vCPUs / 32 GiB after a retained 16-GiB OOM. Native forward index only; 0.013% additional intended-gene bases.\nCache state not normalized; includes index serialization. Thread availability does not imply full CPU utilization.', fontsize=8, color='#444444')
fig.subplots_adjust(left=.07, right=.98, top=.74, bottom=.27, wspace=.25)
for suffix in ['.svg', '.png']:
    fig.savefig(out.with_suffix(suffix), dpi=180, facecolor='white')
print(out.with_suffix('.png'))
