#!/usr/bin/env python3
"""Short synthetic calibration for site annotation estimates, not a genome benchmark."""
import argparse, datetime, hashlib, json, random, re, subprocess, time
from pathlib import Path

p = argparse.ArgumentParser()
p.add_argument('--records', type=int, default=1)
p.add_argument('--queries', type=int, default=1000)
p.add_argument('--hits-per-aso', type=int, default=10)
p.add_argument('--energy-engine', choices=['vienna','rust'], default='vienna')
a = p.parse_args()
count = a.queries*a.hits_per_aso
assert 0 < a.records <= count and count % a.records == 0
root = Path('benchmarks/ddg/iterations')/(datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%f')+f'-estimate-sites-{count}-records{a.records}-{a.energy_engine}')
root.mkdir(parents=True)
rng = random.Random(20260908)
rc = lambda s: s.translate(str.maketrans('ACGT','TGCA'))[::-1]
targets = []
seen = set()
while len(targets) < a.queries:
    s = ''.join(rng.choice('ACGT') for _ in range(20))
    if s not in seen:
        targets.append(s)
        seen.add(s)
queries = [dict(id=f'q{i}', sequence=rc(t), intended_genes=['intended']) for i,t in enumerate(targets)]
references = []
sites = []
oracle_pairs = [(q['sequence'], t) for q,t in zip(queries,targets)]
for rid in range(a.records):
    sequence = ''
    for j in range(count//a.records):
        index = rid*(count//a.records)+j
        qi = index % a.queries
        target = targets[qi]
        edit = index % 7
        if edit < 3:
            off = target[:8]+'A'*(edit+1)+target[8:]
        elif edit < 6:
            off = target[:8]+target[8+(edit-2):]
        else:
            off = target[:8]+('A' if target[8] != 'A' else 'C')+target[9:]
        start = len(sequence)
        end = start+len(off)
        sequence += off+'NNN'
        sites.append(dict(type='site', query_id=f'q{qi}', record_id=f'r{rid}',
                          site_id=[f'q{qi}',f'r{rid}',str(start),str(end)],
                          alignment=dict(start=start,end=end)))
        oracle_pairs.append((queries[qi]['sequence'], off))
    references.append(dict(id=f'r{rid}', sequence=sequence))
write_rows = lambda path, rows: path.write_text(''.join(json.dumps(r)+'\n' for r in rows))
write_rows(root/'queries.jsonl', queries)
write_rows(root/'reference.jsonl', references)
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
write_rows(root/'report.jsonl', [dict(type='manifest',mode='report',queries_sha256=sha(root/'queries.jsonl'),reference_sha256=sha(root/'reference.jsonl'))]+sites+[dict(type='run_complete',reported_sites=count)])
binary = Path('target/release/oofft-ddg')
engine = Path('/home/barneyh/.local/bin/RNAduplex')
record = dict(label=root.name, threads=4, queries=a.queries, annotated_sites=count, records=a.records,
              workload='Synthetic near-match intervals of length 17–23; no genome discovery or large-reference IO measured',
              attempts=[], complete=False, sha256={str(f):sha(f) for f in [binary,engine,root/'queries.jsonl',root/'reference.jsonl',root/'report.jsonl']})
try:
    for name, command, stdin in [
        ('viennarna',[str(engine),'--noconv'],''.join(x.replace('T','U')+'\n'+y.replace('T','U')+'\n' for x,y in oracle_pairs)),
        ('ooff',[str(binary),'--sites',str(root/'report.jsonl'),'--queries',str(root/'queries.jsonl'),
                 '--reference',str(root/'reference.jsonl'),'--rnaduplex',str(engine),'--threads','4','--timeout','300','--energy-engine',a.energy_engine],None)]:
        start = time.perf_counter()
        with (root/(name+'.stdout')).open('w') as output:
            result = subprocess.run(command,input=stdin,stdout=output,stderr=subprocess.PIPE,text=True,timeout=330)
        seconds = time.perf_counter()-start
        (root/(name+'.stderr')).write_text(result.stderr)
        record['attempts'].append(dict(engine=name,repetition=0,command=command,seconds=seconds,returncode=result.returncode))
        assert result.returncode == 0, result.stderr[-1000:]
        print(name, round(seconds,3),'seconds',flush=True)
    energies = [float(re.search(r'\(\s*(-?\d+\.\d+)\s*\)\s*$',line)[1]) for line in (root/'viennarna.stdout').read_text().splitlines() if '&' in line]
    actual = [json.loads(line) for line in (root/'ooff.stdout').read_text().splitlines()]
    annotated = [r for r in actual if r['type']=='site']
    assert len(annotated) == count and len(energies) == len(oracle_pairs)
    for i,(row,expected) in enumerate(zip(annotated,sites)):
        qi = i % a.queries
        on,off = energies[qi],energies[a.queries+i]
        annotation = row.pop('energy_annotation')
        assert abs(row.pop('ddg')-(off-on)) <= 1e-12
        assert annotation['dg_target'] == on and annotation['dg_other'] == off
        assert row == expected
    record.update(complete=True,all_fields_match=True)
    print('PASS',count,'sites;',root,flush=True)
except Exception as error:
    record['error'] = str(error)
    raise
finally:
    (root/'result.json').write_text(json.dumps(record,indent=2)+'\n')
