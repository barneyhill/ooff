#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
while kill -0 30663 2>/dev/null; do sleep 15; done
printf 'incomplete\n' > results/fm-round14.success
cp target/release/ooff-index data/round14-stage/ooff-index-before-range-union
cp -a data/round14-stage/src/. src/
cp -a data/round14-stage/tests/. tests/
find src tests -type f -exec touch {} +
export PATH="/home/ubuntu/.cargo/bin:$PATH"
OMP_NUM_THREADS=2 cargo test --locked --all-targets
cargo build --release --locked --bin ooff-index --bin ooff
bash benchmarks/build_comparator_ec2.sh /home/ubuntu/comparators/sassy-0.2.6-fast-interval
comparator=/home/ubuntu/comparators/sassy-0.2.6-fast-interval/target/release/sassy-comparator
for k in 0 1 2 3; do
    python3 benchmarks/run_iteration.py --label "fm-word-union-sites32-k$k" --reference data/reference-v1/reference.fa --native target/release/ooff-index --sassy "$comparator" --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 32 -k "$k" --repetitions 1 --site-tuples --timeout 300
done
python3 benchmarks/run_iteration.py --label fm-word-union-sites1000 --reference data/reference-v1/reference.fa --native target/release/ooff-index --sassy "$comparator" --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 1000 -k 3 --repetitions 3 --timeout 1200
for repetition in 1 2 3; do
    python3 benchmarks/run_cli_iteration.py --label "cli-word-union-report32-r$repetition" --queries benchmarks/sassy_1000/SCN2A_unfiltered_1000.fa --mode report --n 32 --reverse-index data/fm-reverse-v1 --annotation-cache data/fm-production-v1/annotation-cache.json
done
python3 - <<'PY'
from pathlib import Path
from collections import Counter
import json, subprocess
root=Path('benchmarks/iterations')
baseline=sorted(root.glob('*-cli-borrowed-json-report32-r3'))[-1]
def sites(folder):
    return Counter(json.dumps(row,sort_keys=True) for row in map(json.loads,(folder/'output.jsonl').open()) if row['type']=='site')
expected=sites(baseline)
comparison=sorted(root.glob('*-fm-word-union-sites32-k3/result.json'))[-1]
for current in sorted(root.glob('*-cli-word-union-report32-r*')):
    assert sites(current)==expected
    (current/'all-fields-equality.json').write_text(json.dumps({'baseline':str(baseline),'all_site_fields_equal':True,'sites':sum(expected.values())})+'\n')
    subprocess.run(['python3','benchmarks/audit_cli_output.py','--cli',str(current),'--comparison',str(comparison),'--index','data/fm-production-v1'],check=True)
PY
sha256sum target/release/ooff-index target/release/ooff > results/fm-round14.success
