//! Exact FM-index search prototype. The index is reusable between query batches.
//! libsais constructs suffix arrays; candidate discovery and edit search are Rust.
use libsais::{SuffixArrayConstruction, ThreadCount};
use rustc_hash::FxHashMap;
use std::{
    fs::File,
    io::{self, Read, Write},
};

const MAGIC: &[u8; 8] = b"OOFFFM01";
const COMPACT_MAGIC: &[u8; 8] = b"OOFFFC01";

#[derive(Clone, Copy)]
struct Compact {
    sample_rate: usize,
    primary: usize,
    samples: usize,
}

/// Compact distinct intervals within one u32-addressed FM shard.
/// Stores absolute start/span and record number/minimum cost in two words.
#[derive(Default)]
pub struct SiteSet(FxHashMap<u64, u64>);

impl SiteSet {
    pub fn insert(&mut self, record: usize, start: usize, end: usize, cost: usize) {
        assert!(start <= u32::MAX as usize && end > start && end - start < 256 && cost < 256);
        assert!(record <= u32::MAX as usize);
        let key = ((start as u64) << 8) | (end - start) as u64;
        let value = ((record as u64) << 8) | cost as u64;
        let previous = self.0.entry(key).or_insert(value);
        debug_assert_eq!(*previous >> 8, record as u64);
        *previous = (*previous).min(value);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_sites(self) -> impl Iterator<Item = (usize, usize, usize, usize)> {
        self.0.into_iter().map(|(key, value)| {
            let start = (key >> 8) as usize;
            (
                (value >> 8) as usize,
                start,
                start + (key & 255) as usize,
                (value & 255) as usize,
            )
        })
    }
}

/// Coarse position bins bound the record lookup without changing boundaries.
pub struct RecordLookup {
    starts: Vec<usize>,
    bins: Vec<usize>,
}

impl RecordLookup {
    pub fn new(starts: Vec<usize>, text_bases: usize) -> Self {
        assert!(!starts.is_empty() && starts.windows(2).all(|w| w[0] < w[1]));
        let bins = (0..=(text_bases >> 16) + 1)
            .map(|bin| {
                starts
                    .partition_point(|&s| s <= (bin << 16))
                    .saturating_sub(1)
            })
            .collect();
        Self { starts, bins }
    }

    pub fn preceding(&self, position: usize) -> Option<usize> {
        let bin = position >> 16;
        let lo = *self.bins.get(bin)?;
        let hi = (self.bins.get(bin + 1)? + 1).min(self.starts.len());
        (lo + self.starts[lo..hi].partition_point(|&s| s <= position)).checked_sub(1)
    }
}

// Minimum prefix balance and net balance for four DP vertical differences.
fn column_min(pv: u64, mv: u64, depth: usize, m: usize, k: usize) -> usize {
    static TABLE: std::sync::OnceLock<[(i8, i8); 256]> = std::sync::OnceLock::new();
    let table = TABLE.get_or_init(|| {
        std::array::from_fn(|key| {
            let (mut balance, mut minimum) = (0i8, 0i8);
            for bit in 0..4 {
                balance += ((key >> bit) & 1) as i8 - ((key >> (bit + 4)) & 1) as i8;
                minimum = minimum.min(balance);
            }
            (minimum, balance)
        })
    });
    // Outside this diagonal band the length difference alone exceeds k.
    let start = depth.saturating_sub(k).min(m);
    let end = (depth + k).min(m);
    let prefix = if start == 64 {
        u64::MAX
    } else {
        (1u64 << start) - 1
    };
    let initial =
        depth as i32 + (pv & prefix).count_ones() as i32 - (mv & prefix).count_ones() as i32;
    if start == end {
        return initial as usize;
    }
    let width = end - start;
    let mask = u64::MAX >> (64 - width);
    let (mut p, mut v) = ((pv >> start) & mask, (mv >> start) & mask);
    let (mut balance, mut minimum) = (initial, initial);
    for _ in 0..width.div_ceil(4) {
        let (low, delta) = table[((p & 15) | ((v & 15) << 4)) as usize];
        minimum = minimum.min(balance + low as i32);
        balance += delta as i32;
        p >>= 4;
        v >>= 4;
    }
    minimum as usize
}

#[derive(Clone, Debug)]
struct RankBlock {
    masks: [u64; 4],
    counts: [u32; 4],
}

pub struct Index {
    // Full SA initially: direct locate, with its memory cost reported explicitly.
    sa: Vec<u32>,
    ranks: Vec<RankBlock>,
    cumulative: [u32; 4],
    mapping: Option<memmap2::Mmap>,
    mapped_len: usize,
    serialized: Option<Vec<u8>>,
    compact: Option<Compact>,
}

fn encode(base: u8) -> u8 {
    match base {
        b'A' => 1,
        b'C' => 2,
        b'G' => 3,
        b'T' => 4,
        _ => 5,
    }
}

impl Index {
    pub fn build(text: &[u8]) -> Self {
        Self::build_with_threads(text, ThreadCount::openmp_default())
    }

