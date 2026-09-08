//! Exact column reuse for a fixed ASO and a lexicographic walk of target prefixes.
use super::{INF, REVERSE_PAIR, SequencePair, encode, mismatch, pair, parameters};

#[derive(Default)]
pub struct PrefixWorkspace {
    a: Vec<usize>,
    b: Vec<usize>,
    dp: Vec<i32>,
    generic: Vec<i32>,
    one_n: Vec<i32>,
    terminal: Vec<i32>,
    types: Vec<usize>,
    row_min: Vec<i32>,
    snapshots: Vec<i32>,
    prefix_best: Vec<i32>,
    pub columns_computed: u64,
    pub columns_reused: u64,
}
impl PrefixWorkspace {
    pub fn energies(&mut self, pairs: &[SequencePair<'_>]) -> Result<Vec<i32>, &'static str> {
        let mut order: Vec<_> = (0..pairs.len()).collect();
        order.sort_unstable_by(|&i, &j| {
            pairs[i]
                .0
                .cmp(pairs[j].0)
                .then_with(|| pairs[i].1.iter().rev().cmp(pairs[j].1.iter().rev()))
        });
        let mut output = vec![INF; pairs.len()];
        for index in order {
            output[index] = self.energy(pairs[index].0, pairs[index].1)?;
        }
        Ok(output)
    }
    pub fn energy(&mut self, aso: &[u8], target: &[u8]) -> Result<i32, &'static str> {
        if aso.is_empty() || target.is_empty() {
            return Err("Empty energy sequence");
        }
        let a = encode(aso)?;
        let mut b = encode(target)?;
        b.reverse();
        let shared = if a == self.a {
            b.iter().zip(&self.b).take_while(|(x, y)| x == y).count()
        } else {
            0
        };
        // A predecessor's mismatch/exterior cost sees the following target base.
        // Recompute the final shared column; all earlier columns and their
        // minima are identical, even when the new target has a different length.
        let start = shared.saturating_sub(1);
        let (n, m) = (a.len(), b.len());
        let cells = n.checked_mul(m).ok_or("Energy matrix overflow")?;
        let Self {
            dp,
            generic,
            one_n,
            terminal,
            types,
            row_min,
            snapshots,
            prefix_best,
            ..
        } = self;
        dp.resize(cells, INF);
        generic.resize(cells, INF);
        one_n.resize(cells, INF);
        terminal.resize(cells, INF);
        types.resize(cells, 0);
        row_min.resize(n, INF);
        snapshots.resize(cells, INF);
        prefix_best.resize(m, INF);
        let mut best = if start > 0 {
            prefix_best[start - 1]
        } else {
            INF
        };
        if start > 0 {
            row_min.copy_from_slice(&snapshots[(start - 1) * n..start * n]);
        } else {
            row_min.fill(INF);
        }
        let p = parameters();
        let costs = super::fast::costs();
        for j in start..m {
            for i in 0..n {
                let t = pair(a[i], b[j]);
                types[j * n + i] = t;
                if t == 0 {
                    types[j * n + i] = 0;
                    dp[j * n + i] = INF;
                    generic[j * n + i] = INF;
                    one_n[j * n + i] = INF;
                    terminal[j * n + i] = INF;
                    continue;
                }
                let reverse = REVERSE_PAIR[t];
                let mut value = p.duplex_init
                    + p.exterior(
                        t,
                        i.checked_sub(1).map(|k| a[k]),
                        j.checked_sub(1).map(|l| b[l]),
                    );
                {
                    // Stacks, bulges and the explicitly tabulated small loops.
                    let mut consider = |u: usize, v: usize| {
                        if i <= u || j <= v {
                            return;
                        }
                        let (k, l) = (i - u - 1, j - v - 1);
                        let previous_type = types[l * n + k];
                        if previous_type == 0 {
                            return;
                        }
                        let e = p.loop_energy(
                            [u, v],
                            [previous_type, reverse],
                            [a[k + 1], b[l + 1], a[i - 1], b[j - 1]],
                        );
                        value = value.min(dp[l * n + k] + e);
                    };
                    // Keep loop shapes constant at each call so LLVM can fold
                    // table selection and gap arithmetic before the hot loop.
                    consider(0, 0);
                    consider(0, 1);
                    consider(1, 0);
                    consider(1, 1);
                    consider(1, 2);
                    consider(2, 1);
                    consider(2, 2);
                    consider(2, 3);
                    consider(3, 2);
                }
                let terminal_penalty = if t > 2 { p.terminal_au } else { 0 };
                if i > 0 && j > 0 {
                    for gap in 2..=30.min(j - 1) {
                        value = value.min(
                            terminal[(j - gap - 1) * n + i - 1] + p.bulge[gap] + terminal_penalty,
                        );
                    }
                    for gap in 2..=30.min(i - 1) {
                        value = value.min(
                            terminal[(j - 1) * n + i - gap - 1] + p.bulge[gap] + terminal_penalty,
                        );
                    }
                }
                if i > 1 && j > 1 {
                    // 1 x n loops use a distinct terminal mismatch table.
                    let right_1n = mismatch(&p.mismatch1nI, reverse, b[j - 1], a[i - 1]);
                    for gap in 3..=29.min(j - 1) {
                        value = value.min(
                            one_n[(j - gap - 1) * n + i - 2] + costs.generic[1][gap] + right_1n,
                        );
                    }
                    for gap in 3..=29.min(i - 1) {
                        value = value.min(
                            one_n[(j - 2) * n + i - gap - 1] + costs.generic[gap][1] + right_1n,
                        );
                    }
                }
                if i >= 3 && j >= 3 {
                    // All remaining loops have u,v>=2 and total size>=6. Their
                    // predecessor mismatch is already included in `generic`.
                    let right = mismatch(&p.mismatchI, reverse, b[j - 1], a[i - 1]);
                    for u in 2..=28.min(i - 1) {
                        let k = i - u - 1;
                        if row_min[k] + costs.row[u] + right >= value {
                            continue;
                        }
                        let low = 2.max(6usize.saturating_sub(u));
                        let high = (30 - u).min(j - 1);
                        if low > high {
                            continue;
                        }
                        let mut candidate = INF;
                        for v in low..=high {
                            candidate =
                                candidate.min(generic[(j - v - 1) * n + k] + costs.generic[u][v]);
                        }
                        value = value.min(candidate + right);
                    }
                }
                dp[j * n + i] = value;
                terminal[j * n + i] = value + terminal_penalty;
                if i + 1 < n && j + 1 < m {
                    generic[j * n + i] = value + mismatch(&p.mismatchI, t, a[i + 1], b[j + 1]);
                    one_n[j * n + i] = value + mismatch(&p.mismatch1nI, t, a[i + 1], b[j + 1]);
                    row_min[i] = row_min[i].min(generic[j * n + i]);
                }
                best = best
                    .min(value + p.exterior(reverse, b.get(j + 1).copied(), a.get(i + 1).copied()));
            }
            prefix_best[j] = best;
            snapshots[j * n..(j + 1) * n].copy_from_slice(row_min);
        }

        self.columns_reused += start as u64;
        self.columns_computed += (m - start) as u64;
        self.a = a;
        self.b = b;
        Ok(best)
    }
}
