use crate::{geometry::RealPoint, noise::NoiseNode, random::{HashFn, Seed}, utils};

pub struct TileNode<const DIM: usize, T> where T: HashFn {
    hash_fn: T,
}

impl<const DIM: usize, T> TileNode<DIM, T> where T: HashFn {
    pub fn new(seed: &Seed) -> Self {
        Self { hash_fn: T::from_seed(seed) }
    }
}

impl<T> NoiseNode<1> for TileNode<1, T> where T: HashFn {
    fn value_at(&self, point: RealPoint<1>) -> f64 {
        let point = point.floor();
        let hash = self.hash_fn.hash_1u64(point[0].to_bits());

        utils::f64_from_mantissa(hash, 0.0, 1.0)
    }
}

impl<T> NoiseNode<2> for TileNode<2, T> where T: HashFn {
    fn value_at(&self, point: RealPoint<2>) -> f64 {
        let point = point.floor();
        let hash = self.hash_fn.hash_2u64(point[0].to_bits(), point[1].to_bits());

        utils::f64_from_mantissa(hash, 0.0, 1.0)
    }
}

impl<T> NoiseNode<3> for TileNode<3, T> where T: HashFn {
    fn value_at(&self, point: RealPoint<3>) -> f64 {
        let point = point.floor();
        let hash = self.hash_fn.hash_3u64(
            point[0].to_bits(),
            point[1].to_bits(),
            point[2].to_bits(),
        );

        utils::f64_from_mantissa(hash, 0.0, 1.0)
    }
}

impl<T> NoiseNode<4> for TileNode<4, T> where T: HashFn {
    fn value_at(&self, point: RealPoint<4>) -> f64 {
        let point = point.floor();
        let hash = self.hash_fn.hash_4u64(
            point[0].to_bits(),
            point[1].to_bits(),
            point[2].to_bits(),
            point[3].to_bits(),
        );

        utils::f64_from_mantissa(hash, 0.0, 1.0)
    }
}

impl<const DIM: usize, T> NoiseNode<DIM> for TileNode<DIM, T> where T: HashFn {
    default fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let hash = self.hash_fn.hash_bytes(point.floor().as_bytes());

        utils::f64_from_mantissa(hash, 0.0, 1.0)
    }
}
