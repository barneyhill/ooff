#!/usr/bin/env python3
"""Retain every classic-tool preparation/search attempt, including failures."""
import argparse, csv, datetime, hashlib, json, os, platform, shutil, signal, subprocess, time
from pathlib import Path

def refresh():
    root=Path('benchmarks/classic/iterations')
    rows=[]
    for p in sorted(root.glob('*/result.json')):
        x=json.loads(p.read_text())
        row={k:x.get(k,'') for k in ['id','label','returncode','timeout','wall_seconds','command']}
        row.update({k:x.get('metadata',{}).get(k,'') for k in ['tool','queries','repetition','phase','threads']})
        rows.append(row)
    if not rows: return
    with Path('benchmarks/classic/summary.csv').open('w',newline='') as f:
        w=csv.DictWriter(f,fieldnames=rows[0]);w.writeheader();w.writerows(rows)
    text='# Classic-aligner benchmark iterations\n\nEvery invocation is retained, including preparation, failed attempts, and pilots.\nRaw commands, logs, timing, and outputs are in each iteration directory.\n\nThe README uses powers-of-ten batches. Other sizes and the user-cancelled 26,643-ASO verification remain retained; see [batch-selection.json](classic/batch-selection.json).\n\n| ID | Task | Wall seconds | Exit | Timeout |\n| --- | --- | ---: | ---: | --- |\n'
    for r in rows:
        seconds=f"{r['wall_seconds']:.6f}" if r['wall_seconds'] is not None else 'unavailable'
        text+=f"| [{r['id']}](classic/iterations/{r['id']}/result.json) | {r['label']} | {seconds} | {r['returncode']} | {r['timeout']} |\n"
    screenings=[]
    for p in sorted(root.glob('*/screening.json')):
        x=json.loads(p.read_text())
        screenings.append({k:x.get(k,'') for k in ['tool','queries','repetition','phase','threads','complete','wall_seconds','recovered','utc']})
    if screenings:
        with Path('benchmarks/classic/screening.csv').open('w',newline='') as f:
            w=csv.DictWriter(f,fieldnames=screenings[0]);w.writeheader();w.writerows(screenings)
        text+='\n## Composite screening runs\n\nElapsed time includes all mapper/conversion/verification stages; pilot runs are labelled.\n\n| Tool | ASOs | Rep | Phase | Threads | Seconds | Recovered | Complete |\n| --- | ---: | ---: | --- | ---: | ---: | ---: | --- |\n'
        for r in screenings:
            text+=f"| {r['tool']} | {r['queries']} | {r['repetition']} | {r['phase']} | {r['threads']} | {r['wall_seconds']:.6f} | {r['recovered']} | {r['complete']} |\n"
    Path('benchmarks/CLASSIC_ITERATIONS.md').write_text(text)

def run(label,command,timeout=3600,metadata=None):
    stamp=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S.%f')
    directory=Path('benchmarks/classic/iterations')/(stamp+'-'+label)
    storage=os.environ.get('OOFF_CLASSIC_STORAGE')
    if storage:
        physical=Path(storage).resolve()/directory.name
        physical.mkdir(parents=True)
        directory.parent.mkdir(parents=True,exist_ok=True)
        directory.symlink_to(physical,target_is_directory=True)
    else:
        directory.mkdir(parents=True)
    executable=shutil.which(command[0])
    provenance={}
    for name in [executable, *[str(p) for p in Path('benchmarks/classic').iterdir() if p.suffix in ('.py','.rs','.sh')]]:
        if name and Path(name).is_file():
            with Path(name).open('rb') as f: provenance[name]=hashlib.file_digest(f,'sha256').hexdigest()
    record=dict(id=directory.name,label=label,command=command,metadata=metadata or {},started_utc=stamp,sha256=provenance,host=platform.uname()._asdict())
    (directory/'started.json').write_text(json.dumps(record,indent=2)+'\n')
    started=time.perf_counter()
    with (directory/'stdout').open('wb') as out, (directory/'stderr').open('wb') as err:
        process=subprocess.Popen(['/usr/bin/time','-v','-o',str(directory/'resources.txt'),*command],stdout=out,stderr=err,start_new_session=True)
        expired=False
        try: code=process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            expired=True;os.killpg(process.pid,signal.SIGTERM)
            try: code=process.wait(timeout=10)
            except subprocess.TimeoutExpired: os.killpg(process.pid,signal.SIGKILL);code=process.wait()
    record.update(returncode=code,timeout=expired,wall_seconds=time.perf_counter()-started)
    (directory/'result.json').write_text(json.dumps(record,indent=2)+'\n')
    refresh()
    print(json.dumps(record),flush=True)
    return directory,record

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--label',required=True);p.add_argument('--timeout',type=int,default=3600);p.add_argument('command',nargs=argparse.REMAINDER)
    a=p.parse_args();command=a.command[1:] if a.command[:1]==['--'] else a.command
    _,record=run(a.label,command,a.timeout)
    raise SystemExit(0 if record['returncode']==0 else 1)
