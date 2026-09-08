use serde::Deserialize;
#[derive(Deserialize)]
struct Case {
    aso: String,
    target: String,
    cents: i32,
}

#[test]
fn independent_rust_energy_matches_pinned_vienna_oracle() {
    let cases: Vec<Case> =
        serde_json::from_str(include_str!("../fixtures/energy_golden.json")).unwrap();
    let mut workspace = oofft::energy::DuplexWorkspace::default();
    for (i, case) in cases.iter().enumerate() {
        assert_eq!(
            workspace
                .energy(case.aso.as_bytes(), case.target.as_bytes())
                .unwrap(),
            case.cents,
            "reused case {i}"
        );
        assert_eq!(
            oofft::energy::duplex_energy_scalar(case.aso.as_bytes(), case.target.as_bytes())
                .unwrap(),
            case.cents,
            "scalar case {i}"
        );
        assert_eq!(
            oofft::energy::duplex_energy(case.aso.as_bytes(), case.target.as_bytes()).unwrap(),
            case.cents,
            "case {i}: {} / {}",
            case.aso,
            case.target
        );
    }
    for (i, case) in cases.iter().enumerate().rev() {
        assert_eq!(
            workspace
                .energy(case.aso.as_bytes(), case.target.as_bytes())
                .unwrap(),
            case.cents,
            "reverse reused case {i}"
        );
    }
}

#[test]
fn batch_energy_matches_oracle_with_mixed_lanes_and_tails() {
    let cases: Vec<Case> =
        serde_json::from_str(include_str!("../fixtures/energy_golden.json")).unwrap();
    let pairs: Vec<_> = cases
        .iter()
        .map(|c| (c.aso.as_bytes(), c.target.as_bytes()))
        .collect();
    let mut workspace = oofft::energy::BatchWorkspace::default();
    let actual = workspace.energies(&pairs).unwrap();
    for (actual, case) in actual.iter().zip(&cases) {
        assert_eq!(*actual, case.cents);
    }
    // Force every oracle case through a complete vector, including uncommon
    // dimensions that otherwise take the scalar tail path.
    for width in [1, 4, 8, 16] {
        workspace.limit_lanes(width);
        for case in &cases {
            let pair = (case.aso.as_bytes(), case.target.as_bytes());
            assert_eq!(
                workspace.energies(&[pair; 16]).unwrap(),
                vec![case.cents; 16]
            );
        }
    }
    assert!(
        workspace
            .energies(&[(b"".as_slice(), b"A".as_slice()); 8])
            .is_err()
    );
    assert!(
        workspace
            .energies(&[(b"AX".as_slice(), b"UU".as_slice()); 8])
            .is_err()
    );
}

#[test]
fn shared_prefix_columns_match_scalar_across_branches_and_target_lengths() {
    let aso = b"ACGUACGUACGUACGUACGU";
    let targets: &[&[u8]] = &[
        b"ACGUACGUACGUACGUACGU",
        b"CGUACGUACGUACGUACGU",
        b"GUACGUACGUACGUACGU",
        b"GACGUACGUACGUACGUACGU",
        b"UUUACGUACGUACGUACGUACGU",
        b"GACGUACGUACGUACGUACGA",
        b"NNNACGUACGUACGUACGUACGA",
        b"A",
        b"U",
        b"UUUACGUACGUACGUACGUACGU",
    ];
    let mut workspace = oofft::energy::PrefixWorkspace::default();
    for target in targets.iter().chain(targets.iter().rev()) {
        assert_eq!(
            workspace.energy(aso, target).unwrap(),
            oofft::energy::duplex_energy_scalar(aso, target).unwrap()
        );
    }
    let pairs: Vec<_> = targets
        .iter()
        .map(|&target| (aso.as_slice(), target))
        .collect();
    let values = workspace.energies(&pairs).unwrap();
    for (value, target) in values.iter().zip(targets) {
        assert_eq!(
            *value,
            oofft::energy::duplex_energy_scalar(aso, target).unwrap()
        );
    }
    assert!(workspace.columns_reused > 0);
    let cases: Vec<Case> =
        serde_json::from_str(include_str!("../fixtures/energy_golden.json")).unwrap();
    let pairs: Vec<_> = cases
        .iter()
        .map(|c| (c.aso.as_bytes(), c.target.as_bytes()))
        .collect();
    for (value, case) in workspace.energies(&pairs).unwrap().iter().zip(&cases) {
        assert_eq!(*value, case.cents);
    }
}

#[test]
fn batch_dispatch_and_workspace_mode_changes_preserve_energies() {
    let mut workspace = oofft::energy::BatchWorkspace::default();
    #[cfg(all(feature = "energy-batch", target_arch = "aarch64"))]
    assert_eq!(workspace.selected_lanes(), 4, "NEON must be used on the Pi");
    #[cfg(not(feature = "energy-batch"))]
    assert_eq!(workspace.selected_lanes(), 1);
    let a = (
        b"ACGUACGUACGUACGUACGU".as_slice(),
        b"UGCAUGCAUGCAUGCAUGCA".as_slice(),
    );
    let b = (
        b"GGGUACGUACGUACGUACGU".as_slice(),
        b"UGCAUGCAUGCAUGCACCC".as_slice(),
    );
    for limit in [0, 1, 4, 8, 16] {
        workspace.limit_lanes(limit);
        assert!(workspace.selected_lanes() <= limit.max(1));
        for mode in [1, 0, 1, 2, 1, 0] {
            workspace.enable_shared_prefix(mode == 1);
            workspace.enable_minplus(mode == 2);
            let pairs: Vec<_> = (0..35).map(|i| if i % 3 == 0 { b } else { a }).collect();
            let expected: Vec<_> = pairs
                .iter()
                .map(|&(a, b)| oofft::energy::duplex_energy_scalar(a, b).unwrap())
                .collect();
            assert_eq!(workspace.energies(&pairs).unwrap(), expected);
            assert!(workspace.energies(&[]).unwrap().is_empty());
        }
    }
}
