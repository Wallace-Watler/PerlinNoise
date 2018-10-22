extern crate rand;

use rand::prng;
use rand::rngs;

fn main() {
    const PROGRAM_SEED: [u8; 32] = [0; 32];
    const ITERATIONS: u32 = 1;
    const GRADIENT_DISTANCE: u32 = 10;
    const AMPLITUDE: f64 = 1.0;
    const PERIODIC: bool = false;
    const NUM_GRADIENTS: u32 = 6;

    struct PerlinNoise {
        seed: [u8; 32],
        iterations: u32,
        gradient_distance: f64,
        amplitude: f64,
        periodic: bool,
        num_gradients: u32
    }

    impl PerlinNoise {
        fn new(mut self, seed: [u8; 32], iterations: u32, gradient_distance: f64, amplitude: f64, num_gradients: u32) -> PerlinNoise {
            
        }
    }

    for x in 0..50 {
        println!("{} ", perlin_noise_1d(x as f64, ITERATIONS, GRADIENT_DISTANCE, AMPLITUDE, PERIODIC, NUM_GRADIENTS));
    }
}

fn perlin_noise_1d(x: f64, iterations: u32, gradient_distance: u32, amplitude: f64, periodic: bool, num_gradients: u32) -> f64 {

}