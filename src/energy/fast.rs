//! Factor predecessor-dependent mismatch terms out of generic loop evaluation.
use super::{INF, REVERSE_PAIR, encode_into, mismatch, pair, parameters};
use std::sync::OnceLock;

#[inline]
fn min_sum_reversed(left: &[i32], right: &[i32]) -> i32 {
    #[cfg(all(target_arch = "aarch64", feature = "energy-neon"))]
    {
        assert_eq!(left.len(), right.len());
        if left.len() < 4 {
            return left
                .iter()
                .zip(right.iter().rev())
                .map(|(a, b)| a + b)
                .min()
                .unwrap_or(INF);
        }
        use std::arch::aarch64::*;
        // AArch64 provides NEON. Every vector load covers four elements wholly
        // inside its checked slice; arithmetic stays within the energy range.
        unsafe {
            let mut minimum = vdupq_n_s32(i32::MAX);
            let n = left.len();
            let mut k = 0;
            while k + 4 <= n {
                let a = vld1q_s32(left.as_ptr().add(k));
                let b = vld1q_s32(right.as_ptr().add(n - k - 4));
                let reversed_halves = vrev64q_s32(b);
                let reversed = vextq_s32::<2>(reversed_halves, reversed_halves);
                minimum = vminq_s32(minimum, vaddq_s32(a, reversed));
                k += 4;
            }
            let mut result = vminvq_s32(minimum);
            while k < n {
                result = result.min(left[k] + right[n - k - 1]);
                k += 1;
            }
            result
        }
    }
    #[cfg(not(all(target_arch = "aarch64", feature = "energy-neon")))]
    {
        left.iter()
            .zip(right.iter().rev())
            .map(|(a, b)| a + b)
            .min()
            .unwrap_or(INF)
    }
}

pub(super) struct Costs {
    pub(super) generic: [[i32; 31]; 31],
    pub(super) row: [i32; 31],
}
pub(super) fn costs() -> &'static Costs {
    static COSTS: OnceLock<Costs> = OnceLock::new();
    COSTS.get_or_init(|| {
        let p = parameters();
        let mut result = Costs {
            generic: [[INF; 31]; 31],
            row: [INF; 31],
        };
        for u in 1usize..=30 {
            for v in 1usize..=30 - u {
                let value =
                    p.internal_loop[u + v] + ((u.abs_diff(v) as i32) * p.ninio[2]).min(p.max_ninio);
                result.generic[u][v] = value;
                if u >= 2 && v >= 2 && u + v >= 6 {
                    result.row[u] = result.row[u].min(value);
                }
            }
        }
        result
    })
}

/// Reusable per-worker buffers. No previous pair's score is reused.
#[derive(Default)]
pub struct DuplexWorkspace {
    a: Vec<usize>,
    b: Vec<usize>,
    dp: Vec<i32>,
    generic: Vec<i32>,
    one_n: Vec<i32>,
    terminal: Vec<i32>,
    types: Vec<usize>,
    row_min: Vec<i32>,
    #[cfg(feature = "energy-prune")]
    prefix: Vec<usize>,
    #[cfg(feature = "energy-prune")]
    suffix: Vec<usize>,
}

