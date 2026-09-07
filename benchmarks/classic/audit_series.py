#!/usr/bin/env python3
"""Audit planned screening coverage and shared timing/witness invariants."""
import json
from collections import defaultdict
from pathlib import Path

root = Path('benchmarks/classic/iterations')
groups = defaultdict(list)
for path in sorted(root.glob('*/screening.json')):
    row = json.loads(path.read_text())
    if row['phase'] != 'measurement':
        continue
    assert row['threads'] == 8, path
    stages = [json.loads(Path(p).read_text()) for p in row['stages']]
    assert abs(sum(s['wall_seconds'] for s in stages) - row['wall_seconds']) < 1e-6, path
    assert all(s['metadata']['threads'] == 8 for s in stages), path
    if row['complete']:
        assert all(s['returncode'] == 0 and not s['timeout'] for s in stages), path
        assert len(row['witnesses']) == row['recovered'], path
        for site in row['witnesses'].values():
            assert 0 <= site['distance'] <= 3 and 17 <= site['end'] - site['start'] <= 23, path
    row['artifact'] = str(path)
    row['timed_out'] = any(s['timeout'] for s in stages)
    failure_path = path.parent / 'resource_failure.json'
    row['resource_failure'] = None
    if failure_path.exists():
        failure = json.loads(failure_path.read_text())
        assert not row['complete'] and failure['kind'] == 'out_of_memory', path
        assert any(s['returncode'] == 137 and not s['timeout'] for s in stages), path
        assert any(f"Killed process {failure['pid']} ({row['tool']})" in line for line in failure['evidence']), path
        row['resource_failure'] = failure['kind']
    groups[row['tool'], row['queries']].append(row)

checks = []
for tool in ['ooff', 'bwa', 'blast', 'minimap2', 'sassy']:
    for count in [10, 100, 1000, 10000, 100000]:
        rows = groups[tool, count]
        complete = [r for r in rows if r['complete']]
        timeouts = [r for r in rows if r['timed_out']]
        if len(complete) == 3:
            assert {r['repetition'] for r in complete} == {1, 2, 3}, (tool, count)
            assert len({tuple(sorted(r['witnesses'])) for r in complete}) == 1, (tool, count)
            state = 'three completed repetitions'
        elif not complete and any(r['resource_failure'] for r in rows):
            state = 'kernel-confirmed out-of-memory failure; remaining repetitions not run'
        elif not complete and timeouts:
            state = 'recorded timeout; remaining repetitions skipped'
        else:
            state = 'pending or incomplete'
        if complete:
            native = groups['ooff', count]
            native = [r for r in native if r['complete']]
            assert native, 'Native witness reference missing'
            truth = set(native[0]['witnesses'])
            assert all(set(r['witnesses']) <= truth for r in complete), (tool, count)
        checks.append(dict(tool=tool, queries=count, state=state, artifacts=[r['artifact'] for r in rows]))

result = dict(complete=all(c['state'] != 'pending or incomplete' for c in checks), checks=checks,
              scope='Artifact coverage, elapsed-time sums, thread metadata and witness consistency; does not replace independent scalar verification or biological validation.')
Path('benchmarks/classic/series-audit.json').write_text(json.dumps(result, indent=2) + '\n')
for check in checks:
    if check['state'] == 'pending or incomplete':
        print(f"Pending: {check['tool']} / {check['queries']} ASOs")
print('Complete:', result['complete'])
raise SystemExit(0 if result['complete'] else 1)
