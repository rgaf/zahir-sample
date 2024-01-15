use crate::{geometry::RealPoint, noise::NoiseNode};

pub struct OverlayNode<'a, const DIM: usize, T, U> where T: NoiseNode<DIM>, U: NoiseNode<DIM> {
    lhs: &'a T,
    rhs: &'a U,
}

impl<'a, const  DIM: usize, T, U> OverlayNode<'a, DIM, T, U>
where T: NoiseNode<DIM>, U: NoiseNode<DIM> {
    pub fn new(lhs: &'a T, rhs: &'a U) -> Self {
        Self { lhs, rhs }
    }
}

impl<'a, const  DIM: usize, T, U> NoiseNode<DIM> for OverlayNode<'a, DIM, T, U>
where T: NoiseNode<DIM>, U: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let lhs_value = self.lhs.value_at(point);
        let rhs_value = self.rhs.value_at(point);

        if lhs_value < 0.5 {
            2.0 * lhs_value * rhs_value
        } else {
            let value = (1.0 - lhs_value) * (1.0 - rhs_value);

            value.mul_add(-2.0, 1.0)
        }
    }
}
