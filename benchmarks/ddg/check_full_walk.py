import json,pathlib,subprocess,datetime
root=pathlib.Path('/tmp')/('oofft-walk-check-'+datetime.datetime.now().strftime('%H%M%S%f'));root.mkdir()
seq='ACGTCAGTACGATCGTACGCTAGCTAGGCTACGTACGTTACGATCGTACGA'
(root/'gene.fa').write_text('>gene\n'+seq[:24]+seq[24:28].lower()+seq[28:]+'\n')
(root/'reference.fa').write_text('>other|other\n'+seq+'\n')
(root/'records.jsonl').write_text(json.dumps(dict(id='other',sequence=seq,genes=['other'],contig='test',strand='+',blocks=[dict(start=0,end=len(seq))]))+'\n')
for name,extra in [('forward',[]),('reverse',['--reverse-records'])]:
 subprocess.run(['target/debug/oofft-index','build','--reference',str(root/'reference.fa'),'--output',str(root/name),*extra],check=True,stdout=subprocess.DEVNULL)
subprocess.run(['target/debug/oofft-index','cache-annotations','--index',str(root/'forward'),'--reference',str(root/'reference.fa'),'--annotations',str(root/'records.jsonl'),'--output',str(root/'cache.json')],check=True,stdout=subprocess.DEVNULL)
subprocess.run(['python3','benchmarks/ddg/full_walk.py','--gene',str(root/'gene.fa'),'--reference-root',str(root),'--index',str(root/'forward'),'--reverse-index',str(root/'reverse'),'--annotation-cache',str(root/'cache.json'),'--binary','target/debug/oofft','--ddg-binary','target/debug/oofft-ddg','--threads','2','--output',str(root/'output')],check=True)
x=json.load(open(root/'output/result.json'));assert x['complete']
rows=[json.loads(l) for l in (root/'output/ddg-work/annotated.jsonl').open()];sites=[r for r in rows if r['type']=='site'];assert len(sites)==x['reported_sites']
queries={r['id']:r for r in map(json.loads,(root/'output/queries.jsonl').open())}
assert all(r['energy_annotation']['ddg_sign']=='dg_other - dg_target' for r in sites)
print(root, len(queries),len(sites))

import gzip,hashlib
command=['python3','benchmarks/ddg/walk_chunked.py','--gene',str(root/'gene.fa'),'--reference-root',str(root),'--index',str(root/'forward'),'--annotation-cache',str(root/'cache.json'),'--binary','target/debug/oofft','--ddg-binary','target/debug/oofft-ddg','--threads','2','--chunk-queries','3','--output',str(root/'chunked')]
subprocess.run(command,check=True)
actual=[]
for chunk in json.load(open(root/'chunked/result.json'))['chunks']:
 for arc in chunk['archives']:
  raw=gzip.decompress(pathlib.Path(arc['path']).read_bytes());assert hashlib.sha256(raw).hexdigest()==arc['uncompressed_sha256']
  if 'annotated' in arc['path']:actual.extend(json.loads(l) for l in raw.splitlines() if json.loads(l)['type']=='site')
key=lambda r:tuple(r['site_id'])
assert sorted(actual,key=key)==sorted(sites,key=key)
print('CHUNKED PASS: all',len(actual),'site rows match monolithic pipeline; archives verified')
