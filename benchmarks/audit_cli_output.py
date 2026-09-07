#!/usr/bin/env python3
"""Compare annotated CLI intervals to a recorded paired matcher comparison."""
import argparse
from collections import Counter
import json
from pathlib import Path

ap = argparse.ArgumentParser(description=__doc__)
ap.add_argument('--cli', required=True, type=Path)
ap.add_argument('--comparison', required=True, type=Path)
ap.add_argument('--index', required=True, type=Path)
args = ap.parse_args()
comparison = json.loads(args.comparison.read_text())
assert comparison['complete'] and comparison['matching_counts_and_signatures']
expected = comparison['runs']['ooff']['output']
assert expected['mode'] == 'sites'
manifest = json.loads((args.index / 'manifest.json').read_text())
record_ids = {r['header'].split('|')[0]: r['global_id']
              for shard in manifest['shards'] for r in shard['records']}
counts, signatures = Counter(), Counter()
unique = set()
query_numbers = {q.split('|')[0]: i for i, q in enumerate(expected['query_ids'])}
site_tuples = set()
complete = None
for line in (args.cli / 'output.jsonl').open():
    row = json.loads(line)
    if row['type'] == 'run_complete':
        complete = row
    if row['type'] != 'site':
        continue
    q = row['query_id']
    site = row['alignment']
    a, b, d = site['start'], site['end'], site['edit_distance']
    rid = record_ids[row['record_id']]
    key = q, rid, a, b
    assert key not in unique, ('duplicate', key)
    unique.add(key)
    site_tuples.add((query_numbers[q], rid, a, b, d))
    assert row['offtarget_genes'] and sum(x['end']-x['start'] for x in row['genomic_blocks']) == b-a
    counts[q] += 1
    signatures[q] = (signatures[q] + ((rid << 40) ^ (a << 8) ^ ((b-a) << 3) ^ d)) % (1 << 64)
assert complete and complete['reported_sites'] == len(unique)
for output in [r['output'] for r in comparison['runs'].values()]:
    for run in output['runs']:
        for i, q in enumerate(output['query_ids']):
            # FASTA benchmark headers may include intended-gene suffixes.
            q = q.split('|')[0]
            assert counts[q] == run['counts'][i], ('count', q)
            assert signatures[q] == run['signature'][i], ('signature', q)
result = dict(complete=True, sites=len(unique), unique_sites=True,
              matching_per_query_counts_and_signatures=True,
              genomic_block_lengths_valid=True, comparison=str(args.comparison),
              limitation='additive signatures supplement fixture oracles; they are not full site-set proofs')
tuple_file = args.comparison.parent / 'sassy.sites.csv'
if tuple_file.exists():
    expected_tuples = set()
    for line in tuple_file.open():
        repetition, *site = map(int, line.strip().split(','))
        if repetition == 0:
            expected_tuples.add(tuple(site))
    assert site_tuples == expected_tuples, 'annotated CLI site set differs from Sassy'
    result['exact_site_tuples_equal_to_sassy'] = True
    result['limitation'] = 'full tuple equality verifies this dataset and edit budget; fixture oracles cover additional edge cases'
(args.cli / 'audit.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result))
