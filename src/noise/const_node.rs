use crate::{geometry::RealPoint, noise::NoiseNode};

pub struct ConstNode {
    value: f64,
}

impl ConstNode {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl<const DIM: usize> NoiseNode<DIM> for ConstNode {
    fn value_at(&self, _: RealPoint<DIM>) -> f64 {
        self.value
    }
}
