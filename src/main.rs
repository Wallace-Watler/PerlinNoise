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
    use rand::rngs;
    use rand::SeedableRng;

    pub struct PerlinNoise1D {
        base_seed: u64,
        iterations: u32,
        gradient_distance: f64,
        amplitude: f64,
        periodic: bool,
        num_gradients: u32,
        output_size: f64
    }

    impl PerlinNoise1D {
        pub fn new(seed: u64, iterations: u32, gradient_distance: f64, amplitude: f64, num_gradients: u32) -> PerlinNoise1D {
            PerlinNoise1D {
                base_seed: seed,
                iterations,
                gradient_distance,
                amplitude,
                periodic: num_gradients != 0,
                num_gradients,
                output_size: gradient_distance * num_gradients as f64
            }
        }

        pub fn perlin_noise_1d(&self, x: f64) -> f64 {
            let (gradient_index0, gradient_index1) = match self.periodic {
                true => {
                    let index0 = ((x % self.output_size) / self.gradient_distance) as i32;
                    (index0, (index0 + 1) % self.num_gradients as i32)
                },
                false => {
                    let index0 = (x / self.gradient_distance) as i32;
                    (index0, index0 + 1)
                }
            };

            let base_rng = rngs::StdRng::seed_from_u64(self.base_seed);

            let mut sum = 0.0;
            for iter in 2..(self.iterations + 2) {

            }
            sum
        }
    }
}