#!/usr/bin/env python3
"""Render a plot-friendly ledger from immutable measured iteration outputs."""
import csv
import json
import statistics
from pathlib import Path

root=Path("benchmarks/iterations")
rows=[]
for path in sorted(root.glob("*/result.json")):
    x=json.loads(path.read_text())
    if x.get("kind") in ("comparator_build", "native_build"):
        rows.append(dict(id=x["id"],kind=x["kind"],label=x["configuration"]["label"],reference="",queries="",k="",mode=x["kind"].replace("_", " "),threads=x["configuration"].get("threads",4),preparation_seconds=x.get("wall_seconds",""),load_seconds="",ooff_seconds="",sassy_seconds="",speedup="",first_batch_speedup="",peak_rss_mib="",status="complete" if x["returncode"]==0 else "BUILD FAILED",artifact=str(path)))
        continue
    if x.get("kind")=="index_build":
        rows.append(dict(id=x["id"],kind="index_build",label=x["configuration"]["label"],reference=x["configuration"]["reference"],queries="",k="",mode="build",threads=x["configuration"].get("threads",8),preparation_seconds=x["wall_seconds"],load_seconds="",ooff_seconds="",sassy_seconds="",speedup="",first_batch_speedup="",peak_rss_mib="",status="complete" if x["returncode"]==0 else "FAILED",artifact=str(path)))
        continue
    if x.get("kind")=="production_cli":
        cfg=x["configuration"]; manifest=x.get("manifest",{}); done=x.get("run_complete",{})
        rows.append(dict(id=x["id"],kind="production_cli",label=cfg["label"],reference="data/reference-v1/reference.fa",queries=x["queries"],k=cfg["k"],mode=cfg["mode"]+" annotated CLI",threads=1,preparation_seconds="",load_seconds=manifest.get("index_load_seconds",""),ooff_seconds=done.get("search_and_output_seconds",""),sassy_seconds="",speedup="",first_batch_speedup="",peak_rss_mib="",status="complete" if x["returncode"]==0 and done else "FAILED/TIMEOUT",artifact=str(path)))
        continue
    if not {"ooff","sassy"}<=x.get("runs",{}).keys():continue
    a,b=x["runs"]["ooff"],x["runs"]["sassy"]
    config=x["configuration"]
    out=a.get("output",{})
    first=""
    if all(r.get("returncode")==0 for r in [a,b]):
        first=(b["output"]["load_seconds"]+b["output"]["runs"][0]["search_seconds"])/(a["output"]["load_seconds"]+a["output"]["runs"][0]["search_seconds"])
    rows.append(dict(id=x["id"],kind="search",label=config["label"]+(" [reused Sassy baseline]" if x.get("sassy_measurement_reused_from") else ""),reference=config["reference"],queries=len(out.get("query_ids",[])) or config["n"],k=config["k"],mode=config["mode"],threads=out.get("threads",1),preparation_seconds=out.get("index_preparation_seconds",""),load_seconds=out.get("load_seconds",""),ooff_seconds=a.get("median_search_seconds",""),sassy_seconds=b.get("median_search_seconds",""),speedup=x.get("speedup") or "",first_batch_speedup=first,peak_rss_mib=max(r.get("resources",{}).get("max_rss_kib",0) for r in [a,b])/1024,status="matched; complete" if x.get("complete") and x.get("matching_counts_and_signatures") else "MISMATCH" if x.get("complete") else "FAILED/TIMEOUT",artifact=str(path)))
# Extra CSV columns retain per-engine costs and result cardinality for plotting.
for row in rows:
    artifact=Path(row["artifact"])
    x=json.loads(artifact.read_text())
    if x.get("aborted_reason"):
        row["status"]="ABORTED: prerequisite failed"
    native=x.get("runs",{}).get("ooff",{})
    sassy=x.get("runs",{}).get("sassy",{})
    out=native.get("output",{})
    run=(out.get("runs") or [{}])[0]
    row["ooff_peak_rss_mib"]=native.get("resources",{}).get("max_rss_kib",0)/1024 if native else ""
    row["sassy_peak_rss_mib"]=sassy.get("resources",{}).get("max_rss_kib",0)/1024 if sassy else ""
    row["index_bytes"]=out.get("index_bytes",x.get("output",{}).get("index_bytes",x.get("output",{}).get("cache_bytes","")))
    row["reported_sites"]=sum(run["counts"]) if "counts" in run else x.get("run_complete",{}).get("reported_sites","")
    row["ooff_wall_seconds"]=native.get("resources",{}).get("wall_seconds",x.get("wall_seconds",""))
    row["sassy_wall_seconds"]=sassy.get("resources",{}).get("wall_seconds","")
    row["output_bytes"]=x.get("output_bytes","")
    row["exact_site_tuples_equal"]=x.get("exact_site_tuples_equal_and_unique","")
    row["reused_sassy_baseline"]=x.get("sassy_measurement_reused_from","")
    cfg=x.get("configuration",{})
    row["query_file"]=cfg.get("queries","")
    row["repetitions"]=cfg.get("repetitions",1 if row["kind"]=="production_cli" else "")
    row["writes_site_csv"]=cfg.get("site_tuples",False) if row["kind"]=="search" else ""
    row["cache_policy"]=x.get("cache_policy","")
    row["sassy_binary"]=cfg.get("sassy","")
    measured_engine_times=[r["engine_seconds"] for r in sassy.get("output",{}).get("runs",[]) if "engine_seconds" in r]
    row["sassy_engine_seconds"]=statistics.median(measured_engine_times) if measured_engine_times else ""
    row["ooff_first_search_seconds"]=(out.get("runs") or [{}])[0].get("search_seconds","")
    row["sassy_first_search_seconds"]=(sassy.get("output",{}).get("runs") or [{}])[0].get("search_seconds","")
    if row["kind"] in ["index_build","production_cli"]:
        resources=artifact.parent/"resources.txt"
        if resources.exists():
            for line in resources.read_text().splitlines():
                if "Maximum resident set size (kbytes):" in line:
                    rss=int(line.split(":")[-1])/1024
                    row["peak_rss_mib"]=rss
                    row["ooff_peak_rss_mib"]=rss
def fmt(x):return f"{x:.6f}" if isinstance(x,float) else str(x) if x not in ("",None) else "—"
table="## Iterations\n\n| ID | Change | Dataset | Queries | k | Mode / threads | Build s | Load s | ooff search s | Sassy search s | Search speedup | First-batch speedup | Peak RSS MiB (max of pair) | Result | Artifacts |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n"
for r in rows:
    table+="| "+" | ".join(map(fmt,[r["id"],r["label"],r["reference"],r["queries"],r["k"],f'{r["mode"]} / {r["threads"]}',r["preparation_seconds"],r["load_seconds"],r["ooff_seconds"],r["sassy_seconds"],r["speedup"],r["first_batch_speedup"],r["peak_rss_mib"],r["status"],f'[JSON]({Path(r["artifact"]).relative_to("benchmarks")})']))+" |\n"
p=Path("benchmarks/NATIVE_ITERATIONS.md");content=p.read_text();before=content.split("## Iterations",1)[0];after=content.split("## Interpretation",1)[1]
p.write_text(before+table+"\n## Interpretation"+after)
if rows:
    with (root/"summary.csv").open("w",newline="") as f:
        writer=csv.DictWriter(f,fieldnames=list(rows[0]));writer.writeheader();writer.writerows(rows)
print(f"Rendered {len(rows)} measured iterations with actual query counts.")
