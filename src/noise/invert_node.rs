use crate::{geometry::RealPoint, noise::NoiseNode};

pub struct InvertNode<'a, const DIM: usize, T> where T: NoiseNode<DIM> {
    source: &'a T,
}

impl<'a, const DIM: usize, T> InvertNode<'a, DIM, T> where T: NoiseNode<DIM> {
    pub fn new(source: &'a T) -> Self {
        Self { source }
    }
}

impl<'a, const DIM: usize, T> NoiseNode<DIM> for InvertNode<'a, DIM, T>
where T: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        1.0 - self.source.value_at(point)
    }
}

