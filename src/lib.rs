//! Full-query unit-cost matching against oriented, known RNA sequence.
pub mod fm;
pub mod indexed;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub id: String,
    pub sequence: String,
    pub intended_genes: Vec<String>,
    #[serde(default)]
    pub allele: Option<String>,
    /// Caller-supplied energy annotation; never used to exclude candidate sites.
    #[serde(default)]
    pub ddg: Option<f64>,
}

/// Blocks are genomic intervals in transcript order. The sequence is RNA-sense.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Block {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub id: String,
    pub sequence: String,
    pub genes: Vec<String>,
    #[serde(default)]
    pub transcripts: Vec<String>,
    pub contig: String,
    pub strand: String,
    pub blocks: Vec<Block>,
}

impl Record {
    pub fn validate(&self) -> Result<(), String> {
        self.validate_metadata(self.sequence.len())?;
        normalize(&self.sequence, false)?;
        Ok(())
    }

    pub fn validate_metadata(&self, sequence_length: usize) -> Result<(), String> {
        if self.id.is_empty()
            || self.contig.is_empty()
            || self.genes.is_empty()
            || self.genes.iter().any(|g| g.is_empty())
        {
            return Err("records require IDs, contig and nonempty gene associations".into());
        }
        if self.strand != "+" && self.strand != "-" {
            return Err("record strand must be + or -".into());
        }
        if sequence_length == 0
            || self.blocks.is_empty()
            || self.blocks.iter().any(|b| b.end <= b.start)
            || self.blocks.iter().map(|b| b.end - b.start).sum::<usize>() != sequence_length
        {
            return Err("block lengths must equal nonempty sequence length".into());
        }
        if self.blocks.windows(2).any(|w| {
            if self.strand == "+" {
                w[0].end > w[1].start
            } else {
                w[1].end > w[0].start
            }
        }) {
            return Err("blocks must be nonoverlapping and in transcript order".into());
        }
        Ok(())
    }

    pub fn map_interval(&self, start: usize, end: usize) -> Vec<Block> {
        let mut offset = 0;
        let mut mapped = Vec::new();
        for block in &self.blocks {
            let len = block.end - block.start;
            let a = start.max(offset);
            let b = end.min(offset + len);
            if a < b {
                mapped.push(if self.strand == "+" {
                    Block {
                        start: block.start + a - offset,
                        end: block.start + b - offset,
                    }
                } else {
                    Block {
                        start: block.end - (b - offset),
                        end: block.end - (a - offset),
                    }
                });
            }
            offset += len;
        }
        mapped
    }
}

pub fn normalize(sequence: &str, query: bool) -> Result<Vec<u8>, String> {
    sequence
        .bytes()
        .map(|b| match b.to_ascii_uppercase() {
            b'U' => Ok(b'T'),
            b @ (b'A' | b'C' | b'G' | b'T') => Ok(b),
            b'N' | b'R' | b'Y' | b'S' | b'W' | b'K' | b'M' | b'B' | b'D' | b'H' | b'V'
                if !query =>
            {
                Ok(b'N')
            }
            _ => Err("sequence must contain DNA/RNA letters; queries must be unambiguous".into()),
        })
        .collect()
}

