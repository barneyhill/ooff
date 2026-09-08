//! Independent scalar intermolecular duplex DP, using integer cent-kcal energies.
//! Only the pinned RNA:RNA / 37 C / default-salt model is currently supported.
//! Empirical parameter provenance is documented alongside the tables.
use serde::Deserialize;
use std::sync::OnceLock;
mod batch;
mod prefix;
pub use prefix::PrefixWorkspace;
mod fast;
pub use batch::{BatchWorkspace, SequencePair};
pub use fast::DuplexWorkspace;

const INF: i32 = 10_000_000;
const REVERSE_PAIR: [usize; 8] = [0, 2, 1, 4, 3, 6, 5, 7];

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Parameters {
    stack: Vec<i32>,
    bulge: Vec<i32>,
    internal_loop: Vec<i32>,
    mismatchExt: Vec<i32>,
    mismatchI: Vec<i32>,
    mismatch1nI: Vec<i32>,
    mismatch23I: Vec<i32>,
    dangle5: Vec<i32>,
    dangle3: Vec<i32>,
    int11: Vec<i32>,
    int21: Vec<i32>,
    int22: Vec<i32>,
    ninio: Vec<i32>,
    terminal_au: i32,
    duplex_init: i32,
    max_ninio: i32,
}

fn parameters() -> &'static Parameters {
    static PARAMETERS: OnceLock<Parameters> = OnceLock::new();
    PARAMETERS.get_or_init(|| {
        serde_json::from_str(include_str!("turner2004_37c.json")).expect("Pinned energy tables")
    })
}

// Admissible lower bounds, derived from the same complete energy tables.
// The independently minimized mismatch terms can only underestimate a real
// loop energy, so these bounds skip work without excluding an optimal duplex.
struct Bounds {
    loops: [[i32; 31]; 31],
    row: [i32; 31],
}
fn bounds() -> &'static Bounds {
    static BOUNDS: OnceLock<Bounds> = OnceLock::new();
    BOUNDS.get_or_init(|| {
        let p = parameters();
        let minimum = |v: &[i32]| *v.iter().min().unwrap();
        let stack = minimum(&p.stack);
        let mut result = Bounds {
            loops: [[INF; 31]; 31],
            row: [INF; 31],
        };
        for u in 0..=30 {
            for v in 0..=30 - u {
                let lo = u.min(v);
                let hi = u.max(v);
                let value = if hi == 0 {
                    stack
                } else if lo == 0 {
                    p.bulge[hi] + if hi == 1 { stack } else { 0 }
                } else if hi == 1 {
                    minimum(&p.int11)
                } else if lo == 1 && hi == 2 {
                    minimum(&p.int21)
                } else if lo == 2 && hi == 2 {
                    minimum(&p.int22)
                } else {
                    let table = if lo == 1 {
                        &p.mismatch1nI
                    } else if lo == 2 && hi == 3 {
                        &p.mismatch23I
                    } else {
                        &p.mismatchI
                    };
                    p.internal_loop[u + v]
                        + ((hi - lo) as i32 * p.ninio[2]).min(p.max_ninio)
                        + 2 * minimum(table)
                };
                result.loops[u][v] = value;
                result.row[u] = result.row[u].min(value);
            }
        }
        result
    })
}

fn encode(sequence: &[u8]) -> Result<Vec<usize>, &'static str> {
    sequence
        .iter()
        .map(|b| match b.to_ascii_uppercase() {
            b'A' => Ok(1),
            b'C' => Ok(2),
            b'G' => Ok(3),
            b'U' | b'T' => Ok(4),
            b'N' => Ok(0),
            _ => Err("Energy sequences must contain A/C/G/T/U/N"),
        })
        .collect()
}

fn encode_into(sequence: &[u8], output: &mut Vec<usize>) -> Result<(), &'static str> {
    output.clear();
    for b in sequence {
        output.push(match b.to_ascii_uppercase() {
            b'A' => 1,
            b'C' => 2,
            b'G' => 3,
            b'U' | b'T' => 4,
            b'N' => 0,
            _ => return Err("Energy sequences must contain A/C/G/T/U/N"),
        });
    }
    Ok(())
}

#[inline]
fn pair(a: usize, b: usize) -> usize {
    match (a, b) {
        (2, 3) => 1,
        (3, 2) => 2,
        (3, 4) => 3,
        (4, 3) => 4,
        (1, 4) => 5,
        (4, 1) => 6,
        _ => 0,
    }
}

#[inline]
fn mismatch(table: &[i32], pair: usize, a: usize, b: usize) -> i32 {
    table[(pair * 5 + a) * 5 + b]
}

impl Parameters {
    #[inline]
    fn exterior(&self, t: usize, left: Option<usize>, right: Option<usize>) -> i32 {
        let dangling = match (left, right) {
            (Some(a), Some(b)) => mismatch(&self.mismatchExt, t, a, b),
            (Some(a), None) => self.dangle5[t * 5 + a],
            (None, Some(b)) => self.dangle3[t * 5 + b],
            (None, None) => 0,
        };
        dangling + if t > 2 { self.terminal_au } else { 0 }
    }

