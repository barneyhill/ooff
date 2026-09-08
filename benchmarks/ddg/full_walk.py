#!/usr/bin/env python3
"""Time the real report -> ddG CLI workflow for every eligible gene-body 20mer.

Eight single-thread report processes share immutable indexes through mmap; their
reports are merged, then one ddG process uses all eight threads and its global
cache. All site rows are retained. No per-query hit cap. Index builds excluded.
"""
import argparse, concurrent.futures, datetime, hashlib, json, os, subprocess, time
from pathlib import Path


def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda: f.read(1024 * 1024), b''): h.update(b)
    return h.hexdigest()


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--gene', type=Path, required=True)
    p.add_argument('--reference-root', type=Path, required=True)
    p.add_argument('--index', type=Path, required=True)
    p.add_argument('--reverse-index', type=Path, required=True)
    p.add_argument('--annotation-cache', type=Path, required=True)
    p.add_argument('--binary', type=Path, default=Path('target/release/oofft'))
    p.add_argument('--ddg-binary', type=Path, default=Path('target/release/oofft-ddg'))
    p.add_argument('--threads', type=int, default=8)
    p.add_argument('--timeout', type=float, default=7200)
    p.add_argument('--include-masked', action='store_true')
    p.add_argument('--output', type=Path, required=True)
    a = p.parse_args()
    assert a.threads > 0 and a.timeout > 0
    root = a.output.resolve(); root.mkdir(parents=True, exist_ok=False)
    record = dict(label=root.name, complete=False, threads=a.threads,
                  scope='Full SCN2A gene-body 20mer walk; other-gene sites, k<=3; Ensembl110 human gene-body/transcript reference including repeats',
                  include_masked=a.include_masked, index_build_included=False,
                  output_policy='retain every JSONL site with alignment/coordinates and ddG; no cap', attempts=[])
    def save():
        (root/'result.json').write_text(json.dumps(record, indent=2)+'\n')
        with (root/'iterations.md').open('w') as out:
            out.write('# Full SCN2A walk measurement\n\n')
            out.write(f"Complete: {record['complete']}. Threads: {a.threads}. Include masked design windows: {a.include_masked}.\n\n")
            out.write('| Stage | Seconds | Exit code |\n|---|---:|---:|\n')
            for r in record['attempts']:
                out.write(f"| {r['stage']} | {r['seconds']:.6f} | {r['returncode']} |\n")
            out.write('\nCommands and input hashes: `result.json`. No index-build time included.\n')
    save()
    start = time.perf_counter()
    deadline = start + a.timeout
    try:
        lines = a.gene.read_text().splitlines()
        assert sum(l.startswith('>') for l in lines) == 1
        sequence = ''.join(l.strip() for l in lines if not l.startswith('>'))
        windows = {}
        for i in range(len(sequence)-19):
            raw = sequence[i:i+20]
            target = raw.upper() if a.include_masked else raw
            if set(target) <= set('ACGT'): windows.setdefault(target, []).append(i)
        queries = []
        with (root/'origins.jsonl').open('w') as origins:
            for i, (target, positions) in enumerate(windows.items()):
                q = dict(id=f'scn2a_{i}', sequence=target.translate(str.maketrans('ACGT','TGCA'))[::-1], intended_genes=['ENSG00000136531'])
                queries.append(q)
                origins.write(json.dumps(dict(id=q['id'], positions=positions))+'\n')
        assert queries
        full_queries = root/'queries.jsonl'
        full_queries.write_text(''.join(json.dumps(q)+'\n' for q in queries))
        record.update(queries=len(queries), walk_positions=sum(map(len, windows.values())), gene_bases=len(sequence))
        paths = [a.gene, full_queries, a.binary, a.ddg_binary, a.index/'manifest.json', a.reverse_index/'manifest.json', a.annotation_cache]
        record['sha256'] = {str(f):sha(f) for f in paths}
        record['host'] = dict(uname=list(os.uname()), cpus=len(os.sched_getaffinity(0)))
        record['setup_seconds'] = time.perf_counter()-start
        save()
        # Round-robin partitions spread repeat-associated high-hit ASOs.
        jobs = []
        for i in range(a.threads):
            path = root/f'queries-{i}.jsonl'
            path.write_text(''.join(json.dumps(q)+'\n' for q in queries[i::a.threads]))
            command = [str(a.binary.resolve()), 'report', '--queries', str(path), '--reference', str(a.reference_root/'reference.fa'),
                       '--index', str(a.index), '--reverse-index', str(a.reverse_index), '--annotations', str(a.reference_root/'records.jsonl'),
                       '--annotation-cache', str(a.annotation_cache), '--policy', 'other-gene', '--reference-release', 'Ensembl110-GRCh38',
                       '--scope', record['scope'], '--biotype-policy', 'retained reference-v1 manifest', '-k', '3']
            jobs.append((i, command))
        record['discovery_commands'] = [c for _,c in jobs]; save()
        def discover(job):
            i, command = job
            t = time.perf_counter()
            with (root/f'report-{i}.jsonl').open('wb') as out, (root/f'discovery-{i}.stderr').open('wb') as err:
                result = subprocess.run(command, stdout=out, stderr=err, timeout=max(1, deadline-time.perf_counter()))
            return dict(stage=f'discovery-worker-{i}', seconds=time.perf_counter()-t, returncode=result.returncode, command=command)
        t = time.perf_counter()
        with concurrent.futures.ThreadPoolExecutor(max_workers=a.threads) as pool:
            for result in pool.map(discover, jobs):
                record['attempts'].append(result); save()
                assert result['returncode'] == 0, result
        record['discovery_wall_seconds'] = time.perf_counter()-t
        print('DISCOVERY', record['discovery_wall_seconds'], flush=True)
        t = time.perf_counter(); total = 0; manifest = None
        with (root/'report.jsonl').open('wb') as merged:
            for i,_ in jobs:
                path = root/f'report-{i}.jsonl'
                with path.open('rb') as source:
                    header = json.loads(next(source)); assert header['type']=='manifest'
                    if manifest is None:
                        manifest = header
                        manifest['queries_sha256'] = sha(full_queries)
                        manifest['threads'] = a.threads
                        manifest['benchmark_discovery_partitioning'] = 'round-robin independent CLI processes'
                        merged.write(json.dumps(manifest).encode()+b'\n')
                    else: assert header['reference_sha256'] == manifest['reference_sha256']
                    last = None
                    for line in source:
                        if last is not None: merged.write(last)
                        last = line
                    tail = json.loads(last); assert tail['type']=='run_complete'
                    total += tail['reported_sites']
            merged.write(json.dumps(dict(type='run_complete', reported_sites=total)).encode()+b'\n')
        record['merge_seconds'] = time.perf_counter()-t
        record['reported_sites'] = total
        record['report_bytes'] = (root/'report.jsonl').stat().st_size
        record['sha256'][str(a.reference_root/'reference.fa')] = manifest['reference_sha256']
        save(); print('MERGED', total, 'sites', record['report_bytes'], 'bytes', flush=True)
        command = [str(a.ddg_binary.resolve()), '--queries', str(full_queries), '--reference', str(a.reference_root/'reference.fa'),
                   '--sites', str(root/'report.jsonl'), '--energy-engine', 'rust', '--threads', str(a.threads),
                   '--timeout', str(max(1, deadline-time.perf_counter())), '--work-dir', str(root/'ddg-work')]
        record['ddg_command'] = command; save()
        t = time.perf_counter()
        # stdout is a byte-for-byte copy of the retained ddg-work spool; avoid a
        # second full disk copy. This still measures the CLI's stdout emission.
        with open(os.devnull, 'wb') as out, (root/'ddg.stderr').open('wb') as err:
            result = subprocess.run(command, stdout=out, stderr=err, timeout=max(1,deadline-time.perf_counter())+5)
        record['attempts'].append(dict(stage='ddg', seconds=time.perf_counter()-t, returncode=result.returncode, command=command))
        record['total_wall_seconds'] = time.perf_counter()-start
        save(); assert result.returncode == 0
        record['ddg_manifest'] = json.loads((root/'ddg-work/manifest.json').read_text())
        assert record['ddg_manifest']['annotated_sites'] == total
        record['complete'] = True
        print('COMPLETE', record['total_wall_seconds'], 'seconds', flush=True)
    except BaseException as e:
        record['error'] = repr(e)
        raise
    finally: save()

if __name__ == '__main__': main()
