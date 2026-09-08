//! Live independent oracle test, in addition to the pinned Rust-only CI fixtures.
use serde::Deserialize;
use std::{
    fs,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};
#[derive(Deserialize)]
struct Case {
    aso: String,
    target: String,
    cents: i32,
}

#[test]
#[ignore = "requires RNA_DUPLEX pointing to pinned ViennaRNA 2.7.0 RNAduplex"]
fn batch_outputs_match_live_vienna_rna_for_every_case() {
    let binary =
        std::env::var_os("RNA_DUPLEX").expect("Set RNA_DUPLEX to the pinned Vienna executable");
    let version = Command::new(&binary).arg("--version").output().unwrap();
    assert!(version.status.success());
    assert!(String::from_utf8_lossy(&version.stdout).contains("2.7.0"));
    let mut cases: Vec<Case> =
        serde_json::from_str(include_str!("../fixtures/energy_golden.json")).unwrap();
    for target in [
        "ACGUACGUACGUACGUACGU",
        "CGUACGUACGUACGUACGU",
        "GUACGUACGUACGUACGU",
        "GACGUACGUACGUACGUACGU",
        "UUUACGUACGUACGUACGUACGU",
        "GACGUACGUACGUACGUACGA",
        "NNNACGUACGUACGUACGUACGA",
    ] {
        let aso = "ACGUACGUACGUACGUACGU";
        cases.push(Case {
            aso: aso.into(),
            target: target.into(),
            cents: oofft::energy::duplex_energy_scalar(aso.as_bytes(), target.as_bytes()).unwrap(),
        });
    }
    for bits in 0..64usize {
        let leader: String = (0..5)
            .map(|i| b"ACGU"[(bits >> (2 * i)) & 3] as char)
            .collect();
        let aso = "ACGUACGUACGUACGUACGU";
        let target = format!("{leader}ACGUACGUACGUACG");
        cases.push(Case {
            aso: aso.into(),
            cents: oofft::energy::duplex_energy_scalar(aso.as_bytes(), target.as_bytes()).unwrap(),
            target,
        });
    }
    let input = cases
        .iter()
        .map(|c| {
            format!(
                "{}\n{}\n",
                c.aso.replace('T', "U"),
                c.target.replace('T', "U")
            )
        })
        .collect::<String>();
    let path = std::env::temp_dir().join(format!(
        "oofft-live-vienna-{}-{}.input",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::write(&path, input).unwrap();
    let output = Command::new(binary)
        .arg("--noconv")
        .stdin(Stdio::from(fs::File::open(path).unwrap()))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    let expected: Vec<i32> = text
        .lines()
        .filter(|line| line.contains('&'))
        .map(|line| {
            let energy: f64 = line
                .rsplit_once('(')
                .unwrap()
                .1
                .trim_end_matches(')')
                .trim()
                .parse()
                .unwrap();
            (energy * 100.0).round() as i32
        })
        .collect();
    assert_eq!(expected.len(), cases.len());
    let pairs: Vec<_> = cases
        .iter()
        .map(|c| (c.aso.as_bytes(), c.target.as_bytes()))
        .collect();
    assert_eq!(
        oofft::energy::PrefixWorkspace::default()
            .energies(&pairs)
            .unwrap(),
        expected
    );
    let mut workspace = oofft::energy::BatchWorkspace::default();
    assert_eq!(workspace.energies(&pairs).unwrap(), expected);
    workspace.enable_shared_prefix(true);
    assert_eq!(workspace.energies(&pairs).unwrap(), expected);
    assert!(workspace.reused_columns() > 0);
    workspace.limit_lanes(8);
    assert_eq!(workspace.energies(&pairs).unwrap(), expected);
    workspace.limit_lanes(16);
    assert_eq!(workspace.energies(&pairs).unwrap(), expected);
    workspace.enable_shared_paths(true);
    for width in [1, 4, 8, 16] {
        workspace.limit_lanes(width);
        assert_eq!(workspace.energies(&pairs).unwrap(), expected);
    }
    workspace.enable_minplus(true);
    for width in [1, 4, 8, 16] {
        workspace.limit_lanes(width);
        assert_eq!(workspace.energies(&pairs).unwrap(), expected);
    }
    for (case, &energy) in cases.iter().zip(&expected) {
        assert_eq!(energy, case.cents);
        // Force even rare sequence lengths through a full vector, not tails.
        let pair = (case.aso.as_bytes(), case.target.as_bytes());
        assert_eq!(workspace.energies(&[pair; 16]).unwrap(), vec![energy; 16]);
    }
}

#[test]
fn minplus_matches_pinned_vienna_for_both_widths() {
    let cases: Vec<Case> =
        serde_json::from_str(include_str!("../fixtures/energy_golden.json")).unwrap();
    let mut workspace = oofft::energy::BatchWorkspace::default();
    workspace.enable_minplus(true);
    for width in [1, 4, 8, 16] {
        workspace.limit_lanes(width);
        for case in &cases {
            let pair = (case.aso.as_bytes(), case.target.as_bytes());
            assert_eq!(
                workspace.energies(&[pair; 16]).unwrap(),
                vec![case.cents; 16],
                "width={width}, aso={}, target={}",
                case.aso,
                case.target
            );
        }
    }
}
