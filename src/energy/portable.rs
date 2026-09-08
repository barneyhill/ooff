//! One integer-energy recurrence, monomorphized by pulp for the detected CPU.
use super::{INF, SequencePair};
use crate::energy::{Parameters, REVERSE_PAIR, encode, fast, pair, parameters};
#[inline(always)]
fn cell<const REUSE: bool>(i: usize, j: usize, n: usize, m: usize) -> usize {
    if REUSE { j * n + i } else { i * m + j }
}
#[derive(Default)]
pub(super) struct VectorWorkspace<const LANES: usize> {
    dp: Vec<[i32; LANES]>,
    generic: Vec<[i32; LANES]>,
    one_n: Vec<[i32; LANES]>,
    terminal: Vec<[i32; LANES]>,
    types: Vec<[i32; LANES]>,
    row_min: Vec<[i32; LANES]>,
    previous_a: Vec<[i32; LANES]>,
    previous_b: Vec<[i32; LANES]>,
    snapshots: Vec<[i32; LANES]>,
    prefix_best: Vec<[i32; LANES]>,
    pub(super) columns_reused: u64,
    range_minima: Vec<[i32; LANES]>,
    diagonal_minima: Vec<[i32; LANES]>,
}

impl<const LANES: usize> VectorWorkspace<LANES> {
    #[inline(always)]
    unsafe fn range_query<S: pulp::Simd>(
        &self,
        ops: Ops<S>,
        slot: usize,
        weight: usize,
        lo: usize,
        hi: usize,
        n: usize,
    ) -> S::i32s {
        let length = hi - lo + 1;
        let level = (usize::BITS - 1 - length.leading_zeros()) as usize;
        debug_assert!(level < 5);
        let base = ((slot * 3 + weight) * 5 + level) * n;
        unsafe {
            ops.minimum(
                ops.load(self.range_minima[base + lo].as_ptr().cast()),
                ops.load(
                    self.range_minima[base + hi + 1 - (1 << level)]
                        .as_ptr()
                        .cast(),
                ),
            )
        }
    }
    #[inline(always)]
    pub(super) unsafe fn energy<S: pulp::Simd, const REUSE: bool, const MINPLUS: bool>(
        &mut self,
        simd: S,
        inputs: [SequencePair<'_>; LANES],
    ) -> Result<[i32; LANES], &'static str> {
        if !REUSE {
            self.previous_a.clear();
            self.previous_b.clear();
        }
        let ops = Ops(simd);
        assert_eq!(LANES, S::I32_LANES);

        let (n, m) = (inputs[0].0.len(), inputs[0].1.len());
        if n == 0 || m == 0 {
            return Err("Empty energy sequence");
        }
        let mut a = vec![[0i32; LANES]; n];
        let mut b = vec![[0i32; LANES]; m];
        for (lane, input) in inputs.iter().enumerate() {
            for (i, base) in encode(input.0)?.into_iter().enumerate() {
                a[i][lane] = base as i32;
            }
            for (j, base) in encode(input.1)?.into_iter().rev().enumerate() {
                b[j][lane] = base as i32;
            }
        }
        let pair_table: [i32; 25] = std::array::from_fn(|i| pair(i / 5, i % 5) as i32);
        let reverse_table = REVERSE_PAIR.map(|t| t as i32);
        let shared = if REUSE && a == self.previous_a {
            b.iter()
                .zip(&self.previous_b)
                .take_while(|(x, y)| x == y)
                .count()
        } else {
            0
        };
        let start = shared.saturating_sub(1);
        let cells = n.checked_mul(m).ok_or("Energy matrix overflow")?;
        self.dp.resize(cells, [INF; LANES]);
        self.generic.resize(cells, [INF; LANES]);
        self.one_n.resize(cells, [INF; LANES]);
        self.terminal.resize(cells, [INF; LANES]);
        self.types.resize(cells, [0; LANES]);
        self.row_min.resize(n, [INF; LANES]);
        if REUSE {
            self.snapshots.resize(cells, [INF; LANES]);
            self.prefix_best.resize(m, [INF; LANES]);
        }
        if start > 0 {
            self.row_min
                .copy_from_slice(&self.snapshots[(start - 1) * n..start * n]);
        } else {
            self.row_min.fill([INF; LANES]);
        }
        if MINPLUS {
            self.range_minima.resize(32 * 3 * 5 * n, [INF; LANES]);
            self.diagonal_minima.resize(32, [INF; LANES]);
        }
        let p = parameters();
        let costs = fast::costs();
        // Every load/store below addresses exactly LANES elements of a checked
        // fixed-size array. All lanes use the same dimensions; only pair types
        // and empirical costs differ. No vector gather reads outside a table.
        unsafe {
            let infinity = ops.splat(INF);
            let mut best = if start > 0 {
                ops.load(self.prefix_best[start - 1].as_ptr().cast())
            } else {
                infinity
            };
            for major in start..(if MINPLUS {
                n + m - 1
            } else if REUSE {
                m
            } else {
                n
            }) {
                for minor in 0..(if MINPLUS || REUSE { n } else { m }) {
                    if MINPLUS && (minor > major || major - minor >= m) {
                        continue;
                    }
                    let (i, j) = if MINPLUS {
                        (minor, major - minor)
                    } else if REUSE {
                        (minor, major)
                    } else {
                        (major, minor)
                    };
                    let index = cell::<REUSE>(i, j, n, m);
                    let types = ops.gather(
                        pair_table.as_ptr(),
                        ops.add(
                            ops.mul(ops.load(a[i].as_ptr().cast()), ops.splat(5)),
                            ops.load(b[j].as_ptr().cast()),
                        ),
                    );
                    ops.store(self.types[index].as_mut_ptr().cast(), types);
                    let mask = ops.greater(types, ops.zero());
                    if ops.any_mask(mask) == 0 {
                        self.dp[index] = [INF; LANES];
                        self.generic[index] = [INF; LANES];
                        self.one_n[index] = [INF; LANES];
                        self.terminal[index] = [INF; LANES];
                        continue;
                    }
                    let reverse = ops.gather(reverse_table.as_ptr(), types);
                    let mut value = ops.add(
                        ops.splat(p.duplex_init),
                        ops.exterior8(
                            p,
                            types,
                            i.checked_sub(1).map(|k| ops.load(a[k].as_ptr().cast())),
                            j.checked_sub(1).map(|l| ops.load(b[l].as_ptr().cast())),
                        ),
                    );
                    macro_rules! consider {
                        ($u:expr, $v:expr) => {
                            if i > $u && j > $v {
                                let (k, l) = (i - $u - 1, j - $v - 1);
                                let prior = cell::<REUSE>(k, l, n, m);
                                let energy = ops.loop8::<$u, $v>(
                                    p,
                                    ops.load(self.types[prior].as_ptr().cast()),
                                    reverse,
                                    [
                                        ops.load(a[k + 1].as_ptr().cast()),
                                        ops.load(b[l + 1].as_ptr().cast()),
                                        ops.load(a[i - 1].as_ptr().cast()),
                                        ops.load(b[j - 1].as_ptr().cast()),
                                    ],
                                );
                                value = ops.minimum(
                                    value,
                                    ops.add(ops.load(self.dp[prior].as_ptr().cast()), energy),
                                );
                            }
                        };
                    }
                    consider!(0, 0);
                    consider!(0, 1);
                    consider!(1, 0);
                    consider!(1, 1);
                    consider!(1, 2);
                    consider!(2, 1);
                    consider!(2, 2);
                    consider!(2, 3);
                    consider!(3, 2);
                    let terminal_penalty =
                        ops.bit_and(ops.greater(types, ops.splat(2)), ops.splat(p.terminal_au));
                    if i > 0 && j > 0 {
                        for gap in 2..=30.min(j - 1) {
                            let cost = ops.add(terminal_penalty, ops.splat(p.bulge[gap]));
                            value = ops.minimum(
                                value,
                                ops.add(
                                    cost,
                                    ops.load(
                                        self.terminal[cell::<REUSE>(i - 1, j - gap - 1, n, m)]
                                            .as_ptr()
                                            .cast(),
                                    ),
                                ),
                            );
                        }
                        for gap in 2..=30.min(i - 1) {
                            let cost = ops.add(terminal_penalty, ops.splat(p.bulge[gap]));
                            value = ops.minimum(
                                value,
                                ops.add(
                                    cost,
                                    ops.load(
                                        self.terminal[cell::<REUSE>(i - gap - 1, j - 1, n, m)]
                                            .as_ptr()
                                            .cast(),
                                    ),
                                ),
                            );
                        }
                    }
                    if i > 1 && j > 1 {
                        let right = ops.mismatch8(
                            &p.mismatch1nI,
                            reverse,
                            ops.load(b[j - 1].as_ptr().cast()),
                            ops.load(a[i - 1].as_ptr().cast()),
                        );
                        for gap in 3..=29.min(j - 1) {
                            let cost = ops.add(right, ops.splat(costs.generic[1][gap]));
                            value = ops.minimum(
                                value,
                                ops.add(
                                    cost,
                                    ops.load(
                                        self.one_n[cell::<REUSE>(i - 2, j - gap - 1, n, m)]
                                            .as_ptr()
                                            .cast(),
                                    ),
                                ),
                            );
                        }
                        for gap in 3..=29.min(i - 1) {
                            let cost = ops.add(right, ops.splat(costs.generic[gap][1]));
                            value = ops.minimum(
                                value,
                                ops.add(
                                    cost,
                                    ops.load(
                                        self.one_n[cell::<REUSE>(i - gap - 1, j - 2, n, m)]
                                            .as_ptr()
                                            .cast(),
                                    ),
                                ),
                            );
                        }
                    }
                    if i >= 3 && j >= 3 {
                        let right = ops.mismatch8(
                            &p.mismatchI,
                            reverse,
                            ops.load(b[j - 1].as_ptr().cast()),
                            ops.load(a[i - 1].as_ptr().cast()),
                        );
                        if MINPLUS {
                            let total = i + j;
                            for d in 6..=30.min(total.saturating_sub(2)) {
                                let previous_sum = total - d - 2;
                                let lo = i.saturating_sub(d - 1);
                                let hi = previous_sum.min(i - 3);
                                if lo > hi {
                                    continue;
                                }
                                let slot = previous_sum % 32;
                                let constant = ops.add(right, ops.splat(p.internal_loop[d]));
                                let bound = ops.add(
                                    constant,
                                    ops.load(self.diagonal_minima[slot].as_ptr().cast()),
                                );
                                if ops.any_mask(ops.bit_and(mask, ops.greater(value, bound))) == 0 {
                                    continue;
                                }
                                let center = 2 * (i as i32) - d as i32 - 2;
                                let alpha = p.ninio[2];
                                let mut candidate = ops.add(
                                    ops.splat(p.max_ninio),
                                    self.range_query(ops, slot, 0, lo, hi, n),
                                );
                                let left_hi = (hi as i32).min(center.div_euclid(2));
                                if left_hi >= lo as i32 {
                                    candidate = ops.minimum(
                                        candidate,
                                        ops.add(
                                            ops.splat(alpha * center),
                                            self.range_query(ops, slot, 1, lo, left_hi as usize, n),
                                        ),
                                    );
                                }
                                let right_lo = (lo as i32).max((center + 1).div_euclid(2));
                                if right_lo <= hi as i32 {
                                    candidate = ops.minimum(
                                        candidate,
                                        ops.add(
                                            ops.splat(-alpha * center),
                                            self.range_query(
                                                ops,
                                                slot,
                                                2,
                                                right_lo as usize,
                                                hi,
                                                n,
                                            ),
                                        ),
                                    );
                                }
                                value = ops.minimum(value, ops.add(candidate, constant));
                            }
                        } else {
                            for u in 2..=28.min(i - 1) {
                                let k = i - u - 1;
                                let bound = ops.add(
                                    right,
                                    ops.add(
                                        ops.splat(costs.row[u]),
                                        ops.load(self.row_min[k].as_ptr().cast()),
                                    ),
                                );
                                let improves = ops.bit_and(mask, ops.greater(value, bound));
                                if ops.any_mask(improves) == 0 {
                                    continue;
                                }
                                let low = 2.max(6usize.saturating_sub(u));
                                let high = (30 - u).min(j - 1);
                                let mut candidate = infinity;
                                for v in low..=high {
                                    candidate = ops.minimum(
                                        candidate,
                                        ops.add(
                                            ops.load(
                                                self.generic[cell::<REUSE>(k, j - v - 1, n, m)]
                                                    .as_ptr()
                                                    .cast(),
                                            ),
                                            ops.splat(costs.generic[u][v]),
                                        ),
                                    );
                                }
                                value = ops.minimum(value, ops.add(candidate, right));
                            }
                        }
                    }
                    value = ops.select(infinity, value, mask);
                    ops.store(self.dp[index].as_mut_ptr().cast(), value);
                    ops.store(
                        self.terminal[index].as_mut_ptr().cast(),
                        ops.select(infinity, ops.add(value, terminal_penalty), mask),
                    );
                    if i + 1 < n && j + 1 < m {
                        let next_a = ops.load(a[i + 1].as_ptr().cast());
                        let next_b = ops.load(b[j + 1].as_ptr().cast());
                        let generic = ops.select(
                            infinity,
                            ops.add(value, ops.mismatch8(&p.mismatchI, types, next_a, next_b)),
                            mask,
                        );
                        let one = ops.mismatch8(&p.mismatch1nI, types, next_a, next_b);
                        ops.store(self.generic[index].as_mut_ptr().cast(), generic);
                        ops.store(
                            self.one_n[index].as_mut_ptr().cast(),
                            ops.select(infinity, ops.add(value, one), mask),
                        );
                        ops.store(
                            self.row_min[i].as_mut_ptr().cast(),
                            ops.minimum(generic, ops.load(self.row_min[i].as_ptr().cast())),
                        );
                    }
                    let exterior = ops.exterior8(
                        p,
                        reverse,
                        b.get(j + 1).map(|v| ops.load(v.as_ptr().cast())),
                        a.get(i + 1).map(|v| ops.load(v.as_ptr().cast())),
                    );
                    best = ops.minimum(best, ops.select(infinity, ops.add(value, exterior), mask));
                }
                if MINPLUS {
                    let slot = major % 32;
                    let mut diagonal_minimum = infinity;
                    for k in 0..n {
                        let g = if k <= major && major - k < m && k + 1 < n && major - k + 1 < m {
                            ops.load(
                                self.generic[cell::<REUSE>(k, major - k, n, m)]
                                    .as_ptr()
                                    .cast(),
                            )
                        } else {
                            infinity
                        };
                        diagonal_minimum = ops.minimum(diagonal_minimum, g);
                        for w in 0..3 {
                            let offset = match w {
                                1 => -2 * p.ninio[2] * k as i32,
                                2 => 2 * p.ninio[2] * k as i32,
                                _ => 0,
                            };
                            ops.store(
                                self.range_minima[((slot * 3 + w) * 5) * n + k]
                                    .as_mut_ptr()
                                    .cast(),
                                ops.add(g, ops.splat(offset)),
                            );
                        }
                    }
                    ops.store(
                        self.diagonal_minima[slot].as_mut_ptr().cast(),
                        diagonal_minimum,
                    );
                    for w in 0..3 {
                        for level in 1..5 {
                            for k in 0..n {
                                let left = ops.load(
                                    self.range_minima[((slot * 3 + w) * 5 + level - 1) * n + k]
                                        .as_ptr()
                                        .cast(),
                                );
                                let next = k + (1 << (level - 1));
                                let right = if next < n {
                                    ops.load(
                                        self.range_minima
                                            [((slot * 3 + w) * 5 + level - 1) * n + next]
                                            .as_ptr()
                                            .cast(),
                                    )
                                } else {
                                    infinity
                                };
                                ops.store(
                                    self.range_minima[((slot * 3 + w) * 5 + level) * n + k]
                                        .as_mut_ptr()
                                        .cast(),
                                    ops.minimum(left, right),
                                );
                            }
                        }
                    }
                }
                if REUSE {
                    ops.store(self.prefix_best[major].as_mut_ptr().cast(), best);
                    self.snapshots[major * n..(major + 1) * n].copy_from_slice(&self.row_min);
                }
            }
            if REUSE {
                self.columns_reused += (start * LANES) as u64;
                self.previous_a = a;
                self.previous_b = b;
            }
            let mut result = [INF; LANES];
            ops.store(result.as_mut_ptr().cast(), best);
            Ok(result)
        }
    }
}

