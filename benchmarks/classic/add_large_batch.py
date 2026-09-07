#!/usr/bin/env python3
"""Extend the retained allele pool with unique, reproducible reference 20mers."""
import argparse
import csv
import hashlib
import json
from pathlib import Path

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--reference', type=Path, default=Path('data/ec2/SCN2A-reference-v1.fa'))
    parser.add_argument('--root', type=Path, default=Path('data/classic-v1'))
    args = parser.parse_args()
    lines = args.reference.read_text().splitlines()
    assert lines[0] == '>gene_body:ENSG00000136531|ENSG00000136531'
    assert sum(line.startswith('>') for line in lines) == 1
    sequence = ''.join(lines[1:])
    rc = lambda s: s.translate(str.maketrans('ACGT', 'TGCA'))[::-1]
    prefix = (args.root / 'queries-26643.fa').read_text().splitlines()
    targets = (args.root / 'targets-26643.fa').read_text().splitlines()
    assert len(prefix) == len(targets) == 26643 * 2
    rows = []
    seen = set()
    for i in range(26643):
        aso = prefix[2*i+1]
        assert prefix[2*i] == f'>q{i}|ENSG00000136531'
        assert targets[2*i] == f'>q{i}' and targets[2*i+1] == rc(aso)
        assert len(aso) == 20 and set(aso) <= set('ACGT') and aso not in seen
        seen.add(aso)
        rows.append((aso, 'retained_allele_pool', None))
    candidates = {}
    for start in range(len(sequence)-19):
        target = sequence[start:start+20]
        if set(target) <= set('ACGT'):
            candidates.setdefault(target, start)
    # Hash ordering avoids dependence on dictionary/set iteration or RNG version.
    for target in sorted(candidates, key=lambda s: (hashlib.sha256(s.encode()).digest(), s)):
        aso = rc(target)
        if aso in seen: continue
        seen.add(aso)
        rows.append((aso, 'reference_gene_body', candidates[target]))
        if len(rows) == 100000: break
    assert len(rows) == len(seen) == 100000
    outputs = {
        'queries-100000.fa': ''.join(f'>q{i}|ENSG00000136531\n{aso}\n' for i, (aso, _, _) in enumerate(rows)),
        'targets-100000.fa': ''.join(f'>q{i}\n{rc(aso)}\n' for i, (aso, _, _) in enumerate(rows)),
    }
    for name, content in outputs.items():
        path = args.root / name
        if path.exists(): assert path.read_text() == content
        else:
            with path.open('x') as f: f.write(content)
    metadata = args.root / 'queries-100000-origins.csv'
    if not metadata.exists():
        with metadata.open('x', newline='') as f:
            writer = csv.writer(f); writer.writerow(['query_id', 'origin', 'gene_body_start_0based'])
            writer.writerows((f'q{i}', origin, start) for i, (_, origin, start) in enumerate(rows))
    manifest = dict(queries=100000, unique_sequences=100000, retained_allele_queries=26643,
                    added_reference_queries=73357, intended_gene='ENSG00000136531',
                    reference_path=str(args.reference), reference_sha256=digest(args.reference),
                    prefix_sha256=digest(args.root/'queries-26643.fa'),
                    generator_sha256=digest(Path(__file__)),
                    selection='Existing allele prefix, then distinct valid reference 20mers ordered by SHA256(target); first occurrence supplies coordinates.',
                    scope='Mixed allele-derived and reference-derived SCN2A ASOs; not 100000 observed variants or experimentally validated therapeutics.',
                    files={name:digest(args.root/name) for name in [*outputs, metadata.name]})
    (args.root/'large-batch-manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
    print(json.dumps(manifest))

if __name__ == '__main__': main()
