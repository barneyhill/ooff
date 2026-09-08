#!/usr/bin/env python3
"""Run and retain a paired benchmark, including failures/timeouts, then append MD."""
import argparse
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import signal
import statistics
import subprocess
import tarfile
import time


def digest(path):
    h=hashlib.sha256()
    with open(path,"rb") as f:
        for block in iter(lambda:f.read(1024*1024),b""):h.update(block)
    return h.hexdigest()


def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--label",required=True)
    ap.add_argument("--reference",required=True)
    ap.add_argument("--queries",default="benchmarks/sassy_1000/SCN2A_unfiltered_1000.fa")
    ap.add_argument("--mode",default="screen",choices=["screen","endpoints","sites"])
    ap.add_argument("--n",type=int,default=1000)
    ap.add_argument("-k",type=int,default=3)
    ap.add_argument("--repetitions",type=int,default=3)
    ap.add_argument("--timeout",type=int,default=300)
    ap.add_argument("--native",default="target/release/oofft-bench")
    ap.add_argument("--sassy",default="/home/ubuntu/comparators/sassy-0.2.6/target/release/sassy-comparator")
    ap.add_argument("--index",default=None)
    ap.add_argument("--distance-strata",action="store_true")
    ap.add_argument("--mmap",action="store_true")
    ap.add_argument("--automaton",action="store_true")
    ap.add_argument("--reverse-index")
    ap.add_argument("--site-tuples",action="store_true")
    ap.add_argument("--cold",action="store_true")
    ap.add_argument("--reuse-sassy",help="Development only: reuse an explicitly recorded identical-workload Sassy measurement")
    args=ap.parse_args()
    if args.site_tuples and args.mode != "sites":ap.error("--site-tuples requires --mode sites")
    now=dt.datetime.now(dt.timezone.utc)
    ident=now.strftime("%Y%m%dT%H%M%S")+"-"+args.label
    folder=Path("benchmarks/iterations")/ident
    folder.mkdir(parents=True,exist_ok=False)
    with tarfile.open(folder/"source.tar.gz","w:gz") as archive:
        for path in ["Cargo.toml","Cargo.lock","rust-toolchain.toml",".cargo","src","tests","benchmarks/engine_driver.rs","benchmarks/sassy_engine.rs","benchmarks/run_iteration.py","benchmarks/cache_state.py","benchmarks/compare_site_files.py"]:
            archive.add(path)
        comparator_root=Path(args.sassy).resolve().parents[2]
        comparator_lock=comparator_root/"Cargo.lock"
        if comparator_lock.exists():archive.add(comparator_lock,arcname="external-comparator-Cargo.lock")
        for source in ["Cargo.toml", "src/main.rs", "src/engine_driver.rs"]:
            path=comparator_root/source
            if path.exists():archive.add(path,arcname="external-comparator/"+source)
    info={"id":ident,"utc":now.isoformat(),"configuration":vars(args),"runs":{}}
    info["queries_sha256"]=digest(args.queries)
    reference_manifest=Path(args.reference).parent/"manifest.json"
    if reference_manifest.exists():info["reference_manifest"]=json.loads(reference_manifest.read_text())
    if Path(args.reference).stat().st_size<100_000_000:info["reference_sha256"]=digest(args.reference)
    info["cache_policy"]="fresh processes; OS page cache not flushed; per-repetition timings retained"
    if args.cold:info["cache_policy"]="POSIX_FADV_DONTNEED on benchmark reference/index/query files before each engine; measured residual residency retained; subsequent repetitions warm"
    (folder/"started.json").write_text(json.dumps(info,indent=2)+"\n")
    options=["--reference",args.reference,"--queries",args.queries,"--mode",args.mode,"--queries-limit",str(args.n),"-k",str(args.k),"--repetitions",str(args.repetitions)]
    for name,binary in [("sassy",args.sassy),("ooff",args.native)]:
        if name=="sassy" and args.reuse_sassy:
            baseline=json.loads(Path(args.reuse_sassy).read_text())
            assert not args.cold and not args.site_tuples, "fresh measurement required for cold or tuple comparison"
            for field in ["reference","queries","mode","n","k"]:
                assert baseline["configuration"][field]==getattr(args,field), ("baseline workload mismatch",field)
            assert baseline["queries_sha256"]==info["queries_sha256"]
            assert baseline["runs"]["sassy"]["returncode"]==0
            info["runs"]["sassy"]=baseline["runs"]["sassy"]
            info["sassy_measurement_reused_from"]=args.reuse_sassy
            info["claim_scope"]="development ratio against retained baseline; fresh paired measurement required for final acceptance"
            continue
        stats=folder/(name+".time.json")
        engine_options=(["search","--index",args.index] if name=="ooff" and args.index else [])+options
        if name=="ooff" and args.distance_strata:engine_options.append("--distance-strata")
        if name=="ooff" and args.mmap:engine_options.append("--mmap")
        if name=="ooff" and args.automaton:engine_options.append("--automaton")
        if name=="ooff" and args.reverse_index:engine_options.extend(["--reverse-index",args.reverse_index])
        if args.site_tuples:engine_options.extend(["--site-tuples",str(folder/(name+".sites.csv"))])
        command=["/usr/bin/time","-f",'{"max_rss_kib":%M,"wall_seconds":%e,"user_seconds":%U,"system_seconds":%S}',"-o",str(stats),binary]+engine_options
        cache_residency=None
        if args.cold:
            from cache_state import evict
            cache_paths=[Path(args.reference),Path(args.queries)]
            for directory in [args.index,args.reverse_index]:
                if directory:cache_paths.extend(p for p in Path(directory).iterdir() if p.is_file())
            cache_residency=evict(cache_paths)
        start=time.monotonic()
        result={"command":command,"binary_sha256":digest(binary),"cache_residency_before_process":cache_residency}
        with (folder/(name+".json")).open("x") as stdout,(folder/(name+".stderr")).open("x") as stderr:
            process=subprocess.Popen(command,stdout=stdout,stderr=stderr,start_new_session=True)
            result["pid"]=process.pid
            (folder/(name+".process.json")).write_text(json.dumps(result,indent=2)+"\n")
            try:
                result["returncode"]=process.wait(timeout=args.timeout)
                result["timeout"]=False
            except subprocess.TimeoutExpired:
                os.killpg(process.pid,signal.SIGTERM)
                process.wait()
                result.update(returncode=process.returncode,timeout=True)
        result["elapsed_seconds"]=time.monotonic()-start
        if stats.exists():
            try:result["resources"]=json.loads(stats.read_text().splitlines()[-1])
            except (ValueError,IndexError):pass
        if result["returncode"]==0:
            result["output"]=json.loads((folder/(name+".json")).read_text())
            result["median_search_seconds"]=statistics.median(r["search_seconds"] for r in result["output"]["runs"])
        info["runs"][name]=result
        (folder/"result.json").write_text(json.dumps(info,indent=2)+"\n")
        print(json.dumps({"engine":name,"elapsed":result["elapsed_seconds"],"returncode":result["returncode"],"timeout":result["timeout"]}),flush=True)
    a,b=info["runs"]["ooff"],info["runs"]["sassy"]
    complete=all(r.get("returncode")==0 and r.get("output",{}).get("complete") for r in [a,b])
    equal=False
    if complete:
        outputs=[r["output"] for r in [a,b]]
        expected=outputs[0]["runs"][0]
        equal=all(run["counts"]==expected["counts"] and run["signature"]==expected["signature"] for out in outputs for run in out["runs"])
        equal &= outputs[0]["query_ids"]==outputs[1]["query_ids"] and outputs[0]["reference_bases"]==outputs[1]["reference_bases"]
    if args.site_tuples and complete:
        from compare_site_files import compare
        comparison=compare(folder/"ooff.sites.csv",folder/"sassy.sites.csv",folder/"site-comparison-buckets")
        expected_rows=sum(sum(r["counts"]) for r in a["output"]["runs"])
        exact_equal = comparison["equal"] and comparison["unique"] and comparison["left_rows"]==expected_rows
        info["exact_site_tuples_equal_and_unique"] = exact_equal
        info["site_tuples"] = comparison["left_rows"]
        info["site_comparison"] = comparison
        equal &= exact_equal
    speedup=b["median_search_seconds"]/a["median_search_seconds"] if complete and equal else None
    info.update(complete=complete,matching_counts_and_signatures=equal,speedup=speedup)
    if complete:
        info["actual_queries"]=len(a["output"]["query_ids"])
        info["first_batch_speedup"]=(b["output"]["load_seconds"]+b["output"]["runs"][0]["search_seconds"])/(a["output"]["load_seconds"]+a["output"]["runs"][0]["search_seconds"])
        info["process_speedup"]=b["resources"]["wall_seconds"]/a["resources"]["wall_seconds"]
        if a.get("output",{}).get("index_preparation_seconds") is not None:
            info["index_build_seconds"]=a["output"]["index_preparation_seconds"]
    (folder/"result.json").write_text(json.dumps(info,indent=2)+"\n")
    def fmt(x):return "—" if x is None else f"{x:.6f}"
    rss=max(r.get("resources",{}).get("max_rss_kib",0) for r in [a,b])/1024
    load=a.get("output",{}).get("load_seconds")
    state="matched; complete" if complete and equal else "MISMATCH" if complete else "FAILED/TIMEOUT"
    row=f'| {ident} | {now.strftime("%H:%M:%S")} | {args.label} | {args.reference}; n={info.get("actual_queries",args.n)}; k={args.k} | {args.mode}; 1 | load {fmt(load)} | {fmt(a.get("median_search_seconds"))} | {fmt(b.get("median_search_seconds"))} | {fmt(speedup)} | {rss:.1f} | {state} | [{ident}]({folder}/result.json) |\n'
    ledger=Path("BENCHMARKS.md")
    content=ledger.read_text()
    content=content.replace("\n## Interpretation",row+"\n## Interpretation",1)
    ledger.write_text(content)
    with open("benchmarks/iterations/runs.jsonl","a") as f:f.write(json.dumps(info)+"\n")
    subprocess.run(["python3","benchmarks/refresh_ledger.py"],check=True)
    print(row,flush=True)
    if not (complete and equal):
        raise SystemExit(1)


if __name__=="__main__":main()
