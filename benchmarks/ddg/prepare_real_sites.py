#!/usr/bin/env python3
"""Discover actual SCN2A ASO sites in SCN1A RNA and measure sequence reuse."""
import argparse, datetime, hashlib, json, subprocess, time
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--queries',type=int,default=100);a=p.parse_args()
stamp=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%f')
root=Path('data/ddg-real-sites')/(stamp+f'-scn2a{a.queries}-scn1a');root.mkdir(parents=True)
evidence=Path('benchmarks/ddg/iterations')/(stamp+f'-real-sites-discovery-{a.queries}');evidence.mkdir(parents=True)
fasta=lambda p: ''.join(l.strip().upper() for l in p.read_text().splitlines() if not l.startswith('>'))
source_on=Path('data/ddg-inputs/SCN2A.fa');source_off=Path('data/ddg-inputs/SCN1A.fa')
on,off=fasta(source_on),fasta(source_off)
unique={}
for i in range(len(on)-19):
    target=on[i:i+20]
    if set(target)<=set('ACGT'):unique.setdefault(target,i)
items=list(unique.items())
if not 1 <= a.queries <= len(items):
    raise ValueError(f"Requested {a.queries} distinct ASOs, but source provides {len(items)}")
selected=[items[i*(len(items)-1)//max(1,a.queries-1)] for i in range(a.queries)]
queries=[dict(id=f'q{i}',sequence=t.translate(str.maketrans('ACGT','TGCA'))[::-1],intended_genes=['SCN2A']) for i,(t,position) in enumerate(selected)]
(root/'queries.jsonl').write_text(''.join(json.dumps(q)+'\n' for q in queries))
# Deliberately use RNA-coordinate metadata, not an invented genomic mapping.
reference=dict(id='ENST00000674923',sequence=off,genes=['SCN1A'],transcripts=['ENST00000674923'],
               contig='transcript:ENST00000674923',strand='+',blocks=[dict(start=0,end=len(off))])
(root/'reference.jsonl').write_text(json.dumps(reference)+'\n')
command=['target/release/oofft','report','--queries',str(root/'queries.jsonl'),'--reference',str(root/'reference.jsonl'),
         '--policy','other-gene','--reference-release','local-oligoai-canonical-premrna','--scope','single SCN1A RNA; RNA-coordinate benchmark metadata',
         '--biotype-policy','SCN1A only','-k','3']
record=dict(label=evidence.name,threads=1,complete=False,workload='Actual distinct SCN2A ASOs against real SCN1A pre-mRNA, exhaustive k3 interval discovery; no energy timing',
            queries=a.queries,input_directory=str(root),attempts=[],selection=[dict(query_id=f'q{i}',on_target_start=position,target=t) for i,(t,position) in enumerate(selected)],
            sha256={str(f):hashlib.sha256(f.read_bytes()).hexdigest() for f in [source_on,source_off,root/'queries.jsonl',root/'reference.jsonl',Path(command[0])]})
try:
    start=time.perf_counter()
    with (root/'report.jsonl').open('w') as output:
        result=subprocess.run(command,stdout=output,stderr=subprocess.PIPE,text=True,timeout=180)
    seconds=time.perf_counter()-start
    (evidence/'stderr').write_text(result.stderr)
    record['attempts'].append(dict(engine='discovery',repetition=0,seconds=seconds,returncode=result.returncode,command=command,measurement_kind='site_discovery'))
    assert result.returncode==0,result.stderr
    count=0;pairs=set();by_query={};last=None
    for line in (root/'report.jsonl').open():
        row=json.loads(line);last=row
        if row['type']=='site':
            target=off[row['alignment']['start']:row['alignment']['end']]
            pairs.add((row['query_id'],target));count+=1
            by_query[row['query_id']]=by_query.get(row['query_id'],0)+1
    assert last['type']=='run_complete' and last['reported_sites']==count
    record.update(complete=True,all_fields_match=False,reported_sites=count,distinct_energy_pairs=len(pairs),sites_per_query=by_query,
                  note='Discovery completed; this iteration does not establish energy equality')
    print(count,'actual sites;',len(pairs),'distinct ASO/target-sequence pairs;',seconds,'seconds;',root,flush=True)
except Exception as error:
    record['error']=str(error);raise
finally:
    (evidence/'result.json').write_text(json.dumps(record,indent=2)+'\n')
