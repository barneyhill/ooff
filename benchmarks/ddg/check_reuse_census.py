#!/usr/bin/env python3
"""Check exact keys, duplicate ASOs, N, length, prefix reuse and repetition handling."""
import json, subprocess, tempfile
from pathlib import Path
root=Path(tempfile.mkdtemp(prefix='oofft-census-check-'))
(root/'reference.fa').write_text('>r0\n'+'A'*40+'\n>r1\n'+'N'*40+'\n')
(root/'queries.fa').write_text('>q0\n'+'T'*20+'\n>q1\n'+'T'*20+'\n>q2\n'+'C'*20+'\n')
(root/'sites.csv').write_text('0,0,0,0,17,3\n0,1,0,1,18,3\n0,0,0,0,18,2\n0,2,1,0,17,3\n1,0,0,0,17,3\n')
result=subprocess.run(['target/release/examples/energy-reuse-census',str(root/'reference.fa'),str(root/'queries.fa'),str(root/'sites.csv'),str(root/'out'),'100'],check=True,capture_output=True,text=True)
r=json.loads(result.stdout)
assert r['intervals']==4 and r['distinct_sequence_pairs']==3 and r['distinct_asos']==2
assert r['ambiguous_intervals']==1 and r['stopped_at_next_repetition']
assert r['prefix_columns']==52 and r['prefix_columns_reusable']==16
pairs=[json.loads(line) for line in (root/'out/sample-pairs.jsonl').read_text().splitlines()]
assert {(p['aso'],p['target']) for p in pairs}=={('U'*20,'A'*17),('U'*20,'A'*18),('C'*20,'N'*17)}
assert (root/'out/unique-pairs.u128le').stat().st_size==3*16
print('PASS',root)

subprocess.run(['target/release/examples/energy-reuse-census','expand',str(root/'out'),str(root/'expanded.jsonl')],check=True)
assert [json.loads(line) for line in (root/'expanded.jsonl').read_text().splitlines()]==pairs

f=json.loads(subprocess.check_output(['target/release/examples/energy-reuse-census','frontier-stats',str(root/'out')],text=True))
assert f['pairs']==3 and f['independent_columns']==52 and f['trie_nodes']==35
assert f['frontier_columns_with_16_lane_padding']==35*16
