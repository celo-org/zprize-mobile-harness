use ark_bls12_377 as bls377;




use ark_ff::fields::Field;
use ark_ff::PrimeField;
use ark_serialize::CanonicalDeserialize;
use ark_serialize::CanonicalSerialize;
use ark_serialize::Write;
use ark_std::rand::{
    Rng,
};
use ark_std::Zero;
use duration_string::DurationString;

use rand::RngCore;
use std::fs::File;
use std::time::Duration;
use std::time::Instant;

pub fn gen_random_vectors<R: RngCore>(
    n: usize,
    rng: &mut R,
) -> (
    Vec<bls377::G1Affine>,
    Vec<<bls377::Fr as PrimeField>::BigInt>,
) {
    let num_bytes = bls377::Fr::zero().serialized_size();
    let mut points = Vec::<bls377::G1Affine>::new();
    let mut scalars = Vec::<<bls377::Fr as PrimeField>::BigInt>::new();
    let mut bytes = vec![0; num_bytes];
    let mut scalar;
    for _i in 0..n {
        loop {
            rng.fill_bytes(&mut bytes[..]);
            scalar = bls377::Fr::from_random_bytes(&bytes);
            if scalar.is_some() {
                break;
            }
        }
        scalars.push(scalar.unwrap().into_repr());

        let point: bls377::G1Projective = rng.gen();
        points.push(point.into());
    }
    (points, scalars)
}

pub fn serialize_input(
    dir: &str,
    points: &[bls377::G1Affine],
    scalars: &[<bls377::Fr as PrimeField>::BigInt],
) {
    let points_path = format!("{}{}", dir, "/points");
    let scalars_path = format!("{}{}", dir, "/scalars");
    let f1 = File::create(points_path).unwrap();
    let f2 = File::create(scalars_path).unwrap();
    points.serialize(&f1);
    scalars.serialize(&f2);
}

pub fn deserialize_input(
    dir: &str,
) -> (
    Vec<bls377::G1Affine>,
    Vec<<bls377::Fr as PrimeField>::BigInt>,
) {
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
) -> String {
    let output_path = format!("{}{}", output_dir, "/results.txt");
    let mut output_file = File::create(output_path).expect("output file creation failed");
    let mut total_duration = Duration::ZERO;
    for i in 0..iterations {
        let start = Instant::now();
        let _result = ark_ec::msm::VariableBaseMSM::multi_scalar_mul(points, scalars);
        let time = start.elapsed();
        writeln!(output_file, "iteration {}: {:?}", i + 1, time);
        total_duration += time;
    }
    let mean = total_duration / iterations;
    write!(output_file, "Mean across all iterations: {:?}", mean);
    println!(
        "Average time to execute MSM with {} points and {} scalars and {} iterations is: {:?}",
        points.len(),
        scalars.len(),
        iterations,
        mean
    );
    let d: String = DurationString::from(mean).into();
    d
}

/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;
    use rand::thread_rng;
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_zprize_RustMSM_benchmarkMSMRandom(
        env: JNIEnv,
        _: JClass,
        java_dir: JString,
        java_iters: JString,
        java_num_elems: JString,
    ) -> jstring {
        let mut rng = thread_rng();
        let base: i32 = 2;

        let num_elems = env
            .get_string(java_num_elems)
            .expect("invalid string")
            .as_ptr();
        let rust_num_elems = CStr::from_ptr(num_elems).to_str().expect("string invalid");
        let num_elems_val: u32 = rust_num_elems.parse().unwrap();
        let num_elems_exp = base.pow(num_elems_val);

        let (points, scalars) = gen_random_vectors(num_elems_exp.try_into().unwrap(), &mut rng);
        let dir = env.get_string(java_dir).expect("invalid string").as_ptr();
        let rust_dir = CStr::from_ptr(dir).to_str().expect("string invalid");

        let iters = env.get_string(java_iters).expect("invalid string").as_ptr();
        let rust_iters = CStr::from_ptr(iters).to_str().expect("string invalid");
        let iters_val: u32 = rust_iters.parse().unwrap();

        let mean_time = benchmark_msm(&rust_dir, &points[..], &scalars[..], iters_val);

        let output = env.new_string(mean_time).unwrap();

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_zprize_RustMSM_benchmarkMSMFile(
        env: JNIEnv,
        _: JClass,
        java_dir: JString,
        java_iters: JString,
    ) -> jstring {
        let mut rng = thread_rng();
        let base: i32 = 2;

        let dir = env.get_string(java_dir).expect("invalid string").as_ptr();
        let rust_dir = CStr::from_ptr(dir).to_str().expect("string invalid");

        let iters = env.get_string(java_iters).expect("invalid string").as_ptr();
        let rust_iters = CStr::from_ptr(iters).to_str().expect("string invalid");
        let iters_val: u32 = rust_iters.parse().unwrap();

        let (points, scalars) = deserialize_input(&rust_dir);
        let mean_time = benchmark_msm(&rust_dir, &points[..], &scalars[..], iters_val);

        let output = env.new_string(mean_time).unwrap();

        output.into_inner()
    }
}
