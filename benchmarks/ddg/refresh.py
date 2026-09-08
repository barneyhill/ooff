#!/usr/bin/env python3
"""Export every retained attempt, including failures, for plotting and audit."""
import csv
import json
import statistics
from pathlib import Path

root = Path('benchmarks/ddg')
rows = []
lines = ['# ΔΔG iterations — Raspberry Pi and EC2', '',
         'Times are whole-process seconds unless the scope column says kernel_compute; transcript runs use the actual OligoAI v2 oracle, '
         'and site runs use direct RNAduplex. Synthetic estimate-sites workloads '
         'are throughput calibrations, not genome discovery benchmarks. '
         'Transcript mode uses four RNAplex workers plus one concurrent RNAduplex '
         'process unless stated; the worker column gives each site/kernel run’s worker count. '
         'Speedups are reported only for iterations that passed every output-field comparison.', '',
         '| Iteration | Workers | Scope | Status | Baseline | Baseline median s | oofft median s | Speedup |',
         '| --- | ---: | --- | --- | --- | ---: | ---: | ---: |']
for path in sorted(root.glob('iterations/*/result.json')):
    record = json.loads(path.read_text())
    passed = record.get('complete', False) and record.get('all_fields_match', False)
    times = {}
    case_count = record.get('cases', '')
    if not case_count:
        for attempt in record['attempts']:
            raw = path.parent/f"{attempt.get('engine', attempt.get('stage', 'unknown'))}-{attempt.get('repetition', 0)}.stdout"
            if attempt.get('measurement_kind') == 'kernel_compute' and raw.exists():
                try:
                    case_count = json.loads(raw.read_text()).get('cases', '')
                except (ValueError, OSError):
                    continue
                if case_count:
                    break
    for attempt in record['attempts']:
        rows.append(dict(iteration=path.parent.name, threads=record['threads'],
                         platform=record.get('platform','Raspberry Pi 5'),
                         queries=record.get('queries',''), cases=case_count, sites=record.get('sites',record.get('annotated_sites',record.get('reported_sites',''))),
                         native_pairs=attempt.get('profile',{}).get('native_pairs',''),
                         native_unique_pairs=attempt.get('profile',{}).get('native_unique_pairs',''),
                         workload=record.get('workload', record.get('label', '')),
                         measurement_kind=attempt.get('measurement_kind', 'whole_process'),
                         verified=passed, engine=attempt.get('engine', attempt.get('stage', 'unknown')),
                         repetition=attempt.get('repetition', 0), seconds=attempt['seconds'],
                         returncode=attempt['returncode'], artifact=str(path)))
        if attempt['returncode'] == 0:
            times.setdefault(attempt.get('engine', attempt.get('stage', 'unknown')), []).append(attempt['seconds'])
    baseline_name = next((name for name in ('oligoai', 'vienna-baseline', 'vienna-library', 'rust-baseline', 'native-baseline', 'baseline', 'viennarna') if name in times), None)
    baseline = statistics.median(times[baseline_name]) if baseline_name else None
    scope = ', '.join(sorted({a.get('measurement_kind', 'whole_process') for a in record['attempts']}))
    native_name = next((name for name in ('ooff', 'rust-kernel') if name in times), None)
    native = statistics.median(times[native_name]) if native_name else None
    fmt = lambda x: f'{x:.6f}' if x is not None else '—'
    ratio = f'{baseline/native:.2f}×' if passed and baseline and native else '—'
    status = 'PASS' if passed else ('COMPLETE (not energy-verified)' if record.get('complete') else 'INTERRUPTED' if record.get('interrupted') else 'RUNNING' if 'chunks' in record and not record.get('error') else 'FAILED')
    lines.append(f'| [{path.parent.name}]({path.parent.name}/result.json) | '
                 f'{record["threads"]} | {scope} | {status} | {baseline_name or "—"} | '
                 f'{fmt(baseline)} | {fmt(native)} | {ratio} |')
# Links are relative to the ledger's directory.
lines = [line.replace('](', '](iterations/') if line.startswith('| [') else line for line in lines]
lines += ['', 'The first fixture attempt failed because the parser read a structure parenthesis '
          'as the energy delimiter. Its raw output is retained. The corrected parser reads '
          'the final parenthesized energy.', '',
          'Real workloads: 128 distinct non-exact SCN2A designs; 128 rows repeating '
          '16 designs; and 1,000 exact SCN1A matches (the application shortcut). '
          'These are distinct performance scenarios, not interchangeable speedup claims.', '']
(root/'ITERATIONS.md').write_text('\n'.join(lines))
with (root/'iterations.csv').open('w', newline='') as stream:
    writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
    writer.writeheader()
    writer.writerows(rows)