    pub fn build_with_threads(text: &[u8], threads: ThreadCount) -> Self {
        assert!(
            text.len() < i32::MAX as usize - 1,
            "shard exceeds i32 suffix-array limit"
        );
        let mut encoded: Vec<_> = text.iter().map(|&b| encode(b)).collect();
        encoded.push(0); // unique terminal sentinel
        let sa: Vec<i32> = SuffixArrayConstruction::for_text(&encoded)
            .in_owned_buffer()
            .multi_threaded(threads)
            .run()
            .expect("suffix array construction failed")
            .into_vec();
        let mut totals = [0u32; 4];
        for &b in &encoded {
            if (1..=4).contains(&b) {
                totals[b as usize - 1] += 1;
            }
        }
        let mut cumulative = [1; 4];
        for c in 1..4 {
            cumulative[c] = cumulative[c - 1] + totals[c - 1];
        }
        let mut ranks = Vec::with_capacity(encoded.len() / 64 + 1);
        let mut counts = [0u32; 4];
        for block_start in (0..encoded.len()).step_by(64) {
            let mut block = RankBlock {
                masks: [0; 4],
                counts,
            };
            for (bit, &suffix) in sa[block_start..(block_start + 64).min(sa.len())]
                .iter()
                .enumerate()
            {
                let pos = suffix as usize;
                let c = encoded[if pos == 0 { encoded.len() - 1 } else { pos - 1 }];
                if (1..=4).contains(&c) {
                    block.masks[c as usize - 1] |= 1u64 << bit;
                    counts[c as usize - 1] += 1;
                }
            }
            ranks.push(block);
        }
        if encoded.len() % 64 == 0 {
            ranks.push(RankBlock {
                masks: [0; 4],
                counts,
            });
        }
        Self {
            sa: sa.into_iter().map(|p| p as u32).collect(),
            ranks,
            cumulative,
            mapping: None,
            mapped_len: 0,
            serialized: None,
            compact: None,
        }
    }

