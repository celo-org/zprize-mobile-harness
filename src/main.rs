use rand::thread_rng;
use celo_zprize::{gen_random_vectors, benchmark_msm};

fn main() {
    let mut rng = thread_rng();
    println!("Generating elements");
    let (points, scalars) = gen_random_vectors(256, &mut rng);
    println!("Generated elements");
    benchmark_msm(&points[..], &scalars[..], 100);
}
