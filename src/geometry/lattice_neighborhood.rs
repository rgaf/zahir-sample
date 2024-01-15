use crate::geometry::Point;

pub struct LatticeNeighborhood<const DIM: usize> {
    origin: Point<DIM>,
    include_self: bool,
    current_index: usize,
}

impl<const DIM: usize> LatticeNeighborhood<DIM> {
    const NUM_NEIGHBORS: usize = 3usize.pow(DIM as u32);
    const SELF_INDEX: usize = Self::NUM_NEIGHBORS / 2;

    pub fn new(origin: Point<DIM>, include_self: bool) -> Self {
        Self { origin, include_self, current_index: 0 }
    }
}

impl<const DIM: usize> Iterator for LatticeNeighborhood<DIM> {
    type Item = Point<DIM>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= Self::NUM_NEIGHBORS {
            None
        } else {
            if !self.include_self && self.current_index == Self::SELF_INDEX {
                self.current_index += 1;

                return self.next();
            };

            let mut coordinates = [0i32; DIM];

            for dim in 0..DIM {
                let idx = self.current_index as i32;
                let divisor = 3i32.pow(dim as u32);
                let modulus = divisor * 3;

                coordinates[dim] = (idx % modulus) / divisor - 1;
            };

            self.current_index += 1;

            Some(self.origin + Point::new(coordinates))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Point;

    #[test]
    fn lattice_neighborhood_2d() {
        let point = Point::new([5, 2]);
        let neighbors: Vec<Point<2>> = point.neighbors().collect();
        let neighbors_and_self: Vec<Point<2>> = point.neighbors_and_self().collect();

        assert_eq!(neighbors, vec![
            Point::new([4, 1]),
            Point::new([5, 1]),
            Point::new([6, 1]),

            Point::new([4, 2]),
            Point::new([6, 2]),

            Point::new([4, 3]),
            Point::new([5, 3]),
            Point::new([6, 3]),
        ]);

        assert_eq!(neighbors_and_self, vec![
            Point::new([4, 1]),
            Point::new([5, 1]),
            Point::new([6, 1]),

            Point::new([4, 2]),
            Point::new([5, 2]),
            Point::new([6, 2]),

            Point::new([4, 3]),
            Point::new([5, 3]),
            Point::new([6, 3]),
        ]);
    }

    #[test]
    fn lattice_neighborhood_3d() {
        let point = Point::new([5, 2, 8]);
        let neighbors: Vec<Point<3>> = point.neighbors().collect();
        let neighbors_and_self: Vec<Point<3>> = point.neighbors_and_self().collect();

        assert_eq!(neighbors, vec![
            Point::new([4, 1, 7]),
            Point::new([5, 1, 7]),
            Point::new([6, 1, 7]),
            Point::new([4, 2, 7]),
            Point::new([5, 2, 7]),
            Point::new([6, 2, 7]),
            Point::new([4, 3, 7]),
            Point::new([5, 3, 7]),
            Point::new([6, 3, 7]),

            Point::new([4, 1, 8]),
            Point::new([5, 1, 8]),
            Point::new([6, 1, 8]),
            Point::new([4, 2, 8]),
            Point::new([6, 2, 8]),
            Point::new([4, 3, 8]),
            Point::new([5, 3, 8]),
            Point::new([6, 3, 8]),

            Point::new([4, 1, 9]),
            Point::new([5, 1, 9]),
            Point::new([6, 1, 9]),
            Point::new([4, 2, 9]),
            Point::new([5, 2, 9]),
            Point::new([6, 2, 9]),
            Point::new([4, 3, 9]),
            Point::new([5, 3, 9]),
            Point::new([6, 3, 9]),
        ]);

        assert_eq!(neighbors_and_self, vec![
            Point::new([4, 1, 7]),
            Point::new([5, 1, 7]),
            Point::new([6, 1, 7]),
            Point::new([4, 2, 7]),
            Point::new([5, 2, 7]),
            Point::new([6, 2, 7]),
            Point::new([4, 3, 7]),
            Point::new([5, 3, 7]),
            Point::new([6, 3, 7]),

            Point::new([4, 1, 8]),
            Point::new([5, 1, 8]),
            Point::new([6, 1, 8]),
            Point::new([4, 2, 8]),
            Point::new([5, 2, 8]),
            Point::new([6, 2, 8]),
            Point::new([4, 3, 8]),
            Point::new([5, 3, 8]),
            Point::new([6, 3, 8]),

            Point::new([4, 1, 9]),
            Point::new([5, 1, 9]),
            Point::new([6, 1, 9]),
            Point::new([4, 2, 9]),
            Point::new([5, 2, 9]),
            Point::new([6, 2, 9]),
            Point::new([4, 3, 9]),
            Point::new([5, 3, 9]),
            Point::new([6, 3, 9]),
        ]);
    }
}
