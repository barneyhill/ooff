#!/usr/bin/env python3
"""Build the witness-emitting benchmark path and map original IDs to eligible IDs."""
import hashlib,json,os,shutil,subprocess
from pathlib import Path
from command import run

def main():
    threads=int(os.environ.get("OOFF_BENCH_THREADS",len(os.sched_getaffinity(0))))
    index=f"data/fm-classic-{threads}-v1"
    stage=Path('data/classic-stage/ooff-index-witnesses.rs')
    destination=Path('src/bin/ooff-index.rs')
    assert stage.is_file()
    shutil.copy2(destination,'data/classic-stage/saved-binaries/ooff-index-source-before-witnesses.rs')
    shutil.copy2(stage,destination);destination.touch()
    for label,cmd in [
        ('native-witness-tests',['env',f'OMP_NUM_THREADS={threads}','/home/ubuntu/.cargo/bin/cargo','test','--locked','--all-targets','-j',str(threads)]),
        ('native-witness-build',['/home/ubuntu/.cargo/bin/cargo','build','--release','--locked','--bin','ooff-index','-j',str(threads)]),
    ]:
        _,result=run(label,cmd,600)
        assert result['returncode']==0,result
    _,result=run(f'build-ooff-index-{threads}',['env',f'OMP_NUM_THREADS={threads}','target/release/ooff-index','build','--reference','data/reference-v1/reference.fa','--output',index,'--shard-bases','1300000000'],1800,dict(kind='index_build',tool='ooff',threads=threads))
    assert result['returncode']==0,result
    root=Path('data/classic-v1')
    host=dict(cpu=json.loads(subprocess.check_output(['lscpu','--json'],text=True)),allowed_cpus=sorted(os.sched_getaffinity(0)),memory=Path('/proc/meminfo').read_text())
    (root/'host.json').write_text(json.dumps(host,indent=2)+'\n')
    eligible=json.loads((root/'records.json').read_text())
    by_header={r['header']:r['id'] for r in eligible};assert len(by_header)==len(eligible)
    source=Path(index)/'reference-info.json'
    original=json.loads(source.read_text())
    ids=[]
    for r in original['records']:
        rid=by_header.get(r['header'])
        if rid is None:assert all(g=='ENSG00000136531' for g in r['header'].split('|',1)[1].split(','))
        ids.append(rid)
    assert sum(x is not None for x in ids)==len(eligible)
    path=root/'native-record-map.json'
    with path.open('x') as f:json.dump(ids,f)
    provenance=dict(original_info_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),eligible_records_sha256=hashlib.sha256((root/'records.json').read_bytes()).hexdigest(),mapping_sha256=hashlib.sha256(path.read_bytes()).hexdigest(),native_binary_sha256=hashlib.sha256(Path('target/release/ooff-index').read_bytes()).hexdigest())
    (root/'witness-provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
    _,result=run('archive-witness-source',['tar','-czf',str(root/'source-with-witnesses.tar.gz'),'Cargo.toml','Cargo.lock','src','tests','benchmarks/classic'],600)
    assert result['returncode']==0,result

if __name__=='__main__':main()
