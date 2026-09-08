#!/usr/bin/env python3
"""Pin diverse direct-RNAduplex energies for Rust-only CI differential tests."""
import hashlib, json, random, re, subprocess
from decimal import Decimal
from pathlib import Path
rng = random.Random(314159265)
pairs = []
for _ in range(1200):
    pairs.append((''.join(rng.choices('ACGUN', weights=[10,10,10,10,1], k=rng.randrange(1,46))),
                  ''.join(rng.choices('ACGUN', weights=[10,10,10,10,1], k=rng.randrange(1,46)))))
for length in [16,18,20,25,32]:
    for _ in range(100):
        target = ''.join(rng.choice('ACGU') for _ in range(length))
        aso = target.translate(str.maketrans('ACGU','UGCA'))[::-1]
        off = list(target)
        for _ in range(rng.randrange(4)):
            pos = rng.randrange(len(off))
            mode = rng.randrange(3)
            if mode == 0: off[pos] = rng.choice('ACGU')
            elif mode == 1: off.insert(pos,rng.choice('ACGU'))
            else: off.pop(pos)
        pairs.append((aso,''.join(off)))
for a in 'ACGUN':
    for b in 'ACGUN':
        for n in [1,17,20,23,40]: pairs.append((a*n,b*n))
binary = Path('/home/barneyh/.local/bin/RNAduplex')
command = [str(binary),'--noconv']
result = subprocess.run(command,input=''.join(a+'\n'+b+'\n' for a,b in pairs),capture_output=True,text=True,check=True)
lines = [line for line in result.stdout.splitlines() if '&' in line]
assert len(lines) == len(pairs)
rows = [dict(aso=a,target=b,cents=int(Decimal(re.search(r'\(\s*(-?\d+\.\d+)\s*\)\s*$',line)[1])*100))
        for (a,b),line in zip(pairs,lines)]
path = Path('fixtures/energy_golden.json')
path.write_text(json.dumps(rows,separators=(',',':'))+'\n')
Path('benchmarks/ddg/energy-golden-provenance.json').write_text(json.dumps(dict(command=command,
    vienna_version='2.7.0',cases=len(rows),binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
    fixture_sha256=hashlib.sha256(path.read_bytes()).hexdigest()),indent=2)+'\n')
print(len(rows),'golden energy cases')
