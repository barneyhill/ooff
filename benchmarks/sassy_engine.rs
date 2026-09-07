// Compiled only in a separate comparator crate on EC2. Never linked into ooff.
mod engine {
    use sassy::{EncodedPatterns, Searcher, profiles::Iupac};
    pub const NAME: &str = "sassy2-0.2.6-iupac-batched";
    pub struct Prepared { searcher: Searcher<Iupac>, encoded: EncodedPatterns<Iupac> }
    impl Prepared {
        pub fn new(patterns: &[Vec<u8>]) -> Self {
            let mut searcher = Searcher::<Iupac>::new_fwd().with_max_n_frac(0.0);
            let encoded = searcher.encode_patterns(patterns);
            Self { searcher, encoded }
        }
        pub fn find(&mut self, text: &[u8], k: usize, screen: bool) -> Vec<(usize, usize)> {
            let hits = if screen { self.searcher.search_encoded_patterns(&self.encoded,text,k) }
                else { self.searcher.search_all_encoded_patterns(&self.encoded,text,k) };
            hits.iter().map(|m| (m.pattern_idx,m.text_end)).collect()
        }
    }
}
include!("engine_driver.rs");
