extern crate rand;

use perlin_noise::PerlinNoise1D;

fn main() {
    const SEED: u64 = 0;
    const ITERATIONS: u32 = 1;
    const GRADIENT_DISTANCE: f64 = 10.0;
    const AMPLITUDE: f64 = 1.0;
    const NUM_GRADIENTS: u32 = 6;

    let test_perlin_noise_1d = PerlinNoise1D::new(SEED, ITERATIONS, GRADIENT_DISTANCE, AMPLITUDE, NUM_GRADIENTS);

    let mut domain: Vec<f64> = vec!();
    for x in 0..50 { domain.push(x as f64); }
    print!("{:?} ", test_perlin_noise_1d.perlin_noise_1d(domain));
}

mod perlin_noise {
    use rand::rngs::StdRng;
    use rand::SeedableRng;
    use rand::Rng;

    pub struct PerlinNoise1D {
        seed: u64,
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
                seed,
                iterations,
                base_gradient_distance: gradient_distance,
                max_amplitude,
                periodic: num_gradients != 0,
                num_gradients,
                output_size: gradient_distance * num_gradients as f64
            }
        }

        pub fn perlin_noise_1d(&self, domain: Vec<f64>) -> Vec<f64> {
            let domain_abs = domain.iter().map(|x| x.abs()).collect();
            let mut rng = StdRng::seed_from_u64(self.seed);

            let mut range_sum: Vec<f64> = self.perlin_noise_1d_iter(&domain_abs, 0, &rng);
            for iter in 1..self.iterations {
                let range = self.perlin_noise_1d_iter(&domain_abs, iter, &rng);
                range_sum = range_sum.iter().zip(range).map(|(x, y)| x + y).collect();
            }
            range_sum
        }

        fn perlin_noise_1d_iter(&self, domain_abs: &Vec<f64>, iter: u32, rng: &mut StdRng) -> Vec<f64> {
            let gradient_distance = self.base_gradient_distance / (1 << iter) as f64;
            let account_for_periodic = |x: f64| -> f64 {
                match self.periodic {
                    true => x % self.output_size,
                    false => x
                }
            };
            let gradient_index_min = (account_for_periodic(domain_abs[0]) / gradient_distance) as i32;
            let gradient_index_max = (account_for_periodic(domain_abs[domain_abs.len() - 1]) / gradient_distance) as i32;
            let gradient_index_diff = gradient_index_max - gradient_index_min;

            let mut gradients = vec!();
            for _i in 0..(gradient_index_diff + 1) {
                gradients.push(2 * rng.gen_bool(0.5) as i8 + 1);
            }

            let mut range: Vec<f64> = vec!();
            for x in domain_abs {
                let frac = self.fade(x % gradient_distance);
                let (gradient_index0, gradient_index1) = self.gradient_indices(x, gradient_distance);

                range.push(gradients[gradient_index0] as f64 * frac + gradients[gradient_index1] as f64 * (1.0 - frac));
            }
            range
        }

        fn fade(&self, t: f64) -> f64 {
            t.powi(3) * (6.0 * t*t - 15.0 * t + 10.0)
        }

        fn gradient_indices(&self, x: &f64, gradient_distance: f64) -> (usize, usize) {
            match self.periodic {
                true => {
                    let index0 = ((x % self.output_size) / gradient_distance) as usize;
                    (index0, (index0 + 1) % self.num_gradients as usize)
                },
                false => {
                    let index0 = (x / gradient_distance) as usize;
                    (index0, index0 + 1)
                }
            }
        }
    }
}