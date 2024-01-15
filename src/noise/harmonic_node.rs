use crate::{geometry::RealPoint, noise::NoiseNode};

pub struct HarmonicNode<'a, const DIM: usize, T> where T: NoiseNode<DIM> {
    source: &'a T,
    num_octaves: usize,
    persistence: f64,
    lacunarity: f64,
}

impl<'a, const DIM: usize, T> HarmonicNode<'a, DIM, T> where T: NoiseNode<DIM> {
    pub fn new(source: &'a T, num_octaves: usize, persistence: f64, lacunarity: f64) -> Self {
        Self { source, num_octaves, persistence, lacunarity }
    }
}

impl<'a, const DIM: usize, T> NoiseNode<DIM> for HarmonicNode<'a, DIM, T>
where T: NoiseNode<DIM> {
    fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        let mut value: f64 = 0.0;

        let mut max_value: f64 = 0.0;
        let mut frequency: f64 = 1.0;
        let mut amplitude: f64 = 1.0;

        for _ in 0..self.num_octaves {
            value += self.source.value_at(point * frequency) * amplitude;

            max_value += amplitude;
            amplitude *= self.persistence;
            frequency *= self.lacunarity;
        };

        value / max_value
    }
}
