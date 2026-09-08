#!/usr/bin/env python3
"""Hash every library loop energy separately by gap shape for compact CI evidence."""
import hashlib, json, subprocess
from pathlib import Path
binary = Path('/tmp/oofft-loop-oracle')
subprocess.run(['cc','-O2','-I/home/barneyh/.local/include','benchmarks/ddg/loop_oracle.c',
                '/home/barneyh/.local/lib/libRNA.a','-lm','-fopenmp','-o',str(binary)],check=True)
process = subprocess.Popen([str(binary)],stdout=subprocess.PIPE)
rows = []
for u in range(31):
    for v in range(31-u):
        data = process.stdout.read(6*6*5**4*4)
        assert len(data) == 6*6*5**4*4
        rows.append(dict(u=u,v=v,sha256=hashlib.sha256(data).hexdigest()))
assert process.stdout.read() == b'' and process.wait() == 0
Path('fixtures/energy_loops_golden.json').write_text(json.dumps(rows,separators=(',',':'))+'\n')
print(len(rows),'loop shapes;',len(rows)*6*6*5**4,'individual energy evaluations')
