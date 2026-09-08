#!/usr/bin/env python3
"""Full ASO walk using existing CLIs, eight worker slots, bounded report storage.
Every query belongs to exactly one chunk: no exact ASO/target cache reuse is lost
between chunks because ASO sequences are distinct. Every report and annotation
row is losslessly archived before its temporary uncompressed file is removed.
"""
import argparse, concurrent.futures, datetime, gzip, hashlib, json, os, subprocess, threading, time
from pathlib import Path
from full_walk import sha


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--gene',type=Path,required=True)
    p.add_argument('--reference-root',type=Path,required=True)
    p.add_argument('--index',type=Path,required=True)
    p.add_argument('--reverse-index',type=Path)
    p.add_argument('--platform-label',default='EC2 c7i.2xlarge, 8 hardware threads, 16 GiB')
    p.add_argument('--annotation-cache',type=Path,required=True)
    p.add_argument('--binary',type=Path,default=Path('target/release/oofft'))
    p.add_argument('--ddg-binary',type=Path,default=Path('target/release/oofft-ddg'))
    p.add_argument('--threads',type=int,default=8)
    p.add_argument('--chunk-queries',type=int,default=256)
    p.add_argument('--timeout',type=float,default=7200)
    p.add_argument('--output',type=Path,required=True)
    a=p.parse_args();assert min(a.threads,a.chunk_queries,a.timeout)>0
    root=a.output.resolve();root.mkdir(parents=True,exist_ok=False)
    start=time.perf_counter();deadline=start+a.timeout
    state=dict(label=root.name,platform=a.platform_label,threads=a.threads,
               complete=False,workload='All repeat-filtered SCN2A gene-body 20mers; all other-gene intervals k<=3 across full human RNA reference, native ddG; complete gzip JSONL outputs',
               query_mask_policy='Exclude windows overlapping lowercase or non-ACGT bases; retain repeats in searched reference',
               reference_scope='Ensembl110 GRCh38 human gene-body and transcript records; not intergenic chromosome sequence',
               index_policy='Prebuilt forward and reverse indexes; builds excluded' if a.reverse_index else 'Prebuilt forward index only; builds excluded',
               execution='Eight worker slots; each runs discovery, native ddG, gzip sequentially; each ASO occurs in only one chunk',
               ddg_sign='dg_other - dg_target',energy_cache_pairs_per_process=4000000,chunk_queries=a.chunk_queries,attempts=[],chunks=[])
    def save():
        (root/'result.json').write_text(json.dumps(state,indent=2)+'\n')
        lines=['# Complete SCN2A design walk benchmark','',state['workload'],'',f"Complete: {state['complete']}; {len(state['chunks'])} chunks completed.",'',
               '| Chunk | Queries | Sites | Discovery s | DDG s | Archive s |','|---|---:|---:|---:|---:|---:|']
        for r in sorted(state['chunks'],key=lambda r:r['chunk']):
            lines.append(f"| {r['chunk']} | {r['queries']} | {r['sites']} | {r['discovery_seconds']:.6f} | {r['ddg_seconds']:.6f} | {r['archive_seconds']:.6f} |")
        lines+=['','Stage times are per-worker durations and overlap. Total wall time is recorded separately. Every raw JSONL row is retained in gzip archives.','']
        (root/'iterations.md').write_text('\n'.join(lines))
    def command_run(command,out,err):
        t=time.perf_counter()
        with out.open('wb') as output,err.open('wb') as errors:
            result=subprocess.run(command,stdout=output,stderr=errors,timeout=max(1,deadline-time.perf_counter()))
        if result.returncode:raise RuntimeError(f'Command failed {command}: {err.read_text()[-2000:]}')
        return time.perf_counter()-t
    def archive(path):
        dest=path.with_suffix(path.suffix+'.gz');h=hashlib.sha256();size=0
        with path.open('rb') as src,dest.open('xb') as raw,gzip.GzipFile(filename='',fileobj=raw,mode='wb',compresslevel=1,mtime=0) as out:
            for block in iter(lambda:src.read(1024*1024),b''):
                h.update(block);size+=len(block);out.write(block)
        result=dict(path=str(dest),uncompressed_bytes=size,compressed_bytes=dest.stat().st_size,uncompressed_sha256=h.hexdigest())
        # This invocation's disposable spool only; lossless raw bytes retained.
        path.unlink()
        return result
    try:
        source=a.gene.read_text().splitlines();assert sum(l.startswith('>') for l in source)==1
        sequence=''.join(l.strip() for l in source if not l.startswith('>'))
        unique={}
        for i in range(len(sequence)-19):
            t=sequence[i:i+20]
            if set(t)<=set('ACGT'):unique.setdefault(t,[]).append(i)
        queries=[]
        with (root/'origins.jsonl').open('w') as out:
            for i,(target,positions) in enumerate(unique.items()):
                q=dict(id=f'scn2a_{i}',sequence=target.translate(str.maketrans('ACGT','TGCA'))[::-1],intended_genes=['ENSG00000136531'])
                queries.append(q);out.write(json.dumps(dict(id=q['id'],positions=positions))+'\n')
        (root/'queries.jsonl').write_text(''.join(json.dumps(q)+'\n' for q in queries))
        state.update(queries=len(queries),walk_positions=sum(map(len,unique.values())),gene_bases=len(sequence))
        state['sha256']={str(f):sha(f) for f in [a.gene,a.binary,a.ddg_binary,a.index/'manifest.json',a.annotation_cache,root/'queries.jsonl']}
        if a.reverse_index: state['sha256'][str(a.reverse_index/'manifest.json')]=sha(a.reverse_index/'manifest.json')
        state['runner_sha256']=sha(Path(__file__))
        state['setup_seconds']=time.perf_counter()-start;save()
        def worker(job):
            n,qs=job;t=time.perf_counter();folder=root/f'chunk-{n:04d}';folder.mkdir()
            if os.statvfs(root).f_bavail*os.statvfs(root).f_frsize<5*1024**3:raise RuntimeError('Less than5GiB free; stopping before storage exhaustion')
            qpath=folder/'queries.jsonl';qpath.write_text(''.join(json.dumps(q)+'\n' for q in qs))
            report=folder/'report.jsonl'
            discovery=[str(a.binary.resolve()),'report','--queries',str(qpath),'--reference',str(a.reference_root/'reference.fa'),
                       '--index',str(a.index),'--annotations',str(a.reference_root/'records.jsonl'),'--annotation-cache',str(a.annotation_cache),
                       '--policy','other-gene','--reference-release','Ensembl110-GRCh38','--scope',state['reference_scope'],
                       '--biotype-policy','retained reference-v1 manifest','-k','3']
            if a.reverse_index: discovery.extend(['--reverse-index',str(a.reverse_index)])
            ddg=[str(a.ddg_binary.resolve()),'--queries',str(qpath),'--reference',str(a.reference_root/'reference.fa'),'--sites',str(report),
                 '--energy-engine','rust','--threads','1','--timeout',str(max(1,deadline-time.perf_counter())),'--work-dir',str(folder/'ddg-work')]
            (folder/'commands.json').write_text(json.dumps(dict(discovery=discovery,ddg=ddg),indent=2))
            d=command_run(discovery,report,folder/'discovery.stderr')
            ddg[ddg.index('--timeout')+1]=str(max(1,deadline-time.perf_counter()))
            (folder/'commands.json').write_text(json.dumps(dict(discovery=discovery,ddg=ddg),indent=2))
            e=command_run(ddg,Path(os.devnull),folder/'ddg.stderr')
            manifest=json.loads((folder/'ddg-work/manifest.json').read_text());assert manifest['complete']
            arch_start=time.perf_counter()
            archives=[archive(report),archive(folder/'ddg-work/annotated.jsonl')]
            row=dict(chunk=n,queries=len(qs),sites=manifest['annotated_sites'],discovery_seconds=d,ddg_seconds=e,archive_seconds=time.perf_counter()-arch_start,
                     wall_seconds=time.perf_counter()-t,cache=manifest['energy_cache'],profile=manifest['profile'],archives=archives)
            (folder/'result.json').write_text(json.dumps(row,indent=2)+'\n')
            return row
        jobs=[(i//a.chunk_queries,queries[i:i+a.chunk_queries]) for i in range(0,len(queries),a.chunk_queries)]
        with concurrent.futures.ThreadPoolExecutor(max_workers=a.threads) as pool:
            pending={pool.submit(worker,j):j[0] for j in jobs}
            try:
                for f in concurrent.futures.as_completed(pending):
                    row=f.result();state['chunks'].append(row)
                    for stage in ('discovery','ddg','archive'):
                        state['attempts'].append(dict(engine='ooff',repetition=row['chunk'],measurement_kind=stage+'_worker',seconds=row[stage+'_seconds'],returncode=0))
                    save();print('CHUNK',row['chunk'],'sites',row['sites'],'done',len(state['chunks']),'of',len(jobs),'wall',time.perf_counter()-start,flush=True)
            except BaseException:
                for f in pending:f.cancel()
                raise
        state.update(complete=True,total_wall_seconds=time.perf_counter()-start,sites=sum(r['sites'] for r in state['chunks']))
        state['attempts'].append(dict(engine='ooff',repetition=0,measurement_kind='full_walk_end_to_end',seconds=state['total_wall_seconds'],returncode=0))
        print('COMPLETE',state['total_wall_seconds'],'seconds',state['sites'],'sites',flush=True)
    except BaseException as error:
        state.update(error=repr(error),elapsed_seconds=time.perf_counter()-start)
        raise
    finally:save()

if __name__=='__main__':main()
