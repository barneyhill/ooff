#!/usr/bin/env python3
"""Compare retained native binary against branch-extension reuse, on all CPUs."""
import csv
import datetime
import json
import os
import shutil
import statistics
import sys
from pathlib import Path
from command import run
sys.path.insert(0, 'benchmarks')
from compare_site_files import compare

stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S')
root = Path('data/classic-stage') / ('extension-reuse-' + stamp)
root.mkdir()
shutil.copy2('target/release/ooff-index', root / 'baseline-binary')
for name in ['src/fm.rs', 'src/bin/ooff-index.rs', 'tests/fm.rs']:
    shutil.copy2(name, root / name.replace('/', '_'))
    shutil.copy2(Path('data/extension-stage') / name, name)
    Path(name).touch()
threads = len(os.sched_getaffinity(0))
for label, cmd in [
    ('tests', ['env', f'OMP_NUM_THREADS={threads}', '/home/ubuntu/.cargo/bin/cargo', 'test', '--locked', '--all-targets', '-j', str(threads)]),
    ('build', ['/home/ubuntu/.cargo/bin/cargo', 'build', '--release', '--locked', '--bin', 'ooff-index', '-j', str(threads)]),
    ('archive', ['tar', '-czf', str(root / 'source.tar.gz'), 'src', 'tests', 'Cargo.toml', 'Cargo.lock', 'benchmarks/classic/extension_experiment.py']),
]:
    _, result = run('extension-reuse-' + label, cmd, 600, dict(phase='optimization', threads=threads))
    assert result['returncode'] == 0, result

def command(variant, n, k=3):
    binary = str(root / 'baseline-binary') if variant == 'retained' else 'target/release/ooff-index'
    query = 'benchmarks/sassy_benchmark/SCN2A_patterns.fa' if n == 32 else 'benchmarks/sassy_1000/SCN2A_unfiltered_1000.fa'
    cmd = [binary, 'search', '--index', f'data/fm-classic-{threads}-v1', '--reverse-index', 'data/fm-reverse-v1', '--reference', 'data/reference-v1/reference.fa', '--queries', query, '--queries-limit', str(n), '--mode', 'sites', '--threads', str(threads), '--repetitions', '1', '--mmap', '-k', str(k)]
    if variant == 'reuse': cmd += ['--reuse-extensions']
    return cmd

checks = []
for k in range(4):
    for variant in ['retained', 'disabled', 'reuse']:
        path = root / f'{variant}-k{k}.csv'
        _, result = run(f'extension-gate-{variant}-k{k}', command(variant, 32, k) + ['--site-tuples', str(path)], 600, dict(phase='optimization_correctness', threads=threads))
        assert result['returncode'] == 0, result
        if variant != 'retained':
            check = compare(root / f'retained-k{k}.csv', path, root / f'buckets-{variant}-k{k}')
            checks.append(dict(variant=variant, k=k, **check))
            (root / 'correctness.json').write_text(json.dumps(checks, indent=2) + '\n')
            assert check['equal'] and check['unique'], check

rows = []
truth = None
orders = [['retained', 'disabled', 'reuse'], ['reuse', 'retained', 'disabled'], ['disabled', 'reuse', 'retained']]
for repetition in range(4):
    phase = 'warmup' if repetition == 0 else 'optimization'
    for variant in orders[repetition % 3]:
        directory, result = run(f'extension-{phase}-{variant}-r{repetition}', command(variant, 1000), 900, dict(phase=phase, tool='ooff', queries=1000, threads=threads, repetition=repetition))
        assert result['returncode'] == 0, result
        output = json.loads((directory / 'stdout').read_text()); assert output['complete']
        batch = output['runs'][0]
        signature = (batch['counts'], batch['signature'])
        if truth is None: truth = signature
        assert truth == signature
        rows.append(dict(variant=variant, phase=phase, repetition=repetition, threads=threads, search_seconds=batch['search_seconds'], wall_seconds=result['wall_seconds'], sites=sum(batch['counts']), artifact=str(directory)))
        with (root / 'results.csv').open('w', newline='') as f:
            writer = csv.DictWriter(f, fieldnames=rows[0]); writer.writeheader(); writer.writerows(rows)
medians = {v: statistics.median(r['search_seconds'] for r in rows if r['variant'] == v and r['phase'] == 'optimization') for v in orders[0]}
summary = dict(complete=True, rows=rows, checks=checks, medians=medians, retained_over_reuse=medians['retained']/medians['reuse'], disabled_over_reuse=medians['disabled']/medians['reuse'])
(root / 'result.json').write_text(json.dumps(summary, indent=2) + '\n')
text = '# Reusing exact index extensions\n\nEight CPUs available to every variant. Full 1,000-query warmup per variant is recorded separately. Subsequent runs rotate order.\n\n| Variant | Phase | Rep | Search seconds | Process seconds |\n| --- | --- | ---: | ---: | ---: |\n'
for r in rows: text += f"| {r['variant']} | {r['phase']} | {r['repetition']} | {r['search_seconds']:.6f} | {r['wall_seconds']:.6f} |\n"
text += f"\nRetained / reuse median ratio: {summary['retained_over_reuse']:.4f}. Disabled / reuse: {summary['disabled_over_reuse']:.4f}. Below 1 means regression.\nArtifacts: `{root}`.\n"
Path('benchmarks/EXTENSION_EXPERIMENT.md').write_text(text)
Path('results/extension-experiment.success').write_text(str(root) + '\n')
