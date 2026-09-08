use serde_json::Value;
use std::{
    fs,
    path::Path,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

fn run(mode: &str, reference: &Path, index: Option<&Path>, k: usize, extra: &[&str]) -> Output {
    run_queries(
        mode,
        reference,
        index,
        k,
        extra,
        Path::new("fixtures/queries.jsonl"),
    )
}
fn run_queries(
    mode: &str,
    reference: &Path,
    index: Option<&Path>,
    k: usize,
    extra: &[&str],
    queries: &Path,
) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_oofft"));
    if !mode.is_empty() {
        command.arg(mode);
    }
    command
        .arg("--queries")
        .arg(queries)
        .arg("--reference")
        .arg(reference)
        .args([
            "--policy",
            "other-gene",
            "--reference-release",
            "fixture-v1",
            "--scope",
            "synthetic",
            "--biotype-policy",
            "all-fixture-records",
            "-k",
        ])
        .arg(k.to_string());
    if let Some(index) = index {
        command
            .arg("--index")
            .arg(index)
            .args(["--annotations", "fixtures/reference.jsonl"]);
    }
    command.args(extra).output().unwrap()
}
fn rows(output: &Output) -> Vec<Value> {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

#[test]
fn indexed_cli_matches_exhaustive_cli_with_wrapped_fasta() {
    let root = std::env::temp_dir().join(format!(
        "oofft-index-cli-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&root).unwrap();
    let reference = root.join("reference.fa");
    let index = root.join("index");
    let mut fasta = String::new();
    for line in fs::read_to_string("fixtures/reference.jsonl")
        .unwrap()
        .lines()
    {
        let r: Value = serde_json::from_str(line).unwrap();
        let genes = r["genes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|g| g.as_str().unwrap())
            .collect::<Vec<_>>()
            .join(",");
        fasta.push_str(&format!(">{}|{genes}\n", r["id"].as_str().unwrap()));
        for chunk in r["sequence"].as_str().unwrap().as_bytes().chunks(7) {
            fasta.push_str(std::str::from_utf8(chunk).unwrap());
            fasta.push('\n');
        }
    }
    fs::write(&reference, &fasta).unwrap();
    let build = Command::new(env!("CARGO_BIN_EXE_oofft-index"))
        .args(["build", "--reference"])
        .arg(&reference)
        .arg("--output")
        .arg(&index)
        .args(["--shard-bases", "25"])
        .env("OMP_NUM_THREADS", "2")
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    for k in 0..=3 {
        let ordinary = rows(&run(
            "report",
            Path::new("fixtures/reference.jsonl"),
            None,
            k,
            &[],
        ));
        let indexed = rows(&run("report", &reference, Some(&index), k, &[]));
        let sites = |r: &Vec<Value>| {
            let mut s: Vec<_> = r
                .iter()
                .filter(|r| r["type"] == "site")
                .map(Value::to_string)
                .collect();
            s.sort();
            s
        };
        assert_eq!(sites(&ordinary), sites(&indexed));
        let summary = |r: &Vec<Value>| {
            r.iter()
                .filter(|r| r["type"] == "query_summary")
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(summary(&ordinary), summary(&indexed));
        let plain_counts = rows(&run(
            "",
            Path::new("fixtures/reference.jsonl"),
            None,
            k,
            &["--genes", "--chunk-bases", "3"],
        ));
        let indexed_counts = rows(&run(
            "",
            &reference,
            Some(&index),
            k,
            &["--genes", "--threads", "2"],
        ));
        assert_eq!(summary(&plain_counts), summary(&indexed_counts));
        assert_eq!(indexed_counts[0]["mode"], "summary");
        assert!(!indexed_counts.iter().any(|r| r["type"] == "site"));
        for count in summary(&indexed_counts) {
            let mut unique = std::collections::BTreeMap::<String, usize>::new();
            let mut genes = std::collections::BTreeSet::new();
            for site in ordinary
                .iter()
                .filter(|r| r["type"] == "site" && r["query_id"] == count["query_id"])
            {
                let key =
                    serde_json::json!([site["contig"], site["strand"], site["genomic_blocks"]])
                        .to_string();
                let d = site["alignment"]["edit_distance"].as_u64().unwrap() as usize;
                unique
                    .entry(key)
                    .and_modify(|old| *old = (*old).min(d))
                    .or_insert(d);
                for gene in site["offtarget_genes"].as_array().unwrap() {
                    genes.insert(gene.as_str().unwrap());
                }
            }
            let mut bins = [0u64; 4];
            for &d in unique.values() {
                bins[d] += 1;
            }
            for (d, expected) in bins.iter().enumerate() {
                assert_eq!(
                    count["edit_distance_counts"][d.to_string()],
                    if d <= k {
                        serde_json::json!(expected)
                    } else {
                        Value::Null
                    }
                );
            }
            assert_eq!(count["total_sites"], unique.len());
            assert_eq!(count["offtarget_genes"], serde_json::json!(genes));
            assert!(count.get("ddg").is_none());
        }
        let record_counts = rows(&run(
            "summary",
            &reference,
            Some(&index),
            k,
            &["--count-unit", "record-interval", "--threads", "1"],
        ));
        for count in summary(&record_counts) {
            let n = ordinary
                .iter()
                .filter(|r| r["type"] == "site" && r["query_id"] == count["query_id"])
                .count();
            assert_eq!(count["total_sites"], n);
            assert!(count.get("offtarget_genes").is_none());
        }
        let flagged = rows(&run("", &reference, Some(&index), k, &["--sites"]));
        assert_eq!(sites(&flagged), sites(&indexed));

        let screen = rows(&run("screen", &reference, Some(&index), k, &[]));
        assert_eq!(screen.iter().filter(|r| r["type"] == "site").count(), 1);
        assert_eq!(screen.last().unwrap()["type"], "run_complete");
    }
    let capped = rows(&run(
        "report",
        &reference,
        Some(&index),
        3,
        &["--max-sites", "1"],
    ));
    assert_eq!(
        capped
            .iter()
            .find(|r| r["type"] == "query_summary")
            .unwrap()["status"],
        "incomplete"
    );
    let reverse = root.join("reverse");
    let built = Command::new(env!("CARGO_BIN_EXE_oofft-index"))
        .args(["build", "--reference"])
        .arg(&reference)
        .arg("--output")
        .arg(&reverse)
        .args(["--shard-bases", "25", "--reverse-records"])
        .env("OMP_NUM_THREADS", "2")
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    assert!(
        !run("screen", &reference, Some(&reverse), 3, &[])
            .status
            .success()
    );
    let queries_fa = root.join("queries.fa");
    let mut query_fasta = String::new();
    for line in fs::read_to_string("fixtures/queries.jsonl")
        .unwrap()
        .lines()
    {
        let q: Value = serde_json::from_str(line).unwrap();
        let genes = q["intended_genes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|g| g.as_str().unwrap())
            .collect::<Vec<_>>()
            .join(",");
        query_fasta.push_str(&format!(
            ">{}|{}\n{}\n",
            q["id"].as_str().unwrap(),
            genes,
            q["sequence"].as_str().unwrap()
        ));
    }
    fs::write(&queries_fa, query_fasta).unwrap();
    let cache = root.join("annotations-cache.json");
    let cached = Command::new(env!("CARGO_BIN_EXE_oofft-index"))
        .args(["cache-annotations", "--index"])
        .arg(&index)
        .arg("--reference")
        .arg(&reference)
        .args(["--annotations", "fixtures/reference.jsonl", "--output"])
        .arg(&cache)
        .output()
        .unwrap();
    assert!(
        cached.status.success(),
        "{}",
        String::from_utf8_lossy(&cached.stderr)
    );
    for k in 0..=3 {
        let forward = rows(&run(
            "summary",
            &reference,
            Some(&index),
            k,
            &["--threads", "1"],
        ));
        let paired = rows(&run(
            "summary",
            &reference,
            Some(&index),
            k,
            &[
                "--threads",
                "2",
                "--reverse-index",
                reverse.to_str().unwrap(),
                "--annotation-cache",
                cache.to_str().unwrap(),
            ],
        ));
        let summaries = |r: Vec<Value>| {
            r.into_iter()
                .filter(|r| r["type"] == "query_summary")
                .collect::<Vec<_>>()
        };
        assert_eq!(summaries(forward), summaries(paired));
        let search = |paired: bool| {
            let mut command = Command::new(env!("CARGO_BIN_EXE_oofft-index"));
            command
                .args(["search", "--index"])
                .arg(&index)
                .arg("--reference")
                .arg(&reference)
                .arg("--queries")
                .arg(&queries_fa)
                .args(["--mode", "sites", "--repetitions", "1", "--mmap", "-k"])
                .arg(k.to_string());
            if paired {
                command.arg("--reverse-index").arg(&reverse);
            }
            let out = command.output().unwrap();
            assert!(
                out.status.success(),
                "{}",
                String::from_utf8_lossy(&out.stderr)
            );
            serde_json::from_slice::<Value>(&out.stdout).unwrap()
        };
        let (plain, paired) = (search(false), search(true));
        assert_eq!(plain["runs"][0]["counts"], paired["runs"][0]["counts"]);
        assert_eq!(
            plain["runs"][0]["signature"],
            paired["runs"][0]["signature"]
        );
        let ordinary = rows(&run("report", &reference, Some(&index), k, &[]));
        let paired = rows(&run(
            "report",
            &reference,
            Some(&index),
            k,
            &["--reverse-index", reverse.to_str().unwrap()],
        ));
        let site_rows = |rows: Vec<Value>| {
            let mut sites: Vec<_> = rows
                .into_iter()
                .filter(|r| r["type"] == "site")
                .map(|r| r.to_string())
                .collect();
            sites.sort();
            sites
        };
        let cached = rows(&run(
            "report",
            &reference,
            Some(&index),
            k,
            &[
                "--reverse-index",
                reverse.to_str().unwrap(),
                "--annotation-cache",
                cache.to_str().unwrap(),
            ],
        ));
        let expected = site_rows(ordinary);
        assert_eq!(expected, site_rows(paired));
        assert_eq!(expected, site_rows(cached));
    }
    let annotation_copy = root.join("annotations.jsonl");
    let copied_cache = root.join("copied-annotation-cache.json");
    fs::copy("fixtures/reference.jsonl", &annotation_copy).unwrap();
    oofft::indexed::AnnotationCache::create(&index, &reference, &annotation_copy, &copied_cache)
        .unwrap();
    // SAFETY: the uniquely owned files are unchanged while this object lives.
    let opened = unsafe {
        oofft::indexed::ReferenceIndex::open_cached_immutable(
            &index,
            &reference,
            &annotation_copy,
            Some(&copied_cache),
        )
    }
    .unwrap();
    drop(opened);
    fs::write(
        &annotation_copy,
        format!("{}\n", fs::read_to_string(&annotation_copy).unwrap()),
    )
    .unwrap();
    // SAFETY: the files remain immutable during this rejected open attempt.
    assert!(
        unsafe {
            oofft::indexed::ReferenceIndex::open_cached_immutable(
                &index,
                &reference,
                &annotation_copy,
                Some(&copied_cache),
            )
        }
        .is_err()
    );
    let compact = root.join("compact");
    let converted = Command::new(env!("CARGO_BIN_EXE_oofft-index"))
        .args(["compact", "--index"])
        .arg(&index)
        .arg("--output")
        .arg(&compact)
        .output()
        .unwrap();
    assert!(
        converted.status.success(),
        "{}",
        String::from_utf8_lossy(&converted.stderr)
    );
    for mode in ["summary", "screen", "report"] {
        for k in 0..=3 {
            let full = rows(&run(mode, &reference, Some(&index), k, &[]));
            let small = rows(&run(mode, &reference, Some(&compact), k, &[]));
            let stable = |rows: Vec<Value>| {
                let mut r = rows
                    .into_iter()
                    .filter(|r| r["type"] != "manifest" && r["type"] != "run_complete")
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>();
                r.sort();
                r
            };
            assert_eq!(stable(full), stable(small));
        }
    }
    let many_queries = root.join("many-queries.jsonl");
    let base: Vec<Value> = fs::read_to_string("fixtures/queries.jsonl")
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let input: String = (0..37)
        .map(|i| {
            let mut q = base[i % base.len()].clone();
            q["id"] = serde_json::json!(format!("query-{i}"));
            if i % 3 == 0 {
                q["sequence"] = serde_json::json!(&q["sequence"].as_str().unwrap()[..17]);
            }
            if i % 3 == 1 {
                q["sequence"] =
                    serde_json::json!(format!("{}AAA", q["sequence"].as_str().unwrap()));
            }

            q.to_string() + "\n"
        })
        .collect();
    fs::write(&many_queries, input).unwrap();
    let one = rows(&run_queries(
        "summary",
        &reference,
        Some(&index),
        3,
        &["--threads", "1", "--genes"],
        &many_queries,
    ));
    let four = rows(&run_queries(
        "summary",
        &reference,
        Some(&index),
        3,
        &["--threads", "4", "--genes"],
        &many_queries,
    ));
    let summaries = |rows: Vec<Value>| {
        rows.into_iter()
            .filter(|r| r["type"] == "query_summary")
            .collect::<Vec<_>>()
    };
    let plain = rows(&run_queries(
        "summary",
        Path::new("fixtures/reference.jsonl"),
        None,
        3,
        &["--genes", "--chunk-bases", "3"],
        &many_queries,
    ));
    let expected = summaries(one);
    assert_eq!(expected, summaries(four));
    assert_eq!(expected, summaries(plain));
    // The subprocess has closed its maps. A changed source must be rejected.
    fs::write(&reference, format!("{fasta}N\n")).unwrap();
    let changed = run("screen", &reference, Some(&index), 3, &[]);
    assert!(!changed.status.success());
    assert!(changed.stdout.is_empty());
}