#[derive(Clone, Copy)]
struct Ops<S>(S);
impl<S: pulp::Simd> Ops<S> {
    #[inline(always)]
    unsafe fn load(self, p: *const i32) -> S::i32s {
        // Callers supply a full, checked LANES-element array.
        unsafe { p.cast::<S::i32s>().read_unaligned() }
    }
    #[inline(always)]
    unsafe fn store(self, p: *mut i32, v: S::i32s) {
        unsafe { p.cast::<S::i32s>().write_unaligned(v) }
    }
    #[inline(always)]
    fn splat(self, v: i32) -> S::i32s {
        self.0.splat_i32s(v)
    }
    #[inline(always)]
    fn zero(self) -> S::i32s {
        self.splat(0)
    }
    #[inline(always)]
    fn add(self, a: S::i32s, b: S::i32s) -> S::i32s {
        self.0.add_i32s(a, b)
    }
    #[inline(always)]
    fn mul(self, a: S::i32s, b: S::i32s) -> S::i32s {
        self.0.mul_i32s(a, b)
    }
    #[inline(always)]
    fn minimum(self, a: S::i32s, b: S::i32s) -> S::i32s {
        self.0.min_i32s(a, b)
    }
    #[inline(always)]
    fn bit_and(self, a: S::i32s, b: S::i32s) -> S::i32s {
        self.0.and_i32s(a, b)
    }
    #[inline(always)]
    fn greater(self, a: S::i32s, b: S::i32s) -> S::i32s {
        self.0
            .select_i32s(self.0.greater_than_i32s(a, b), self.splat(-1), self.zero())
    }
    #[inline(always)]
    fn any_mask(self, v: S::i32s) -> i32 {
        i32::from(
            self.0
                .first_true_m32s(self.0.less_than_i32s(v, self.zero()))
                < S::I32_LANES,
        )
    }
    #[inline(always)]
    fn select(self, a: S::i32s, b: S::i32s, mask: S::i32s) -> S::i32s {
        self.0
            .select_i32s(self.0.less_than_i32s(mask, self.zero()), b, a)
    }
    #[inline(always)]
    unsafe fn gather(self, p: *const i32, indices: S::i32s) -> S::i32s {
        // pulp has no portable gather. Parameter indices are bounded by the
        // recurrence's encoded bases/pair types; NEON needs scalar table reads.
        let mut idx = [0i32; 16];
        let mut values = [0i32; 16];
        unsafe {
            self.store(idx.as_mut_ptr(), indices);
        }
        for lane in 0..S::I32_LANES {
            values[lane] = unsafe { *p.add(idx[lane] as usize) };
        }
        unsafe { self.load(values.as_ptr()) }
    }

