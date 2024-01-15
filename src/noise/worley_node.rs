use std::{cell::RefCell, marker::PhantomData};
use rand::RngCore;
use rand_chacha::ChaCha8Rng;
use crate::{
    geometry::{DistanceMetric, Point, RealPoint},
    noise::NoiseNode,
    random::{HashFn, Seed, Seedable},
    utils,
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum WorleyPaintMethod {
    Value,
    Distance,
}

pub struct WorleyNode<const DIM: usize, T, U>
where T: DistanceMetric, U: HashFn {
    phantom: PhantomData<T>,
    hash_fn: U,
    rng: RefCell<ChaCha8Rng>,
    paint_method: WorleyPaintMethod,
}

impl<const DIM: usize, T, U> WorleyNode<DIM, T, U>
where T: DistanceMetric, U: HashFn {
    pub fn new(seed: &Seed, paint_method: WorleyPaintMethod) -> Self {
        Self {
            phantom: PhantomData,
            hash_fn: U::from_seed(seed),
            rng: RefCell::new(ChaCha8Rng::from_seed(seed)),
            paint_method,
        }
    }

    fn hypercube_seed_point(&self, hypercube: Point<DIM>) -> (RealPoint<DIM>, u64) {
        let real_hypercube = hypercube.to_real_point();
        let hash = self.hash_fn.hash_bytes(hypercube.as_bytes());

        let mut rng = self.rng.borrow_mut();

        rng.set_word_pos(0);
        rng.set_stream(hash);

        let mut coordinates = [0.0f64; DIM];

        for dim in 0..DIM {
            let fp_mod = utils::f64_from_mantissa(rng.next_u64(), 0.0, 1.0);

            coordinates[dim] = real_hypercube[dim] + fp_mod;
        };

        (RealPoint::new(coordinates), hash)
    }
}

impl<const DIM: usize, T, U> NoiseNode<DIM> for WorleyNode<DIM, T, U>
where T: DistanceMetric, U: HashFn {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let mut candidates = point.to_lattice_point().neighbors_and_self().map(|hypercube| {
            let (seed_point, seed_value) = self.hypercube_seed_point(hypercube);
            let distance = (seed_point - point).magnitude::<T>();

            (seed_value, distance)
        }).collect::<Vec<(u64, f64)>>();

        candidates.sort_by(|a, b| {
            let lhs_distance = a.1;
            let rhs_distance = b.1;

            rhs_distance.partial_cmp(&lhs_distance).unwrap()
        });

        let (seed_value, distance) = candidates.pop().unwrap();

        match self.paint_method {
            WorleyPaintMethod::Value => {
                utils::f64_from_mantissa(seed_value, 0.0, 1.0)
            },

            WorleyPaintMethod::Distance => {
                let (_, other_distance) = candidates.pop().unwrap();

                let max_distance = T::hypercube_diagonal_magnitude::<DIM>();
                let unbiased_value = (other_distance - distance) / max_distance;

                unbiased_value
            },
        }
    }
}
