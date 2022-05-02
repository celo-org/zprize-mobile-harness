use ark_ec::short_weierstrass_jacobian::GroupProjective;
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
    random_multi_scalar_mul(100, 5, &mut rng);
}

pub fn random_multi_scalar_mul<R: RngCore>(
    n: usize,
    iterations: usize,
    rng: &mut R,
) -> () {
    let num_bytes = bls377::Fr::zero().serialized_size();
    let mut points = Vec::<G1Affine>::new();
    let mut scalars = Vec::<<bls377::Fr as PrimeField>::BigInt>::new();
    for i in 0..n {
        let mut bytes = vec![0; num_bytes];
        let mut scalar;
        loop {
            rng.fill_bytes(&mut bytes[..]);
            scalar = bls377::Fr::from_random_bytes(&bytes);
            if scalar.is_some() {
                break;
            }
        }
        scalars.push(scalar.unwrap().into_repr());

        let mut point: G1Projective = rng.gen();
        points.push(point.into());
    }
    let start = Instant::now();
    let result = ark_ec::msm::VariableBaseMSM::multi_scalar_mul(&points[..], &scalars[..]);
    let duration = start.elapsed();

    println!("Time to execute MSM is: {:?}", duration);
}
