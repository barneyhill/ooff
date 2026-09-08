// External EC2 comparator only. This is never linked into the ooff crate.
use clap::Parser;
use sassy::{EncodedPatterns, Searcher, profiles::Iupac};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    reference: PathBuf,
    #[arg(long)]
    queries: PathBuf,
    #[arg(long, default_value_t = 8)]
    threads: usize,
    #[arg(long, default_value_t = 65536)]
    chunk_bases: usize,
    #[arg(short = 'k', default_value_t = 3)]
    k: usize,
}

fn fasta(path: &PathBuf) -> Vec<(String, Vec<u8>)> {
    let mut records: Vec<(String, Vec<u8>)> = Vec::new();
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        if let Some(id) = line.strip_prefix('>') {
            records.push((id.split_whitespace().next().unwrap().to_owned(), Vec::new()));
        } else {
            records
                .last_mut()
                .expect("FASTA header")
                .1
                .extend(line.trim().bytes().map(|b| b.to_ascii_uppercase()));
        }
    }
    records
}

type Witness = (usize, usize, usize, usize);
fn screen(
    queries: &[(String, Vec<u8>)],
    reference: &[(String, Vec<u8>)],
    mut active: Vec<usize>,
    k: usize,
    chunk: usize,
) -> Vec<Witness> {
    let mut witnesses = Vec::new();
    let mut searcher = Searcher::<Iupac>::new_fwd().with_max_n_frac(0.0);
    let encode = |searcher: &mut Searcher<Iupac>, active: &[usize]| -> EncodedPatterns<Iupac> {
        searcher.encode_patterns(
            &active
                .iter()
                .map(|&i| queries[i].1.clone())
                .collect::<Vec<_>>(),
        )
    };
    let mut encoded = encode(&mut searcher, &active);
    for (rid, (_, sequence)) in reference.iter().enumerate() {
        let mut lo = 0;
        while lo < sequence.len() {
            if !matches!(sequence[lo], b'A' | b'C' | b'G' | b'T') {
                lo += 1;
                continue;
            }
            let mut end = lo;
            while end < sequence.len() && matches!(sequence[end], b'A' | b'C' | b'G' | b'T') {
                end += 1;
            }
            for core in (lo..end).step_by(chunk) {
                let a = core.saturating_sub(20 + k).max(lo);
                let b = (core + chunk).min(end);
                let hits = searcher.search_encoded_patterns(&encoded, &sequence[a..b], k);
                let mut retired = vec![false; active.len()];
                for hit in hits {
                    if retired[hit.pattern_idx] {
                        continue;
                    }
                    let finish = a + hit.text_end;
                    if finish <= core {
                        continue;
                    }
                    let q = active[hit.pattern_idx];
                    let verifier = oofft::IntervalDistance::new(&queries[q].1);
                    for span in 20 - k..=20 + k {
                        if span > finish - a {
                            continue;
                        }
                        let start = finish - span;
                        if verifier.distance(&sequence[start..finish]) <= k {
                            witnesses.push((q, rid, start, finish));
                            retired[hit.pattern_idx] = true;
                            break;
                        }
                    }
                }
                if retired.iter().any(|&r| r) {
                    active = active
                        .into_iter()
                        .enumerate()
                        .filter_map(|(i, q)| (!retired[i]).then_some(q))
                        .collect();
                    if active.is_empty() {
                        return witnesses;
                    }
                    encoded = encode(&mut searcher, &active);
                }
            }
            lo = end;
        }
    }
    witnesses
}

fn main() {
    let args = Args::parse();
    assert!(args.threads > 0 && args.chunk_bases > 0 && args.k <= 3);
    // Inputs are already reverse-complemented ASOs and the common eligible RNA reference.
    let queries = fasta(&args.queries);
    let reference = fasta(&args.reference);
    assert!(!queries.is_empty() && !reference.is_empty());
    assert!(queries.iter().all(|(_,q)| q.len()==20 && q.iter().all(|b| matches!(b,b'A'|b'C'|b'G'|b'T'))));
    let workers = args.threads.min(queries.len());
    let mut witnesses = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|worker| {
                let active = (worker..queries.len()).step_by(workers).collect();
                let queries = &queries;
                let reference = &reference;
                let args = &args;
                scope.spawn(move || screen(queries, reference, active, args.k, args.chunk_bases))
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect::<Vec<_>>()
    });
    witnesses.sort_unstable();
    let mut out = BufWriter::new(std::io::stdout().lock());
    for (q, r, a, b) in &witnesses {
        writeln!(
            out,
            "{}\t{}\t{}\t{}",
            queries[*q].0,
            reference[*r].0,
            a + 1,
            b
        )
        .unwrap();
    }
    out.flush().unwrap();
    eprintln!(
        "{}",
        serde_json::json!({"complete":true,"engine":"sassy2-0.2.6","threads":workers,"queries":queries.len(),"witnesses":witnesses.len(),"index":false})
    );
}
