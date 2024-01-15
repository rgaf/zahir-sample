use crate::{geometry::RealPoint, noise::NoiseNode, utils};

pub struct LerpNode<'a, const DIM: usize, T, U, V>
where T: NoiseNode<DIM>, U: NoiseNode<DIM>, V: NoiseNode<DIM> {
    bias: &'a T,
    lhs: &'a U,
    rhs: &'a V,
}

impl<'a, const  DIM: usize, T, U, V> LerpNode<'a, DIM, T, U, V>
where T: NoiseNode<DIM>, U: NoiseNode<DIM>, V: NoiseNode<DIM> {
    pub fn new(bias: &'a T, lhs: &'a U, rhs: &'a V) -> Self {
        Self { bias, lhs, rhs }
    }
}

impl<'a, const  DIM: usize, T, U, V> NoiseNode<DIM> for LerpNode<'a, DIM, T, U, V>
where T: NoiseNode<DIM>, U: NoiseNode<DIM>, V: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let bias = self.bias.value_at(point);
        let lhs = self.lhs.value_at(point);
        let rhs = self.rhs.value_at(point);

        utils::lerp(bias, lhs, rhs)
    }
}
