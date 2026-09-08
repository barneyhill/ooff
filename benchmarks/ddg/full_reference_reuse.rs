//! Exact full-sequence pair census of a retained discovery CSV (not energy timing).
use rustc_hash::{FxHashMap, FxHashSet};
use serde_json::json;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    time::Instant,
};
fn fasta(path: &str) -> Vec<(String, Vec<u8>)> {
    let mut records: Vec<(String, Vec<u8>)> = Vec::new();
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        if let Some(id) = line.strip_prefix('>') {
            records.push((id.to_owned(), Vec::new()));
        } else {
            records
                .last_mut()
                .unwrap()
                .1
                .extend(line.trim().bytes().map(|b| match b.to_ascii_uppercase() {
                    b'T' => b'U',
                    x => x,
                }));
        }
    }
    records
}
fn unpack(key: u128) -> Vec<u8> {
    (0..23)
        .map(|i| ((key >> (3 * (22 - i))) & 7) as usize)
        .take_while(|&b| b != 0)
        .map(|b| b"?ACGUN"[b])
        .collect()
}
fn expand(directory: &str, output: &str) {
    let directory = std::path::Path::new(directory);
    let asos: Vec<String> =
        serde_json::from_slice(&std::fs::read(directory.join("asos.json")).unwrap()).unwrap();
    let packed = std::fs::read(directory.join("unique-pairs.u128le")).unwrap();
    assert_eq!(packed.len() % 16, 0);
    let mut out = BufWriter::new(
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(output)
            .unwrap(),
    );
    for bytes in packed.chunks_exact(16) {
        let key = u128::from_le_bytes(bytes.try_into().unwrap());
        let qid = (key >> 69) as usize;
        let mut target = unpack(key);
        target.reverse();
        writeln!(out,"{}",json!({"aso":asos[qid],"target":String::from_utf8(target).unwrap(),"canonical_query":qid})).unwrap();
    }
    out.flush().unwrap();
    println!("Expanded {} exact pairs", packed.len() / 16);
}

