use rand::SeedableRng;
use rand_chacha::{ChaCha8Rng, ChaCha12Rng, ChaCha20Rng};
use crate::random::Seed;

pub trait Seedable: Sized {
    fn from_seed(seed: &Seed) -> Self;
    fn from_entropy() -> Self {
        let seed = Seed::from_entropy();

        Self::from_seed(&seed)
    }
}

macro_rules! chacha_seedable_impl {
    ($ChaChaXRng:ident) => {
        impl Seedable for $ChaChaXRng {
            fn from_seed(seed: &Seed) -> Self {
                <Self as SeedableRng>::from_seed(seed.bytes)
            }
        }
    }
}

chacha_seedable_impl!(ChaCha20Rng);
chacha_seedable_impl!(ChaCha12Rng);
chacha_seedable_impl!(ChaCha8Rng);
