#!/usr/bin/env python3
"""Retain a timed production CLI run, including complete annotated output."""
import argparse
import datetime as dt
import hashlib
import json
import os
import signal
from pathlib import Path
import subprocess
import tarfile
import time

ap = argparse.ArgumentParser(description=__doc__)
ap.add_argument('--label', required=True)
ap.add_argument('--queries', required=True)
ap.add_argument('--n', type=int, default=1000)
ap.add_argument('--mode', choices=['screen', 'report'], default='screen')
ap.add_argument('-k', type=int, default=3)
ap.add_argument("--reverse-index")
ap.add_argument("--annotation-cache")
ap.add_argument("--queries-jsonl", action="store_true", help="Preserve input JSONL query metadata, including allele associations")
args = ap.parse_args()
ident = dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%S') + '-' + args.label
folder = Path('benchmarks/iterations') / ident
folder.mkdir(exist_ok=False)
queries = []
for line in Path(args.queries).read_text().splitlines():
    if args.queries_jsonl:
        if line.strip():
            queries.append(json.loads(line))
    elif line.startswith('>'):
        header = line[1:]
        ident_q, _, genes = header.partition('|')
        queries.append(dict(id=ident_q, sequence='', intended_genes=genes.split(',') if genes else ['ENSG00000136531']))
    elif line.strip():
        queries[-1]['sequence'] += line.strip()
queries = queries[:args.n]
query_path = folder / 'queries.jsonl'
query_path.write_text(''.join(json.dumps(q) + '\n' for q in queries))
with tarfile.open(folder / 'source.tar.gz', 'w:gz') as archive:
    for p in ['src', 'Cargo.toml', 'Cargo.lock', 'benchmarks/run_cli_iteration.py']:
        archive.add(p)
command = ['target/release/ooff', args.mode, '--queries', str(query_path),
           '--reference', 'data/reference-v1/reference.fa', '--index', 'data/fm-production-v1',
           '--annotations', 'data/reference-v1/records.jsonl', '--policy', 'other-gene',
           '--reference-release', 'Ensembl-110-GRCh38-primary-assembly',
           '--scope', 'gene bodies and mature transcripts; see reference-v1 manifest',
           '--biotype-policy', 'all except non-transcribed/non-translated pseudogenes', '-k', str(args.k)]
if args.reverse_index:
    command.extend(["--reverse-index", args.reverse_index])
if args.annotation_cache:
    command.extend(["--annotation-cache", args.annotation_cache])
result = dict(id=ident, kind='production_cli', configuration=vars(args), command=command,
              binary_sha256=hashlib.sha256(Path(command[0]).read_bytes()).hexdigest(),
              cache_policy='fresh process; OS cache not flushed', queries=len(queries))
(folder / 'started.json').write_text(json.dumps(result, indent=2) + '\n')
start = time.monotonic()
with (folder / 'output.jsonl').open('x') as out, (folder / 'stderr').open('x') as err:
    proc = subprocess.Popen(['/usr/bin/time', '-v', '-o', str(folder / 'resources.txt')] + command,
                          stdout=out, stderr=err, start_new_session=True)
    result["pid"] = proc.pid
    (folder / "process.json").write_text(json.dumps(result, indent=2) + "\n")
    try:
        proc.wait(timeout=600)
        result["timeout"] = False
    except subprocess.TimeoutExpired:
        os.killpg(proc.pid, signal.SIGTERM)
        proc.wait()
        result["timeout"] = True
result.update(returncode=proc.returncode, wall_seconds=time.monotonic() - start,
              output_bytes=(folder / 'output.jsonl').stat().st_size)
for line in (folder / 'output.jsonl').open():
    try:
        row = json.loads(line)
    except json.JSONDecodeError:
        result["partial_json_line"] = True
        continue
    if row['type'] in ['manifest', 'run_complete']:
        result[row['type']] = row
(folder / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
with Path('benchmarks/CLI_ITERATIONS.md').open('a') as ledger:
    ledger.write(f'\n- `{ident}`: {args.mode}, n={len(queries)}, k={args.k}, '
                 f'wall={result["wall_seconds"]:.6f}s, bytes={result["output_bytes"]}, '
                 f'exit={proc.returncode}; [artifacts](iterations/{ident}/result.json).\n')
subprocess.run(["python3", "benchmarks/refresh_ledger.py"], check=True)
print(json.dumps(result, indent=2))
