use crate::{geometry::RealPoint, noise::NoiseNode, utils};

pub struct SigmoidNode<'a, const DIM: usize, T> where T: NoiseNode<DIM> {
    source: &'a T,
    beta: f64,
}

impl<'a, const DIM: usize, T> SigmoidNode<'a, DIM, T> where T: NoiseNode<DIM> {
    pub fn new(source: &'a T, beta: f64) -> Self {
        Self { source, beta }
    }
}

impl<'a, const DIM: usize, T> NoiseNode<DIM> for SigmoidNode<'a, DIM, T> where T: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        utils::sigmoid(self.beta, self.source.value_at(point))
    }
}
