use std::{fmt, mem, ops::*, slice};
use crate::{point_ops_impl, geometry::{DistanceMetric, Point, VertexNeighborhood}};

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct RealPoint<const DIM: usize> {
    coordinates: [f64; DIM],
}

impl<const DIM: usize> RealPoint<DIM> {
    pub const fn new(coordinates: [f64; DIM]) -> Self {
        Self { coordinates }
    }

    pub fn iter(&self) -> slice::Iter<'_, f64> {
        self.coordinates.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, f64> {
        self.coordinates.iter_mut()
    }

    pub fn vertex_neighborhood(self) -> VertexNeighborhood<DIM> {
        VertexNeighborhood::new(self)
    }

    pub fn abs(self) -> Self {
        Self { coordinates: self.coordinates.map(|coordinate| coordinate.abs()) }
    }

    pub fn floor(self) -> Self {
        Self { coordinates: self.coordinates.map(|coordinate| coordinate.floor()) }
    }

    pub fn ceil(self) -> Self {
        Self { coordinates: self.coordinates.map(|coordinate| coordinate.ceil()) }
    }

    pub fn round(self) -> Self {
        Self { coordinates: self.coordinates.map(|coordinate| coordinate.round()) }
    }

    pub fn trunc(self) -> Self {
        Self { coordinates: self.coordinates.map(|coordinate| coordinate.trunc()) }
    }

    pub fn fract(self) -> Self {
        Self { coordinates: self.coordinates.map(|coordinate| coordinate.fract()) }
    }

    pub fn powi(self, exp: i32) -> Self {
        Self { coordinates: self.coordinates.map(|coordinate| coordinate.powi(exp)) }
    }

    pub fn powf(self, exp: f64) -> Self {
        Self { coordinates: self.coordinates.map(|coordinate| coordinate.powf(exp)) }
    }

    pub fn is_finite(self) -> bool {
        self.iter().all(|coordinate| coordinate.is_finite())
    }

    pub fn sum(self) -> f64 {
        self.iter().fold(0.0f64, |acc, &elem| acc + elem)
    }

    pub fn dot_prod(self, rhs: Self) -> f64 {
        self.mul(rhs).sum()
    }

    pub fn magnitude<T: DistanceMetric>(self) -> f64 {
        T::magnitude(self)
    }

    pub fn normalize<T: DistanceMetric>(self) -> Self {
        self.div(self.magnitude::<T>())
    }

    pub fn to_lattice_point(self) -> Point<DIM> {
        Point::new(self.coordinates.map(|coordinate| coordinate.floor() as i32))
    }

    pub fn as_bytes(&self) -> &[u8] {
        let ptr = &self.coordinates as *const f64;
        let num_bytes = DIM * mem::size_of::<f64>();

        unsafe { slice::from_raw_parts(ptr as *const u8, num_bytes) }
    }
}

impl<const DIM: usize> Index<usize> for RealPoint<DIM> {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.coordinates[idx]
    }
}

impl<const DIM: usize> IndexMut<usize> for RealPoint<DIM> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.coordinates[idx]
    }
}

impl<const DIM: usize> fmt::Display for RealPoint<DIM> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut coordinates = self.iter();

        write!(f, "(")?;
        write!(f, "{:?}", coordinates.next().unwrap())?;

        for coordinate in coordinates {
            write!(f, ", {:?}", coordinate)?;
        };

        write!(f, ")")
    }
}

point_ops_impl!(RealPoint, f64, Add, add, AddAssign, add_assign, 0.0f64);
point_ops_impl!(RealPoint, f64, Sub, sub, SubAssign, sub_assign, 0.0f64);
point_ops_impl!(RealPoint, f64, Mul, mul, MulAssign, mul_assign, 0.0f64);
point_ops_impl!(RealPoint, f64, Div, div, DivAssign, div_assign, 0.0f64);
