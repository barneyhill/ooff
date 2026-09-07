// Shared input, policy, chunk ownership, interval verification and timing for
// the production-native and external Sassy benchmark engines.
use clap::Parser;
use serde_json::json;
use std::{fs::File, io::{BufRead,BufReader,BufWriter,Write}, path::PathBuf, time::Instant};

#[derive(Parser)]
struct BenchArgs {
    #[arg(long)] queries: PathBuf,
    #[arg(long)] site_tuples: Option<PathBuf>,
    #[arg(long)] reference: PathBuf,
    #[arg(short='k',default_value_t=3)] k: usize,
    #[arg(long,default_value="screen",value_parser=["screen","endpoints","sites"])] mode: String,
    #[arg(long,default_value_t=1000)] queries_limit: usize,
    #[arg(long,default_value_t=3)] repetitions: usize,
    #[arg(long,default_value_t=65536)] chunk_bases: usize,
    #[arg(long,default_value="ENSG00000136531")] intended_gene: String,
}

fn read_fasta(path: &std::path::Path) -> Vec<(String,Vec<u8>)> {
    let mut records: Vec<(String,Vec<u8>)> = Vec::new();
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let line=line.unwrap();
        if let Some(id)=line.strip_prefix('>') { records.push((id.to_owned(),Vec::new())); }
        else { records.last_mut().expect("FASTA header").1.extend(ooff::normalize(line.trim(),false).unwrap()); }
    }
    records
}

fn main() {
    let args=BenchArgs::parse();
    assert!(args.k<=3 && args.chunk_bases>0 && args.repetitions>0);
    let load=Instant::now();
    let input=read_fasta(&args.queries);
    let query_ids:Vec<_>=input.iter().take(args.queries_limit).map(|x|x.0.clone()).collect();
    let intended:Vec<Vec<String>>=query_ids.iter().map(|id|id.split_once('|').map(|(_,g)|g).unwrap_or(&args.intended_gene).split(',').map(str::to_owned).collect()).collect();
    let patterns:Vec<_>=input.iter().take(args.queries_limit).map(|x| {
        assert!(x.1.len()==20 && !x.1.contains(&b'N'));
        ooff::reverse_complement(&x.1)
    }).collect();
    assert!(!patterns.is_empty());
    let verifiers: Vec<_> = patterns.iter().map(|p| ooff::IntervalDistance::new(p)).collect();
    let reference=read_fasta(&args.reference);
    assert!(!reference.is_empty());
    let reference_bases:usize=reference.iter().map(|r|r.1.len()).sum();
    let load_seconds=load.elapsed().as_secs_f64();
    let mut tuples=args.site_tuples.as_ref().map(|p| BufWriter::new(std::fs::OpenOptions::new().write(true).create_new(true).open(p).unwrap()));
    let mut runs=Vec::new();
    for repetition in 0..args.repetitions {
        let started=Instant::now();
        let mut engine_seconds=0.0;
        let mut counts=vec![0u64;patterns.len()];
        let mut signature=vec![0u64;patterns.len()];
        let mut active:Vec<usize>=(0..patterns.len()).collect();
        let mut prepared=engine::Prepared::new(&patterns);
        for (rid,(header,text)) in reference.iter().enumerate() {
            let genes=header.split_once('|').expect("reference FASTA must annotate gene IDs").1;
            if active.iter().all(|&q|genes.split(',').all(|g|intended[q].iter().any(|i|i==g))) {continue;}
            let mut run_start=0;
            while run_start<text.len() {
                if text[run_start]==b'N' {run_start+=1;continue;}
                let mut run_end=run_start;
                while run_end<text.len() && text[run_end]!=b'N' {run_end+=1;}
                for a in (run_start..run_end).step_by(args.chunk_bases) {
                    let lo=a.saturating_sub(20+args.k).max(run_start);
                    let hi=(a+args.chunk_bases).min(run_end);
                    let engine_started=Instant::now();
                    let hits=prepared.find(&text[lo..hi],args.k,args.mode=="screen");
                    engine_seconds+=engine_started.elapsed().as_secs_f64();
                    for (q,end) in hits {
                        let original=active[q];
                        if genes.split(',').all(|g|intended[original].iter().any(|i|i==g)) {continue;}
                        let end=lo+end;
                        if end<=a {continue;}
                        if args.mode=="screen" {counts[original]=1;continue;}
                        if args.mode=="endpoints" {
                            counts[original]+=1;
                            signature[original]=signature[original].wrapping_add(((rid as u64)<<32) ^ end as u64);
                        } else {
                            for span in 20-args.k..=20+args.k {
                                if span>end-lo {continue;}
                                let start=end-span;
                                let d=verifiers[original].distance(&text[start..end]);
                                if d<=args.k {
                                    if let Some(out)=&mut tuples { writeln!(out,"{repetition},{original},{rid},{start},{end},{d}").unwrap(); }
                                    counts[original]+=1;
                                    signature[original]=signature[original].wrapping_add(((rid as u64)<<40)^((start as u64)<<8)^((span as u64)<<3)^d as u64);
                                }
                            }
                        }
                    }
                    if args.mode=="screen" && active.iter().any(|&q|counts[q]>0) {
                        active.retain(|&q|counts[q]==0);
                        if active.is_empty() {break;}
                        prepared=engine::Prepared::new(&active.iter().map(|&q|patterns[q].clone()).collect::<Vec<_>>());
                    }
                }
                if active.is_empty() {break;}
                run_start=run_end;
            }
            if active.is_empty() {break;}
        }
        if let Some(out)=&mut tuples {out.flush().unwrap();}
        runs.push(json!({"repetition":repetition,"search_seconds":started.elapsed().as_secs_f64(),"engine_seconds":engine_seconds,"counts":counts,"signature":signature}));
    }
    println!("{}",json!({"engine":engine::NAME,"mode":args.mode,"k":args.k,"threads":1,"query_ids":query_ids,"reference_bases":reference_bases,"reference_records":reference.len(),"load_seconds":load_seconds,"runs":runs,"complete":true}));
}
