use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn parallel_queries_preserve_sites_and_screen_witnesses() {
    let root = std::env::temp_dir().join(format!(
        "ooff-parallel-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&root).unwrap();
    let reference = root.join("reference.fa");
    let queries = root.join("queries.fa");
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
        fasta.push_str(&format!(
            ">{}|{}\n{}\n",
            r["id"].as_str().unwrap(),
            genes,
            r["sequence"].as_str().unwrap()
        ));
    }
    fs::write(&reference, fasta).unwrap();
    let mut fasta = String::new();
    for copy in 0..3 {
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
            fasta.push_str(&format!(
                ">{}-{copy}|{}\n{}\n",
                q["id"].as_str().unwrap(),
                genes,
                q["sequence"].as_str().unwrap()
            ));
        }
    }
    fs::write(&queries, fasta).unwrap();
    for reverse in [false, true] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_ooff-index"));
        command
            .args(["build", "--reference"])
            .arg(&reference)
            .arg("--output")
            .arg(root.join(if reverse { "reverse" } else { "forward" }))
            .args(["--shard-bases", "100"]);
        if reverse {
            command.arg("--reverse-records");
        }
        let output = command.env("OMP_NUM_THREADS", "2").output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    for mode in ["sites", "screen"] {
        for k in 0..=3 {
            let mut baseline = None;
            for threads in [1, 2, 16] {
                let tuples = root.join(format!("{mode}-{k}-{threads}.csv"));
                let output = Command::new(env!("CARGO_BIN_EXE_ooff-index"))
                    .args(["search", "--index"])
                    .arg(root.join("forward"))
                    .arg("--reverse-index")
                    .arg(root.join("reverse"))
                    .arg("--reference")
                    .arg(&reference)
                    .arg("--queries")
                    .arg(&queries)
                    .args(["--mode", mode, "-k"])
                    .arg(k.to_string())
                    .arg("--threads")
                    .arg(threads.to_string())
                    .args(["--repetitions", "2", "--mmap"])
                    .arg("--site-tuples")
                    .arg(&tuples)
                    .output()
                    .unwrap();
                assert!(
                    output.status.success(),
                    "{}",
                    String::from_utf8_lossy(&output.stderr)
                );
                let mut actual: Value = serde_json::from_slice(&output.stdout).unwrap();
                actual.as_object_mut().unwrap().remove("load_seconds");
                actual.as_object_mut().unwrap().remove("threads");
                for run in actual["runs"].as_array_mut().unwrap() {
                    run.as_object_mut().unwrap().remove("search_seconds");
                    run.as_object_mut().unwrap().remove("query_seconds");
                }
                let raw = fs::read_to_string(tuples).unwrap();
                let rows: BTreeSet<_> = raw.lines().map(str::to_owned).collect();
                assert_eq!(rows.len(), raw.lines().count(), "duplicate parallel output");
                let result = (actual, rows);
                if let Some(expected) = &baseline {
                    assert_eq!(&result, expected);
                } else {
                    baseline = Some(result);
                }
            }
        }
    }
}
