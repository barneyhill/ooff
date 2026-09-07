#!/usr/bin/env bash
set -euo pipefail
# Usage: bash benchmarks/classic/isolated_job.sh TOOL SOURCE_URL [pilot|measurement]
cd /home/ubuntu/ooff
tool="${1:?tool required}"; source_url="${2:?source URL required}"; phase="${3:-pilot}"
export OMP_NUM_THREADS="$(nproc)"
export OOFF_BENCH_THREADS="$OMP_NUM_THREADS"
run_timeout="${OOFF_RUN_TIMEOUT_SECONDS:-600}"
mkdir -p results data/classic-v1
if [[ ! -f "data/classic-v1/inputs-ready-$tool.json" ]]; then
    python3 benchmarks/classic/command.py --label "$tool-fetch-inputs" --timeout 4000 -- python3 benchmarks/classic/fetch_inputs.py --source "$source_url" --tool "$tool"
fi
if [[ ! -f "data/classic-v1/index-ready-$tool.json" ]]; then
    case "$tool" in
      bwa) ;; # Reuse the already measured BWA build; original timing/hashes retained.
      blast) python3 benchmarks/classic/command.py --label build-blast-index --timeout 1800 -- makeblastdb -in data/classic-v1/eligible.fa -dbtype nucl -parse_seqids -out data/classic-v1/blast ;;
      minimap2) python3 benchmarks/classic/command.py --label build-minimap2-index --timeout 1800 -- minimap2 -t "$OOFF_BENCH_THREADS" -k 7 -w 5 -d data/classic-v1/minimap2-k7w5.mmi data/classic-v1/eligible.fa ;;
      *) exit 2 ;;
    esac
    python3 - "$tool" <<'PY'
import json,os,subprocess,sys
from pathlib import Path
tool=sys.argv[1]
x=dict(complete=True,tool=tool,allowed_cpus=sorted(os.sched_getaffinity(0)),cpu=json.loads(subprocess.check_output(['lscpu','--json'],text=True)),versions=subprocess.check_output(['dpkg-query','-W'],text=True))
Path(f'data/classic-v1/index-ready-{tool}.json').write_text(json.dumps(x,indent=2)+'\n')
PY
fi
python3 benchmarks/classic/add_small_batches.py
python3 benchmarks/classic/test_verify.py
if [[ "$phase" == pilot ]]; then
    python3 benchmarks/classic/scale.py --tools "$tool" --phase pilot --counts 100 --repetitions 1 --timeout "$run_timeout" --run-timeout "$run_timeout"
else
    [[ "$phase" == measurement ]]
    python3 benchmarks/classic/scale.py --tools "$tool" --phase measurement --repetitions 3 --timeout "$run_timeout" --run-timeout "$run_timeout"
fi
