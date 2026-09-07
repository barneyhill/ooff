#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
export PATH="/home/ubuntu/.cargo/bin:$PATH"
# The controller verified this specific prior benchmark process live before
# queuing the dependent sequence. Avoid timing/compilation contention.
while kill -0 15349 2>/dev/null; do sleep 15; done
cargo build --release --locked --bin ooff-index --bin ooff
bash benchmarks/build_comparator_ec2.sh
python3 benchmarks/run_iteration.py --label fm-paired-exact-tuples32 --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 32 -k 3 --repetitions 1 --site-tuples --timeout 600
python3 benchmarks/run_cli_iteration.py --label cli-paired-report32 --queries benchmarks/sassy_1000/SCN2A_unfiltered_1000.fa --mode report --n 32 --reverse-index data/fm-reverse-v1
python3 - <<'PY'
from pathlib import Path
import subprocess
root=Path('benchmarks/iterations')
cli=sorted(root.glob('*-cli-paired-report32'))[-1]
comparison=sorted(root.glob('*-fm-paired-exact-tuples32/result.json'))[-1]
subprocess.run(['python3','benchmarks/audit_cli_output.py','--cli',str(cli),'--comparison',str(comparison),'--index','data/fm-production-v1'],check=True)
PY
for k in 0 1 2; do
    python3 benchmarks/run_iteration.py --label "fm-paired-exact-tuples32-k$k" --reference data/reference-v1/reference.fa --native target/release/ooff-index --index data/fm-production-v1 --reverse-index data/fm-reverse-v1 --mmap --mode sites --n 32 -k "$k" --repetitions 1 --site-tuples --timeout 600
done
