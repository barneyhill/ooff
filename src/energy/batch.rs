//! Independent duplexes occupy SIMD lanes; no approximate band or edit cutoff.
use super::{DuplexWorkspace, INF};
use std::collections::BTreeMap;

pub type SequencePair<'a> = (&'a [u8], &'a [u8]);

pub struct BatchWorkspace {
    scalar: DuplexWorkspace,
    #[cfg(feature = "energy-batch")]
    portable: portable::Workspaces,
    max_lanes: usize,
    shared_prefix: bool,
    shared_paths: bool,
    minplus: bool,
    prefix_scalar: super::PrefixWorkspace,
}
impl Default for BatchWorkspace {
    fn default() -> Self {
        Self {
            scalar: DuplexWorkspace::default(),
            #[cfg(feature = "energy-batch")]
            portable: portable::Workspaces::default(),
            max_lanes: 16,
            shared_prefix: false,
            shared_paths: false,
            minplus: false,
            prefix_scalar: super::PrefixWorkspace::default(),
        }
    }
}
impl BatchWorkspace {
    /// Experimental prefix-path scheduling; current measured workloads regress.
    pub fn enable_shared_paths(&mut self, enable: bool) {
        self.shared_paths = enable;
        if enable {
            self.shared_prefix = true;
            self.minplus = false;
        }
    }
    pub fn enable_minplus(&mut self, enable: bool) {
        self.minplus = enable;
        if enable {
            self.shared_prefix = false;
        }
    }

    pub fn enable_shared_prefix(&mut self, enable: bool) {
        self.shared_prefix = enable;
        self.shared_paths = false;
        if enable {
            self.minplus = false;
        }
    }
    pub fn reused_columns(&self) -> u64 {
        let scalar = self.prefix_scalar.columns_reused;
        #[cfg(feature = "energy-batch")]
        {
            scalar + self.portable.reused_columns()
        }
        #[cfg(not(feature = "energy-batch"))]
        {
            scalar
        }
    }

    /// Limit width for reproducible A/B tests; normal callers use automatic dispatch.
    pub fn limit_lanes(&mut self, lanes: usize) {
        self.max_lanes = lanes;
    }
    pub fn selected_lanes(&self) -> usize {
        #[cfg(feature = "energy-batch")]
        {
            portable::lanes(self.max_lanes)
        }
        #[cfg(not(feature = "energy-batch"))]
        {
            1
        }
    }
    pub fn simd_available() -> bool {
        Self::default().selected_lanes() > 1
    }

    pub fn energies(&mut self, pairs: &[SequencePair<'_>]) -> Result<Vec<i32>, &'static str> {
        #[cfg(feature = "energy-batch")]
        {
            portable::dispatch(self, pairs)
        }
        #[cfg(not(feature = "energy-batch"))]
        {
            self.compute(pairs, 1, |_, _, _, _| unreachable!())
        }
    }

    #[inline(always)]
    fn compute(
        &mut self,
        pairs: &[SequencePair<'_>],
        width: usize,
        mut vector: impl FnMut(
            &mut Self,
            &[SequencePair<'_>],
            &[usize],
            &mut [i32],
        ) -> Result<(), &'static str>,
    ) -> Result<Vec<i32>, &'static str> {
        let mut output = vec![INF; pairs.len()];
        let mut buckets = BTreeMap::<(usize, usize), Vec<usize>>::new();
        for (index, (a, b)) in pairs.iter().enumerate() {
            buckets.entry((a.len(), b.len())).or_default().push(index);
        }
        for indices in buckets.values_mut() {
            if self.shared_prefix {
                indices.sort_unstable_by(|&i, &j| {
                    pairs[i]
                        .0
                        .cmp(pairs[j].0)
                        .then_with(|| pairs[i].1.iter().rev().cmp(pairs[j].1.iter().rev()))
                });
            }
            let scheduled;
            let indices = if self.shared_prefix && self.shared_paths && width > 1 {
                scheduled = schedule_shared(pairs, indices, width);
                &scheduled
            } else {
                &*indices
            };
            for group in indices.chunks(width) {
                if width > 1 && group.len() == width {
                    vector(self, pairs, group, &mut output)?;
                    continue;
                }
                for &index in group {
                    output[index] = if self.shared_prefix {
                        self.prefix_scalar.energy(pairs[index].0, pairs[index].1)?
                    } else {
                        self.scalar.energy(pairs[index].0, pairs[index].1)?
                    };
                }
            }
        }
        Ok(output)
    }
}

// Keep each SIMD lane on one ASO's lexicographically adjacent target path.
// Adjacent output packs otherwise replace every lane's ASO and lose all prefix
// reuse. Longest remaining paths are grouped together to avoid padded lanes.
fn schedule_shared(pairs: &[SequencePair<'_>], sorted: &[usize], width: usize) -> Vec<usize> {
    use std::collections::BinaryHeap;
    let mut paths = BinaryHeap::new();
    let mut begin = 0;
    while begin < sorted.len() {
        let mut end = begin + 1;
        while end < sorted.len() && pairs[sorted[end]].0 == pairs[sorted[begin]].0 {
            end += 1;
        }
        paths.push((end - begin, begin));
        begin = end;
    }
    let mut output = Vec::with_capacity(sorted.len());
    while !paths.is_empty() {
        // A single large ASO group still needs all lanes. Partition its sorted
        // path; each lane then advances by one target, rather than by width.
        while paths.len() < width && paths.peek().unwrap().0 > 1 {
            let (len, start) = paths.pop().unwrap();
            let half = len / 2;
            paths.push((half, start));
            paths.push((len - half, start + half));
        }
        if paths.len() < width {
            while let Some((len, start)) = paths.pop() {
                output.extend_from_slice(&sorted[start..start + len]);
            }
            break;
        }
        let lanes: Vec<_> = (0..width).map(|_| paths.pop().unwrap()).collect();
        let rounds = lanes.iter().map(|p| p.0).min().unwrap();
        for step in 0..rounds {
            for &(_, start) in &lanes {
                output.push(sorted[start + step]);
            }
        }
        for (len, start) in lanes {
            if len > rounds {
                paths.push((len - rounds, start + rounds));
            }
        }
    }
    output
}

#[cfg(feature = "energy-batch")]
#[path = "portable.rs"]
mod portable;
