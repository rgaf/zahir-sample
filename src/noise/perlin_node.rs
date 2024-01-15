use crate::{
    geometry::RealPoint,
    noise::{gen_gradients, perlin_1d, perlin_2d, perlin_3d, NoiseNode},
    random::{HashFn, Seed},
    utils,
};

pub struct PerlinNode<const DIM: usize, T> where T: HashFn {
    hash_fn: T,
    gradients: Vec<RealPoint<DIM>>,
}

impl<const DIM: usize, T> PerlinNode<DIM, T> where T: HashFn {
    const NUM_GRADIENTS: usize = 2usize.pow((DIM as u32) + 3);

    pub fn new(seed: &Seed) -> Self {
        let hash_fn = T::from_seed(seed);

        let gradients = if DIM < 4 {
            Vec::with_capacity(0)
        } else {
            gen_gradients(seed, Self::NUM_GRADIENTS)
        };

        Self { hash_fn, gradients }
    }

    // N-dimensional Perlin noise generates values in the range [-X, X], where X = sqrt(N) / 2
    pub fn unbias(x: f64) -> f64 {
        (x * 2.0f64) / (DIM as f64).sqrt()
    }

    pub fn noise_value_for(&self, point: RealPoint<DIM>, vertex: RealPoint<DIM>) -> f64 {
        let hash = self.hash_fn.hash_bytes(vertex.as_bytes()) as usize;
        let gradient = self.gradients[hash % Self::NUM_GRADIENTS];
        let inner_point = vertex - point;

        inner_point.dot_prod(gradient)
    }
}

impl<T> NoiseNode<1> for PerlinNode<1, T> where T: HashFn {
    fn value_at(&self, point: RealPoint<1>) -> f64 {
        perlin_1d(&self.hash_fn, point)
    }
}

impl<T> NoiseNode<2> for PerlinNode<2, T> where T: HashFn {
    fn value_at(&self, point: RealPoint<2>) -> f64 {
        perlin_2d(&self.hash_fn, point)
    }
}

impl<T> NoiseNode<3> for PerlinNode<3, T> where T: HashFn {
    fn value_at(&self, point: RealPoint<3>) -> f64 {
        perlin_3d(&self.hash_fn, point)
    }
}

impl<const DIM: usize, T> NoiseNode<DIM> for PerlinNode<DIM, T> where T: HashFn {
    default fn value_at(&self, point: RealPoint<DIM>) -> f64 {
        use itertools::Itertools;

        let mut noise_values: Vec<f64> = point.vertex_neighborhood().map(|vertex| {
            self.noise_value_for(point, vertex)
        }).collect();

        let smoothed_coordinates: Vec<f64> = (point - point.floor()).iter().map(|&coordinate| {
            utils::smoothstep(coordinate)
        }).collect();

        for dim in 0..DIM {
            let bias = smoothed_coordinates[dim];
            let new_noise_values: Vec<f64> = noise_values.iter().tuples().map(|(&lhs, &rhs)| {
                utils::lerp(bias, lhs, rhs)
            }).collect();

            noise_values.clear();
            noise_values.extend(&new_noise_values)
        };

        let noise_value = Self::unbias(noise_values[0]);

        utils::smoothstep(utils::neg_unit_to_unit(noise_value))
    }
}
