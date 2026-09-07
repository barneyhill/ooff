#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
export PATH="/home/ubuntu/.cargo/bin:$PATH"
while kill -0 20537 2>/dev/null; do sleep 15; done
cargo build --release --locked --bin ooff-index --bin ooff
python3 benchmarks/run_index_build.py --label annotation-cache-build --reference data/reference-v1/reference.fa --output data/fm-production-v1/annotation-cache.json --annotations data/reference-v1/records.jsonl --index data/fm-production-v1
for repetition in 1 2 3; do
    python3 benchmarks/run_cli_iteration.py --label "cli-cached-screen1000-r$repetition" --queries benchmarks/sassy_1000/SCN2A_unfiltered_1000.fa --annotation-cache data/fm-production-v1/annotation-cache.json
    python3 benchmarks/run_cli_iteration.py --label "cli-cached-paired-report32-r$repetition" --queries benchmarks/sassy_1000/SCN2A_unfiltered_1000.fa --mode report --n 32 --reverse-index data/fm-reverse-v1 --annotation-cache data/fm-production-v1/annotation-cache.json
done
python3 benchmarks/run_cli_iteration.py --label cli-cached-screen10000 --queries data/query-sets-v1/scn2a-10000.fa --n 10000 --annotation-cache data/fm-production-v1/annotation-cache.json
python3 - <<'PY'
from pathlib import Path
import subprocess
root=Path('benchmarks/iterations')
comparison=sorted(root.glob('*-fm-paired-exact-tuples32/result.json'))[-1]
for cli in sorted(root.glob('*-cli-cached-paired-report32-r*')):
    subprocess.run(['python3','benchmarks/audit_cli_output.py','--cli',str(cli),'--comparison',str(comparison),'--index','data/fm-production-v1'],check=True)
PY
for dataset in scn2a-10000 mixed-genes-1000 random-1000 low-complexity-1000; do
    python3 benchmarks/run_iteration.py --label "fm-range-final-$dataset-screen" --reference data/reference-v1/reference.fa --queries "data/query-sets-v1/$dataset.fa" --native target/release/ooff-index --index data/fm-production-v1 --mmap --mode screen --n 10000 -k 3 --repetitions 3 --timeout 600
done
python3 benchmarks/run_iteration.py --label fm-range-allele-shortlist32-sites --reference data/reference-v1/reference.fa --queries benchmarks/sassy_benchmark/SCN2A_patterns.fa --native target/release/ooff-index --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 32 -k 3 --repetitions 3 --site-tuples --timeout 600
python3 benchmarks/run_iteration.py --label fm-range-v1-low-complexity-screen --reference data/reference-v1/reference.fa --queries data/query-sets-v1/low-complexity-1000.fa --native target/release/ooff-index --index data/fm-production-v1 --mmap --mode screen --n 1000 -k 3 --repetitions 3 --timeout 600 --sassy /home/ubuntu/comparators/sassy-0.2.6-v1/target/release/sassy-comparator
python3 benchmarks/run_iteration.py --label fm-range-v1-original1000-screen --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --mmap --mode screen --n 1000 -k 3 --repetitions 3 --timeout 600 --sassy /home/ubuntu/comparators/sassy-0.2.6-v1/target/release/sassy-comparator
python3 benchmarks/run_iteration.py --label fm-range-cold-original1000-screen --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --mmap --mode screen --n 1000 -k 3 --repetitions 3 --cold --timeout 600
