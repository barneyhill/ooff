#!/usr/bin/env bash
set -euo pipefail
cd /home/ubuntu/ooff
predecessor="${1:?Final inventory predecessor PID required}"
while kill -0 "$predecessor" 2>/dev/null; do sleep 15; done
export PATH="/home/ubuntu/.cargo/bin:$PATH"
sudo growpart /dev/nvme0n1 1
sudo resize2fs /dev/nvme0n1p1
python3 benchmarks/classic/command.py --label install-classic-tools --timeout 600 -- sudo env DEBIAN_FRONTEND=noninteractive apt-get install -y bwa ncbi-blast+ minimap2
mkdir -p data/classic-stage/saved-binaries
cp -a target/release/ooff-index data/classic-stage/saved-binaries/ooff-index-single-thread
cp data/classic-stage/ooff-index.rs src/bin/ooff-index.rs
cp data/classic-stage/parallel_index.rs tests/parallel_index.rs
find src tests -type f -exec touch {} +
python3 benchmarks/classic/command.py --label native-parallel-tests --timeout 600 -- env OMP_NUM_THREADS=2 cargo test --locked --all-targets
python3 benchmarks/classic/command.py --label native-parallel-build --timeout 600 -- cargo build --locked --release --bin ooff-index
python3 benchmarks/classic/command.py --label prepare-eligible-reference --timeout 600 -- python3 benchmarks/classic/prepare.py
python3 benchmarks/classic/command.py --label build-bwa-index --timeout 5400 -- bwa index -p data/classic-v1/bwa data/classic-v1/eligible.fa
python3 benchmarks/classic/command.py --label build-blast-index --timeout 1800 -- makeblastdb -in data/classic-v1/eligible.fa -dbtype nucl -parse_seqids -out data/classic-v1/blast
python3 benchmarks/classic/command.py --label build-minimap2-index --timeout 1800 -- minimap2 -t 16 -k 7 -w 5 -d data/classic-v1/minimap2-k7w5.mmi data/classic-v1/eligible.fa
python3 - <<'PY'
import json,subprocess,hashlib,shutil
from pathlib import Path
versions={}
for tool,args in [('bwa',[]),('blastn',['-version']),('minimap2',['--version'])]:
    p=Path(shutil.which(tool));x=subprocess.run([str(p),*args],capture_output=True,text=True)
    versions[tool]=dict(path=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),stdout=x.stdout,stderr=x.stderr)
Path('data/classic-v1/tool-versions.json').write_text(json.dumps(versions,indent=2)+'\n')
PY
sha256sum target/release/ooff-index > results/classic-setup.success