impl DuplexWorkspace {
    pub fn energy(&mut self, aso: &[u8], target: &[u8]) -> Result<i32, &'static str> {
        if aso.is_empty() || target.is_empty() {
            return Err("Empty energy sequence");
        }
        let Self {
            a,
            b,
            dp,
            generic,
            one_n,
            terminal,
            types,
            row_min,
            #[cfg(feature = "energy-prune")]
            prefix,
            #[cfg(feature = "energy-prune")]
            suffix,
        } = self;
        encode_into(aso, a)?;
        encode_into(target, b)?;
        b.reverse();
        let (n, m) = (a.len(), b.len());
        let cells = n.checked_mul(m).ok_or("Energy matrix overflow")?;
        dp.resize(cells, INF);
        generic.resize(cells, INF);
        one_n.resize(cells, INF);
        terminal.resize(cells, INF);
        types.resize(cells, 0);
        row_min.resize(n, INF);
        row_min.fill(INF);
        let p = parameters();
        let costs = costs();
        let mut best = INF;
        #[cfg(feature = "energy-prune")]
        let (step_bound, exterior_bound) = {
            let matrix = (n + 1).checked_mul(m + 1).ok_or("Energy matrix overflow")?;
            prefix.resize(matrix, 0);
            suffix.resize(matrix, 0);
            prefix.fill(0);
            suffix.fill(0);
            // A relaxed ordered matching permits arbitrary gaps at the global
            // minimum loop cost. Every real duplex is one of these matchings.
            let bounds = super::bounds();
            let step = *bounds.loops.iter().flatten().min().unwrap();
            debug_assert!(step <= 0);
            let exterior = p
                .mismatchExt
                .iter()
                .chain(&p.dangle3)
                .chain(&p.dangle5)
                .copied()
                .min()
                .unwrap()
                .min(0)
                + p.terminal_au.min(0);
            for i in (0..n).rev() {
                for j in (0..m).rev() {
                    let paired = usize::from(pair(a[i], b[j]) != 0);
                    suffix[i * (m + 1) + j] = suffix[(i + 1) * (m + 1) + j]
                        .max(suffix[i * (m + 1) + j + 1])
                        .max(suffix[(i + 1) * (m + 1) + j + 1] + paired);
                }
            }
            // Seed an upper bound using valid uninterrupted helices, with the
            // exact initiation and both exterior contributions.
            for i in 0..n {
                for j in 0..m {
                    let t = pair(a[i], b[j]);
                    types[i * m + j] = t;
                    prefix[(i + 1) * (m + 1) + j + 1] = prefix[i * (m + 1) + j + 1]
                        .max(prefix[(i + 1) * (m + 1) + j])
                        .max(prefix[i * (m + 1) + j] + usize::from(t != 0));
                    dp[i * m + j] = INF;
                    if t == 0 {
                        continue;
                    }
                    let mut helix = p.duplex_init
                        + p.exterior(
                            t,
                            i.checked_sub(1).map(|k| a[k]),
                            j.checked_sub(1).map(|l| b[l]),
                        );
                    if i > 0 && j > 0 && types[(i - 1) * m + j - 1] != 0 {
                        helix = helix.min(
                            dp[(i - 1) * m + j - 1]
                                + p.stack[types[(i - 1) * m + j - 1] * 8 + REVERSE_PAIR[t]],
                        );
                    }
                    dp[i * m + j] = helix;
                    best = best.min(
                        helix
                            + p.exterior(
                                REVERSE_PAIR[t],
                                b.get(j + 1).copied(),
                                a.get(i + 1).copied(),
                            ),
                    );
                }
            }
            (step, exterior)
        };
        for i in 0..n {
            for j in 0..m {
                let t = pair(a[i], b[j]);
                types[i * m + j] = t;
                #[cfg(feature = "energy-prune")]
                let irrelevant = {
                    let transitions = prefix[i * (m + 1) + j] + suffix[(i + 1) * (m + 1) + j + 1];
                    let lower = i64::from(p.duplex_init)
                        + 2 * i64::from(exterior_bound)
                        + transitions as i64 * i64::from(step_bound);
                    lower >= i64::from(best)
                };
                #[cfg(not(feature = "energy-prune"))]
                let irrelevant = false;
                if t == 0 || irrelevant {
                    types[i * m + j] = 0;
                    dp[i * m + j] = INF;
                    generic[i * m + j] = INF;
                    one_n[i * m + j] = INF;
                    terminal[i * m + j] = INF;
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
                        let previous_type = types[k * m + l];
                        if previous_type == 0 {
                            return;
                        }
                        let e = p.loop_energy(
                            [u, v],
                            [previous_type, reverse],
                            [a[k + 1], b[l + 1], a[i - 1], b[j - 1]],
                        );
                        value = value.min(dp[k * m + l] + e);
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
                            terminal[(i - 1) * m + j - gap - 1] + p.bulge[gap] + terminal_penalty,
                        );
                    }
                    for gap in 2..=30.min(i - 1) {
                        value = value.min(
                            terminal[(i - gap - 1) * m + j - 1] + p.bulge[gap] + terminal_penalty,
                        );
                    }
                }
                if i > 1 && j > 1 {
                    // 1 x n loops use a distinct terminal mismatch table.
                    let right_1n = mismatch(&p.mismatch1nI, reverse, b[j - 1], a[i - 1]);
                    for gap in 3..=29.min(j - 1) {
                        value = value.min(
                            one_n[(i - 2) * m + j - gap - 1] + costs.generic[1][gap] + right_1n,
                        );
                    }
                    for gap in 3..=29.min(i - 1) {
                        value = value.min(
                            one_n[(i - gap - 1) * m + j - 2] + costs.generic[gap][1] + right_1n,
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
                        let predecessors = &generic[k * m + j - high - 1..k * m + j - low];
                        let penalties = &costs.generic[u][low..=high];
                        let candidate = min_sum_reversed(predecessors, penalties);
                        value = value.min(candidate + right);
                    }
                }
                dp[i * m + j] = value;
                terminal[i * m + j] = value + terminal_penalty;
                if i + 1 < n && j + 1 < m {
                    generic[i * m + j] = value + mismatch(&p.mismatchI, t, a[i + 1], b[j + 1]);
                    one_n[i * m + j] = value + mismatch(&p.mismatch1nI, t, a[i + 1], b[j + 1]);
                    row_min[i] = row_min[i].min(generic[i * m + j]);
                }
                best = best
                    .min(value + p.exterior(reverse, b.get(j + 1).copied(), a.get(i + 1).copied()));
            }
        }
        Ok(best)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vector_reduction_matches_scalar_for_all_tail_lengths() {
        let mut seed = 7654321u64;
        for n in 0..=65 {
            for _ in 0..64 {
                let mut next = || {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    ((seed >> 32) % 20_000_001) as i32 - 10_000_000
                };
                let a: Vec<_> = (0..n).map(|_| next()).collect();
                let b: Vec<_> = (0..n).map(|_| next()).collect();
                assert_eq!(
                    min_sum_reversed(&a, &b),
                    a.iter()
                        .zip(b.iter().rev())
                        .map(|(a, b)| a + b)
                        .min()
                        .unwrap_or(INF)
                );
            }
        }
    }
}