fn frontier_stats(directory: &str) {
    let packed =
        std::fs::read(std::path::Path::new(directory).join("unique-pairs.u128le")).unwrap();
    assert_eq!(packed.len() % 16, 0);
    let mut previous: Option<u128> = None;
    let mut columns = 0u64;
    let mut shared = 0u64;
    let mut depth_counts = [0u64; 23];
    let mut nodes = 0u64;
    let mut padded_nodes = 0u64;
    for bytes in packed.chunks_exact(16) {
        let key = u128::from_le_bytes(bytes.try_into().unwrap());
        let length = unpack(key).len();
        let common = if let Some(prev) = previous {
            assert!(prev < key);
            if prev >> 69 == key >> 69 {
                (((key ^ prev).leading_zeros() as usize - 59) / 3).min(length)
            } else {
                nodes += depth_counts.iter().sum::<u64>();
                padded_nodes += depth_counts
                    .iter()
                    .map(|n| n.div_ceil(16) * 16)
                    .sum::<u64>();
                depth_counts.fill(0);
                0
            }
        } else {
            0
        };
        columns += length as u64;
        shared += common as u64;
        for count in &mut depth_counts[common..length] {
            *count += 1;
        }
        previous = Some(key);
    }
    nodes += depth_counts.iter().sum::<u64>();
    padded_nodes += depth_counts
        .iter()
        .map(|n| n.div_ceil(16) * 16)
        .sum::<u64>();
    assert_eq!(nodes, columns - shared);
    println!(
        "{}",
        json!({"pairs":packed.len()/16,"independent_columns":columns,"trie_nodes":nodes,"frontier_columns_with_16_lane_padding":padded_nodes,"ideal_state_reduction":columns as f64/nodes as f64,"ideal_state_reduction_with_padding":columns as f64/padded_nodes as f64,"scope":"Work-count model only; assumes separate lookahead handling and shared ancestor DP. No energy kernel implemented or runtime speedup claimed."})
    );
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 3 && args[1] == "frontier-stats" {
        frontier_stats(&args[2]);
        return;
    }
    if args.len() == 4 && args[1] == "expand" {
        expand(&args[2], &args[3]);
        return;
    }
    assert_eq!(
        args.len(),
        6,
        "reference.fa queries.fa sites.csv output-directory sample-count"
    );
    let started = Instant::now();
    let reference = fasta(&args[1]);
    let queries = fasta(&args[2]);
    let sample_count: usize = args[5].parse().unwrap();
    let out = std::path::Path::new(&args[4]);
    std::fs::create_dir(out).unwrap();
    let mut canonical = FxHashMap::default();
    let mut asos = Vec::new();
    let ids: Vec<_> = queries
        .iter()
        .map(|(_, a)| {
            let next = canonical.len();
            *canonical.entry(a.clone()).or_insert_with(|| {
                asos.push(a.clone());
                next
            })
        })
        .collect();
    assert!(u32::try_from(asos.len()).is_ok());
    let mut counts = vec![0u64; asos.len()];
    let mut unique = FxHashSet::<u128>::default();
    let mut intervals = 0u64;
    let mut ambiguous = 0u64;
    let mut stopped_at_next_repetition = false;
    let mut line = String::new();
    let mut input = BufReader::with_capacity(1 << 20, File::open(&args[3]).unwrap());
    while input.read_line(&mut line).unwrap() != 0 {
        let row: Vec<usize> = line
            .trim_end()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        assert_eq!(row.len(), 6);
        if row[0] != 0 {
            assert_eq!(row[0], 1);
            stopped_at_next_repetition = true;
            break;
        }
        let (qid, rid, start, end) = (row[1], row[2], row[3], row[4]);
        assert!(row[5] <= 3);
        let target = &reference[rid].1[start..end];
        assert!(
            (17..=23).contains(&target.len()),
            "unexpected target length"
        );
        let mut key = (ids[qid] as u128) << 69;
        let mut has_n = false;
        for (i, &b) in target.iter().rev().enumerate() {
            let code = match b {
                b'A' => 1u128,
                b'C' => 2,
                b'G' => 3,
                b'U' => 4,
                b'N' => {
                    has_n = true;
                    5
                }
                _ => panic!("invalid reference base"),
            };
            key |= code << (3 * (22 - i));
        }
        ambiguous += u64::from(has_n);
        counts[ids[qid]] += 1;
        unique.insert(key);
        intervals += 1;
        if intervals.is_multiple_of(5_000_000) {
            eprintln!(
                "{intervals} intervals; {} unique pairs; {:.1}s",
                unique.len(),
                started.elapsed().as_secs_f64()
            );
        }
        line.clear();
    }
    let mut keys: Vec<_> = unique.into_iter().collect();
    keys.sort_unstable();
    let mut distinct = vec![0usize; asos.len()];
    let mut columns = 0u64;
    let mut reused = 0u64;
    let mut previous: Option<(u128, usize, usize)> = None;
    let mut lengths = vec![0u64; 24];
    for &key in &keys {
        let qid = (key >> 69) as usize;
        distinct[qid] += 1;
        let target = unpack(key);
        let len = target.len();
        columns += len as u64;
        lengths[len] += 1;
        if let Some((prev, prev_q, prev_len)) = previous
            && qid == prev_q
        {
            let common = (((key ^ prev).leading_zeros() as usize - 59) / 3)
                .min(len)
                .min(prev_len);
            reused += common.saturating_sub(1) as u64;
        }
        previous = Some((key, qid, len));
    }
    let n = sample_count.min(keys.len());
    let mut sample = BufWriter::new(File::create(out.join("sample-pairs.jsonl")).unwrap());
    for i in 0..n {
        let key = keys[i * (keys.len() - 1) / (n - 1).max(1)];
        let qid = (key >> 69) as usize;
        let mut target = unpack(key);
        target.reverse();
        writeln!(sample,"{}",json!({"aso":String::from_utf8_lossy(&asos[qid]),"target":String::from_utf8(target).unwrap(),"canonical_query":qid})).unwrap();
    }
    sample.flush().unwrap();
    // Retain exact sorted keys so larger samples and scheduling studies do not
    // need to rescan all 45 million intervals. Encoding is documented above.
    let mut packed = BufWriter::new(File::create(out.join("unique-pairs.u128le")).unwrap());
    for key in &keys {
        packed.write_all(&key.to_le_bytes()).unwrap();
    }
    packed.flush().unwrap();
    std::fs::write(
        out.join("asos.json"),
        serde_json::to_vec(
            &asos
                .iter()
                .map(|a| String::from_utf8_lossy(a))
                .collect::<Vec<_>>(),
        )
        .unwrap(),
    )
    .unwrap();
    let record = json!({"complete":true,"measurement_kind":"sequence_reuse_census","seconds":started.elapsed().as_secs_f64(),"reference_records":reference.len(),"reference_bases":reference.iter().map(|r|r.1.len()).sum::<usize>(),"queries":queries.len(),"distinct_asos":asos.len(),"repetition":0,"stopped_at_next_repetition":stopped_at_next_repetition,"intervals":intervals,"ambiguous_intervals":ambiguous,"distinct_sequence_pairs":keys.len(),"deduplication_factor":intervals as f64/keys.len() as f64,"prefix_columns":columns,"prefix_columns_reusable":reused,"scalar_prefix_work_factor":columns as f64/(columns-reused) as f64,"length_counts":lengths,"per_aso_intervals":counts,"per_aso_distinct_pairs":distinct,"sample_pairs":n});
    std::fs::write(
        out.join("census.json"),
        serde_json::to_vec_pretty(&record).unwrap(),
    )
    .unwrap();
    println!("{record}");
}
