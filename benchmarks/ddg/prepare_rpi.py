#!/usr/bin/env python3
"""Prepare reproducible real-reference workloads without changing the source data."""
import argparse,hashlib,json
from pathlib import Path
p=argparse.ArgumentParser()
p.add_argument('--reference',type=Path,default=Path('/home/barneyh/oligoai-v2/references/human_canonical_premrna.fa'))
a=p.parse_args()
root=Path('data/ddg-inputs');root.mkdir(parents=True,exist_ok=True)
selected_records={'ENST00000375437':'SCN2A','ENST00000674923':'SCN1A'}
records={};current=None
with a.reference.open() as source:
 for line in source:
  if line.startswith('>'):
   current=line[1:].split()[0]
   if current in selected_records:records[current]=[]
  elif current in selected_records:records[current].append(line.strip().upper())
assert set(records)==set(selected_records), 'Missing required canonical transcripts'
for transcript,name in selected_records.items():
 text='>'+transcript+'\n'+''.join(records[transcript])+'\n'
 path=root/(name+'.fa')
 if path.exists():assert path.read_text()==text, 'Existing input differs; preserve and inspect it'
 else:path.write_text(text)
def fasta(p):return ''.join(l.strip().upper() for l in p.read_text().splitlines() if not l.startswith('>'))
on=fasta(root/'SCN2A.fa');off=fasta(root/'SCN1A.fa');off_windows={off[i:i+20] for i in range(len(off)-19)}
unique={on[i:i+20] for i in range(len(on)-19) if 'N' not in on[i:i+20] and on[i:i+20] not in off_windows}
selected=sorted(unique,key=lambda s:hashlib.sha256(s.encode()).digest())[:128]
complement=str.maketrans('ACGT','TGCA')
for name,seqs in [('unique128',selected),('repeated128',selected[:16]*8),('exact1000',sorted(s for s in off_windows if 'N' not in s)[:1000])]:
 rows=[dict(id=f'q{i}',aso=s.translate(complement)[::-1],target=s) for i,s in enumerate(seqs)]
 (root/f'{name}.jsonl').write_text(''.join(json.dumps(r)+'\n' for r in rows))
manifest=dict(source_directory='/home/barneyh/oligoai-v2/references',on_target='SCN2A ENST00000375437',off_target='SCN1A ENST00000674923',on_target_bases=len(on),off_target_bases=len(off),selection='Distinct non-exact SCN2A 20mers sorted by SHA256; repeated workload cycles first 16 eight times; exact workload is first 1000 sorted distinct SCN1A 20mers.',sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in root.iterdir() if p.suffix in ['.fa','.jsonl']})
(root/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps(manifest,indent=2))
