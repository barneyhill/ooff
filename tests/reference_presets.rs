use flate2::{Compression, write::GzEncoder};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

fn temp() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "oofft-reference-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&path).unwrap();
    path
}
fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_oofft"))
        .args(args)
        .output()
        .unwrap()
}
fn success(output: Output) -> Vec<Value> {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}
fn gzip(path: &Path, text: &str) {
    let mut out = GzEncoder::new(fs::File::create(path).unwrap(), Compression::default());
    out.write_all(text.as_bytes()).unwrap();
    out.finish().unwrap();
}
fn toy(dir: &Path) -> (PathBuf, PathBuf) {
    let target = "ACGTGATCTAGCTACGATGC";
    let reverse = "GCATCGTAGCTAGATCACGT";
    let fasta = dir.join("genome.fa.gz");
    gzip(
        &fasta,
        &format!(
            ">chr1\n{target}nnnnn{}AAAAA{reverse}\n",
            target.to_lowercase()
        ),
    );
    let gtf = dir.join("genes.gtf.gz");
    let mut text = String::new();
    for (id, name, a, b, strand) in [
        ("ENSG000001", "TARGET", 1, 20, "+"),
        ("ENSG000002", "OTHER", 26, 45, "+"),
        ("ENSG000003", "REVERSE", 51, 70, "-"),
    ] {
        text += &format!(
            "chr1\tfixture\tgene\t{a}\t{b}\t.\t{strand}\t.\tgene_id \"{id}\"; gene_name \"{name}\"; gene_biotype \"protein_coding\";\n"
        );
        for (start, end) in [(a, a + 9), (a + 10, b)] {
            text += &format!(
                "chr1\tfixture\texon\t{start}\t{end}\t.\t{strand}\t.\tgene_id \"{id}\"; transcript_id \"tx-{id}\"; tag \"basic\"; tag \"canonical\";\n"
            );
        }
    }
    gzip(&gtf, &text);
    (fasta, gtf)
}
fn prepare(dir: &Path, fasta: &Path, gtf: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_oofft"))
        .args(["reference", "prepare", "toy", "--fasta"])
        .arg(fasta)
        .arg("--gtf")
        .arg(gtf)
        .arg("--cache-dir")
        .arg(dir)
        .args(["--threads", "2", "--shard-bases", "30"])
        .args(extra)
        .output()
        .unwrap()
}
fn search(dir: &Path, query: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_oofft"))
        .arg("--queries")
        .arg(query)
        .args(["--reference", "toy", "--cache-dir"])
        .arg(dir)
        .args(["-k", "0", "--threads", "2"])
        .args(extra)
        .output()
        .unwrap()
}

#[test]
fn prepared_fasta_gtf_search_exclusions_and_atomic_rebuild() {
    let dir = temp();
    let (fa, gtf) = toy(&dir);
    let built = success(prepare(&dir, &fa, &gtf, &[]));
    assert_eq!(built[0]["records"], 6);
    let directory = PathBuf::from(built[0]["directory"].as_str().unwrap());
    let source = fs::read_to_string(directory.join("reference.fa")).unwrap();
    assert!(source.contains("acgtgatctagctacgatgc")); // masked sequence retained
    assert!(source.contains(">mature:tx-ENSG000003|ENSG000003\nACGTGATCTAGCTACGATGC")); // minus-strand exon order
    let q = dir.join("asos.fa.gz");
    gzip(&q, ">aso1 description\nGCATCGTAGCT\nAGATCACGT\n");
    let all = success(search(&dir, &q, &[]));
    assert_eq!(all[1]["total_sites"], 3); // transcript/gene-body duplicates coalesce
    assert_eq!(all[0]["reference_release"], "custom-toy");
    assert_eq!(all[0]["count_unit"], "genomic-site");
    let excluded = success(search(&dir, &q, &["--exclude", "target"]));
    assert_eq!(excluded[1]["total_sites"], 2);
    assert_eq!(excluded[0]["exclude"], serde_json::json!(["ENSG000001"]));
    let ids = success(search(
        &dir,
        &q,
        &["--exclude", "ENSG000001.4,OTHER", "--exclude", "REVERSE"],
    ));
    assert_eq!(ids[1]["total_sites"], 0);
    let typo = search(&dir, &q, &["--exclude", "TYPO"]);
    assert!(!typo.status.success());
    assert!(typo.stdout.is_empty());
    assert!(String::from_utf8_lossy(&typo.stderr).contains("unknown excluded gene"));
    let jsonl = dir.join("queries.jsonl");
    fs::write(
        &jsonl,
        "{\"id\":\"aso1\",\"sequence\":\"GCATCGTAGCTAGATCACGT\",\"intended_genes\":[\"TARGET\"]}\n",
    )
    .unwrap();
    assert_eq!(
        success(search(&dir, &jsonl, &["--exclude", "OTHER"]))[1]["total_sites"],
        1
    );
    let current = dir.join("custom-toy-v1/current.json");
    let before = fs::read(&current).unwrap();
    assert!(prepare(&dir, &fa, &gtf, &[]).status.success());
    assert_eq!(fs::read(&current).unwrap(), before);
    let broken = dir.join("broken.gtf");
    fs::write(&broken,"chr_missing\tfixture\tgene\t1\t20\t.\t+\t.\tgene_id \"bad\"; gene_biotype \"protein_coding\";\n").unwrap();
    assert!(!prepare(&dir, &fa, &broken, &["--rebuild"]).status.success());
    assert_eq!(fs::read(&current).unwrap(), before);
    assert_eq!(success(search(&dir, &q, &[]))[1]["total_sites"], 3);
    let newer = success(prepare(&dir, &fa, &gtf, &["--rebuild"]));
    assert_ne!(newer[0]["directory"], built[0]["directory"]);
    assert!(directory.join("reference.fa").exists()); // old generation retained
}