    #[inline(always)]
    unsafe fn index8(self, outer: S::i32s, width: i32, inner: S::i32s) -> S::i32s {
        self.add(self.mul(outer, self.splat(width)), inner)
    }

    #[inline(always)]
    unsafe fn mismatch8(self, table: &[i32], t: S::i32s, a: S::i32s, b: S::i32s) -> S::i32s {
        // t is 0..6 and bases are 0..4; the empirical table has 8*5*5 entries.
        unsafe { self.gather(table.as_ptr(), self.index8(self.index8(t, 5, a), 5, b)) }
    }

    #[inline(always)]
    unsafe fn exterior8(
        self,
        p: &Parameters,
        t: S::i32s,
        a: Option<S::i32s>,
        b: Option<S::i32s>,
    ) -> S::i32s {
        unsafe {
            let dangling = match (a, b) {
                (Some(a), Some(b)) => self.mismatch8(&p.mismatchExt, t, a, b),
                (Some(a), None) => self.gather(p.dangle5.as_ptr(), self.index8(t, 5, a)),
                (None, Some(b)) => self.gather(p.dangle3.as_ptr(), self.index8(t, 5, b)),
                (None, None) => self.zero(),
            };
            self.add(
                dangling,
                self.bit_and(self.greater(t, self.splat(2)), self.splat(p.terminal_au)),
            )
        }
    }

