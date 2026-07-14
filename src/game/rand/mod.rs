pub trait DeterministicRng: Iterator<Item = u64> {
    fn get_seed(&self) -> Seed;
    fn set_seed(self, new_seed: Seed) -> Self;
}

pub struct Seed(pub u64);

pub mod lehmer;
