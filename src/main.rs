use rand::thread_rng;
use celo_zprize::{serialize_input, deserialize_input, gen_random_vectors, benchmark_msm};

fn main() {
    let mut rng = thread_rng();
    println!("Generating elements");
    let dir = ".";
    let base: i32 = 2;
    let n_elems = base.pow(16);
    let (points, scalars) = gen_random_vectors(n_elems.try_into().unwrap(), &mut rng);
    serialize_input(".", &points, &scalars);
    let (points, scalars) = deserialize_input(&dir); 
    println!("Generated elements");
    let result = benchmark_msm(&dir, &points[..], &scalars[..], 10);
    println!("result is: {:?}", result);
}
