"""Measure Sassy2 on 1,000 unfiltered SCN2A tiles and merged transcribed loci.

Runs only a benchmark; does not change candidate eligibility or production data.
"""
import argparse
import csv
import json
from pathlib import Path
import re
import resource
import shutil
import subprocess
import time

import numpy as np
import pysam

from analyses.logic import offtarget_filter as ot
from analyses.logic.helpers import _rev_comp_rna

ROOT = Path(__file__).resolve().parents[1]
REFERENCE_LIMIT = 2_000_000_000
HITS_LIMIT = 256 * 1024 * 1024


def limit_output():
    resource.setrlimit(resource.RLIMIT_FSIZE, (HITS_LIMIT, HITS_LIMIT))


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--k', type=int, nargs='+', default=[2, 3])
    ap.add_argument('--n', type=int, default=1000)
    ap.add_argument('--threads', type=int, default=4)
    args = ap.parse_args()
    out = ROOT/'data/exports/sassy_1000'
    out.mkdir(parents=True, exist_ok=True)
    if shutil.disk_usage(out).free < REFERENCE_LIMIT + len(args.k)*HITS_LIMIT + 4_000_000_000:
        raise RuntimeError('Insufficient disk reserve for bounded benchmark')
    fa = pysam.FastaFile(str(ot._REF))
    z = np.load(ROOT/'data/genes/SCN2A.npz')
    chrom = str(z['chrom']); gs = int(z['gene_start']); ge = int(z['gene_end'])
    seq = fa.fetch(chrom, gs-1, ge)
    starts = [i for i in range(len(seq)-19) if set(seq[i:i+20].upper()) <= set('ACGT')]
    chosen = [starts[i] for i in np.linspace(0, len(starts)-1, args.n, dtype=int)]
    patterns = out/'SCN2A_unfiltered_1000.fa'
    patterns.write_text(''.join(f'>q{i}\n{_rev_comp_rna(seq[s:s+20]).replace("U", "T")}\n'
                                for i,s in enumerate(chosen)))
    prep_start = time.monotonic()
    reference = out/'transcribed_loci.fa'
    meta_path = out/'reference.json'
    if not reference.exists() or not meta_path.exists():
        intervals = ot._build_transcribed_intervals(verbose=False)
        bases = records = softmasked = unknown = 0
        with reference.with_suffix('.tmp').open('w') as f:
            for c, (ss, ee) in intervals.items():
                if c not in fa.references:
                    raise ValueError(f'Contig absent from genome: {c}')
                for a,b in zip(ss,ee):
                    text = fa.fetch(c, int(a)-1, int(b))
                    unknown += sum(x.upper() not in 'ACGT' for x in text)
                    # Preserve lowercase bases. Unknown bases split records.
                    # 23-bp overlap covers 20mer +/- 3 edits at chunk boundaries.
                    for m in re.finditer('[ACGTacgt]{17,}', text):
                        for start in range(m.start(), m.end(), 100_000):
                            end = min(start+100_023, m.end())
                            piece = text[start:end]
                            if len(piece) < 17:
                                continue
                            f.write(f'>{c}:{int(a)-1+start}\n{piece}\n')
                            if f.tell() > REFERENCE_LIMIT:
                                raise RuntimeError('Reference exceeded 2 GB storage budget')
                            bases += len(piece)
                            softmasked += sum(x.islower() for x in piece)
                            records += 1
        reference.with_suffix('.tmp').replace(reference)
        meta = dict(bases_including_chunk_overlap=bases, records=records,
                    softmasked_bases=softmasked, unknown_bases_excluded=unknown,
                    preparation_seconds=time.monotonic()-prep_start,
                    annotation='Ensembl 110 transcribed gene-body union; same biotype policy as paper3',
                    genome=str(ot._REF))
        meta_path.write_text(json.dumps(meta,indent=2))
    else:
        meta = json.loads(meta_path.read_text())
    print('Reference: '+json.dumps(meta),flush=True)
    results=[]
    for k in args.k:
        cmd=['/tmp/sassy-v0.2.6','search','--v2','--pattern-fasta',str(patterns),
             '-k',str(k),'--max-n-frac','0','--threads',str(args.threads),str(reference)]
        t=time.monotonic()
        count=gapped=off=0
        hit_queries=set();off_queries=set()
        per_query={}
        print(f'Starting k={k}: streaming hit counts, no large output file',flush=True)
        with subprocess.Popen(cmd,stdout=subprocess.PIPE,text=True,bufsize=1024*1024) as process:
            for r in csv.DictReader(process.stdout,delimiter='\t'):
                count+=1
                gapped+=int('I' in r['cigar'] or 'D' in r['cigar'])
                hit_queries.add(r['pat_id'])
                per_query[r['pat_id']]=per_query.get(r['pat_id'],0)+1
                if count % 1_000_000 == 0:
                    print(f'k={k}: {count:,} alignments streamed in {time.monotonic()-t:.1f}s',flush=True)
                c,offset=r['text_id'].split(':')
                pos=int(offset)+int(r['start'])+1
                if c==chrom and gs-20<=pos<=ge+20:
                    continue
                off+=1;off_queries.add(r['pat_id'])
            if process.wait()!=0:
                raise subprocess.CalledProcessError(process.returncode,cmd)
        elapsed=time.monotonic()-t
        result=dict(queries=args.n,unique_queries=len({_rev_comp_rna(seq[s:s+20]) for s in chosen}),
                    query_set='Evenly spaced reference SCN2A 20mers, no off-target prefilter; not allele-specific designs',
                    queries_overlapping_softmask=sum(any(x.islower() for x in seq[s:s+20]) for s in chosen),
                    k=k,threads=args.threads,seconds=elapsed,reported_alignments=count,
                    gapped_alignments=gapped,off_locus_alignments=off,
                    queries_with_hits=len(hit_queries),queries_with_off_locus_hits=len(off_queries),
                    per_query_reported_alignments=per_query,command=cmd,
                    caveat='Raw local-minimum alignments, not deduplicated loci. Both strands searched. '
                    'No spliced junctions. Lowercase retained; ambiguous bases excluded. '
                    'Search time includes process startup, reference read, alignment, streamed TSV and Python aggregation.')
        results.append(result)
        (out/'timings.json').write_text(json.dumps(dict(reference=meta,runs=results),indent=2))
        print(json.dumps(result),flush=True)


if __name__=='__main__':
    main()
