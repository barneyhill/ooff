#![cfg(unix)]
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};
static FIXTURE_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
fn fixture() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "oofft-ddg-cli-{}-{}-{}",
        FIXTURE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&path).unwrap();
    fs::write(path.join("queries"),"{\"id\":\"a\",\"aso\":\"AAAAAAAAAAAAAAAAAAAA\",\"target\":\"TTTTTTTTTTTTTTTTTTTT\"}\n{\"id\":\"b\",\"asoDnaSequence\":\"AAAAAAAAAAAAAAAAAAAA\",\"targetDnaSequence\":\"TTTTTTTTTTTTTTTTTTTT\"}\n").unwrap();
    fs::write(path.join("off"), ">off\nCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\n").unwrap();
    script(
        &path.join("duplex"),
        "#!/bin/sh\nwhile IFS= read -r a; do IFS= read -r b || exit 2; printf '(((((&))))) 1,5 : 1,5 (-20.00)\\n'; done\n",
    );
    script(
        &path.join("plex"),
        "#!/bin/sh\nawk '/^>/ {print;print \">offtarget\";print \"(((&))) 2,4 : 1,3 (-10.00)\";print \"(((&))) 3,5 : 1,3 (-11.00)\"}' \"$2\"\n",
    );
    path
}
fn script(path: &Path, text: &str) {
    fs::write(path, text).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
}
fn run(path: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_oofft-ddg"))
        .args(["--queries"])
        .arg(path.join("queries"))
        .arg("--off-target")
        .arg(path.join("off"))
        .arg("--rnaplex")
        .arg(path.join("plex"))
        .arg("--rnaduplex")
        .arg(path.join("duplex"))
        .arg("--work-dir")
        .arg(path.join("work"))
        .args(extra)
        .output()
        .unwrap()
}
#[test]
fn ddg_deduplicates_and_preserves_order_energy_sign_and_best_position() {
    let p = fixture();
    let out = run(&p, &["--threads", "2"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let rows: Vec<Value> = String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["id"], "a");
    assert_eq!(rows[1]["id"], "b");
    for r in rows {
        assert_eq!(r["ddg"], 9.0);
        assert_eq!(r["dg_target"], -20.0);
        assert_eq!(r["dg_other"], -11.0);
        assert_eq!(r["dg_other_position"], 3);
    }
    let m: Value =
        serde_json::from_str(&fs::read_to_string(p.join("work/manifest.json")).unwrap()).unwrap();
    assert_eq!(m["unique_scan_asos"], 1);
    assert_eq!(m["unique_on_target_pairs"], 1);
}
#[test]
fn ddg_exact_and_short_reference_do_not_invoke_engines() {
    for off in ["TTTTTTTTTTTTTTTTTTTTTTTTTT", "", "ACG"] {
        let p = fixture();
        fs::write(p.join("off"), off).unwrap();
        script(&p.join("plex"), "#!/bin/sh\nexit 7\n");
        script(&p.join("duplex"), "#!/bin/sh\nexit 7\n");
        let out = run(&p, &[]);
        assert!(out.status.success());
        let r: Value = serde_json::from_str(
            String::from_utf8(out.stdout)
                .unwrap()
                .lines()
                .next()
                .unwrap(),
        )
        .unwrap();
        if off.len() > 20 {
            assert_eq!(r["ddg"], 0.0);
            assert_eq!(r["dg_other_position"], 1);
            assert_eq!(r["exact_off_target_match"], true);
        } else {
            assert!(r["ddg"].is_null());
        }
        assert!(r["dg_target"].is_null());
        assert!(r["dg_other"].is_null());
    }
}
#[test]
fn ddg_missing_off_target_energy_stays_unknown() {
    let p = fixture();
    script(&p.join("plex"), "#!/bin/sh\nexit 0\n");
    let out = run(&p, &[]);
    assert!(out.status.success());
    for line in String::from_utf8(out.stdout).unwrap().lines() {
        let row: Value = serde_json::from_str(line).unwrap();
        assert_eq!(row["dg_target"], -20.0);
        assert!(row["dg_other"].is_null());
        assert!(row["ddg"].is_null());
        assert!(row["dg_other_position"].is_null());
    }
}
#[test]
fn ddg_engine_failure_and_timeout_never_emit_partial_energies() {
    for text in ["#!/bin/sh\nexit 7\n", "#!/bin/sh\nexec sleep 5\n"] {
        let p = fixture();
        script(&p.join("plex"), text);
        let out = run(&p, &["--timeout", "0.1"]);
        assert!(!out.status.success());
        assert!(out.stdout.is_empty());
        assert!(!p.join("work/manifest.json").exists());
    }
}

fn annotation_fixture(fasta: bool) -> PathBuf {
    let p = fixture();
    fs::write(p.join("queries"), "{\"id\":\"a\",\"sequence\":\"AAAAAAAAAAAAAAAAAAAA\",\"target\":\"TTTTTTTTTTTTTTTTTTTT\"}\n").unwrap();
    let reference = if fasta {
        ">r|other\nCCCCCCCCCC\nCCCCCCCCCC\nCCCCCCCCCC\n"
    } else {
        "{\"id\":\"r\",\"sequence\":\"CCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\"}\n"
    };
    fs::write(p.join("reference"), reference).unwrap();
    let digest = |path| format!("{:x}", Sha256::digest(fs::read(path).unwrap()));
    let report = [
        serde_json::json!({"type":"manifest","mode":"report",
            "queries_sha256":digest(p.join("queries")), "reference_sha256":digest(p.join("reference"))}),
        serde_json::json!({"type":"site","query_id":"a","record_id":"r","ddg":99.0,
            "site_id":["a","r","2","19"],"alignment":{"start":2,"end":19,"cigar":"8M3I9M"}}),
        serde_json::json!({"type":"site","query_id":"a","record_id":"r",
            "site_id":["a","r","3","26"],"alignment":{"start":3,"end":26,"cigar":"8M3D12M"}}),
        serde_json::json!({"type":"query_summary","query_id":"a","capped":true,"count_is_lower_bound":true}),
        serde_json::json!({"type":"run_complete","reported_sites":2}),
    ];
    fs::write(
        p.join("report"),
        report
            .iter()
            .map(|v| v.to_string() + "\n")
            .collect::<String>(),
    )
    .unwrap();
    script(
        &p.join("duplex"),
        "#!/bin/sh\nwhile IFS= read -r a; do IFS= read -r b || exit 2; printf '(((&))) 1,3 : 1,3 (-%s.00)\\n' \"${#b}\"; done\n",
    );
    p
}

fn annotate(p: &Path, extra: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_oofft-ddg"))
        .arg("--queries")
        .arg(p.join("queries"))
        .arg("--sites")
        .arg(p.join("report"))
        .arg("--reference")
        .arg(p.join("reference"))
        .arg("--rnaduplex")
        .arg(p.join("duplex"))
        .arg("--rnaplex")
        .arg(p.join("plex"))
        .arg("--work-dir")
        .arg(p.join("work"))
        .args(extra)
        .output()
        .unwrap()
}

#[test]
fn site_annotation_preserves_distinct_intervals_and_report_scope() {
    for fasta in [false, true] {
        let p = annotation_fixture(fasta);
        script(&p.join("plex"), "#!/bin/sh\nexit 7\n");
        let out = annotate(&p, &["--batch-size", "2", "--threads", "2"]);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let rows: Vec<Value> = String::from_utf8(out.stdout)
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(rows[1]["ddg"], 3.0);
        assert_eq!(rows[2]["ddg"], -3.0);
        assert_eq!(rows[1]["supplied_ddg"], 99.0);
        assert_eq!(
            rows[1]["energy_annotation"]["ddg_sign"],
            "dg_other - dg_target"
        );
        assert_eq!(
            rows[1]["energy_annotation"]["scored_record_interval"],
            serde_json::json!([2, 19])
        );
        assert_eq!(rows[1]["energy_annotation"]["scope"], "site");
        assert_eq!(
            rows[1]["energy_annotation"]["on_target_basis"],
            "supplied_target"
        );
        let original: Vec<Value> = fs::read_to_string(p.join("report"))
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        for (mut actual, expected) in rows.into_iter().zip(original) {
            actual.as_object_mut().unwrap().remove("energy_annotation");
            if actual["type"] == "site" {
                actual.as_object_mut().unwrap().remove("ddg");
                if let Some(previous) = actual.as_object_mut().unwrap().remove("supplied_ddg") {
                    actual["ddg"] = previous;
                }
            }
            assert_eq!(actual, expected);
        }
    }
}

#[test]
fn whole_transcript_annotation_is_explicit_and_has_separate_best_position() {
    let p = annotation_fixture(false);
    let out = annotate(&p, &["--whole-transcript"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let rows: Vec<Value> = String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    for r in &rows[1..3] {
        assert_eq!(r["ddg"], 9.0);
        assert_eq!(r["energy_annotation"]["scope"], "whole_transcript");
        assert_eq!(
            r["energy_annotation"]["scored_record_interval"],
            serde_json::json!([0, 30])
        );
        assert_eq!(
            r["energy_annotation"]["whole_transcript_best_position_1based"],
            3
        );
    }
    assert_eq!(rows[1]["alignment"]["start"], 2);
    assert_eq!(rows[2]["alignment"]["start"], 3);
}

#[test]
fn annotation_rejects_wrong_reference_and_truncated_reports_without_stdout() {
    for truncated in [false, true] {
        let p = annotation_fixture(false);
        if truncated {
            let report = fs::read_to_string(p.join("report")).unwrap();
            fs::write(
                p.join("report"),
                report
                    .lines()
                    .take(4)
                    .map(|l| l.to_owned() + "\n")
                    .collect::<String>(),
            )
            .unwrap();
        } else {
            fs::write(
                p.join("reference"),
                "{\"id\":\"r\",\"sequence\":\"AAAA\"}\n",
            )
            .unwrap();
        }
        let out = annotate(&p, &[]);
        assert!(!out.status.success());
        assert!(out.stdout.is_empty());
        assert!(!p.join("work/manifest.json").exists());
    }
}

#[test]
fn native_site_energy_needs_no_vienna_executable() {
    let p = annotation_fixture(false);
    script(&p.join("duplex"), "#!/bin/sh\nexit 7\n");
    script(&p.join("plex"), "#!/bin/sh\nexit 7\n");
    let out = annotate(&p, &["--energy-engine", "rust"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let expected =
        oofft::energy::duplex_energy(b"AAAAAAAAAAAAAAAAAAAA", b"UUUUUUUUUUUUUUUUUUUU").unwrap();
    let rows: Vec<Value> = String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    for row in &rows[1..3] {
        assert_eq!(row["energy_annotation"]["dg_other"], 100000.0);
        assert_eq!(row["ddg"], (10_000_000 - expected) as f64 / 100.0);
    }
}

#[test]
fn native_batch_reuse_preserves_distinct_sites_and_full_sequence_keys() {
    let p = annotation_fixture(false);
    let queries = fs::read_to_string(p.join("queries")).unwrap()
        + "{\"id\":\"b\",\"sequence\":\"GGGGGGGGGGGGGGGGGGGG\",\"target\":\"CCCCCCCCCCCCCCCCCCCC\"}\n";
    fs::write(p.join("queries"), &queries).unwrap();
    let mut report: Vec<Value> = fs::read_to_string(p.join("report"))
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    report[0]["queries_sha256"] = format!("{:x}", Sha256::digest(queries.as_bytes())).into();
    let duplicate = serde_json::json!({"type":"site","query_id":"a","record_id":"r",
        "site_id":["a","r","4","21"],"alignment":{"start":4,"end":21}});
    let other_aso = serde_json::json!({"type":"site","query_id":"b","record_id":"r",
        "site_id":["b","r","4","21"],"alignment":{"start":4,"end":21}});
    report.insert(3, duplicate);
    report.insert(4, other_aso);
    report.last_mut().unwrap()["reported_sites"] = 4.into();
    fs::write(
        p.join("report"),
        report
            .iter()
            .map(|v| v.to_string() + "\n")
            .collect::<String>(),
    )
    .unwrap();
    let out = annotate(&p, &["--energy-engine", "rust", "--threads", "4"]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let rows: Vec<Value> = String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(rows.len(), report.len());
    for (row, original) in rows
        .iter()
        .zip(&report)
        .filter(|(row, _)| row["type"] == "site")
    {
        assert_eq!(row["site_id"], original["site_id"]);
        assert_eq!(row["alignment"], original["alignment"]);
        let aso = if row["query_id"] == "a" { b'A' } else { b'G' };
        let len = (row["alignment"]["end"].as_u64().unwrap()
            - row["alignment"]["start"].as_u64().unwrap()) as usize;
        let expected = oofft::energy::duplex_energy_scalar(&[aso; 20], &vec![b'C'; len]).unwrap();
        assert_eq!(
            row["energy_annotation"]["dg_other"],
            expected as f64 / 100.0
        );
    }
    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(p.join("work/manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["profile"]["native_pairs"], 6);
    assert_eq!(manifest["profile"]["native_unique_pairs"], 5);
}

#[test]
fn intended_energy_is_lazy_cached_across_batches_but_all_queries_are_validated() {
    for invalid_unused in [false, true] {
        let p = annotation_fixture(false);
        let unused = if invalid_unused {
            "Z".repeat(20)
        } else {
            "G".repeat(20)
        };
        let queries = fs::read_to_string(p.join("queries")).unwrap()
            + &format!("{{\"id\":\"unused\",\"sequence\":\"{unused}\"}}\n");
        fs::write(p.join("queries"), &queries).unwrap();
        let mut report: Vec<Value> = fs::read_to_string(p.join("report"))
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        report[0]["queries_sha256"] = format!("{:x}", Sha256::digest(queries.as_bytes())).into();
        fs::write(
            p.join("report"),
            report
                .iter()
                .map(|v| v.to_string() + "\n")
                .collect::<String>(),
        )
        .unwrap();
        let out = annotate(&p, &["--energy-engine", "rust", "--batch-size", "2"]);
        if invalid_unused {
            assert!(!out.status.success());
            assert!(out.stdout.is_empty());
            assert!(String::from_utf8_lossy(&out.stderr).contains("Sequences must contain"));
        } else {
            assert!(
                out.status.success(),
                "{}",
                String::from_utf8_lossy(&out.stderr)
            );
            let manifest: Value =
                serde_json::from_str(&fs::read_to_string(p.join("work/manifest.json")).unwrap())
                    .unwrap();
            // One intended energy, two different off-target intervals. The
            // second site is in another batch; the unused query is never scored.
            assert_eq!(manifest["profile"]["native_pairs"], 3);
            let rows: Vec<Value> = String::from_utf8(out.stdout)
                .unwrap()
                .lines()
                .map(|s| serde_json::from_str(s).unwrap())
                .collect();
            assert_eq!(
                rows[1]["energy_annotation"]["dg_target"],
                rows[2]["energy_annotation"]["dg_target"]
            );
            assert_eq!(rows[1]["supplied_ddg"], 99.0);
            assert_eq!(
                rows[1]["energy_annotation"]["ddg_sign"],
                "dg_other - dg_target"
            );
        }
    }
}

#[test]
fn native_site_fast_path_preserves_unknown_nested_fields_and_existing_ddg() {
    for ddg in [
        None,
        Some(serde_json::json!(null)),
        Some(serde_json::json!(-7.25)),
    ] {
        let p = annotation_fixture(false);
        let mut report: Vec<Value> = fs::read_to_string(p.join("report"))
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        report[2]["custom"] = serde_json::json!({"text":"escaped \"quotes\" and } braces\nλ", "rows":[null, true, 42, {"energy_annotation":"nested only"}]});
        if let Some(ddg) = &ddg {
            report[2]["ddg"] = ddg.clone();
        }
        fs::write(
            p.join("report"),
            report
                .iter()
                .map(|v| format!("  {v}  \n"))
                .collect::<String>(),
        )
        .unwrap();
        let out = annotate(&p, &["--energy-engine", "rust"]);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let rows: Vec<Value> = String::from_utf8(out.stdout)
            .unwrap()
            .lines()
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        for (key, value) in report[2].as_object().unwrap() {
            if key != "ddg" {
                assert_eq!(&rows[2][key], value);
            }
        }
        if let Some(ddg) = ddg {
            assert_eq!(rows[2]["supplied_ddg"], ddg);
        }
        assert_eq!(rows[2]["energy_annotation"]["dg_other"], 100000.0);
    }
}

#[test]
fn parallel_annotation_rejects_control_rows_across_batch_boundaries() {
    for batch_size in ["1", "2", "4"] {
        for duplicate_manifest in [false, true] {
            let p = annotation_fixture(false);
            let report = fs::read_to_string(p.join("report")).unwrap();
            let extra = if duplicate_manifest {
                report.lines().next().unwrap()
            } else {
                "{\"type\":\"query_summary\",\"query_id\":\"a\"}"
            };
            fs::write(p.join("report"), format!("{report}{extra}\n")).unwrap();
            let out = annotate(
                &p,
                &[
                    "--energy-engine",
                    "rust",
                    "--batch-size",
                    batch_size,
                    "--threads",
                    "2",
                ],
            );
            assert!(!out.status.success());
            assert!(out.stdout.is_empty());
            assert!(!p.join("work/manifest.json").exists());
        }
    }
}

// Exercise cache reuse across records, query IDs, batches and worker waves.
fn check_global_cache(live: bool) {
    let mut oracle_energies = None;
    for engine in ["rust", "vienna"] {
        let mut expected = None;
        for (capacity, threads) in [("0", "1"), ("default", "1"), ("default", "4"), ("1", "1")] {
            let p = annotation_fixture(false);
            if live {
                let binary = std::env::var("RNA_DUPLEX").expect("Set RNA_DUPLEX");
                std::os::unix::fs::symlink(binary, p.join("live-duplex")).unwrap();
            }
            fs::write(p.join("queries"), concat!(
                "{\"id\":\"a\",\"sequence\":\"GGGGGGGGGGGGGGGGGGGG\",\"target\":\"CCCCCCCCCCCCCCCCCCCC\"}\n",
                "{\"id\":\"b\",\"sequence\":\"GGGGGGGGGGGGGGGGGGGG\",\"target\":\"CCCCCCCCCCCCCCCCCAAA\"}\n"
            )).unwrap();
            fs::write(
                p.join("reference"),
                ">r\nCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\n>s\nCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC\n",
            )
            .unwrap();
            let digest = |name| format!("{:x}", Sha256::digest(fs::read(p.join(name)).unwrap()));
            let mut report = vec![serde_json::json!({"type":"manifest","mode":"report",
                "queries_sha256":digest("queries"), "reference_sha256":digest("reference")})];
            for i in 0..64 {
                report.push(
                    serde_json::json!({"type":"site","query_id":if i % 2 == 0 {"a"} else {"b"},
                    "record_id":if i % 3 == 0 {"s"} else {"r"}, "custom":i,
                    "alignment":{"start":i % 5,"end":i % 5 + 17 + i % 2}}),
                );
            }
            report.push(serde_json::json!({"type":"run_complete","reported_sites":64}));
            fs::write(
                p.join("report"),
                report.iter().map(|v| format!("{v}\n")).collect::<String>(),
            )
            .unwrap();
            let mut args = vec![
                "--energy-engine",
                engine,
                "--batch-size",
                "2",
                "--threads",
                threads,
            ];
            if capacity != "default" {
                args.extend(["--energy-cache-pairs", capacity]);
            }
            let live_path = p.join("live-duplex");
            // annotate() supplies a default executable; replace the fixture for live checks.
            if live {
                fs::copy(&live_path, p.join("duplex")).unwrap();
            }
            let out = annotate(&p, &args);
            assert!(
                out.status.success(),
                "{}",
                String::from_utf8_lossy(&out.stderr)
            );
            let rows: Vec<Value> = String::from_utf8(out.stdout)
                .unwrap()
                .lines()
                .map(|s| serde_json::from_str(s).unwrap())
                .collect();
            if live {
                let energies: Vec<_> = rows
                    .iter()
                    .filter(|r| r["type"] == "site")
                    .map(|r| {
                        (
                            r["ddg"].clone(),
                            r["energy_annotation"]["dg_target"].clone(),
                            r["energy_annotation"]["dg_other"].clone(),
                        )
                    })
                    .collect();
                if let Some(oracle) = &oracle_energies {
                    assert_eq!(&energies, oracle);
                } else {
                    oracle_energies = Some(energies);
                }
            }
            if let Some(expected) = &expected {
                assert_eq!(&rows, expected);
            } else {
                expected = Some(rows);
            }
            let m: Value =
                serde_json::from_slice(&fs::read(p.join("work/manifest.json")).unwrap()).unwrap();
            if capacity == "default" {
                assert_eq!(m["energy_cache"]["capacity_pairs"], 4_000_000);
                assert!(m["energy_cache"]["hits"].as_u64().unwrap() > 40);
                assert_eq!(m["energy_cache"]["entries"], 4);
            } else if capacity == "1" {
                assert!(m["energy_cache"]["generation_resets"].as_u64().unwrap() > 0);
                assert_eq!(m["energy_cache"]["entries"], 1);
            } else {
                assert_eq!(m["energy_cache"]["entries"], 0);
            }
        }
    }
}
#[test]
fn global_energy_cache_is_default_bounded_and_preserves_annotations() {
    check_global_cache(false);
}
#[test]
#[ignore = "requires RNA_DUPLEX pointing to ViennaRNA"]
fn global_energy_cache_matches_uncached_live_vienna() {
    check_global_cache(true);
}

#[test]
fn direct_fasta_intervals_match_wrapped_records_and_validate_bounds() {
    let sequence = "ccctttccctttccctttccctttcccttt";
    let mut expected = None;
    for wrapped in [false, true] {
        for outside in [false, true] {
            let p = annotation_fixture(true);
            let data = if wrapped {
                sequence
                    .as_bytes()
                    .chunks(7)
                    .map(|b| std::str::from_utf8(b).unwrap().to_owned() + "\r\n")
                    .collect::<String>()
            } else {
                format!("  {sequence} \r\n")
            };
            fs::write(p.join("reference"), format!(">r|other\r\n{data}")).unwrap();
            let mut report: Vec<Value> = fs::read_to_string(p.join("report"))
                .unwrap()
                .lines()
                .map(|s| serde_json::from_str(s).unwrap())
                .collect();
            report[0]["reference_sha256"] = format!(
                "{:x}",
                Sha256::digest(fs::read(p.join("reference")).unwrap())
            )
            .into();
            if outside {
                report[1]["alignment"]["end"] = 100.into();
            }
            fs::write(
                p.join("report"),
                report.iter().map(|v| format!("{v}\n")).collect::<String>(),
            )
            .unwrap();
            let out = annotate(
                &p,
                &[
                    "--energy-engine",
                    "rust",
                    "--batch-size",
                    "1",
                    "--threads",
                    "2",
                ],
            );
            if outside {
                assert!(!out.status.success());
                assert!(out.stdout.is_empty());
                assert!(
                    String::from_utf8_lossy(&out.stderr).contains("Site outside reference record")
                );
            } else {
                assert!(
                    out.status.success(),
                    "{}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let rows: Vec<Value> = String::from_utf8(out.stdout)
                    .unwrap()
                    .lines()
                    .map(|l| serde_json::from_str::<Value>(l).unwrap())
                    .filter(|r| r["type"] == "site")
                    .collect();
                if let Some(expected) = &expected {
                    assert_eq!(&rows, expected);
                } else {
                    expected = Some(rows);
                }
            }
        }
    }
}
