#!/usr/bin/env python3
"""Extract complete interval sequences and pinned oracle energies for kernel tests."""
import argparse, hashlib, json
from decimal import Decimal
from pathlib import Path
p=argparse.ArgumentParser()
p.add_argument('--input',type=Path,required=True)
p.add_argument('--annotation',type=Path,required=True)
p.add_argument('--output',type=Path,required=True)
a=p.parse_args()
queries={r['id']:r['sequence'] for r in map(json.loads,(a.input/'queries.jsonl').open())}
reference={r['id']:r['sequence'] for r in map(json.loads,(a.input/'reference.jsonl').open())}
sha=lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
cases=[]; complete=False
for row in map(json.loads,a.annotation.open()):
    if row['type']=='manifest':
        assert row['queries_sha256']==sha(a.input/'queries.jsonl')
        assert row['reference_sha256']==sha(a.input/'reference.jsonl')
    elif row['type']=='site':
        lo,hi=row['alignment']['start'],row['alignment']['end']
        assert row['energy_annotation']['scope']=='site'
        cases.append(dict(aso=queries[row['query_id']],target=reference[row['record_id']][lo:hi],
                          cents=int(Decimal(str(row['energy_annotation']['dg_other']))*100)))
    elif row['type']=='run_complete':
        assert row['reported_sites']==len(cases); complete=True
assert complete
a.output.write_text(json.dumps(cases,separators=(',',':'))+'\n')
print(json.dumps(dict(cases=len(cases),output=str(a.output),sha256=sha(a.output),oracle_annotation=str(a.annotation),oracle_sha256=sha(a.annotation))))
