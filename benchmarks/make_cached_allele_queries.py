#!/usr/bin/env python3
"""Build variant-associated 20mers from a historical allele-level gene cache.

Only variant-level arrays are decoded; no sample IDs or haplotypes are exported. No additional eligibility,
frequency, position, or thermodynamic filter is applied. This reconstructs a
benchmark pool, not the later handoff's unavailable 6,338-ASO eligible export.
"""
import argparse
import collections
import hashlib
import json
from pathlib import Path
import numpy as np

ap = argparse.ArgumentParser(description=__doc__)
ap.add_argument('--cache', type=Path, required=True)
ap.add_argument('--gene-fasta', type=Path, required=True)
ap.add_argument('--out', type=Path, required=True)
args = ap.parse_args()
data = np.load(args.cache, allow_pickle=False)
assert str(data['chrom']) == '2'
gene_start, gene_end = int(data['gene_start']), int(data['gene_end'])
sequence = ''.join(line.strip() for line in args.gene_fasta.read_text().splitlines()
                   if not line.startswith('>')).upper()
assert len(sequence) == gene_end - gene_start + 1
arrays = {key: data[key] for key in ['pos', 'ref', 'alt', 'other', 'id']}
queries, types, skipped = {}, collections.Counter(), []
complement = str.maketrans('ACGT', 'TGCA')
for i, pos in enumerate(arrays['pos']):
    ref, target, spared = (str(arrays[k][i]).upper() for k in ['ref', 'alt', 'other'])
    vid = str(arrays['id'][i])
    offset = int(pos) - gene_start
    if not ref or not target or not set(ref + target + spared) <= set('ACGT'):
        skipped.append(dict(variant_id=vid, reason='non-ACGT or empty allele'))
        continue
    assert 0 <= offset and offset + len(ref) <= len(sequence), ('outside gene', vid)
    assert sequence[offset:offset+len(ref)] == ref, ('reference mismatch', vid)
    types['SNP' if len(ref) == len(target) == len(spared) == 1 else 'indel' if len({len(ref), len(target), len(spared)}) > 1 else 'MNV'] += 1
    left = sequence[max(0, offset-19):offset]
    right = sequence[offset+len(ref):offset+len(ref)+19]
    context = left + target + right
    allele_start, allele_end = len(left), len(left) + len(target)
    for start in range(max(0, allele_start-19), min(allele_end, len(context)-19)):
        tile = context[start:start+20]
        assert len(tile) == 20
        if not set(tile) <= set('ACGT'):
            skipped.append(dict(variant_id=vid, reason='ambiguous reference window', window_offset=start-allele_start))
            continue
        aso = tile.translate(complement)[::-1]
        queries.setdefault(aso, []).append(dict(variant_id=vid, position=int(pos), ref=ref,
            target=target, spared=spared, window_offset=start-allele_start))
rows = [dict(id=f'scn2a-cache-{i:06}', sequence=aso, intended_genes=['ENSG00000136531'],
             allele=json.dumps(associations, separators=(',', ':')))
        for i, (aso, associations) in enumerate(queries.items())]
assert len(rows) >= 10000
args.out.mkdir(parents=True, exist_ok=False)
outputs = {}
for count in [1000, 10000, len(rows)]:
    label = 'all' if count == len(rows) else str(count)
    selected = [rows[i * (len(rows)-1) // (count-1)] for i in range(count)]
    name = f'scn2a-cached-alleles-{label}'
    fasta = args.out / (name + '.fa')
    fasta.write_text(''.join(f'>{r["id"]}|ENSG00000136531\n{r["sequence"]}\n' for r in selected))
    (args.out / (name + '.jsonl')).write_text(''.join(json.dumps(r) + '\n' for r in selected))
    outputs[name] = dict(queries=count, fasta_sha256=hashlib.sha256(fasta.read_bytes()).hexdigest())
manifest = dict(source_cache=str(args.cache), cache_sha256=hashlib.sha256(args.cache.read_bytes()).hexdigest(),
    gene_fasta_sha256=hashlib.sha256(args.gene_fasta.read_bytes()).hexdigest(),
    source_variant_targets=len(arrays['pos']), variant_types=dict(types), skipped=skipped,
    gene_start_1based=gene_start, gene_end_1based_inclusive=gene_end, chromosome='2', strand='+',
    unique_asos=len(rows), variant_window_associations=sum(map(len, queries.values())), outputs=outputs,
    policy='every ACGT 20mer overlapping the substituted target-allele token; SNPs and indels; no added ddG/position/frequency filter; repeated sequences coalesced with all variant associations retained',
    source_scope='historical local paper3 gene cache, built at min_af=0.01 with nearby-variant filtering and no ddG filter; not the later 6338-ASO eligible export',
    interpretation='variant-associated benchmark queries; no claim of spared-allele discrimination or eligibility')
(args.out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
print(json.dumps(manifest, indent=2))
