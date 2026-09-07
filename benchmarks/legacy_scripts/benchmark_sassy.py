"""Small, reproducible Sassy2 smoke test and SCN2A genome benchmark.

Benchmark only: does not change the production off-target masks.
"""
import argparse
import csv
import json
import re
from pathlib import Path
import subprocess
import time

import numpy as np
import pysam

from analyses.logic import offtarget_filter as ot

ROOT = Path(__file__).resolve().parents[1]


def run(binary, patterns, reference, output, k, v2=True):
    cmd = [str(binary), 'search', '--pattern-fasta', str(patterns), '-k', str(k),
           '--max-n-frac', '0', '--threads', '4']
    if v2:
        cmd.append('--v2')
    cmd.append(str(reference))
    t = time.monotonic()
    with output.open('w') as f:
        subprocess.run(cmd, stdout=f, check=True)
    elapsed = time.monotonic() - t
    with output.open() as f:
        rows = list(csv.DictReader(f, delimiter='\t'))
    return elapsed, rows, cmd


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--binary', type=Path, default=Path('/tmp/sassy-v0.2.6'))
    ap.add_argument('--out', type=Path, default=ROOT/'data/exports/sassy_benchmark')
    ap.add_argument('--n', type=int, default=32)
    args = ap.parse_args()
    out = args.out
    out.mkdir(parents=True, exist_ok=True)
    seq = 'ACGTTGCAAGTCGATCGTAC'
    fixture = dict(exact=seq, lowercase=seq.lower(),
                   substitution=seq[:9]+'C'+seq[10:],
                   insertion=seq[:10]+'A'+seq[10:],
                   deletion=seq[:10]+seq[11:],
                   reverse=ot._aso_to_target_dna(seq), unknown='N'*20)
    fp = out/'fixture_patterns.fa'
    fp.write_text(''.join(f'>q{i}\n{seq}\n' for i in range(16)))
    ft = out/'fixture_reference.fa'
    ft.write_text(''.join(f'>{name}\n{s}\n' for name,s in fixture.items()))
    for v2 in (False, True):
        _, hits, _ = run(args.binary, fp, ft, out/f'fixture_v{2 if v2 else 1}.tsv', 1, v2)
        for q in range(16):
            for name, cost in [('exact',0),('lowercase',0),('substitution',1),
                               ('insertion',1),('deletion',1),('reverse',0)]:
                matching=[r for r in hits if r['pat_id']==f'q{q}' and r['text_id']==name]
                assert matching and min(int(r['cost']) for r in matching)==cost, (v2,name,matching)
            assert not any(r['pat_id']==f'q{q}' and r['text_id']=='unknown' for r in hits)
        assert any('I' in r['cigar'] for r in hits)
        assert any('D' in r['cigar'] for r in hits)
    print('Fixture passed: exact, substitution, insertion, deletion, lowercase, reverse, N exclusion; v1 and v2', flush=True)

    with (ROOT/'data/exports/gene_handoff/SCN2A_20mers.csv').open() as f:
        eligible=[r for r in csv.DictReader(f) if r['passes_design_filters']=='True']
    unique={r['aso_dna']:r for r in eligible}
    pool=sorted(unique.values(),key=lambda r:(int(r['pos']),int(r['aso_offset'])))
    chosen=[pool[i] for i in np.linspace(0,len(pool)-1,args.n,dtype=int)]
    queries=out/'SCN2A_patterns.fa'
    queries.write_text(''.join(f'>q{i}\n{r["aso_dna"]}\n' for i,r in enumerate(chosen)))
    fa=pysam.FastaFile(str(ot._REF))
    for r in chosen:
        s=fa.fetch(r['chrom'],int(r['target_site_start'])-1,int(r['target_site_end']))
        r['reference_softmasked_bases']=sum(c.islower() for c in s)
    with (out/'queries.csv').open('w',newline='') as f:
        w=csv.DictWriter(f,fieldnames=list(chosen[0]));w.writeheader();w.writerows(chosen)
    z=np.load(ROOT/'data/genes/SCN2A.npz')
    chrom=str(z['chrom']);lo=int(z['gene_start'])-ot.ON_TARGET_PAD;hi=int(z['gene_end'])+ot.ON_TARGET_PAD
    # Bounded pilot: a 10 Mb chromosome-2 region plus the SCN2A locus.
    # Split at unknown bases and bound records to avoid giant N-rich alignments.
    reference=out/'reference_10Mb_plus_SCN2A.fa'
    total_bases=0
    with reference.open('w') as f:
        for start,end in [(0,10_000_000),(lo-1,hi)]:
            sequence=fa.fetch(chrom,start,end)
            for match in re.finditer('[ACGTacgt]{18,}',sequence):
                for offset in range(match.start(),match.end(),100_000):
                    piece=sequence[offset:min(offset+100_022,match.end())]
                    if len(piece)<18:
                        continue
                    f.write(f'>{chrom}:{start+offset}\n{piece}\n')
                    total_bases+=len(piece)
    elapsed,hits,cmd=run(args.binary,queries,reference,out/'SCN2A_genome_k2.tsv',2)
    print(f'Bounded reference search: {args.n} queries, {elapsed:.2f}s, {len(hits)} reported alignments',flush=True)
    intervals=ot._build_transcribed_intervals(verbose=False)
    external=[]
    for r in hits:
        contig,offset=r['text_id'].split(':')
        r['start']=str(int(r['start'])+int(offset))
        r['end']=str(int(r['end'])+int(offset))
        r['text_id']=contig
        pos=int(r['start'])+1
        if r['text_id']==chrom and lo<=pos<=hi:
            continue
        if ot._in_transcribed(intervals,r['text_id'],pos,int(r['end'])-int(r['start'])):
            external.append(r)
    if external:
        with (out/'SCN2A_external_transcribed_hits.tsv').open('w',newline='') as f:
            w=csv.DictWriter(f,fieldnames=list(external[0]),delimiter='\t');w.writeheader();w.writerows(external)
    gapped=[r for r in external if 'I' in r['cigar'] or 'D' in r['cigar']]
    result=dict(binary=str(args.binary),mode='Sassy2 --v2',threads=4,queries=args.n,
                reference=str(reference),reference_bases=total_bases,seconds=elapsed,reported_alignments=len(hits),
                external_transcribed_alignments=len(external),
                queries_with_external_hits=len({r['pat_id'] for r in external}),
                gapped_external_alignments=len(gapped),
                queries_with_gapped_external_hits=len({r['pat_id'] for r in gapped}),
                ungapped_le1_external_alignments=sum(int(r['cost'])<=1 and 'I' not in r['cigar'] and 'D' not in r['cigar'] for r in external),
                sampled_queries_with_softmasked_overlap=sum(r['reference_softmasked_bases']>0 for r in chosen),
                command=cmd,notes='Evenly spaced unique passing SCN2A sequences, not a random sample. '
                'Raw reported alignments are not deduplicated biological sites. '
                'Both strands searched; transcribed-region classification matches existing pipeline. '
                'No production filters changed; no clinical safety inference.')
    (out/'summary.json').write_text(json.dumps(result,indent=2))
    print(json.dumps(result,indent=2),flush=True)


if __name__=='__main__':
    main()
