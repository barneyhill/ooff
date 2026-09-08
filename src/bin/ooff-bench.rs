mod engine {
    pub const NAME: &str = "ooff-myers-v1";
    pub struct Prepared(Vec<Vec<u8>>);
    impl Prepared {
        pub fn new(patterns: &[Vec<u8>]) -> Self {
            Self(patterns.to_vec())
        }
        pub fn find(&mut self, text: &[u8], k: usize, _screen: bool) -> Vec<(usize, usize)> {
            self.0
                .iter()
                .enumerate()
                .flat_map(|(q, p)| {
                    oofft::endpoints(p, text, k)
                        .into_iter()
                        .map(move |e| (q, e))
                })
                .collect()
        }
    }
}
include!("../../benchmarks/engine_driver.rs");
