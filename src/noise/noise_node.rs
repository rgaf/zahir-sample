use crate::geometry::RealPoint;

pub trait NoiseNode<const DIM: usize> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64;
}
