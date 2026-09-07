#!/usr/bin/env python3
"""Controlled native ablation: exact tuples first, alternating timed runs second."""
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


def main():
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S')
    root = Path('data/classic-stage') / ('word-cache-' + stamp)
    root.mkdir()
    shutil.copy2('target/release/ooff-index', root / 'baseline-binary')
    for name in ['src/fm.rs', 'src/lib.rs', 'src/bin/ooff-index.rs', 'tests/fm.rs', 'tests/interval_distance.rs']:
        destination = Path(name)
        shutil.copy2(destination, root / name.replace('/', '_'))
        shutil.copy2(Path('data/word-cache-stage') / name, destination)
        destination.touch()
    threads = len(os.sched_getaffinity(0))
    for label, cmd in [
        ('tests', ['env', f'OMP_NUM_THREADS={threads}', '/home/ubuntu/.cargo/bin/cargo', 'test', '--locked', '--all-targets', '-j', str(threads)]),
        ('build', ['/home/ubuntu/.cargo/bin/cargo', 'build', '--release', '--locked', '--bin', 'ooff-index', '-j', str(threads)]),
        ('archive', ['tar', '-czf', str(root / 'source.tar.gz'), 'src', 'tests', 'Cargo.toml', 'Cargo.lock', 'benchmarks/classic/word_cache_experiment.py']),
    ]:
        _, result = run('word-cache-' + label, cmd, 600, dict(phase='optimization', threads=threads))
        assert result['returncode'] == 0, result
    base = ['target/release/ooff-index', 'search', '--index', f'data/fm-classic-{threads}-v1', '--reverse-index', 'data/fm-reverse-v1', '--reference', 'data/reference-v1/reference.fa', '--mmap', '--mode', 'sites', '--threads', '1', '--repetitions', '1']
    checks = []
    for k in range(4):
        outputs = {}
        for cached in [False, True]:
            name = 'cached' if cached else 'baseline'
            output = root / f'{name}-k{k}.csv'
            cmd = base + ['--queries', 'benchmarks/sassy_benchmark/SCN2A_patterns.fa', '--queries-limit', '32', '-k', str(k), '--site-tuples', str(output)]
            if cached:
                cmd += ['--word-cache']
            _, result = run(f'word-cache-gate-{name}-k{k}', cmd, 600, dict(phase='optimization_correctness', tool='ooff', queries=32, threads=1))
            assert result['returncode'] == 0, result
            outputs[name] = output
        check = compare(outputs['baseline'], outputs['cached'], root / f'buckets-k{k}')
        checks.append(dict(k=k, **check))
        (root / 'correctness.json').write_text(json.dumps(checks, indent=2) + '\n')
        assert check['equal'] and check['unique'], check
    rows = []
    truth = None
    for rep in range(1, 4):
        for cached in ([False, True] if rep % 2 else [True, False]):
            name = 'cached' if cached else 'baseline'
            cmd = base + ['--queries', 'benchmarks/sassy_1000/SCN2A_unfiltered_1000.fa', '--queries-limit', '1000', '-k', '3']
            if cached:
                cmd += ['--word-cache']
            directory, result = run(f'word-cache-1000-{name}-r{rep}', cmd, 900, dict(phase='optimization', tool='ooff', queries=1000, repetition=rep, threads=1))
            assert result['returncode'] == 0, result
            output = json.loads((directory / 'stdout').read_text())
            assert output['complete'] and output['word_cache'] == cached
            batch = output['runs'][0]
            signature = (batch['counts'], batch['signature'])
            if truth is None:
                truth = signature
            assert signature == truth, 'Full human 1000-query counts/signatures differ'
            rows.append(dict(variant=name, repetition=rep, search_seconds=batch['search_seconds'], wall_seconds=result['wall_seconds'], sites=sum(batch['counts']), artifact=str(directory)))
            with (root / 'results.csv').open('w', newline='') as f:
                writer = csv.DictWriter(f, fieldnames=rows[0]); writer.writeheader(); writer.writerows(rows)
    medians = {v: statistics.median(r['search_seconds'] for r in rows if r['variant'] == v) for v in ['baseline', 'cached']}
    summary = dict(complete=True, checks=checks, rows=rows, medians=medians, speedup=medians['baseline'] / medians['cached'], scope='Native ablation, one search thread on 8-vCPU/16-GiB host; 32-query exact tuples, 1000-query count/signature checks; fresh processes, OS cache retained')
    (root / 'result.json').write_text(json.dumps(summary, indent=2) + '\n')
    text = '# Exact 10-mer cache experiment\n\n' + summary['scope'] + '.\n\n| Variant | Rep | Search seconds | Process seconds | Sites |\n| --- | ---: | ---: | ---: | ---: |\n'
    for r in rows:
        text += f"| {r['variant']} | {r['repetition']} | {r['search_seconds']:.6f} | {r['wall_seconds']:.6f} | {r['sites']} |\n"
    text += f"\nMedian baseline/cached speed ratio: {summary['speedup']:.4f}. Values below 1 are regressions.\nArtifacts: `{root}`.\n"
    Path('benchmarks/WORD_CACHE_EXPERIMENT.md').write_text(text)
    Path('results/word-cache-experiment.success').write_text(str(root) + '\n')


if __name__ == '__main__':
    main()