    #[inline(always)]
    unsafe fn loop8<const U: usize, const V: usize>(
        self,
        p: &Parameters,
        outer: S::i32s,
        inner: S::i32s,
        bases: [S::i32s; 4],
    ) -> S::i32s {
        // Only the nine explicit small-loop shapes call this function. All lookup
        // indices use canonical/GU pair types (or masked zero) and bases 0..4.
        unsafe {
            let [a, b, c, d] = bases;
            let stem = self.index8(outer, 8, inner);
            if U == 0 || V == 0 {
                let stack = self.gather(p.stack.as_ptr(), stem);
                return if U + V == 0 {
                    stack
                } else {
                    self.add(stack, self.splat(p.bulge[1]))
                };
            }
            if U == 1 && V == 1 {
                return self.gather(p.int11.as_ptr(), self.index8(self.index8(stem, 5, a), 5, b));
            }
            if U == 1 && V == 2 {
                return self.gather(
                    p.int21.as_ptr(),
                    self.index8(self.index8(self.index8(stem, 5, a), 5, d), 5, b),
                );
            }
            if U == 2 && V == 1 {
                let reversed_stem = self.index8(inner, 8, outer);
                return self.gather(
                    p.int21.as_ptr(),
                    self.index8(self.index8(self.index8(reversed_stem, 5, d), 5, a), 5, c),
                );
            }
            if U == 2 && V == 2 {
                return self.gather(
                    p.int22.as_ptr(),
                    self.index8(
                        self.index8(self.index8(self.index8(stem, 5, a), 5, c), 5, d),
                        5,
                        b,
                    ),
                );
            }
            let cost =
                p.internal_loop[U + V] + ((U.abs_diff(V) as i32) * p.ninio[2]).min(p.max_ninio);
            self.add(
                self.splat(cost),
                self.add(
                    self.mismatch8(&p.mismatch23I, outer, a, b),
                    self.mismatch8(&p.mismatch23I, inner, d, c),
                ),
            )
        }
    }
}

