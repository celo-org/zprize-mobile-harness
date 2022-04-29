use ark_ec::short_weierstrass_jacobian::GroupProjective;
use ark_ec::msm::VariableBaseMSM::multi_scalar_mul;
use ark_ec::models::SWModelParameters as Parameters;
use ark_bls12_377 as bls377;
use ark_bls12_377::G1Projective;
use ark_ff::fields::Field;
use ark_std::Zero;
use ark_serialize::CanonicalSerialize;
use rand::RngCore;
//use rand::prelude::Distribution;
use ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

fn main() {
    println!("Hello, world!");
}

pub fn random_multi_scalar_mul<R: RngCore>(
    rng: &mut R,
) -> () {
    let n = 5;
    let num_bytes = bls377::Fr::zero().serialized_size();
    let mut points = Vec::<G1Projective>::new();
    let mut scalars = Vec::<bls377::Fr>::new();
    for i in 0..n {
        let mut point: G1Projective = rng.gen();
        points.push(point);

        let mut bytes = vec![0; num_bytes];
        rng.fill_bytes(&mut bytes[..]);
        let scalar = bls377::Fr::from_random_bytes(&bytes).unwrap();
        scalars.push(scalar);
    }
    let result = multi_scalar_mul(&points[..], &scalars[..]);
}
