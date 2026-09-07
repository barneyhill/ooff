#!/usr/bin/env python3
"""Deterministic held-out/stress sets; preserve input/selection provenance."""
import hashlib
import itertools
import json
from pathlib import Path
import random

out=Path("data/query-sets-v1")
out.mkdir(exist_ok=False)
reference=Path("data/reference-v1/reference.fa")
candidates=[]
scn2a=None
with reference.open() as f:
    while header:=f.readline().strip():
        seq=f.readline().strip().upper()
        if not header.startswith(">gene_body:"):continue
        gene=header.split('|')[1]
        if gene=="ENSG00000136531":scn2a=seq
        if len(seq)<20:continue
        offset=int.from_bytes(hashlib.sha256(gene.encode()).digest()[:8],"little")%(len(seq)-19)
        for attempt in range(min(len(seq)-19,1000)):
            a=(offset+attempt)%(len(seq)-19)
            tile=seq[a:a+20]
            if set(tile)<=set("ACGT"):
                candidates.append((gene,a,tile));break
assert scn2a is not None
rc=lambda s:s.translate(str.maketrans("ACGT","TGCA"))[::-1]
starts=[i for i in range(len(scn2a)-19) if set(scn2a[i:i+20])<=set("ACGT")]
unique={}
for a in starts:unique.setdefault(scn2a[a:a+20],a)
tiles=list(unique.items())
sets={}
sets["scn2a-10000"]=[(f"scn2a-{a}|ENSG00000136531",rc(tile)) for tile,a in [tiles[i*(len(tiles)-1)//9999] for i in range(10000)]]
sets["mixed-genes-1000"]=[(f"{g}-{a}|{g}",rc(tile)) for g,a,tile in [candidates[i*(len(candidates)-1)//999] for i in range(1000)]]
rng=random.Random(20260907)
sets["random-1000"]=[(f"random-{i}|no-intended-gene","".join(rng.choice("ACGT") for _ in range(20))) for i in range(1000)]
stress=[]
for edits in range(4):
    for positions in itertools.combinations(range(20),edits):
        for bases in itertools.product("CGT",repeat=edits):
            seq=list("A"*20)
            for p,b in zip(positions,bases):seq[p]=b
            stress.append((f"stress-{len(stress)}|no-intended-gene","".join(seq)))
            if len(stress)==1000:break
        if len(stress)==1000:break
    if len(stress)==1000:break
sets["low-complexity-1000"]=stress
manifest={"reference":"data/reference-v1/reference.fa","seed":20260907,"sets":{}}
for name,queries in sets.items():
    path=out/(name+".fa")
    path.write_text("".join(f">{name}\n{seq}\n" for name,seq in queries))
    manifest["sets"][name]={"queries":len(queries),"unique_sequences":len({q[1] for q in queries}),"sha256":hashlib.sha256(path.read_bytes()).hexdigest()}
(out/"manifest.json").write_text(json.dumps(manifest,indent=2)+"\n")
print(json.dumps(manifest),flush=True)
