use std::{fmt, mem, ops::*, slice};
use crate::{point_ops_impl, geometry::{LatticeNeighborhood, RealPoint}};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct Point<const DIM: usize> {
    coordinates: [i32; DIM],
}

impl<const DIM: usize> Point<DIM> {
    pub const fn new(coordinates: [i32; DIM]) -> Self {
        Self { coordinates }
    }

    pub fn iter(&self) -> slice::Iter<'_, i32> {
        self.coordinates.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, i32> {
        self.coordinates.iter_mut()
    }

    pub fn neighbors(self) -> LatticeNeighborhood<DIM> {
        LatticeNeighborhood::new(self, false)
    }

    pub fn neighbors_and_self(self) -> LatticeNeighborhood<DIM> {
        LatticeNeighborhood::new(self, true)
    }

    pub fn sum(self) -> i32 {
        self.iter().fold(0i32, |acc, &elem| acc + elem)
    }

    pub fn dot_prod(self, rhs: Self) -> i32 {
        self.mul(rhs).sum()
    }

    pub fn abs(self) -> Self {
        Self { coordinates: self.coordinates.map(|coordinate| coordinate.abs()) }
    }

    pub fn to_real_point(self) -> RealPoint<DIM> {
        RealPoint::new(self.coordinates.map(|coordinate| coordinate as f64))
    }

    pub fn as_bytes(&self) -> &[u8] {
        let ptr = &self.coordinates as *const i32;
        let num_bytes = DIM * mem::size_of::<i32>();

        unsafe { slice::from_raw_parts(ptr as *const u8, num_bytes) }
    }
}

impl<const DIM: usize> Index<usize> for Point<DIM> {
    type Output = i32;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.coordinates[idx]
    }
}

impl<const DIM: usize> IndexMut<usize> for Point<DIM> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.coordinates[idx]
    }
}

impl<const DIM: usize> fmt::Display for Point<DIM> {
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

point_ops_impl!(Point, i32, Add, add, AddAssign, add_assign, 0i32);
point_ops_impl!(Point, i32, Sub, sub, SubAssign, sub_assign, 0i32);
point_ops_impl!(Point, i32, Mul, mul, MulAssign, mul_assign, 0i32);
point_ops_impl!(Point, i32, Div, div, DivAssign, div_assign, 0i32);

#[cfg(test)]
mod tests {
    use crate::geometry::Point;

    #[test]
    fn add() {
        let a = Point::new([0, 10, 25]);
        let b = Point::new([5, 7, -9]);
        let c = a + b;
        let d = a + 2;

        assert_eq!(c, Point::new([5, 17, 16]));
        assert_eq!(d, Point::new([2, 12, 27]));

        let mut e = a;

        e += a;
        assert_eq!(e, Point::new([0, 20, 50]));

        e += 2;
        assert_eq!(e, Point::new([2, 22, 52]));
    }

    #[test]
    fn sub() {
        let a = Point::new([0, 10, 25]);
        let b = Point::new([5, 7, -9]);
        let c = a - b;
        let d = a - 2;

        assert_eq!(c, Point::new([-5, 3, 34]));
        assert_eq!(d, Point::new([-2, 8, 23]));

        let mut e = a;

        e -= a;
        assert_eq!(e, Point::new([0, 0, 0]));

        e -= 2;
        assert_eq!(e, Point::new([-2, -2, -2]));
    }

    #[test]
    fn mul() {
        let a = Point::new([0, 10, 25]);
        let b = Point::new([5, 7, -9]);
        let c = a * b;
        let d = a * 2;

        assert_eq!(c, Point::new([0, 70, -225]));
        assert_eq!(d, Point::new([0, 20, 50]));

        let mut e = a;

        e *= a;
        assert_eq!(e, Point::new([0, 100, 625]));

        e *= 2;
        assert_eq!(e, Point::new([0, 200, 1250]));
    }

    #[test]
    fn div() {
        let a = Point::new([0, 10, 25]);
        let b = Point::new([5, 7, -9]);
        let c = a / b;
        let d = a / 2;

        assert_eq!(c, Point::new([0, 1, -2]));
        assert_eq!(d, Point::new([0, 5, 12]));

        let mut e = a;

        e /= Point::new([3, 3, 3]);
        assert_eq!(e, Point::new([0, 3, 8]));

        e /= 2;
        assert_eq!(e, Point::new([0, 1, 4]));
    }
}