    #[inline]
    fn loop_energy(&self, gaps: [usize; 2], types: [usize; 2], bases: [usize; 4]) -> i32 {
        let [u, v] = gaps;
        let [outer, inner] = types;
        let [a, b, c, d] = bases;
        let largest = u.max(v);
        let smallest = u.min(v);
        let stack = self.stack[outer * 8 + inner];
        if largest == 0 {
            return stack;
        }
        if smallest == 0 {
            return self.bulge[largest]
                + if largest == 1 {
                    stack
                } else {
                    self.terminal_au * (i32::from(outer > 2) + i32::from(inner > 2))
                };
        }
        let stem_index = outer * 8 + inner;
        if u == 1 && v == 1 {
            return self.int11[(stem_index * 5 + a) * 5 + b];
        }
        if smallest == 1 && largest == 2 {
            return if u == 1 {
                self.int21[((stem_index * 5 + a) * 5 + d) * 5 + b]
            } else {
                self.int21[(((inner * 8 + outer) * 5 + d) * 5 + a) * 5 + c]
            };
        }
        if u == 2 && v == 2 {
            return self.int22[(((stem_index * 5 + a) * 5 + c) * 5 + d) * 5 + b];
        }
        let asymmetric = ((largest - smallest) as i32 * self.ninio[2]).min(self.max_ninio);
        let mismatch_table = if smallest == 1 {
            &self.mismatch1nI
        } else if smallest == 2 && largest == 3 {
            &self.mismatch23I
        } else {
            &self.mismatchI
        };
        self.internal_loop[u + v]
            + asymmetric
            + mismatch(mismatch_table, outer, a, b)
            + mismatch(mismatch_table, inner, d, c)
    }
}

/// Minimum duplex energy in 0.01 kcal/mol, matching the default RNAduplex model.
/// The strands are both supplied 5' to 3'; pairing is antiparallel. The best
/// duplex may use only part of either strand. No edit budget or CIGAR is imposed.
pub fn duplex_energy(aso: &[u8], target: &[u8]) -> Result<i32, &'static str> {
    DuplexWorkspace::default().energy(aso, target)
}

/// Straightforward reference recurrence retained for differential verification.
pub fn duplex_energy_scalar(aso: &[u8], target: &[u8]) -> Result<i32, &'static str> {
    if aso.is_empty() || target.is_empty() {
        return Err("Empty energy sequence");
    }
    let a = encode(aso)?;
    let mut b = encode(target)?;
    b.reverse();
    let p = parameters();
    let bounds = bounds();
    let (n, m) = (a.len(), b.len());
    let cells = n.checked_mul(m).ok_or("Energy matrix overflow")?;
    let mut dp = vec![INF; cells];
    let mut row_minimum = vec![INF; n];
    let mut best = INF;
    for i in 0..n {
        for j in 0..m {
            let t = pair(a[i], b[j]);
            if t == 0 {
                continue;
            }
            let mut value = p.duplex_init
                + p.exterior(
                    t,
                    i.checked_sub(1).map(|k| a[k]),
                    j.checked_sub(1).map(|l| b[l]),
                );
            for k in (i.saturating_sub(31)..i).rev() {
                let u = i - k - 1;
                if row_minimum[k] + bounds.row[u] >= value {
                    continue;
                }
                for l in (j.saturating_sub(31 - u)..j).rev() {
                    let v = j - l - 1;
                    let previous = dp[k * m + l];
                    if previous + bounds.loops[u][v] >= value {
                        continue;
                    }
                    let predecessor = pair(a[k], b[l]);
                    if predecessor == 0 {
                        continue;
                    }
                    let energy = p.loop_energy(
                        [u, v],
                        [predecessor, REVERSE_PAIR[t]],
                        [a[k + 1], b[l + 1], a[i - 1], b[j - 1]],
                    );
                    value = value.min(dp[k * m + l] + energy);
                }
            }
            dp[i * m + j] = value;
            row_minimum[i] = row_minimum[i].min(value);
            best = best.min(
                value
                    + p.exterior(
                        REVERSE_PAIR[t],
                        b.get(j + 1).copied(),
                        a.get(i + 1).copied(),
                    ),
            );
        }
    }
    Ok(best)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    #[derive(Deserialize)]
    struct Shape {
        u: usize,
        v: usize,
        sha256: String,
    }
    #[test]
    fn every_loop_energy_matches_independent_library_oracle() {
        let shapes: Vec<Shape> =
            serde_json::from_str(include_str!("../../fixtures/energy_loops_golden.json")).unwrap();
        let p = parameters();
        for shape in shapes {
            let mut hash = Sha256::new();
            let lower_bound = bounds().loops[shape.u][shape.v];
            for outer in 1..=6 {
                for inner in 1..=6 {
                    for a in 0..5 {
                        for b in 0..5 {
                            for c in 0..5 {
                                for d in 0..5 {
                                    let value = p.loop_energy(
                                        [shape.u, shape.v],
                                        [outer, inner],
                                        [a, b, c, d],
                                    );
                                    assert!(value >= lower_bound, "Invalid pruning bound");
                                    hash.update(value.to_le_bytes());
                                }
                            }
                        }
                    }
                }
            }
            assert_eq!(
                format!("{:x}", hash.finalize()),
                shape.sha256,
                "loop {} x {}",
                shape.u,
                shape.v
            );
        }
    }
}
