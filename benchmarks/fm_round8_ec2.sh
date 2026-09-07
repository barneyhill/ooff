#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
export PATH="/home/ubuntu/.cargo/bin:$PATH"
OMP_NUM_THREADS=2 cargo test --locked --all-targets
cargo build --release --locked --bin ooff-index --bin ooff
bash benchmarks/build_comparator_ec2.sh
bash benchmarks/build_comparator_ec2.sh /home/ubuntu/comparators/sassy-0.2.6-v1-iupac /home/ubuntu/ooff/benchmarks/sassy_v1_engine.rs
python3 benchmarks/run_iteration.py --label fm-coarse-flat-verifier-sites32 --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 32 -k 3 --repetitions 3 --site-tuples --timeout 600
for repetition in 1 2 3; do
    python3 benchmarks/run_cli_iteration.py --label "cli-coarse-flat-report32-r$repetition" --queries benchmarks/sassy_1000/SCN2A_unfiltered_1000.fa --mode report --n 32 --reverse-index data/fm-reverse-v1 --annotation-cache data/fm-production-v1/annotation-cache.json
done
python3 - <<'PY'
from pathlib import Path
import subprocess
root=Path('benchmarks/iterations')
comparison=sorted(root.glob('*-fm-coarse-flat-verifier-sites32/result.json'))[-1]
for cli in sorted(root.glob('*-cli-coarse-flat-report32-r*')):
    subprocess.run(['python3','benchmarks/audit_cli_output.py','--cli',str(cli),'--comparison',str(comparison),'--index','data/fm-production-v1'],check=True)
PY
python3 benchmarks/run_iteration.py --label fm-v1-iupac-low-complexity-screen --reference data/reference-v1/reference.fa --queries data/query-sets-v1/low-complexity-1000.fa --native target/release/ooff-index --index data/fm-production-v1 --mmap --mode screen --n 1000 -k 3 --repetitions 3 --timeout 600 --sassy /home/ubuntu/comparators/sassy-0.2.6-v1-iupac/target/release/sassy-comparator
python3 benchmarks/run_iteration.py --label fm-v1-iupac-original1000-screen --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --mmap --mode screen --n 1000 -k 3 --repetitions 3 --timeout 600 --sassy /home/ubuntu/comparators/sassy-0.2.6-v1-iupac/target/release/sassy-comparator
python3 benchmarks/run_iteration.py --label fm-coarse-flat-verifier-fresh-sites1000 --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 1000 -k 3 --repetitions 3 --timeout 1800