#[test]
fn fasta_and_jsonl_queries_agree_and_bare_fasta_has_record_coordinates() {
    let dir = temp();
    let fa = dir.join("queries.fa");
    fs::write(
        &fa,
        ">aso1\nGCATCGTAGCTAGATCACGT\n>aso2\nCCCCCCCCCCCCCCCCCCCC\n",
    )
    .unwrap();
    let native = success(cli(&[
        "--queries",
        fa.to_str().unwrap(),
        "--reference",
        "fixtures/reference.jsonl",
        "--exclude",
        "intended",
    ]));
    let legacy = success(cli(&[
        "--queries",
        "fixtures/queries.jsonl",
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
    ]));
    for i in 1..=2 {
        assert_eq!(
            native[i]["edit_distance_counts"],
            legacy[i]["edit_distance_counts"]
        );
    }
    let reference = dir.join("rna.fa");
    fs::write(
        &reference,
        ">a gene=TARGET\nACGTGATCTAGCTACGATGC\n>b gene=OTHER\nACGTGATCTAGCTACGATGC\n",
    )
    .unwrap();
    let rows = success(cli(&[
        "-q",
        fa.to_str().unwrap(),
        "-r",
        reference.to_str().unwrap(),
        "-k",
        "0",
        "--exclude",
        "TARGET",
    ]));
    assert_eq!(rows[0]["coordinate_scope"], "reference-record");
    assert_eq!(rows[0]["count_unit"], "record-interval");
    assert_eq!(rows[1]["total_sites"], 1);
    assert!(
        !cli(&[
            "-q",
            fa.to_str().unwrap(),
            "-r",
            reference.to_str().unwrap(),
            "--count-unit",
            "genomic-site"
        ])
        .status
        .success()
    );
    fs::write(&fa, ">same\nAAAA\n>same\nCCCC\n").unwrap();
    assert!(
        !cli(&[
            "-q",
            fa.to_str().unwrap(),
            "-r",
            reference.to_str().unwrap()
        ])
        .status
        .success()
    );
}

#[test]
fn presets_are_pinned_and_missing_defaults_are_actionable() {
    let dir = temp();
    let output = cli(&["reference", "list", "--cache-dir", dir.to_str().unwrap()]);
    assert!(output.status.success());
    let list = String::from_utf8(output.stdout).unwrap();
    for name in ["hg38", "mm39", "rn7", "cyno", "rhesus"] {
        assert!(list.contains(name));
    }
    assert!(list.contains("Ensembl 110"));
    let help = cli(&["reference", "prepare", "--help"]);
    assert!(String::from_utf8_lossy(&help.stdout).contains("oofft reference prepare"));
    let missing = cli(&[
        "--queries",
        "fixtures/queries.jsonl",
        "--cache-dir",
        dir.to_str().unwrap(),
    ]);
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("oofft reference prepare hg38"));
    assert!(missing.stdout.is_empty());
    assert!(oofft::reference::key("../escape").is_err());
    assert_eq!(oofft::reference::preset("GRCh38").unwrap().name, "hg38");
    let ambiguous = BTreeMap::from([
        ("ENSG1".into(), "DUP".into()),
        ("ENSG2".into(), "DUP".into()),
    ]);
    assert!(oofft::reference::resolve_exclusions(&["DUP".into()], &ambiguous).is_err());
    assert_eq!(
        oofft::reference::resolve_exclusions(&["ENSG1.5".into()], &ambiguous).unwrap(),
        vec!["ENSG1"]
    );
    assert!(oofft::reference::resolve_exclusions(&["ENSG1.bad".into()], &ambiguous).is_err());
}

