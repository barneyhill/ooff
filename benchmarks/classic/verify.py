#!/usr/bin/env python3
"""Verify reported mapper intervals against source bases and full 20nt queries."""
import argparse,json,mmap,multiprocessing,re,time
from pathlib import Path

def distance(query,target):
    row=list(range(len(query)+1))
    for j,y in enumerate(target):
        nxt=[j+1]
        for i,x in enumerate(query): nxt.append(min(row[i]+(x!=y),row[i+1]+1,nxt[-1]+1))
        row=nxt
    return row[-1]

def span(cigar):
    parts=re.findall(r'(\d+)([MIDNSHP=X])',cigar)
    if ''.join(n+c for n,c in parts)!=cigar: return None
    if any(c in 'NH' for _,c in parts): return None
    return sum(int(n) for n,c in parts if c in 'MD=X')

def candidates(path,kind,root):
    if kind=='native':
        ids=json.loads((root/'native-record-map.json').read_text())
        output=json.loads(Path(path).read_text())
        assert output['complete'] and output['mode']=='screen' and output['k']==3
        run=output['runs'][0]
        assert len(run['counts'])==len(run['witness_intervals'])
        for q,(count,site) in enumerate(zip(run['counts'],run['witness_intervals'])):
            assert bool(count)==bool(site)
            if site:
                rid,a,b,d=site
                assert ids[rid] is not None,'Native returned an intended-gene-only record'
                yield f'q{q}',ids[rid],a,b,d
        return
    with Path(path).open() as f:
        for line in f:
            if kind=='blast':
                q,r,a,b=line.rstrip().split('\t');a=int(a);b=int(b)
                if a<=b:yield q,r,a-1,b,None
                continue
            if line.startswith('@'):continue
            fields=line.rstrip().split('\t')
            if len(fields)<11:raise ValueError('truncated SAM')
            q,flag,r,pos,_,cigar=fields[:6];flag=int(flag)
            if not flag&4 and not flag&16:
                n=span(cigar)
                if n is not None:yield q,r,int(pos)-1,int(pos)-1+n,None
            for tag in fields[11:]:
                if tag.startswith('XA:Z:'):
                    for hit in tag[5:].strip(';').split(';'):
                        if not hit:continue
                        r,pos,cigar,_=hit.split(',')
                        if not pos.startswith('+'):continue
                        n=span(cigar)
                        if n is not None:yield q,r,int(pos)-1,int(pos)-1+n,None

_records = _patterns = _reference = None

def verify_chunk(candidates):
    found={};rejected=0
    for q,r,a,b,reported_distance in candidates:
        if q in found:continue
        if r.startswith('lcl|'):r=r[4:]
        encoded=re.fullmatch(r'(?:gb|emb|dbj|ref)\|(r[0-9]+)\|',r)
        if encoded:r=encoded.group(1)
        record=_records[r]
        if not(0<=a<b<=record['length']) or not 17<=b-a<=23:rejected+=1;continue
        target=_reference[record['offset']+a:record['offset']+b].decode()
        if set(target)-set('ACGT'):rejected+=1;continue
        d=distance(_patterns[q],target)
        if reported_distance is not None:assert d==reported_distance,'Native witness distance mismatch'
        if d<=3:found[q]=dict(record=r,start=a,end=b,distance=d)
        else:rejected+=1
    return found,rejected

def verify(path,kind,queries,root,threads=1):
    global _records,_patterns,_reference
    assert threads>0
    started=time.perf_counter()
    _records={r['id']:r for r in json.loads((root/'records.json').read_text())}
    _patterns={};current=None
    for line in Path(queries).read_text().splitlines():
        if line.startswith('>'):current=line[1:].split()[0];_patterns[current]=''
        else:_patterns[current]+=line.strip()
    found={};seen=0;rejected=0
    workers=min(threads,len(_patterns));assert workers>0
    chunk_size=max(1,min(256,(len(_patterns)+workers-1)//workers))
    with (root/'eligible.fa').open('rb') as f, mmap.mmap(f.fileno(),0,access=mmap.ACCESS_READ) as reference:
        _reference=reference
        # Fork shares the immutable reference and metadata; bounded waves prevent
        # eager task submission from retaining arbitrarily large SAM output.
        pool=multiprocessing.get_context('fork').Pool(workers) if workers>1 else None
        try:
            source=iter(candidates(path,kind,root));exhausted=False
            while not exhausted:
                chunks=[]
                for _ in range(workers):
                    chunk=[]
                    while len(chunk)<chunk_size:
                        try:row=next(source)
                        except StopIteration:exhausted=True;break
                        seen+=1
                        q=row[0]
                        if q not in _patterns:raise ValueError(f'unknown query {q}')
                        if q not in found:chunk.append(row)
                    if chunk:chunks.append(chunk)
                    if exhausted:break
                results=pool.map(verify_chunk,chunks,chunksize=1) if pool else map(verify_chunk,chunks)
                for witnesses,bad in results:
                    rejected+=bad
                    for q,site in witnesses.items():found.setdefault(q,site)
        finally:
            if pool:pool.close();pool.join()
    return dict(queries=len(_patterns),recovered=len(found),witnesses=found,candidate_rows=seen,rejected_before_witness=rejected,verification_seconds=time.perf_counter()-started,verification_workers=workers)

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--input',type=Path,required=True);p.add_argument('--kind',choices=['sam','blast','native'],required=True);p.add_argument('--queries',type=Path,required=True);p.add_argument('--threads',type=int,default=1);p.add_argument('--root',type=Path,default=Path('data/classic-v1'));a=p.parse_args()
    print(json.dumps(verify(a.input,a.kind,a.queries,a.root,a.threads)))
