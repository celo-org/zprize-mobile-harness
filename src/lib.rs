use ark_ec::short_weierstrass_jacobian::GroupProjective;
use ark_serialize::Write;
use ark_serialize::CanonicalDeserialize;
use std::fs::File;
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
use ark_std::rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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

pub fn serialize_input(
    dir: &str,
    points: &[bls377::G1Affine],
    scalars: &[<bls377::Fr as PrimeField>::BigInt],
) -> () {
    let points_path = format!("{}{}", dir, "/points");
    let scalars_path = format!("{}{}", dir, "/scalars");
    let f1 = File::create(points_path).unwrap();
    let f2 = File::create(scalars_path).unwrap();
    points.serialize(&f1);
    scalars.serialize(&f2);
}

pub fn deserialize_input(
    dir: &str,
) -> (Vec<bls377::G1Affine>, Vec<<bls377::Fr as PrimeField>::BigInt>) {
    let points_path = format!("{}{}", dir, "/points");
    let scalars_path = format!("{}{}", dir, "/scalars");
    let f1 = File::open(points_path).unwrap();
    let f2 = File::open(scalars_path).unwrap();
    let points = Vec::<bls377::G1Affine>::deserialize(&f1).unwrap();
    let scalars = Vec::<<bls377::Fr as PrimeField>::BigInt>::deserialize(&f2).unwrap();
    (points, scalars)
}
pub fn benchmark_msm(
    output_dir: &str,
    points: &[bls377::G1Affine],
    scalars: &[<bls377::Fr as PrimeField>::BigInt],
    iterations: u32,
) -> () {
    let output_path = format!("{}{}", output_dir, "/results.txt");
    let mut output_file = File::create(output_path).expect("output file creation failed");
    let mut total_duration = Duration::ZERO;
    for i in 0..iterations {
        let start = Instant::now();
        let result = ark_ec::msm::VariableBaseMSM::multi_scalar_mul(&points[..], &scalars[..]);
        let time = start.elapsed();
        write!(output_file, "iteration {}: {:?}\n", i+1, time);
        total_duration += time;
    }
    let mean = total_duration / iterations;
    write!(output_file, "Mean across all iterations: {:?}", mean);
    println!("Average time to execute MSM with {} points and {} scalars and {} iterations is: {:?}", points.len(), scalars.len(), iterations, mean);
}


/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
       extern crate jni;
       use std::os::raw::{c_char};
       use std::ffi::{CString, CStr};
       use super::*;
       use self::jni::JNIEnv;
       use self::jni::objects::{JClass, JString};
       use self::jni::sys::{jstring};
       use rand::thread_rng;

       #[no_mangle]
       //Java_com_example_greetings_RustGreetings_greeting
       pub unsafe extern fn Java_com_example_zprize_RustMSM_benchmarkMSM(env: JNIEnv, _: JClass, java_dir: JString) -> jstring {
        let mut rng = thread_rng();
        let (points, scalars) = gen_random_vectors(8, &mut rng);
        let dir = env.get_string(java_dir).expect("invalid string").as_ptr();
        let rust_dir = CStr::from_ptr(dir).to_str().expect("string invalid");
        serialize_input(&rust_dir, &points, &scalars);
        let (de_points, de_scalars) = deserialize_input(&rust_dir);
        benchmark_msm(&rust_dir, &de_points[..], &de_scalars[..], 1);

        // output to check that code ran
        let output = env.new_string("hello epsilon").unwrap();//env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");

        output.into_inner()
      }
}