pub fn reverse_complement(sequence: &[u8]) -> Vec<u8> {
    sequence
        .iter()
        .rev()
        .map(|b| match b {
            b'A' => b'T',
            b'C' => b'G',
            b'G' => b'C',
            b'T' => b'A',
            _ => b'N',
        })
        .collect()
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Edit {
    pub operation: char,
    /// 1-based ASO position; RNA-extra bases instead use between-base anchors.
    pub aso_position: Option<usize>,
    pub aso_boundary: Option<usize>,
    pub region: String,
    pub aso_base: Option<char>,
    pub rna_base: Option<char>,
    pub terminal: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Site {
    pub start: usize,
    pub end: usize,
    pub edit_distance: usize,
    /// In RNA-sense order: I consumes pattern only; D consumes RNA only.
    pub cigar: String,
    pub edits: Vec<Edit>,
}

/// Global alignment for a fixed interval; ties prefer diagonal, I, then D.
pub fn align(pattern: &[u8], target: &[u8]) -> (usize, Vec<char>) {
    if pattern.len() <= 20 && target.len() <= 23 {
        return align_small(pattern, target);
    }
    align_general(pattern, target)
}

// Fixed 20mer workload: avoid a heap allocation for every DP row. The scalar
// recurrence and diagonal/I/D tie order are unchanged. Both native annotation
// and the external comparator's interval verifier use this function.
fn align_small(pattern: &[u8], target: &[u8]) -> (usize, Vec<char>) {
    const STRIDE: usize = 24;
    let mut dp = [0u8; 21 * STRIDE];
    for i in 0..=pattern.len() {
        dp[i * STRIDE] = i as u8;
    }
    for (j, cell) in dp.iter_mut().take(target.len() + 1).enumerate() {
        *cell = j as u8;
    }
    for i in 1..=pattern.len() {
        for j in 1..=target.len() {
            let cell = i * STRIDE + j;
            dp[cell] = (dp[cell - STRIDE - 1] + u8::from(pattern[i - 1] != target[j - 1]))
                .min(dp[cell - STRIDE] + 1)
                .min(dp[cell - 1] + 1);
        }
    }
    let cost = dp[pattern.len() * STRIDE + target.len()] as usize;
    let (mut i, mut j) = (pattern.len(), target.len());
    let mut ops = Vec::with_capacity(i + j);
    while i > 0 || j > 0 {
        let cell = i * STRIDE + j;
        if i > 0
            && j > 0
            && dp[cell] == dp[cell - STRIDE - 1] + u8::from(pattern[i - 1] != target[j - 1])
        {
            ops.push(if pattern[i - 1] == target[j - 1] {
                '='
            } else {
                'X'
            });
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[cell] == dp[cell - STRIDE] + 1 {
            ops.push('I');
            i -= 1;
        } else {
            ops.push('D');
            j -= 1;
        }
    }
    ops.reverse();
    (cost, ops)
}

fn align_general(pattern: &[u8], target: &[u8]) -> (usize, Vec<char>) {
    let n = target.len();
    let mut dp = vec![vec![0; n + 1]; pattern.len() + 1];
    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=pattern.len() {
        for j in 1..=n {
            dp[i][j] = (dp[i - 1][j - 1] + usize::from(pattern[i - 1] != target[j - 1]))
                .min(dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1);
        }
    }
    let cost = dp[pattern.len()][n];
    let (mut i, mut j) = (pattern.len(), n);
    let mut ops = Vec::new();
    while i > 0 || j > 0 {
        if i > 0
            && j > 0
            && dp[i][j] == dp[i - 1][j - 1] + usize::from(pattern[i - 1] != target[j - 1])
        {
            ops.push(if pattern[i - 1] == target[j - 1] {
                '='
            } else {
                'X'
            });
            i -= 1;
            j -= 1;
        } else if i > 0 && dp[i][j] == dp[i - 1][j] + 1 {
            ops.push('I');
            i -= 1;
        } else {
            ops.push('D');
            j -= 1;
        }
    }
    ops.reverse();
    (cost, ops)
}

pub fn align_site(pattern: &[u8], target: &[u8], start: usize) -> Site {
    let (cost, ops) = align(pattern, target);
    site(pattern, target, start, cost, &ops)
}

fn region(pos: usize) -> String {
    if (6..=15).contains(&pos) {
        "DNA_gap"
    } else {
        "MOE_wing"
    }
    .into()
}

fn site(pattern: &[u8], target: &[u8], start: usize, cost: usize, ops: &[char]) -> Site {
    let mut cigar = String::new();
    let mut edits = Vec::new();
    let (mut p, mut t) = (0, 0);
    let first_pair = ops
        .iter()
        .position(|op| matches!(op, '=' | 'X'))
        .unwrap_or(ops.len());
    let last_pair = ops
        .iter()
        .rposition(|op| matches!(op, '=' | 'X'))
        .unwrap_or(0);
    let mut index = 0;
    while index < ops.len() {
        let op = ops[index];
        let mut end = index + 1;
        while end < ops.len() && ops[end] == op {
            end += 1;
        }
        cigar.push_str(&format!("{}{op}", end - index));
        index = end;
    }
    for (i, &op) in ops.iter().enumerate() {
        if op != '=' {
            let pos = pattern.len() - p;
            let boundary = pos;
            edits.push(Edit {
                operation: op,
                aso_position: (op != 'D').then_some(pos),
                aso_boundary: (op == 'D').then_some(boundary),
                region: if op == 'D' {
                    if boundary == 5 || boundary == 15 {
                        "wing_gap_boundary".into()
                    } else if (6..15).contains(&boundary) {
                        "DNA_gap".into()
                    } else {
                        "MOE_wing".into()
                    }
                } else {
                    region(pos)
                },
                aso_base: if op != 'D' {
                    Some(reverse_complement(&pattern[p..p + 1])[0] as char)
                } else {
                    None
                },
                rna_base: if op != 'I' {
                    Some(if target[t] == b'T' {
                        'U'
                    } else {
                        target[t] as char
                    })
                } else {
                    None
                },
                terminal: matches!(op, 'I' | 'D') && (i < first_pair || i > last_pair),
            });
        }
        if op != 'D' {
            p += 1;
        }
        if op != 'I' {
            t += 1;
        }
    }
    Site {
        start,
        end: start + target.len(),
        edit_distance: cost,
        cigar,
        edits,
    }
}

/// Native Myers bit-vector infix edit distance, for patterns of 1..=63 bases.
/// The zero entering the low bit of Ph gives free reference-prefix skips.
pub fn endpoints(pattern: &[u8], text: &[u8], k: usize) -> Vec<usize> {
    assert!(!pattern.is_empty() && pattern.len() <= 63);
    let mut masks = [0u64; 256];
    for (i, &base) in pattern.iter().enumerate() {
        masks[base as usize] |= 1 << i;
    }
    let high = 1u64 << (pattern.len() - 1);
    let (mut pv, mut mv, mut score) = (!0u64, 0u64, pattern.len());
    let mut found = Vec::new();
    for (i, &base) in text.iter().enumerate() {
        let eq = masks[base as usize];
        let xv = eq | mv;
        let xh = ((eq & pv).wrapping_add(pv) ^ pv) | eq;
        let mut ph = mv | !(xh | pv);
        let mut mh = pv & xh;
        if ph & high != 0 {
            score += 1;
        }
        if mh & high != 0 {
            score -= 1;
        }
        ph <<= 1;
        mh <<= 1;
        pv = mh | !(xv | ph);
        mv = ph & xv;
        if score <= k {
            found.push(i + 1);
        }
    }
    found
}

/// Reusable full-query, fixed-interval Levenshtein distance without traceback.
/// Unlike `endpoints`, the low Ph bit is one: target prefixes are not free.
pub struct IntervalDistance {
    masks: [u64; 256],
    length: usize,
}

impl IntervalDistance {
    pub fn new(pattern: &[u8]) -> Self {
        assert!(!pattern.is_empty() && pattern.len() <= 64);
        let mut masks = [0; 256];
        for (i, &base) in pattern.iter().enumerate() {
            masks[base as usize] |= 1 << i;
        }
        Self {
            masks,
            length: pattern.len(),
        }
    }

    pub fn distance(&self, target: &[u8]) -> usize {
        let high = 1u64 << (self.length - 1);
        let (mut pv, mut mv, mut score) = (!0u64, 0u64, self.length);
        for &base in target {
            let eq = self.masks[base as usize];
            let xv = eq | mv;
            let xh = ((eq & pv).wrapping_add(pv) ^ pv) | eq;
            let mut ph = mv | !(xh | pv);
            let mut mh = pv & xh;
            score += usize::from(ph & high != 0);
            score -= usize::from(mh & high != 0);
            ph = (ph << 1) | 1;
            mh <<= 1;
            pv = mh | !(xv | ph);
            mv = ph & xv;
        }
        score
    }
}

/// One record/chunk at a time. Myers supplies all endpoints. Fixed-interval
/// verification recovers every start, including non-minimum-cost starts.
/// Return false from emit to retire a query immediately during verification.
pub fn search_chunk(
    patterns: &[Vec<u8>],
    text: &[u8],
    k: usize,
    mut emit: impl FnMut(usize, Site) -> bool,
) {
    if patterns.is_empty() || text.is_empty() {
        return;
    }
    for (q, pattern) in patterns.iter().enumerate() {
        let verifier = IntervalDistance::new(pattern);
        'query: for end in endpoints(pattern, text, k) {
            for span in pattern.len().saturating_sub(k).max(1)..=pattern.len() + k {
                if span > end {
                    continue;
                }
                let start = end - span;
                if verifier.distance(&text[start..end]) > k {
                    continue;
                }
                let (cost, ops) = align(pattern, &text[start..end]);
                if cost <= k && !emit(q, site(pattern, &text[start..end], start, cost, &ops)) {
                    break 'query;
                }
            }
        }
    }
}

/// Windows never cross ambiguity; ownership is by start within each core.
pub fn chunks(text: &[u8], core: usize, overlap: usize) -> Vec<(usize, usize, usize)> {
    let mut result = Vec::new();
    let mut pos = 0;
    while pos < text.len() {
        if text[pos] == b'N' {
            pos += 1;
            continue;
        }
        let start = pos;
        while pos < text.len() && text[pos] != b'N' {
            pos += 1;
        }
        for a in (start..pos).step_by(core) {
            result.push((a, (a + core + overlap).min(pos), (a + core).min(pos)));
        }
    }
    result
}

#[cfg(test)]
mod alignment_storage_tests {
    #[test]
    fn flat_storage_preserves_distances_and_traceback_ties() {
        let mut state = 419u64;
        for m in 0..=20 {
            for n in 0..=23 {
                let mut base = || {
                    state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                    b"ACGT"[(state >> 32) as usize % 4]
                };
                let pattern: Vec<_> = (0..m).map(|_| base()).collect();
                let target: Vec<_> = (0..n).map(|_| base()).collect();
                assert_eq!(
                    super::align_small(&pattern, &target),
                    super::align_general(&pattern, &target)
                );
            }
        }
    }
}
