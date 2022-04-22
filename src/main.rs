use ark_ec::short_weierstrass_jacobian::GroupProjective;
use ark_ec::models::SWModelParameters as Parameters;
use rand::RngCore;
//use rand::prelude::Distribution;
use ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

fn main() {
    println!("Hello, world!");
}

pub fn random_multi_scalar_mul<P: Parameters, R: RngCore>(
    rng: &mut R,
) -> () {
    let n = 5;
    let point: GroupProjective<P> = rng.gen();
    //let standard = Standard::new();
    //let point = D::sample(&standard, rng);
}
