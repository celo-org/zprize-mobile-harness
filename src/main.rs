use celo_zprize::{benchmark_msm, deserialize_input, gen_random_vectors, gen_zero_vectors, serialize_input};
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    println!("Generating elements");
    let dir = ".";
    let base: i32 = 2;
    let n_elems = base.pow(8);
    let (points, scalars) = gen_zero_vectors(n_elems.try_into().unwrap(), &mut rng);
    serialize_input(".", &points, &scalars, true).unwrap();
    let (points, scalars) = deserialize_input(dir).unwrap();
    println!("Generated elements");
    let result = benchmark_msm(dir, &points, &scalars, 10);
    println!("result is: {:?}", result);
}
