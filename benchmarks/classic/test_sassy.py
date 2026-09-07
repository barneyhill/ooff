#!/usr/bin/env python3
"""Check the external Sassy screening adapter against an independent scalar oracle."""
import argparse
import random
import subprocess
import tempfile
from pathlib import Path
from verify import distance

p = argparse.ArgumentParser()
p.add_argument('--binary', required=True)
a = p.parse_args()
root = Path(tempfile.mkdtemp(prefix='ooff-sassy-screen-oracle-'))
rng = random.Random(20260908)
queries = [''.join(rng.choice('ACGT') for _ in range(20)) for _ in range(8)] + ['C'*20, 'T'*20]
records = [queries[0] + 'N' + queries[1][1:] + 'N' + queries[2][:9] + 'A' + queries[2][9:],
           queries[3][:10] + 'N' + queries[3][10:] + 'N' + queries[4][:8] + queries[4][9:] + 'N' + queries[5]]
(root/'queries.fa').write_text(''.join(f'>q{i}\n{q}\n' for i,q in enumerate(queries)))
(root/'reference.fa').write_text(''.join(f'>r{i}\n{r}\n' for i,r in enumerate(records)))
for k in range(4):
    expected = set()
    for qi, query in enumerate(queries):
        for text in records:
            for start in range(len(text)):
                for length in range(20-k, 21+k):
                    target = text[start:start+length]
                    if len(target)==length and 'N' not in target and distance(query,target)<=k:
                        expected.add(f'q{qi}')
    for threads in [1,8]:
        result = subprocess.run([a.binary,'--reference',str(root/'reference.fa'),'--queries',str(root/'queries.fa'),
                                 '--threads',str(threads),'--chunk-bases','19','-k',str(k)],capture_output=True,text=True,check=True,timeout=30)
        found = set()
        for line in result.stdout.splitlines():
            q,r,start,end = line.split('\t'); start=int(start)-1; end=int(end)
            assert q not in found, 'Duplicate witness'
            target=records[int(r[1:])][start:end]
            assert 20-k<=len(target)<=20+k and 'N' not in target
            assert distance(queries[int(q[1:])],target)<=k
            found.add(q)
        assert found==expected,(k,threads,found,expected)
        print(f'PASS k={k}, threads={threads}, {len(found)} witnesses',flush=True)
print(f'Fixture inputs retained at {root}')
