extern crate rand;

use perlin_noise::PerlinNoise1D;

fn main() {
    const SEED: u64 = 0;
    const ITERATIONS: u32 = 1;
    const GRADIENT_DISTANCE: f64 = 10.0;
    const AMPLITUDE: f64 = 1.0;
    const NUM_GRADIENTS: u32 = 6;

    let test_perlin_noise_1d = PerlinNoise1D::new(SEED, ITERATIONS, GRADIENT_DISTANCE, AMPLITUDE, NUM_GRADIENTS);

    for x in 0..50 {
        print!("{} ", test_perlin_noise_1d.perlin_noise_1d(x as f64));
    }
}

mod perlin_noise {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    pub struct PerlinNoise1D {
        base_seed: u64,
        iterations: u32,
        base_gradient_distance: f64,
        max_amplitude: f64,
        periodic: bool,
        num_gradients: u32,
        output_size: f64
    }

    impl PerlinNoise1D {
        pub fn new(seed: u64, iterations: u32, gradient_distance: f64, max_amplitude: f64, num_gradients: u32) -> PerlinNoise1D {
            PerlinNoise1D {
                base_seed: seed,
                iterations,
                base_gradient_distance: gradient_distance,
                max_amplitude,
                periodic: num_gradients != 0,
                num_gradients,
                output_size: gradient_distance * num_gradients as f64
            }
        }

        pub fn perlin_noise_1d(&self, x: f64) -> f64 {
            let fade = |t: f64| -> f64 { t.powi(3) * (6.0 * t*t - 15.0 * t + 10.0) };
            let gradient_distance = |iter: u32| -> f64 { self.base_gradient_distance / (2 as f64).powi(iter as i32) };
            0.0
        }

        fn gradient_indices(&self, x: f64, gradient_distance: f64) -> (i32, i32) {
            match self.periodic {
                true => {
                    let index0 = ((x % self.output_size) / gradient_distance) as i32;
                    (index0, (index0 + 1) % self.num_gradients as i32)
                },
                false => {
                    let index0 = (x / gradient_distance) as i32;
                    (index0, index0 + 1)
                }
            }
        }
    }
}