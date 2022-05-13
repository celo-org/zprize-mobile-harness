use rand::thread_rng;
use celo_zprize::{serialize_input, deserialize_input, gen_random_vectors, benchmark_msm};

fn main() {
    let mut rng = thread_rng();
    println!("Generating elements");
    let dir = ".";
    //let (points, scalars) = deserialize_input(&dir); //gen_random_vectors(8, &mut rng);
    let (points, scalars) = gen_random_vectors(8, &mut rng);
    //serialize_input(".", &points, &scalars);
    println!("Generated elements");
    benchmark_msm(&dir, &points[..], &scalars[..], 10);
}
