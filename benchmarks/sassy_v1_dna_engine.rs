// External comparator only. The v1 API honors without_trace; the batched v2
// API in pinned 0.2.6 does not pass that flag into its pattern-tiling engine.
mod engine {
    use sassy::{Searcher, profiles::Dna};
    pub const NAME: &str = "sassy-0.2.6-v1-dna-without-trace";
    pub struct Prepared { searcher: Searcher<Dna>, patterns: Vec<Vec<u8>> }
    impl Prepared {
        pub fn new(patterns: &[Vec<u8>]) -> Self {
            Self { searcher: Searcher::<Dna>::new_fwd().without_trace(),
                patterns: patterns.to_vec() }
        }
        pub fn find(&mut self, text: &[u8], k: usize, screen: bool) -> Vec<(usize, usize)> {
            if screen {
                self.searcher.search_patterns(&self.patterns, text, k).iter()
                    .map(|m| (m.pattern_idx,m.text_end)).collect()
            } else {
                self.patterns.iter().enumerate().flat_map(|(q,p)|
                    self.searcher.search_all(p, text, k).into_iter().map(move |m| (q,m.text_end))
                ).collect()
            }
        }
    }
}
include!("engine_driver.rs");
