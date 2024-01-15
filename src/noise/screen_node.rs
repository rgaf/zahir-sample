use crate::{geometry::RealPoint, noise::NoiseNode};

pub struct ScreenNode<'a, const DIM: usize, T, U> where T: NoiseNode<DIM>, U: NoiseNode<DIM> {
    lhs: &'a T,
    rhs: &'a U,
}

impl<'a, const  DIM: usize, T, U> ScreenNode<'a, DIM, T, U>
where T: NoiseNode<DIM>, U: NoiseNode<DIM> {
    pub fn new(lhs: &'a T, rhs: &'a U) -> Self {
        Self { lhs, rhs }
    }
}

impl<'a, const  DIM: usize, T, U> NoiseNode<DIM> for ScreenNode<'a, DIM, T, U>
where T: NoiseNode<DIM>, U: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let lhs_value = self.lhs.value_at(point);
        let rhs_value = self.rhs.value_at(point);

        1.0 - (1.0 - lhs_value) * (1.0 - rhs_value)
    }
}
