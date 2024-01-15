use std::marker::PhantomData;
use crate::{geometry::{DistanceMetric, RealPoint}, noise::NoiseNode, utils};

pub struct HypersphereNode<const DIM: usize, T> where T: DistanceMetric {
    frequency: f64,
    phantom: PhantomData<T>,
}

impl<const DIM: usize, T> HypersphereNode<DIM, T> where T: DistanceMetric {
    pub fn new(frequency: f64) -> Self {
        Self { frequency, phantom: PhantomData }
    }
}

impl<const DIM: usize, T> NoiseNode<DIM> for HypersphereNode<DIM, T>
where T: DistanceMetric {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let distance_from_origin = (point * self.frequency).magnitude::<T>();

        let inner_distance = distance_from_origin - distance_from_origin.floor();
        let outer_distance = 1.0 - inner_distance;
        let nearest_distance = inner_distance.min(outer_distance);

        utils::sigmoid(-1.2, nearest_distance.mul_add(-2.0, 1.0))
    }
}
