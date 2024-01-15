use crate::geometry::RealPoint;

pub trait DistanceMetric {
    fn hypercube_diagonal_magnitude<const DIM: usize>() -> f64;
    fn magnitude<const DIM: usize>(point: RealPoint<DIM>) -> f64;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ChebyshevMetric {}

impl DistanceMetric for ChebyshevMetric {
    fn hypercube_diagonal_magnitude<const DIM: usize>() -> f64 {
        1.0
    }

    fn magnitude<const DIM: usize>(point: RealPoint<DIM>) -> f64 {
        *point.abs().iter().reduce(|acc, elem| if acc > elem { acc } else { elem }).unwrap()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EuclideanMetric {}

impl DistanceMetric for EuclideanMetric {
    fn hypercube_diagonal_magnitude<const DIM: usize>() -> f64 {
        (DIM as f64).sqrt()
    }

    fn magnitude<const DIM: usize>(point: RealPoint<DIM>) -> f64 {
        point.dot_prod(point).sqrt()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ManhattanMetric {}

impl DistanceMetric for ManhattanMetric {
    fn hypercube_diagonal_magnitude<const DIM: usize>() -> f64 {
        DIM as f64
    }

    fn magnitude<const DIM: usize>(point: RealPoint<DIM>) -> f64 {
        point.abs().sum()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum MinkowskiMetric<const P: i32, const Q: i32> {}

impl<const P: i32, const Q: i32> MinkowskiMetric<P, Q> {
    const EXP: f64 = (P as f64) / (Q as f64);
    const EXP_RECIP: f64 = (Q as f64) / (P as f64);
}

impl<const P: i32, const Q: i32> DistanceMetric for MinkowskiMetric<P, Q> {
    fn hypercube_diagonal_magnitude<const DIM: usize>() -> f64 {
        (DIM as f64).powf(Self::EXP_RECIP)
    }

    fn magnitude<const DIM: usize>(point: RealPoint<DIM>) -> f64 {
        point.abs().powf(Self::EXP).sum().powf(Self::EXP_RECIP)
    }
}
