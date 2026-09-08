use oofft::fm::Index;
use std::collections::BTreeMap;

#[test]
fn coarse_record_lookup_preserves_boundaries() {
    let starts = vec![3, 10, 65535, 65536, 65540, 131080];
    let lookup = oofft::fm::RecordLookup::new(starts.clone(), 200000);
    for position in [
        0, 2, 3, 4, 9, 10, 65534, 65535, 65536, 65539, 65540, 131071, 131072, 131079, 131080,
        199999,
    ] {
        assert_eq!(
            lookup.preceding(position),
            starts.iter().rposition(|&s| s <= position)
        );
    }
}

fn distance(a: &[u8], b: &[u8]) -> usize {
    let mut row: Vec<_> = (0..=a.len()).collect();
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

fn check(p: &[u8], text: &[u8]) {
    let index = Index::build(text);
    let mut bytes = Vec::new();
    index.write(&mut bytes).unwrap();
    let restored = Index::read(&bytes[..]).unwrap();
    let reverse_text: Vec<_> = text.iter().rev().copied().collect();
    let reverse_index = Index::build(&reverse_text);
    let reverse_pattern: Vec<_> = p.iter().rev().copied().collect();
    assert_eq!(restored.len(), text.len());
    for k in 0..=3.min(p.len() - 1) {
        let mut expected = BTreeMap::new();
        for a in 0..text.len() {
            for b in a + 1..=(a + p.len() + k).min(text.len()) {
                if text[a..b].contains(&b'N') {
                    continue;
                }
                let d = distance(p, &text[a..b]);
                if d <= k {
                    expected.insert((a, b), d);
                }
            }
        }
        let mut actual = BTreeMap::new();
        restored.search(p, k, |a, b, d| {
            assert_eq!(
                d,
                distance(p, &text[a..b]),
                "each witness must report its minimum interval distance"
            );
            let best = actual.entry((a, b)).or_insert(d);
            *best = (*best).min(d);
            false
        });
        assert_eq!(actual, expected, "pattern={:?} text={:?} k={k}", p, text);
        for reverse in [None, Some(&reverse_index)] {
            let mut ranges = BTreeMap::new();
            restored.search_unique(p, k, reverse, |a, b, d| {
                assert!(
                    ranges.insert((a, b), d).is_none(),
                    "duplicate range-expanded interval"
                );
                false
            });
            assert_eq!(
                ranges, expected,
                "range union pattern={p:?} text={text:?} k={k}"
            );
        }
        let mut union = BTreeMap::new();
        let mut cached = BTreeMap::new();
        restored.search_unique_cached(p, k, Some(&reverse_index), true, |a, b, d| {
            assert!(cached.insert((a, b), d).is_none());
            false
        });
        assert_eq!(cached, expected, "cached ranges pattern={p:?} k={k}");
        restored.search_limited(p, k, true, |a, b, d| {
            let best = union.entry((a, b)).or_insert(d);
            *best = (*best).min(d);
            false
        });
        reverse_index.search_limited(&reverse_pattern, k, true, |a, b, d| {
            let best = union.entry((text.len() - b, text.len() - a)).or_insert(d);
            *best = (*best).min(d);
            false
        });
        assert_eq!(
            union, expected,
            "two directions pattern={p:?} text={text:?} k={k}"
        );
        let mut unique = BTreeMap::new();
        restored.search_automaton(p, k, |a, b, d| {
            assert!(unique.insert((a, b), d).is_none(), "duplicate interval");
            false
        });
        assert_eq!(
            unique, expected,
            "automaton pattern={p:?} text={text:?} k={k}"
        );
        let found = restored.search(p, k, |_, _, _| true);
        assert_eq!(found, !expected.is_empty());
    }
}

#[test]
fn fm_matches_scalar_oracle_with_repeats_unknowns_and_rank_boundaries() {
    for n in [19, 20, 21, 63, 64, 65, 127, 128, 129] {
        check(b"AAAAAAAAAAAAAAAAAAAA", &vec![b'A'; n]);
    }
    check(
        b"ACGTGATCTAGCTACGATGC",
        b"ACGTGATCTANNNGCTACGATGCNACGTGATCTAGCTACGATGC",
    );
    check(b"ACGTGATCTAGCTACGATGC", b"ACGTGATCTAGTCTACGATGC");
    check(b"ACGTGATCTAGCTACGATGC", b"CGTGATCTAGCTACGATG");
    check(b"ACGT", b"");
}

#[test]
fn fm_mixed_edit_random_cases() {
    let mut state = 71u64;
    let mut rand = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        b"ACGT"[(state >> 32) as usize % 4]
    };
    for n in 0..32 {
        let p: Vec<_> = (0..20).map(|_| rand()).collect();
        let mut t = p.clone();
        t[n % 20] = rand();
        t.insert((n * 3) % 20, rand());
        t.remove((n * 7) % 20);
        t.splice(0..0, [rand(), rand()]);
        t.extend([rand(), rand()]);
        check(&p, &t);
    }
}

#[test]
fn fm_rejected_witness_does_not_hide_other_sites() {
    let index = Index::build(b"AAAAAAAAAAAAAAAAAAAANAAAAAAAAAAAAAAAAAAAA");
    let mut witness = None;
    assert!(index.search(&[b'A'; 20], 0, |a, b, d| {
        if a < 21 {
            return false;
        }
        witness = Some((a, b, d));
        true
    }));
    assert_eq!(witness, Some((21, 41, 0)));
}

#[test]
fn mapped_indexes_match_owned_indexes() {
    use std::{
        fs::OpenOptions,
        io::Write,
        time::{SystemTime, UNIX_EPOCH},
    };
    let text = b"ACGTGATCTAGCTACGATGCNNNACGTGATCTAGTCTACGATGCACGTGATCTAGCTACGATGC";
    let owned = Index::build(text);
    let path = std::env::temp_dir().join(format!(
        "ooff-fm-map-{}-{}.fm",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap();
    owned.write(&mut file).unwrap();
    file.flush().unwrap();
    // SAFETY: this test owns the uniquely named file and never modifies it after mapping.
    let mapped = unsafe { Index::map_immutable(&file) }.unwrap();
    for k in 0..=3 {
        let collect = |index: &Index| {
            let mut sites = BTreeMap::new();
            index.search(b"ACGTGATCTAGCTACGATGC", k, |a, b, d| {
                let e = sites.entry((a, b)).or_insert(d);
                *e = (*e).min(d);
                false
            });
            sites
        };
        assert_eq!(collect(&owned), collect(&mapped));
    }
}