    pub fn len(&self) -> usize {
        self.sa_len() - 1
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn bytes(&self) -> usize {
        self.storage().map_or_else(
            || self.sa_len() * 4 + (self.sa_len() / 64 + 1) * 48 + 40,
            |b| b.len(),
        )
    }

    #[inline]
    fn sa_len(&self) -> usize {
        if self.storage().is_some() {
            self.mapped_len
        } else {
            self.sa.len()
        }
    }

    #[inline]
    fn storage(&self) -> Option<&[u8]> {
        self.mapping.as_deref().or(self.serialized.as_deref())
    }

    #[inline]
    fn locate(&self, mut row: usize) -> u32 {
        if let Some(bytes) = self.storage() {
            if let Some(compact) = self.compact {
                let blocks = self.mapped_len / 64 + 1;
                let marks = 64 + 48 * blocks;
                let values = marks + 12 * blocks;
                for steps in 0..compact.sample_rate {
                    let bit = row % 64;
                    let block = marks + 12 * (row / 64);
                    let mask = u64::from_le_bytes(bytes[block..block + 8].try_into().unwrap());
                    if mask & (1u64 << bit) != 0 {
                        let prefix =
                            u32::from_le_bytes(bytes[block + 8..block + 12].try_into().unwrap())
                                as usize;
                        let sample =
                            prefix + (mask & ((1u64 << bit).wrapping_sub(1))).count_ones() as usize;
                        assert!(sample < compact.samples, "invalid sampled suffix row");
                        let start = values + 4 * sample;
                        let position =
                            u32::from_le_bytes(bytes[start..start + 4].try_into().unwrap())
                                as usize
                                + steps;
                        assert!(
                            position < self.mapped_len,
                            "invalid sampled suffix position"
                        );
                        return position as u32;
                    }
                    let rank_block = 64 + 48 * (row / 64);
                    let mut character = None;
                    for c in 0..4 {
                        let start = rank_block + 8 * c;
                        let mask = u64::from_le_bytes(bytes[start..start + 8].try_into().unwrap());
                        if mask & (1u64 << bit) != 0 {
                            character = Some(c);
                            break;
                        }
                    }
                    row = if let Some(c) = character {
                        (self.cumulative[c] + self.rank(c, row as u32)) as usize
                    } else {
                        // Unknown/separator symbols sort after DNA; the unique terminal
                        // symbol is the primary row, whose SA=0 is always sampled.
                        assert_ne!(row, compact.primary, "missing primary suffix sample");
                        let dna: usize = (0..4).map(|c| self.rank(c, row as u32) as usize).sum();
                        let total_dna: usize = (0..4)
                            .map(|c| self.rank(c, self.mapped_len as u32) as usize)
                            .sum();
                        1 + total_dna + row - dna - usize::from(compact.primary < row)
                    };
                    assert!(row < self.mapped_len, "invalid compact LF step");
                }
                panic!("compact suffix sampling invariant failed");
            }
            let start = 40 + 4 * row;
            u32::from_le_bytes(bytes[start..start + 4].try_into().unwrap())
        } else {
            self.sa[row]
        }
    }

    #[inline]
    fn rank(&self, c: usize, end: u32) -> u32 {
        let bit = end % 64;
        let mask = (1u64 << bit).wrapping_sub(1);
        if let Some(bytes) = self.storage() {
            let block = if self.compact.is_some() {
                64
            } else {
                40 + 4 * self.mapped_len
            } + 48 * (end as usize / 64);
            let bits =
                u64::from_le_bytes(bytes[block + 8 * c..block + 8 * c + 8].try_into().unwrap());
            let count = u32::from_le_bytes(
                bytes[block + 32 + 4 * c..block + 36 + 4 * c]
                    .try_into()
                    .unwrap(),
            );
            count + (bits & mask).count_ones()
        } else {
            let block = &self.ranks[end as usize / 64];
            block.counts[c] + (block.masks[c] & mask).count_ones()
        }
    }

    #[inline]
    fn extend(&self, c: usize, lo: u32, hi: u32) -> (u32, u32) {
        (
            self.cumulative[c] + self.rank(c, lo),
            self.cumulative[c] + self.rank(c, hi),
        )
    }

    /// Enumerate intervals, or stop at the first accepted witness. The callback
    /// can reject intended-gene hits without pruning other associations.
    /// A site may be visited at multiple costs; callers choose its minimum.
    pub fn search(
        &self,
        pattern: &[u8],
        k: usize,
        mut emit: impl FnMut(usize, usize, usize) -> bool,
    ) -> bool {
        self.search_limited(pattern, k, false, &mut emit)
    }

    /// With `limit_half`, allow at most floor(k/2) edits in the first searched
    /// query half. Union this with a reversed-reference/reversed-query search
    /// for complete k-edit coverage; a single limited pass is not exhaustive.
    pub fn search_limited(
        &self,
        pattern: &[u8],
        k: usize,
        limit_half: bool,
        mut emit: impl FnMut(usize, usize, usize) -> bool,
    ) -> bool {
        let distance = crate::IntervalDistance::new(pattern);
        self.search_ranges_limited(pattern, k, limit_half, |lo, hi, span, cost, word| {
            // A first accepted search path need not have the minimum cost for
            // its interval. Normalize the matched word before returning a
            // screening witness, without waiting for other edit paths.
            let mut target = [0u8; 128];
            for (base, &c) in target.iter_mut().zip(word.iter().rev()) {
                *base = b"ACGT"[c];
            }
            let actual_cost = distance.distance(&target[..span]);
            debug_assert!(actual_cost <= cost);
            for row in lo as usize..hi as usize {
                let start = self.locate(row) as usize;
                if emit(start, start + span, actual_cost) {
                    return true;
                }
            }
            false
        })
    }

    fn search_ranges_limited(
        &self,
        pattern: &[u8],
        k: usize,
        limit_half: bool,
        mut emit: impl FnMut(u32, u32, usize, usize, &[usize]) -> bool,
    ) -> bool {
        assert!(!pattern.is_empty() && pattern.len() <= 64 && k < pattern.len());
        assert!(
            pattern
                .iter()
                .all(|b| matches!(b, b'A' | b'C' | b'G' | b'T'))
        );
        let mut encoded = [0usize; 64];
        for (dest, &b) in encoded.iter_mut().zip(pattern) {
            *dest = encode(b) as usize - 1;
        }
        let p = &encoded[..pattern.len()];
        let mut memo = FxHashMap::default();
        self.visit(
            p,
            p.len(),
            0,
            self.sa_len() as u32,
            k,
            0,
            k,
            limit_half.then_some((p.len() / 2, k / 2)),
            &mut memo,
            &mut [0; 128],
            &mut emit,
        )
    }

    /// Merge matching words before locating their occurrences. Equal-length
    /// distinct words have disjoint suffix-array ranges, so each site is emitted
    /// exactly once at its minimum distance, without a per-occurrence hash table.
    pub fn search_unique(
        &self,
        pattern: &[u8],
        k: usize,
        reverse: Option<&Index>,
        emit: impl FnMut(usize, usize, usize) -> bool,
    ) -> bool {
        self.search_unique_cached(pattern, k, reverse, false, emit)
    }

    /// Experimental per-query exact 10-mer range cache. This skips repeated
    /// backward extensions while canonicalizing reverse-search leaves; it does
    /// not prune candidates or change the index format.
    pub fn search_unique_cached(
        &self,
        pattern: &[u8],
        k: usize,
        reverse: Option<&Index>,
        cache_words: bool,
        mut emit: impl FnMut(usize, usize, usize) -> bool,
    ) -> bool {
        let mut ranges: FxHashMap<(u32, u32, usize), usize> = FxHashMap::default();
        self.search_ranges_limited(pattern, k, reverse.is_some(), |lo, hi, span, cost, _| {
            let prior = ranges.entry((lo, hi, span)).or_insert(cost);
            *prior = (*prior).min(cost);
            false
        });
        if let Some(reverse) = reverse {
            let reversed: Vec<_> = pattern.iter().rev().copied().collect();
            let mut seeds: FxHashMap<u32, (u32, u32)> = FxHashMap::default();
            reverse.search_ranges_limited(&reversed, k, true, |_, _, span, cost, word| {
                // word stores the reverse-index match right-to-left. Reversing
                // this order supplies backward extensions for the original word.
                let (mut lo, mut hi) = (0, self.sa_len() as u32);
                let cached = if cache_words && word.len() >= 10 {
                    let suffix = &word[word.len() - 10..];
                    let key = suffix.iter().fold(0u32, |key, &c| (key << 2) | c as u32);
                    (lo, hi) = *seeds.entry(key).or_insert_with(|| {
                        let (mut a, mut b) = (0, self.sa_len() as u32);
                        for &c in suffix.iter().rev() {
                            (a, b) = self.extend(c, a, b);
                            if a == b {
                                break;
                            }
                        }
                        (a, b)
                    });
                    10
                } else {
                    0
                };
                if lo == hi {
                    return false;
                }
                for &c in word[..word.len() - cached].iter().rev() {
                    (lo, hi) = self.extend(c, lo, hi);
                    if lo == hi {
                        return false;
                    }
                }
                let prior = ranges.entry((lo, hi, span)).or_insert(cost);
                *prior = (*prior).min(cost);
                false
            });
        }
        for ((lo, hi, span), cost) in ranges {
            for row in lo as usize..hi as usize {
                let start = self.locate(row) as usize;
                if emit(start, start + span, cost) {
                    return true;
                }
            }
        }
        false
    }

    /// Traverse distinct reference words with a global Levenshtein DP state.
    /// Each interval is emitted once, at its minimum distance.
    pub fn search_automaton(
        &self,
        pattern: &[u8],
        k: usize,
        mut emit: impl FnMut(usize, usize, usize) -> bool,
    ) -> bool {
        assert!(!pattern.is_empty() && pattern.len() <= 64 && k < pattern.len());
        let mut eq = [0u64; 4];
        for (i, &base) in pattern.iter().rev().enumerate() {
            let c = encode(base) as usize - 1;
            assert!(c < 4);
            eq[c] |= 1u64 << i;
        }
        self.visit_automaton(
            &eq,
            pattern.len(),
            k,
            0,
            self.sa_len() as u32,
            0,
            !0,
            0,
            pattern.len(),
            &mut emit,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn visit_automaton(
        &self,
        eq: &[u64; 4],
        m: usize,
        k: usize,
        lo: u32,
        hi: u32,
        depth: usize,
        pv: u64,
        mv: u64,
        score: usize,
        emit: &mut impl FnMut(usize, usize, usize) -> bool,
    ) -> bool {
        if score <= k && depth > 0 {
            for row in lo as usize..hi as usize {
                let a = self.locate(row) as usize;
                if emit(a, a + depth, score) {
                    return true;
                }
            }
        }
        if depth == m + k {
            return false;
        }
        for (c, &equal) in eq.iter().enumerate() {
            let xv = equal | mv;
            let xh = (((equal & pv).wrapping_add(pv)) ^ pv) | equal;
            let mut ph = mv | !(xh | pv);
            let mut mh = pv & xh;
            let next_score =
                score + ((ph >> (m - 1)) & 1) as usize - ((mh >> (m - 1)) & 1) as usize;
            // Global, fixed-interval alignment: leading reference bases cost edits.
            ph = (ph << 1) | 1;
            mh <<= 1;
            let np = mh | !(xv | ph);
            let nm = ph & xv;
            if column_min(np, nm, depth + 1, m, k) > k {
                continue;
            }
            let (a, b) = self.extend(c, lo, hi);
            if a == b {
                continue;
            }
            if self.visit_automaton(eq, m, k, a, b, depth + 1, np, nm, next_score, emit) {
                return true;
            }
        }
        false
    }

    #[allow(clippy::too_many_arguments)]
    fn visit(
        &self,
        p: &[usize],
        remaining: usize,
        lo: u32,
        hi: u32,
        budget: usize,
        span: usize,
        k: usize,
        half_limit: Option<(usize, usize)>,
        memo: &mut FxHashMap<(u64, u16), usize>,
        word: &mut [usize; 128],
        emit: &mut impl FnMut(u32, u32, usize, usize, &[usize]) -> bool,
    ) -> bool {
        if lo == hi {
            return false;
        }
        let half_limit = if let Some((cut, allowed)) = half_limit {
            if k - budget > allowed {
                return false;
            }
            if remaining <= cut { None } else { half_limit }
        } else {
            None
        };
        // With no edits left there is exactly one continuation. Avoid building
        // memo entries and recursive frames for this deterministic tail.
        if budget == 0 {
            let (mut a, mut b) = (lo, hi);
            for (offset, &c) in p[..remaining].iter().rev().enumerate() {
                word[span + offset] = c;
                (a, b) = self.extend(c, a, b);
                if a == b {
                    return false;
                }
            }
            let length = span + remaining;
            if length > 0 {
                // Different edit paths can reach the same reference word.
                // Deduplicate its whole suffix-array range before locating
                // potentially millions of repeated occurrences. This is the
                // same state key as remaining=0 in the general traversal.
                let key = (a as u64 | ((b as u64) << 32), length as u16);
                if memo.contains_key(&key) {
                    return false;
                }
                memo.insert(key, 0);
                if emit(a, b, length, k, &word[..length]) {
                    return true;
                }
            }
            return false;
        }
        let key = (
            lo as u64 | ((hi as u64) << 32),
            ((remaining as u16) << 8) | span as u16,
        );
        if memo.get(&key).is_some_and(|&prior| prior >= budget) {
            return false;
        }
        memo.insert(key, budget);
        if remaining == 0 && span > 0 && emit(lo, hi, span, k - budget, &word[..span]) {
            return true;
        }
        if remaining > 0 {
            let c = p[remaining - 1];
            let (a, b) = self.extend(c, lo, hi);
            word[span] = c;
            if self.visit(
                p,
                remaining - 1,
                a,
                b,
                budget,
                span + 1,
                k,
                half_limit,
                memo,
                word,
                emit,
            ) {
                return true;
            }
        }
        // I: consume one query base; no reference base.
        if remaining > 0
            && self.visit(
                p,
                remaining - 1,
                lo,
                hi,
                budget - 1,
                span,
                k,
                half_limit,
                memo,
                word,
                emit,
            )
        {
            return true;
        }
        for c in 0..4 {
            let (a, b) = self.extend(c, lo, hi);
            if a == b {
                continue;
            }
            word[span] = c;
            // X: consume a different reference and query base.
            if remaining > 0
                && c != p[remaining - 1]
                && self.visit(
                    p,
                    remaining - 1,
                    a,
                    b,
                    budget - 1,
                    span + 1,
                    k,
                    half_limit,
                    memo,
                    word,
                    emit,
                )
            {
                return true;
            }
            // D: consume a reference base, including terminal RNA-extra bases.
            if self.visit(
                p,
                remaining,
                a,
                b,
                budget - 1,
                span + 1,
                k,
                half_limit,
                memo,
                word,
                emit,
            ) {
                return true;
            }
        }
        false
    }

    /// Convert an existing full-SA index without rebuilding its suffix array.
    /// Sampling by text position guarantees at most sample_rate-1 LF steps.
    pub fn write_compact(&self, mut out: impl Write, sample_rate: usize) -> io::Result<()> {
        if !(1..=256).contains(&sample_rate) || self.compact.is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "require full index and sample rate 1..256",
            ));
        }
        let n = self.sa_len();
        let blocks = n / 64 + 1;
        let mut masks = vec![(0u64, 0u32); blocks];
        let mut samples = Vec::with_capacity(n.div_ceil(sample_rate));
        let mut primary = None;
        for (block, (mask, prefix)) in masks.iter_mut().enumerate() {
            *prefix = samples.len() as u32;
            for row in block * 64..((block + 1) * 64).min(n) {
                let pos = self.locate(row);
                if pos == 0 {
                    primary = Some(row);
                }
                if (pos as usize).is_multiple_of(sample_rate) {
                    *mask |= 1u64 << (row % 64);
                    samples.push(pos);
                }
            }
        }
        let primary = primary
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing primary suffix"))?;
        out.write_all(COMPACT_MAGIC)?;
        for value in [n, blocks] {
            out.write_all(&(value as u64).to_le_bytes())?;
        }
        for value in self.cumulative {
            out.write_all(&value.to_le_bytes())?;
        }
        for value in [sample_rate, primary, samples.len()] {
            out.write_all(&(value as u64).to_le_bytes())?;
        }
        if let Some(bytes) = self.storage() {
            out.write_all(&bytes[40 + 4 * n..])?;
        } else {
            for block in &self.ranks {
                for value in block.masks {
                    out.write_all(&value.to_le_bytes())?;
                }
                for value in block.counts {
                    out.write_all(&value.to_le_bytes())?;
                }
            }
        }
        for (mask, prefix) in masks {
            out.write_all(&mask.to_le_bytes())?;
            out.write_all(&prefix.to_le_bytes())?;
        }
        for value in samples {
            out.write_all(&value.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn write(&self, mut out: impl Write) -> io::Result<()> {
        if let Some(bytes) = self.storage() {
            return out.write_all(bytes);
        }
        out.write_all(MAGIC)?;
        out.write_all(&(self.sa.len() as u64).to_le_bytes())?;
        out.write_all(&(self.ranks.len() as u64).to_le_bytes())?;
        for x in self.cumulative {
            out.write_all(&x.to_le_bytes())?;
        }
        for &x in &self.sa {
            out.write_all(&x.to_le_bytes())?;
        }
        for block in &self.ranks {
            for x in block.masks {
                out.write_all(&x.to_le_bytes())?;
            }
            for x in block.counts {
                out.write_all(&x.to_le_bytes())?;
            }
        }
        Ok(())
    }

    pub fn read(mut input: impl Read) -> io::Result<Self> {
        fn u32read(r: &mut impl Read) -> io::Result<u32> {
            let mut b = [0; 4];
            r.read_exact(&mut b)?;
            Ok(u32::from_le_bytes(b))
        }
        fn u64read(r: &mut impl Read) -> io::Result<u64> {
            let mut b = [0; 8];
            r.read_exact(&mut b)?;
            Ok(u64::from_le_bytes(b))
        }
        let mut magic = [0; 8];
        input.read_exact(&mut magic)?;
        if &magic == COMPACT_MAGIC {
            let mut bytes = magic.to_vec();
            input.read_to_end(&mut bytes)?;
            let (n, cumulative, compact) = Self::validate_storage(&bytes)?;
            return Ok(Self {
                sa: Vec::new(),
                ranks: Vec::new(),
                cumulative,
                mapping: None,
                mapped_len: n,
                serialized: Some(bytes),
                compact,
            });
        }
        if &magic != MAGIC {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid index magic",
            ));
        }
        let n = u64read(&mut input)? as usize;
        let blocks = u64read(&mut input)? as usize;
        if n == 0 || n >= i32::MAX as usize || blocks != n / 64 + 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid index dimensions",
            ));
        }
        let mut cumulative = [0; 4];
        for x in &mut cumulative {
            *x = u32read(&mut input)?;
        }
        let mut sa = Vec::with_capacity(n);
        for _ in 0..n {
            let pos = u32read(&mut input)?;
            if pos >= n as u32 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid suffix position",
                ));
            }
            sa.push(pos);
        }
        let mut ranks = Vec::with_capacity(blocks);
        for _ in 0..blocks {
            let mut block = RankBlock {
                masks: [0; 4],
                counts: [0; 4],
            };
            for x in &mut block.masks {
                *x = u64read(&mut input)?;
            }
            for x in &mut block.counts {
                *x = u32read(&mut input)?;
            }
            ranks.push(block);
        }
        Ok(Self {
            sa,
            ranks,
            cumulative,
            mapping: None,
            mapped_len: 0,
            serialized: None,
            compact: None,
        })
    }

    /// Open a previously constructed index without reading unused suffix rows.
    ///
    /// # Safety
    /// The caller must keep the underlying file immutable for this Index's
    /// entire lifetime, including writes/truncation from other processes.
    pub unsafe fn map_immutable(file: &File) -> io::Result<Self> {
        // SAFETY: the caller guarantees the backing file remains immutable.
        let map = unsafe { memmap2::MmapOptions::new().map(file)? };
        let (n, cumulative, compact) = Self::validate_storage(&map)?;
        Ok(Self {
            sa: Vec::new(),
            ranks: Vec::new(),
            cumulative,
            mapping: Some(map),
            mapped_len: n,
            serialized: None,
            compact,
        })
    }

    fn validate_storage(map: &[u8]) -> io::Result<(usize, [u32; 4], Option<Compact>)> {
        let invalid = || io::Error::new(io::ErrorKind::InvalidData, "invalid mapped index");
        if map.len() < 40 || (&map[..8] != MAGIC && &map[..8] != COMPACT_MAGIC) {
            return Err(invalid());
        }
        let n = u64::from_le_bytes(map[8..16].try_into().unwrap()) as usize;
        let blocks = u64::from_le_bytes(map[16..24].try_into().unwrap()) as usize;
        if n == 0 || n >= i32::MAX as usize || blocks != n / 64 + 1 {
            return Err(invalid());
        }
        let cumulative = std::array::from_fn(|c| {
            u32::from_le_bytes(map[24 + 4 * c..28 + 4 * c].try_into().unwrap())
        });
        if cumulative[0] != 1
            || cumulative.windows(2).any(|w| w[0] > w[1])
            || cumulative[3] > n as u32
        {
            return Err(invalid());
        }
        let compact = if &map[..8] == COMPACT_MAGIC {
            if map.len() < 64 {
                return Err(invalid());
            }
            let sample_rate = u64::from_le_bytes(map[40..48].try_into().unwrap()) as usize;
            let primary = u64::from_le_bytes(map[48..56].try_into().unwrap()) as usize;
            let samples = u64::from_le_bytes(map[56..64].try_into().unwrap()) as usize;
            if !(1..=256).contains(&sample_rate)
                || primary >= n
                || samples != n.div_ceil(sample_rate)
                || map.len() != 64 + 60 * blocks + 4 * samples
            {
                return Err(invalid());
            }
            Some(Compact {
                sample_rate,
                primary,
                samples,
            })
        } else {
            if map.len() != 40 + 4 * n + 48 * blocks {
                return Err(invalid());
            }
            None
        };
        Ok((n, cumulative, compact))
    }
}

#[cfg(test)]
mod compact_tests {
    use super::*;
    #[test]
    fn sampled_suffix_positions_match_every_full_suffix_including_separators() {
        let mut random = 7u64;
        for n in [0, 1, 15, 16, 17, 63, 64, 65, 127, 128, 129, 1024] {
            let text: Vec<_> = (0..n)
                .map(|_| {
                    random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
                    b"ACGTNN"[(random >> 32) as usize % 6]
                })
                .collect();
            let full = Index::build(&text);
            for rate in [1, 2, 8, 16, 32, 256] {
                let mut bytes = Vec::new();
                full.write_compact(&mut bytes, rate).unwrap();
                let compact = Index::read(&bytes[..]).unwrap();
                for row in 0..full.sa_len() {
                    assert_eq!(
                        compact.locate(row),
                        full.locate(row),
                        "n={n} rate={rate} row={row}"
                    );
                }
                for end in 0..=full.sa_len() {
                    for c in 0..4 {
                        assert_eq!(compact.rank(c, end as u32), full.rank(c, end as u32));
                    }
                }
                assert!(Index::read(&bytes[..bytes.len() - 1]).is_err());
            }
        }
    }
}
