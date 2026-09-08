use oofft::{IntervalDistance, align};

#[test]
fn fixed_interval_bit_distance_matches_global_scalar_oracle() {
    let mut state = 19u64;
    let mut sequence = |n| {
        (0..n)
            .map(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                b"ACGT"[(state & 3) as usize]
            })
            .collect::<Vec<_>>()
    };
    for m in [1, 2, 19, 20, 21, 32, 63, 64] {
        for n in 0..=70 {
            for _ in 0..8 {
                let pattern = sequence(m);
                let target = sequence(n);
                assert_eq!(
                    IntervalDistance::new(&pattern).distance(&target),
                    align(&pattern, &target).0
                );
            }
        }
    }
    let pattern = b"ACGTACGTACGTACGTACGT";
    let verifier = IntervalDistance::new(pattern);
    for flank in 0..=8 {
        let mut target = vec![b'T'; flank];
        target.extend(pattern);
        assert_eq!(verifier.distance(&target), flank);
        for position in 0..target.len() {
            let mut edited = target.clone();
            edited[position] = b'A';
            assert_eq!(verifier.distance(&edited), align(pattern, &edited).0);
        }
    }
}
