use rand::Rng;
use crate::random::Seed;

pub trait HashFn {
    fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self;
    fn from_seed(seed: &Seed) -> Self;

    fn hash_1u32(&self, x: u32) -> u64;
    fn hash_2u32(&self, x: u32, y: u32) -> u64;
    fn hash_3u32(&self, x: u32, y: u32, z: u32) -> u64;
    fn hash_4u32(&self, x: u32, y: u32, z: u32, w: u32) -> u64;

    fn hash_1u64(&self, x: u64) -> u64;
    fn hash_2u64(&self, x: u64, y: u64) -> u64;
    fn hash_3u64(&self, x: u64, y: u64, z: u64) -> u64;
    fn hash_4u64(&self, x: u64, y: u64, z: u64, w: u64) -> u64;

    fn hash_bytes(&self, bytes: &[u8]) -> u64;
}
