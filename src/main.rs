extern crate rand;

use perlin_noise::PerlinNoise1D;

fn main() {
    const SEED: u32 = 0;
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
    use rand::Rng;

    pub struct PerlinNoise1D {
        base_seed: u32,
        iterations: u32,
        base_gradient_distance: f64,
        max_amplitude: f64,
        periodic: bool,
        num_gradients: u32,
        output_size: f64
    }

    impl PerlinNoise1D {
        pub fn new(seed: u32, iterations: u32, gradient_distance: f64, max_amplitude: f64, num_gradients: u32) -> PerlinNoise1D {
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

        pub fn perlin_noise_1d(&self, domain: Vec<f64>) -> Vec<f64> {


            let mut range: Vec<f64> = vec!();
            for x in domain {
                let mut sum = 0.0;
                for iter in 0..self.iterations {
                    let gradient_distance = self.base_gradient_distance / (1.0 << iter);
                    let frac = self.fade(x % gradient_distance);
                    let (gradient_index0, gradient_index1) = self.gradient_indices(x, gradient_distance);
                    let gradient0 = self.random_gradient(gradient_index0, iter);
                    let gradient1 = self.random_gradient(gradient_index1, iter);

                    sum += gradient0 as f64 * frac + gradient1 * (1.0 - frac);
                }
                range.push(sum);
            }
            range;
        }

        fn fade(&self, t: f64) -> f64 {
            t.powi(3) * (6.0 * t*t - 15.0 * t + 10.0)
        }

        fn random_gradient(&self, gradient_index: i32, iter: u32) -> i8 {
            match StdRng::from_seed((gradient_index ^ iter) * self.base_seed).gen() {
                true => 1,
                false => -1
            }
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