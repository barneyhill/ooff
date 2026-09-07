#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
while kill -0 26424 2>/dev/null; do sleep 15; done
cp data/round11-stage/main.rs src/main.rs
cp data/round11-stage/lib.rs src/lib.rs
cp data/round11-stage/run_cli_iteration.py benchmarks/run_cli_iteration.py
export PATH="/home/ubuntu/.cargo/bin:$PATH"
OMP_NUM_THREADS=2 cargo test --locked --all-targets
cargo build --release --locked --bin ooff
for repetition in 1 2 3; do
    python3 benchmarks/run_cli_iteration.py --label "cli-borrowed-json-report32-r$repetition" --queries benchmarks/sassy_1000/SCN2A_unfiltered_1000.fa --mode report --n 32 --reverse-index data/fm-reverse-v1 --annotation-cache data/fm-production-v1/annotation-cache.json
done
python3 - <<'PY'
from collections import Counter
from pathlib import Path
import json
import subprocess
root = Path('benchmarks/iterations')
baseline = sorted(root.glob('*-cli-coarse-flat-report32-r3'))[-1]
def sites(path):
    rows = Counter()
    for line in (path / 'output.jsonl').open():
        row = json.loads(line)
        if row['type'] == 'site':
            rows[json.dumps(row, sort_keys=True, separators=(',', ':'))] += 1
    return rows
expected = sites(baseline)
comparison = sorted(root.glob('*-fm-coarse-flat-verifier-sites32/result.json'))[-1]
for current in sorted(root.glob('*-cli-borrowed-json-report32-r*')):
    assert sites(current) == expected
    (current / 'all-fields-equality.json').write_text(json.dumps({'baseline': str(baseline), 'all_site_fields_equal': True, 'sites': sum(expected.values())}, indent=2) + '\n')
    subprocess.run(['python3', 'benchmarks/audit_cli_output.py', '--cli', str(current), '--comparison', str(comparison), '--index', 'data/fm-production-v1'], check=True)
PY
python3 benchmarks/run_cli_iteration.py --label cli-borrowed-json-alleles-all-screen --queries data/cached-allele-queries-v1/scn2a-cached-alleles-all.jsonl --queries-jsonl --mode screen --n 30000 --annotation-cache data/fm-production-v1/annotation-cache.json
python3 - <<'PY'
from pathlib import Path
import json
root = Path('benchmarks/iterations')
current = sorted(root.glob('*-cli-borrowed-json-alleles-all-screen'))[-1]
queries = {q['id']: q for q in map(json.loads, (current / 'queries.jsonl').open())}
summaries = set()
sites = 0
for row in map(json.loads, (current / 'output.jsonl').open()):
    if row['type'] == 'site':
        assert row['allele'] == queries[row['query_id']]['allele']
        sites += 1
    elif row['type'] == 'query_summary':
        assert row['query_id'] not in summaries
        summaries.add(row['query_id'])
assert summaries == queries.keys()
(current / 'allele-metadata-audit.json').write_text(json.dumps({'queries': len(queries), 'sites': sites, 'all_allele_metadata_preserved': True, 'all_queries_summarized': True}, indent=2) + '\n')
PY
