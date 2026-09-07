#!/usr/bin/env python3
"""Record index construction time/RSS independently of warm search iterations."""
import argparse
import datetime as dt
import json
from pathlib import Path
import subprocess
import time

ap=argparse.ArgumentParser(description=__doc__)
ap.add_argument("--reference",required=True)
ap.add_argument("--output",required=True)
ap.add_argument("--label",required=True)
ap.add_argument("--shard-bases",type=int,default=200_000_000)
ap.add_argument("--reverse-records",action="store_true")
ap.add_argument("--annotations")
ap.add_argument("--index")
args=ap.parse_args()
if bool(args.annotations) != bool(args.index):ap.error("--annotations and --index must be supplied together")
args.threads=1 if args.annotations else 8
now=dt.datetime.now(dt.timezone.utc)
ident=now.strftime("%Y%m%dT%H%M%S")+"-"+args.label
folder=Path("benchmarks/iterations")/ident
folder.mkdir(parents=True,exist_ok=False)
command=["/usr/bin/time","-v","-o",str(folder/"resources.txt"),"target/release/ooff-index","build","--reference",args.reference,"--output",args.output,"--shard-bases",str(args.shard_bases)]
if args.annotations:
    command=["/usr/bin/time","-v","-o",str(folder/"resources.txt"),"target/release/ooff-index","cache-annotations","--reference",args.reference,"--output",args.output,"--annotations",args.annotations,"--index",args.index]
else:
    if args.reverse_records:command.append("--reverse-records")
start=time.monotonic()
with (folder/"stdout.json").open("x") as out,(folder/"stderr.log").open("x") as err:
    result=subprocess.run(command,stdout=out,stderr=err)
record={"id":ident,"kind":"index_build","configuration":vars(args),"command":command,"returncode":result.returncode,"wall_seconds":time.monotonic()-start}
if result.returncode==0:record["output"]=json.loads((folder/"stdout.json").read_text())
(folder/"result.json").write_text(json.dumps(record,indent=2)+"\n")
row=f'| {ident} | {now.strftime("%H:%M:%S")} | {args.label} | {args.reference} | index/cache build; threads={args.threads} | {record["wall_seconds"]:.6f} | — | — | — | see artifact | '+("complete" if result.returncode==0 else "FAILED")+f' | [{ident}]({folder}/result.json) |\n'
p=Path("BENCHMARKS.md");p.write_text(p.read_text().replace("\n## Interpretation",row+"\n## Interpretation",1))
with open("benchmarks/iterations/runs.jsonl","a") as f:f.write(json.dumps(record)+"\n")
subprocess.run(["python3","benchmarks/refresh_ledger.py"],check=True)
print(row,flush=True)
raise SystemExit(result.returncode)
