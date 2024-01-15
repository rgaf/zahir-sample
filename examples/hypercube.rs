use std::path::Path;
use minifb::{Key, Window, WindowOptions};

use zahir::geometry::*;
use zahir::noise::*;
use zahir::random::*;
use zahir::utils;

fn main() {
    let width: usize = 800;
    let height: usize = 800;

    let seed = Seed::from_entropy();
    let perlin: PerlinNode<2, Wyhash> = PerlinNode::new(&seed);
    let harmonic = HarmonicNode::new(&perlin, 8, 0.8, 2.0);
    let sigmoid = SigmoidNode::new(&harmonic, -2.0);
    let hypersphere = HypersphereNode::<2, EuclideanMetric>::new(0.2);
    let noise_gen = SoftLightNode::new(&sigmoid, &hypersphere);

    let mut pixel_data: Vec<u32> = vec![0; width * height];
    let mut buffer: Vec<u8> = vec![0; width * height];

    let a = RealPoint::<2>::new([-1.7, -0.25]);

    let mut minv = 100.0;
    let mut maxv = 0.0;

    for y in 0..height {
        for x in 0..width {
            let idx = y * height + x;

            let x = (x as f64) / 80.0;
            let y = (y as f64) / 80.0;

            let value = noise_gen.value_at(RealPoint::<2>::new([x, y]));

            if value > maxv {
                maxv = value;
            };

            if value < minv {
                minv = value;
            };

            let byte = (value * 255.0) as u8;

            pixel_data[idx] = (byte as u32) | ((byte as u32) << 8) | ((byte as u32) << 16);
        };
    };

    let file_path = "noise-big.png".to_owned();

    let mut window = Window::new(
        "Noise",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| panic!("{}", e));

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&pixel_data, width, height).unwrap();
    }
}
