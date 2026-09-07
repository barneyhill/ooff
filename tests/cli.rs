use serde_json::Value;
use std::{
    path::Path,
    process::{Command, Output},
};

fn invoke(mode: &str, reference: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ooff"))
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
        ])
        .args(extra)
        .output()
        .unwrap()
}

fn lines(output: &Output) -> Vec<Value> {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}

#[test]
fn screen_keeps_shared_gene_associations_and_witness_distance() {
    let output = invoke(
        "screen",
        Path::new("fixtures/reference.jsonl"),
        &["-k", "0"],
    );
    let rows = lines(&output);
    assert_eq!(rows[0]["max_edits"], 0);
    let sites: Vec<_> = rows.iter().filter(|r| r["type"] == "site").collect();
    assert_eq!(sites.len(), 1);
    assert_eq!(sites[0]["record_id"], "shared_junction");
    assert_eq!(sites[0]["offtarget_genes"], serde_json::json!(["other"]));
    assert_eq!(sites[0]["genes"].as_array().unwrap().len(), 2);
    assert_eq!(sites[0]["genomic_blocks"].as_array().unwrap().len(), 2);
    assert_eq!(sites[0]["witness_edit_distance"], 0);
    let summaries: Vec<_> = rows
        .iter()
        .filter(|r| r["type"] == "query_summary")
        .collect();
    assert_eq!(summaries[0]["status"], "offtarget_found");
    assert_eq!(summaries[1]["status"], "none_found_within_scope");
    assert_eq!(rows.last().unwrap()["type"], "run_complete");
}

#[test]
fn supplied_energy_is_preserved_without_filtering_sites() {
    let queries =
        std::env::temp_dir().join(format!("ooff-ddg-queries-{}.jsonl", std::process::id()));
    let mut input: Vec<Value> = std::fs::read_to_string("fixtures/queries.jsonl")
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    input[0]["ddg"] = serde_json::json!(99999.25);
    std::fs::write(
        &queries,
        input
            .iter()
            .map(|q| q.to_string() + "\n")
            .collect::<String>(),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_ooff"))
        .args(["report", "--queries"])
        .arg(&queries)
        .args([
            "--reference",
            "fixtures/reference.jsonl",
            "--policy",
            "other-gene",
            "--reference-release",
            "fixture-v1",
            "--scope",
            "synthetic",
            "--biotype-policy",
            "all-fixture-records",
            "-k",
            "0",
        ])
        .output()
        .unwrap();
    let actual = lines(&output);
    let baseline = lines(&invoke(
        "report",
        Path::new("fixtures/reference.jsonl"),
        &["-k", "0"],
    ));
    let site_rows = |rows: Vec<Value>| {
        rows.into_iter()
            .filter(|r| r["type"] == "site")
            .collect::<Vec<_>>()
    };
    let mut actual = site_rows(actual);
    assert!(!actual.is_empty());
    for site in &mut actual {
        assert_eq!(site["ddg"], 99999.25);
        site.as_object_mut().unwrap().remove("ddg");
    }
    assert_eq!(actual, site_rows(baseline));
}

#[test]
fn capped_report_is_incomplete_and_output_is_chunk_invariant() {
    let capped = lines(&invoke(
        "report",
        Path::new("fixtures/reference.jsonl"),
        &["--max-sites", "1"],
    ));
    let summary = capped
        .iter()
        .find(|r| r["type"] == "query_summary")
        .unwrap();
    assert_eq!(summary["status"], "incomplete");
    assert_eq!(summary["count_is_lower_bound"], true);
    let sites = |core| {
        let rows = lines(&invoke(
            "report",
            Path::new("fixtures/reference.jsonl"),
            &["--chunk-bases", core],
        ));
        let mut sites: Vec<_> = rows
            .iter()
            .filter(|r| r["type"] == "site")
            .map(|r| r.to_string())
            .collect();
        sites.sort();
        sites
    };
    assert_eq!(sites("1"), sites("65536"));
}

#[test]
fn unknown_reference_and_invalid_input_do_not_produce_clean_results() {
    let path =
        std::env::temp_dir().join(format!("ooff-unknown-fixture-{}.jsonl", std::process::id()));
    // Retain the fixture: project policy forbids deleting files without permission.
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap();
    use std::io::Write;
    writeln!(f, "{}", serde_json::json!({"id":"unknown", "sequence":"NNNNNNNNNNNNNNNNNNNN", "genes":["other"], "contig":"x", "strand":"+", "blocks":[{"start":0,"end":20}]})).unwrap();
    let output = lines(&invoke("screen", &path, &[]));
    assert!(
        output
            .iter()
            .filter(|r| r["type"] == "query_summary")
            .all(|r| r["status"] == "incomplete")
    );
    assert_eq!(output[0]["unknown_bases_excluded"], 20);
    assert!(
        !invoke(
            "screen",
            Path::new("fixtures/reference.jsonl"),
            &["--chunk-bases", "0"]
        )
        .status
        .success()
    );
    assert!(
        !invoke(
            "report",
            Path::new("fixtures/reference.jsonl"),
            &["-k", "4"]
        )
        .status
        .success()
    );
}
