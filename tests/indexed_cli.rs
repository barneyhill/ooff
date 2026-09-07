use serde_json::Value;
use std::{
    fs,
    path::Path,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

fn run(mode: &str, reference: &Path, index: Option<&Path>, k: usize, extra: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ooff"));
    command
        .args([mode, "--queries", "fixtures/queries.jsonl", "--reference"])
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
        "ooff-index-cli-{}-{}",
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
    let build = Command::new(env!("CARGO_BIN_EXE_ooff-index"))
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
    let built = Command::new(env!("CARGO_BIN_EXE_ooff-index"))
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
    let cached = Command::new(env!("CARGO_BIN_EXE_ooff-index"))
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
        let search = |paired: bool| {
            let mut command = Command::new(env!("CARGO_BIN_EXE_ooff-index"));
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
    ooff::indexed::AnnotationCache::create(&index, &reference, &annotation_copy, &copied_cache)
        .unwrap();
    // SAFETY: the uniquely owned files are unchanged while this object lives.
    let opened = unsafe {
        ooff::indexed::ReferenceIndex::open_cached_immutable(
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
            ooff::indexed::ReferenceIndex::open_cached_immutable(
                &index,
                &reference,
                &annotation_copy,
                Some(&copied_cache),
            )
        }
        .is_err()
    );
    // The subprocess has closed its maps. A changed source must be rejected.
    fs::write(&reference, format!("{fasta}N\n")).unwrap();
    let changed = run("screen", &reference, Some(&index), 3, &[]);
    assert!(!changed.status.success());
    assert!(changed.stdout.is_empty());
}