#[test]
fn native_site_energy_accepts_the_same_fasta_queries_as_discovery() {
    let dir = temp();
    let q = dir.join("query.fa");
    fs::write(&q, ">aso\nGCATCGTAGCTAGATCACGT\n").unwrap();
    let report = cli(&[
        "report",
        "-q",
        q.to_str().unwrap(),
        "-r",
        "fixtures/reference.jsonl",
        "--exclude",
        "intended",
    ]);
    assert!(
        report.status.success(),
        "{}",
        String::from_utf8_lossy(&report.stderr)
    );
    let sites = dir.join("sites.jsonl");
    fs::write(&sites, report.stdout).unwrap();
    let scored = Command::new(env!("CARGO_BIN_EXE_oofft-ddg"))
        .arg("--sites")
        .arg(sites)
        .arg("--queries")
        .arg(q)
        .args([
            "--reference",
            "fixtures/reference.jsonl",
            "--energy-engine",
            "rust",
            "--threads",
            "2",
        ])
        .output()
        .unwrap();
    let rows = success(scored);
    assert!(
        rows.iter()
            .any(|r| r["type"] == "site" && r["energy_annotation"].is_object())
    );
}

#[test]
fn preparation_preserves_splice_junctions_filters_biotypes_and_rejects_truncated_gzip() {
    let dir = temp();
    let fa = dir.join("genome.fa");
    let gtf = dir.join("genes.gtf");
    fs::write(&fa, ">chr1\nAAAACCCCGGGGTTTT\n").unwrap();
    fs::write(&gtf, concat!(
        "chr1\tx\tgene\t1\t16\t.\t-\t.\tgene_id \"G1\"; gene_name \"KEEP\"; gene_type \"lncRNA\";\n",
        "chr1\tx\texon\t1\t4\t.\t-\t.\tgene_id \"G1\"; transcript_id \"T1\";\n",
        "chr1\tx\texon\t9\t12\t.\t-\t.\tgene_id \"G1\"; transcript_id \"T1\";\n",
        "chr1\tx\tgene\t1\t16\t.\t+\t.\tgene_id \"G2\"; gene_type \"processed_pseudogene\";\n",
        "chr1\tx\tgene\t1\t16\t.\t+\t.\tgene_id \"G3\"; gene_type \"transcribed_processed_pseudogene\";\n",
    )).unwrap();
    let result = success(prepare(&dir, &fa, &gtf, &[]));
    let bundle = PathBuf::from(result[0]["directory"].as_str().unwrap());
    assert_eq!(result[0]["records"], 3);
    let rna = fs::read_to_string(bundle.join("reference.fa")).unwrap();
    assert!(rna.contains(">mature:T1|G1\nCCCCTTTT\n"));
    assert!(!rna.contains("G2"));
    assert!(rna.contains("G3"));
    let annotation: Vec<Value> = fs::read_to_string(bundle.join("records.jsonl"))
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let tx = annotation.iter().find(|r| r["id"] == "mature:T1").unwrap();
    assert_eq!(
        tx["blocks"],
        serde_json::json!([{"start":8,"end":12},{"start":0,"end":4}])
    );
    let q = dir.join("query.fa.gz");
    gzip(&q, ">junction\nAAAAGGGG\n");
    let rows = success(search(&dir, &q, &[]));
    assert_eq!(rows[1]["total_sites"], 1); // only the spliced minus-strand record
    let bytes = fs::read(&q).unwrap();
    fs::write(&q, &bytes[..bytes.len() - 5]).unwrap();
    let failed = search(&dir, &q, &[]);
    assert!(!failed.status.success());
    assert!(failed.stdout.is_empty());
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(dir.join("custom-toy-v1/prepare.lock"))
        .unwrap();
    lock.lock().unwrap();
    let busy = prepare(&dir, &fa, &gtf, &[]);
    assert!(!busy.status.success());
    assert!(String::from_utf8_lossy(&busy.stderr).contains("another process"));
}
