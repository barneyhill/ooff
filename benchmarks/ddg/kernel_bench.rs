//! Compute-only scalar benchmark; output validation is performed before timing.
use serde::Deserialize;
use std::{hint::black_box, time::Instant};
#[derive(Deserialize)]
struct Case {
    aso: String,
    target: String,
    cents: i32,
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let cases: Vec<Case> =
        serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
    let repetitions: usize = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(10);
    let threads: usize = args.get(3).map(|s| s.parse().unwrap()).unwrap_or(1);
    assert!(threads > 0);
    let max_lanes: usize = args.get(5).map(|s| s.parse().unwrap()).unwrap_or(16);
    let prefix_mode = args.get(4).is_some_and(|s| s == "prefix");
    let paths_mode = args.get(4).is_some_and(|s| s == "paths");
    let shared_mode = paths_mode || args.get(4).is_some_and(|s| s == "shared");
    let minplus_mode = args.get(4).is_some_and(|s| s == "minplus");
    let batch_mode = minplus_mode || shared_mode || args.get(4).is_some_and(|s| s == "batch");
    let mut workspace = oofft::energy::DuplexWorkspace::default();
    if !batch_mode && !prefix_mode {
        for case in &cases {
            assert_eq!(
                workspace
                    .energy(case.aso.as_bytes(), case.target.as_bytes())
                    .unwrap(),
                case.cents
            );
        }
    }
    if batch_mode {
        let pairs: Vec<_> = cases
            .iter()
            .map(|c| (c.aso.as_bytes(), c.target.as_bytes()))
            .collect();
        let mut batch = oofft::energy::BatchWorkspace::default();
        batch.limit_lanes(max_lanes);
        batch.enable_shared_prefix(shared_mode);
        batch.enable_shared_paths(paths_mode);
        batch.enable_minplus(minplus_mode);
        let values = batch.energies(&pairs).unwrap();
        for (value, case) in values.iter().zip(&cases) {
            assert_eq!(*value, case.cents);
        }
    }
    if prefix_mode {
        let pairs: Vec<_> = cases
            .iter()
            .map(|c| (c.aso.as_bytes(), c.target.as_bytes()))
            .collect();
        let values = oofft::energy::PrefixWorkspace::default()
            .energies(&pairs)
            .unwrap();
        for (value, case) in values.iter().zip(&cases) {
            assert_eq!(*value, case.cents);
        }
    }
    let started = Instant::now();
    let measurements: Vec<(i64, u64, u64)> = std::thread::scope(|scope| {
        cases
            .chunks(cases.len().div_ceil(threads))
            .map(|chunk| {
                scope.spawn(move || {
                    let mut workspace = oofft::energy::DuplexWorkspace::default();
                    let mut checksum = 0i64;
                    if prefix_mode {
                        let mut prefix = oofft::energy::PrefixWorkspace::default();
                        let pairs: Vec<_> = chunk
                            .iter()
                            .map(|c| (c.aso.as_bytes(), c.target.as_bytes()))
                            .collect();
                        for _ in 0..repetitions {
                            for (energy, case) in prefix
                                .energies(black_box(&pairs))
                                .unwrap()
                                .into_iter()
                                .zip(chunk)
                            {
                                assert_eq!(energy, case.cents);
                                checksum += i64::from(energy);
                            }
                        }
                        return (checksum, prefix.columns_computed, prefix.columns_reused);
                    }
                    if batch_mode {
                        let mut batch = oofft::energy::BatchWorkspace::default();
                        batch.limit_lanes(max_lanes);
                        batch.enable_shared_prefix(shared_mode);
                        batch.enable_shared_paths(paths_mode);
                        batch.enable_minplus(minplus_mode);
                        let pairs: Vec<_> = chunk
                            .iter()
                            .map(|c| (c.aso.as_bytes(), c.target.as_bytes()))
                            .collect();
                        for _ in 0..repetitions {
                            for (energy, case) in batch
                                .energies(black_box(&pairs))
                                .unwrap()
                                .into_iter()
                                .zip(chunk)
                            {
                                assert_eq!(energy, case.cents);
                                checksum += i64::from(energy);
                            }
                        }
                        return (checksum, 0, batch.reused_columns());
                    }
                    for _ in 0..repetitions {
                        for case in chunk {
                            checksum += i64::from(
                                workspace
                                    .energy(
                                        black_box(case.aso.as_bytes()),
                                        black_box(case.target.as_bytes()),
                                    )
                                    .unwrap(),
                            );
                        }
                    }
                    (checksum, 0, 0)
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|job| job.join().unwrap())
            .collect()
    });
    let checksum: i64 = measurements.iter().map(|m| m.0).sum();
    let columns_computed: u64 = measurements.iter().map(|m| m.1).sum();
    let columns_reused: u64 = measurements.iter().map(|m| m.2).sum();
    println!(
        "{}",
        serde_json::json!({"complete":true,"cases":cases.len(),"repetitions":repetitions,
        "threads":threads,"batch_mode":batch_mode,"prefix_mode":prefix_mode,"shared_mode":shared_mode,"paths_mode":paths_mode,"minplus_mode":minplus_mode,"columns_computed":columns_computed,"columns_reused":columns_reused,"max_lanes":max_lanes,"simd_available":oofft::energy::BatchWorkspace::simd_available(),"selected_lanes":({let mut ws=oofft::energy::BatchWorkspace::default();ws.limit_lanes(max_lanes);ws.selected_lanes()}),"seconds":started.elapsed().as_secs_f64(),"checksum":checksum})
    );
}
