use ooff::{Block, Record, align, chunks, endpoints, normalize, reverse_complement, search_chunk};
use std::collections::BTreeSet;

// Independent scalar distance: rolling rows, no production traceback or Myers.
fn oracle(a: &[u8], b: &[u8]) -> usize {
    let mut row: Vec<usize> = (0..=a.len()).collect();
    for (j, &y) in b.iter().enumerate() {
        let mut next = vec![j + 1; a.len() + 1];
        for (i, &x) in a.iter().enumerate() {
            next[i + 1] = (row[i] + usize::from(x != y))
                .min(next[i] + 1)
                .min(row[i + 1] + 1);
        }
        row = next;
    }
    row[a.len()]
}

fn expected(pattern: &[u8], text: &[u8], k: usize) -> BTreeSet<(usize, usize, usize)> {
    let mut hits = BTreeSet::new();
    for start in 0..text.len() {
        for end in start + 1..=text.len() {
            let d = oracle(pattern, &text[start..end]);
            if d <= k {
                hits.insert((start, end, d));
            }
        }
    }
    hits
}

fn check(pattern: &[u8], text: &[u8], k: usize) {
    let wanted = expected(pattern, text, k);
    let want_ends: BTreeSet<_> = wanted.iter().map(|x| x.1).collect();
    assert_eq!(
        endpoints(pattern, text, k)
            .into_iter()
            .collect::<BTreeSet<_>>(),
        want_ends
    );
    let mut actual = BTreeSet::new();
    search_chunk(&[pattern.to_vec()], text, k, |q, hit| {
        assert_eq!(q, 0);
        assert_eq!(hit.edits.len(), hit.edit_distance);
        assert!(actual.insert((hit.start, hit.end, hit.edit_distance)));
        true
    });
    assert_eq!(
        actual, wanted,
        "pattern={:?}, text={:?}, k={k}",
        pattern, text
    );
}

#[test]
fn every_edit_position_and_budget() {
    let pattern = b"ACGTGATCTAGCTACGATGC";
    for k in 0..=3 {
        check(pattern, pattern, k);
        for pos in 0..20 {
            let mut substitution = pattern.to_vec();
            substitution[pos] = if pattern[pos] == b'A' { b'C' } else { b'A' };
            check(pattern, &substitution, k);
            let mut deletion = pattern.to_vec();
            deletion.remove(pos);
            check(pattern, &deletion, k);
        }
        for pos in 0..=20 {
            let mut insertion = pattern.to_vec();
            insertion.insert(pos, b'A');
            check(pattern, &insertion, k);
        }
    }
}

#[test]
fn combined_edits_and_deterministic_random_cases() {
    let mut state = 42u64;
    let mut random = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        b"ACGT"[(state >> 32) as usize % 4]
    };
    for n in 0..48 {
        let pattern: Vec<_> = (0..20).map(|_| random()).collect();
        let mut text = pattern.clone();
        text[n % 20] = random();
        text.insert((n * 7) % 20, random());
        text.remove((n * 3) % 20);
        text.splice(0..0, [random(), random()]);
        text.extend([random(), random()]);
        for k in 0..=3 {
            check(&pattern, &text, k);
        }
    }
}

#[test]
fn repeats_ties_and_distinct_queries() {
    let patterns = vec![
        b"AAAAAAAAAAAAAAAAAAAA".to_vec(),
        b"ACGTACGTACGTACGTACGT".to_vec(),
    ];
    let text = b"AAAAAAAAAAAAAAAAAAAAAAACGTACGTACGTACGTACGT";
    for k in 0..=3 {
        let mut actual = [BTreeSet::new(), BTreeSet::new()];
        search_chunk(&patterns, text, k, |q, s| {
            actual[q].insert((s.start, s.end, s.edit_distance));
            true
        });
        for q in 0..2 {
            assert_eq!(actual[q], expected(&patterns[q], text, k));
        }
    }
    assert_eq!(align(b"AA", b"A").1, vec!['I', '=']);
}

#[test]
fn chunks_preserve_all_sites_and_never_cross_unknown_bases() {
    let p = b"ACGTGATCTAGCTACGATGC";
    let mut raw = b"aaa".to_vec();
    raw.extend(p);
    raw.extend(b"nn");
    raw.extend(p);
    raw.extend(b"ttt");
    let text = normalize(std::str::from_utf8(&raw).unwrap(), false).unwrap();
    for k in 0..=3 {
        let wanted: BTreeSet<_> = expected(p, &text, k)
            .into_iter()
            .filter(|&(s, e, _)| !text[s..e].contains(&b'N'))
            .collect();
        for core in [1, 7, 20, 100] {
            let mut actual = BTreeSet::new();
            for (a, b, owned) in chunks(&text, core, 20 + k) {
                search_chunk(&[p.to_vec()], &text[a..b], k, |_, s| {
                    if a + s.start < owned {
                        assert!(actual.insert((a + s.start, a + s.end, s.edit_distance)));
                    }
                    true
                });
            }
            assert_eq!(actual, wanted);
        }
    }
}

#[test]
fn strand_junction_mapping_and_aso_positions() {
    let mut record = Record {
        id: "r".into(),
        sequence: "ACGTACGTACGTACGTACGT".into(),
        genes: vec!["g".into()],
        transcripts: vec!["t".into()],
        contig: "1".into(),
        strand: "+".into(),
        blocks: vec![
            Block {
                start: 100,
                end: 104,
            },
            Block {
                start: 200,
                end: 203,
            },
            Block {
                start: 300,
                end: 313,
            },
        ],
    };
    record.validate().unwrap();
    let mapped = record.map_interval(2, 10);
    assert_eq!(
        mapped.iter().map(|b| (b.start, b.end)).collect::<Vec<_>>(),
        vec![(102, 104), (200, 203), (300, 303)]
    );
    record.strand = "-".into();
    record.blocks.reverse();
    record.validate().unwrap();
    let mapped = record.map_interval(11, 18);
    assert_eq!(
        mapped.iter().map(|b| (b.start, b.end)).collect::<Vec<_>>(),
        vec![(300, 302), (200, 203), (102, 104)]
    );
    assert_eq!(
        reverse_complement(&normalize("acgu", true).unwrap()),
        b"ACGT"
    );
    assert!(normalize("ACGN", true).is_err());
    assert_eq!(normalize("aRy", false).unwrap(), b"ANN");
    let pattern = b"ACGTGATCTAGCTACGATGC";
    let mut target = *pattern;
    target[0] = b'T';
    let mut found = false;
    search_chunk(&[pattern.to_vec()], &target, 1, |_, s| {
        if s.start == 0 && s.end == 20 {
            assert_eq!(s.edits[0].aso_position, Some(20));
            assert_eq!(s.edits[0].region, "MOE_wing");
            assert_eq!(s.edits[0].aso_base, Some('T'));
            found = true;
        }
        true
    });
    assert!(found);
}

#[test]
fn witness_callback_retires_each_query() {
    let mut calls = [0; 2];
    search_chunk(
        &[vec![b'A'; 20], vec![b'A'; 20]],
        &[b'A'; 100],
        3,
        |q, _| {
            calls[q] += 1;
            false
        },
    );
    assert_eq!(calls, [1, 1]);
}
