#!/usr/bin/env python3
"""Prepare shared eligible RNA records and nested allele-specific query batches."""
import hashlib,json
from pathlib import Path

def fasta(path):
    header=None;seq=[]
    with Path(path).open() as f:
        for line in f:
            if line.startswith('>'):
                if header is not None: yield header,''.join(seq)
                header=line[1:].strip();seq=[]
            else: seq.append(line.strip())
    if header is not None: yield header,''.join(seq)

def sha(path):
    with Path(path).open('rb') as f: return hashlib.file_digest(f,'sha256').hexdigest()

root=Path('data/classic-v1');root.mkdir(exist_ok=False)
reference=Path('data/reference-v1/reference.fa')
records=[];excluded=0;bases=0
with (root/'eligible.fa').open('wb') as out:
    for header,sequence in fasta(reference):
        if all(g=='ENSG00000136531' for g in header.split('|',1)[1].split(',')):
            excluded+=1;continue
        rid=f'r{len(records)}'
        out.write(f'>{rid}\n'.encode());offset=out.tell()
        sequence=sequence.upper().replace('U','T')
        out.write(sequence.encode()+b'\n')
        records.append(dict(id=rid,header=header,offset=offset,length=len(sequence)))
        bases+=len(sequence)
(root/'records.json').write_text(json.dumps(records)+'\n')
source=Path('data/cached-allele-queries-v1/scn2a-cached-alleles-all.fa')
queries=list(fasta(source));assert len(queries)==26643
trans=str.maketrans('ACGT','TGCA')
for n in [100,1000,5000,10000,26643]:
    with (root/f'queries-{n}.fa').open('x') as native, (root/f'targets-{n}.fa').open('x') as aligned:
        for i,(header,sequence) in enumerate(queries[:n]):
            assert len(sequence)==20 and set(sequence)<=set('ACGT')
            native.write(f'>q{i}|ENSG00000136531\n{sequence}\n')
            aligned.write(f'>q{i}\n{sequence.translate(trans)[::-1]}\n')
manifest=dict(reference_source=str(reference),reference_source_sha256=sha(reference),reference_sha256=sha(root/'eligible.fa'),reference_bases=bases,reference_records=len(records),excluded_intended_gene_records=excluded,scope='Ensembl110 GRCh38 RNA-sense gene bodies and mature transcripts; same other-gene eligibility as ooff',query_source=str(source),query_source_sha256=sha(source),query_counts=[100,1000,5000,10000,26643],query_sha256={p.name:sha(p) for p in root.glob('*-*.fa')})
(root/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print(json.dumps(manifest))
