use crate::geometry::RealPoint;

pub struct VertexNeighborhood<const DIM: usize> {
    origin: RealPoint<DIM>,
    current_index: usize,
}

impl<const DIM: usize> VertexNeighborhood<DIM> {
    const NUM_VERTICES: usize = 2usize.pow(DIM as u32);

    pub fn new(point: RealPoint<DIM>) -> Self {
        Self { origin: point.floor(), current_index: 0 }
    }
}

impl<const DIM: usize> Iterator for VertexNeighborhood<DIM> {
    type Item = RealPoint<DIM>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= Self::NUM_VERTICES {
            None
        } else {
            let mut coordinates = [0.0f64; DIM];

            for dim in 0..DIM {
                if (self.current_index & (1 << dim)) > 0 {
                    coordinates[dim] = 1.0f64;
                };
            };

            self.current_index += 1;

            Some(self.origin + RealPoint::new(coordinates))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::RealPoint;

    #[test]
    fn vertex_neighborhood_2d() {
        let point = RealPoint::new([1.2, -5.9]);
        let vertices: Vec<RealPoint<2>> = point.vertex_neighborhood().collect();

        assert_eq!(vertices, vec![
            RealPoint::new([1.0, -6.0]),
            RealPoint::new([2.0, -6.0]),
            RealPoint::new([1.0, -5.0]),
            RealPoint::new([2.0, -5.0]),
        ]);
    }

    #[test]
    fn vertex_neighborhood_3d() {
        let point = RealPoint::new([1.2, -5.9, 8.5]);
        let vertices: Vec<RealPoint<3>> = point.vertex_neighborhood().collect();

        assert_eq!(vertices, vec![
            RealPoint::new([1.0, -6.0, 8.0]),
            RealPoint::new([2.0, -6.0, 8.0]),
            RealPoint::new([1.0, -5.0, 8.0]),
            RealPoint::new([2.0, -5.0, 8.0]),
            RealPoint::new([1.0, -6.0, 9.0]),
            RealPoint::new([2.0, -6.0, 9.0]),
            RealPoint::new([1.0, -5.0, 9.0]),
            RealPoint::new([2.0, -5.0, 9.0]),
        ]);
    }
}
