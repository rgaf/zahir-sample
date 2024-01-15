mod distance_metric;
mod lattice_neighborhood;
mod point;
mod real_point;
mod vertex_neighborhood;

pub use distance_metric::{
    ChebyshevMetric,
    DistanceMetric,
    EuclideanMetric,
    ManhattanMetric,
    MinkowskiMetric,
};

pub use lattice_neighborhood::LatticeNeighborhood;
pub use point::Point;
pub use real_point::RealPoint;
pub use vertex_neighborhood::VertexNeighborhood;

#[macro_export]
macro_rules! point_ops_impl {
    ($vt:ident, $st:ident, $tr:ident, $fn:ident, $mut_tr:ident, $mut_fn:ident, $zero:literal) => {
        impl<const DIM: usize> $tr for $vt<DIM> {
            type Output = Self;

            fn $fn(self, rhs: Self) -> Self {
                let mut coordinates = [$zero; DIM];

                for (idx,  (&lhs, &rhs)) in self.iter().zip(rhs.iter()).enumerate() {
                    coordinates[idx] = lhs.$fn(rhs);
                };

                Self { coordinates }
            }
        }

        impl<const DIM: usize> $tr<$st> for $vt<DIM> {
            type Output = Self;

            fn $fn(self, scalar: $st) -> Self {
                Self { coordinates: self.coordinates.map(|coordinate| coordinate.$fn(scalar)) }
            }
        }

        impl<const DIM: usize> $mut_tr for $vt<DIM> {
            fn $mut_fn(&mut self, rhs: Self) {
                for (lhs, &rhs) in self.iter_mut().zip(rhs.iter()) {
                    lhs.$mut_fn(rhs);
                };
            }
        }

        impl<const DIM: usize> $mut_tr<$st> for $vt<DIM> {
            fn $mut_fn(&mut self, scalar: $st) {
                for coordinate in self.iter_mut() {
                    coordinate.$mut_fn(scalar);
                }
            }
        }
    }
}