#[derive(Default)]
pub(super) struct Workspaces {
    narrow: VectorWorkspace<4>,
    medium: VectorWorkspace<8>,
    wide: VectorWorkspace<16>,
}
impl Workspaces {
    pub(super) fn reused_columns(&self) -> u64 {
        self.narrow.columns_reused + self.medium.columns_reused + self.wide.columns_reused
    }
}

fn arch(limit: usize) -> pulp::Arch {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if limit >= 16 {
            return pulp::Arch::new();
        }
        if limit >= 8
            && let Some(simd) = pulp::x86::V3::try_new()
        {
            return pulp::Arch::V3(simd);
        }
        pulp::Arch::Scalar
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        if limit >= 4 {
            pulp::Arch::new()
        } else {
            pulp::Arch::Scalar
        }
    }
}
struct Width;
impl pulp::WithSimd for Width {
    type Output = usize;
    fn with_simd<S: pulp::Simd>(self, _: S) -> usize {
        S::I32_LANES
    }
}
pub(super) fn lanes(limit: usize) -> usize {
    arch(limit).dispatch(Width)
}

pub(super) fn dispatch(
    workspace: &mut super::BatchWorkspace,
    pairs: &[SequencePair<'_>],
) -> Result<Vec<i32>, &'static str> {
    arch(workspace.max_lanes).dispatch(Run { workspace, pairs })
}
struct Run<'a, 'b, 'c> {
    workspace: &'a mut super::BatchWorkspace,
    pairs: &'b [SequencePair<'c>],
}
impl pulp::WithSimd for Run<'_, '_, '_> {
    type Output = Result<Vec<i32>, &'static str>;
    #[inline(always)]
    fn with_simd<S: pulp::Simd>(self, simd: S) -> Self::Output {
        self.workspace
            .compute(
                self.pairs,
                S::I32_LANES,
                |ws, pairs, group, output| match S::I32_LANES {
                    4 => score(
                        simd,
                        &mut ws.portable.narrow,
                        ws.shared_prefix,
                        ws.minplus,
                        pairs,
                        group,
                        output,
                    ),
                    8 => score(
                        simd,
                        &mut ws.portable.medium,
                        ws.shared_prefix,
                        ws.minplus,
                        pairs,
                        group,
                        output,
                    ),
                    16 => score(
                        simd,
                        &mut ws.portable.wide,
                        ws.shared_prefix,
                        ws.minplus,
                        pairs,
                        group,
                        output,
                    ),
                    _ => unreachable!("unsupported pulp vector width"),
                },
            )
    }
}
#[inline(always)]
fn score<S: pulp::Simd, const N: usize>(
    simd: S,
    ws: &mut VectorWorkspace<N>,
    reuse: bool,
    minplus: bool,
    pairs: &[SequencePair<'_>],
    group: &[usize],
    output: &mut [i32],
) -> Result<(), &'static str> {
    let inputs = std::array::from_fn(|lane| pairs[group[lane]]);
    // Dispatch enters pulp's target-feature context; group width and dimensions
    // were checked by the bucket scheduler before the kernel's array accesses.
    let values = unsafe {
        if minplus {
            ws.energy::<S, false, true>(simd, inputs)?
        } else if reuse {
            ws.energy::<S, true, false>(simd, inputs)?
        } else {
            ws.energy::<S, false, false>(simd, inputs)?
        }
    };
    for (&index, value) in group.iter().zip(values) {
        output[index] = value;
    }
    Ok(())
}
