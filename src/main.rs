use ark_ec::short_weierstrass_jacobian::GroupProjective;
use std::time::Duration;
use ark_ff::PrimeField;
use std::time::Instant;
use ark_ec::msm;
use ark_ec::models::SWModelParameters as Parameters;
use ark_bls12_377 as bls377;
use ark_bls12_377::{G1Affine, G1Projective};
use ark_ff::fields::Field;
use ark_std::Zero;
use ark_serialize::CanonicalSerialize;
use rand::RngCore;
use rand::thread_rng;
//use rand::prelude::Distribution;
use ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

fn main() {
    let mut rng = thread_rng();
    println!("Generating elements");
    let (points, scalars) = gen_random_vectors(4096, &mut rng);
    println!("Generated elements");
    benchmark_msm(&points[..], &scalars[..], 1);
}

pub fn gen_random_vectors<R: RngCore>(
    n: usize,
    rng: &mut R,
) -> (Vec<bls377::G1Affine>, Vec<<bls377::Fr as PrimeField>::BigInt>) {
    let num_bytes = bls377::Fr::zero().serialized_size();
    let mut points = Vec::<bls377::G1Affine>::new();
    let mut scalars = Vec::<<bls377::Fr as PrimeField>::BigInt>::new();
    let mut bytes = vec![0; num_bytes];
    let mut scalar;
    for i in 0..n {
        loop {
            rng.fill_bytes(&mut bytes[..]);
            scalar = bls377::Fr::from_random_bytes(&bytes);
            if scalar.is_some() {
                break;
            }
        }
        scalars.push(scalar.unwrap().into_repr());

        let mut point: bls377::G1Projective = rng.gen();
        points.push(point.into());
    }
    (points, scalars)
}

pub fn benchmark_msm(
    points: &[bls377::G1Affine],
    scalars: &[<bls377::Fr as PrimeField>::BigInt],
    iterations: u32,
) -> () {
    let mut duration = Duration::ZERO;
    for i in 0..iterations {
        let start = Instant::now();
        let result = ark_ec::msm::VariableBaseMSM::multi_scalar_mul(&points[..], &scalars[..]);
        duration += start.elapsed();
    }
    println!("Average time to execute MSM is: {:?}", duration / iterations);
}
