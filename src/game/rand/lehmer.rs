use crate::game::rand::{DeterministicRng, Seed};

pub struct LehmerRNG<const M: u64 = { 2 << 48 }, const A: u64 = 44_485_709_377_909>(u64);

impl<const M: u64, const A: u64> Iterator for LehmerRNG<M, A> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self.0.overflowing_mul(A).0 % M;
        Some(self.0)
    }
}

impl LehmerRNG {
    pub fn new() -> Self {
        Self(1)
    }
}

impl DeterministicRng for LehmerRNG {
    fn get_seed(&self) -> Seed {
        Seed(self.0)
    }

    fn set_seed(mut self, new_seed: Seed) -> Self {
        self.0 = new_seed.0;
        self
    }
}
