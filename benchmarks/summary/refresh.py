#!/usr/bin/env python3
"""Export every retained native-summary attempt for inspection and plotting."""
import csv,json
from pathlib import Path
root=Path(__file__).resolve().parent
rows=[]
lines=['# Native summary benchmark iterations','','Whole-process timings include index/reference loading, search, deduplication and summary output. Index construction is excluded. Completion checks are structural; independent equivalence tests are separate.','','| Iteration | Complete | Seconds | Queries | Count unit | Sites | Output bytes |','| --- | --- | ---: | ---: | --- | ---: | ---: |']
for path in sorted((root/'iterations').glob('*/result.json')):
 r=json.loads(path.read_text());manifest=r.get('manifest') or {}
 resources={}
 try:resources=json.loads((path.parent/'resources.json').read_text())
 except (OSError,ValueError):pass
 row=dict(iteration=path.parent.name,complete=r['complete'],platform=r['platform'],threads=manifest.get('threads',''),queries=r.get('queries',''),count_unit=manifest.get('count_unit',''),seconds=r.get('seconds',0),total_sites=r.get('total_sites',''),output_bytes=r.get('output_bytes',''),max_rss_kib=resources.get('max_rss_kib',''),error=r.get('error',''),caveat=r.get('caveat',''))
 rows.append(row)
 lines.append(f"| [{row['iteration']}](iterations/{row['iteration']}/result.json) | {row['complete']} | {row['seconds']:.6f} | {row['queries']} | {row['count_unit']} | {row['total_sites']} | {row['output_bytes']} |")
(root/'ITERATIONS.md').write_text('\n'.join(lines)+'\n')
if rows:
 with (root/'iterations.csv').open('w',newline='') as f:
  w=csv.DictWriter(f,fieldnames=list(rows[0]));w.writeheader();w.writerows(rows)
