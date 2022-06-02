use ark_bls12_377 as bls377;
use ark_ff::fields::Field;
use ark_ff::PrimeField;
use ark_serialize::CanonicalDeserialize;
use ark_serialize::CanonicalSerialize;
use ark_serialize::Write;
use ark_std::rand::Rng;
use ark_std::Zero;
use duration_string::DurationString;

use std::cmp;
use rand::RngCore;
use std::fs::File;
use std::time::Duration;
use std::time::Instant;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum HarnessError {
    #[error("could not serialize")]
    SerializationError(#[from] ark_serialize::SerializationError),

    #[error("coudl not open file")]
    FileOpenError(#[from] std::io::Error),
}

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
    append: bool,
) -> Result<(), HarnessError> {
    let points_path = format!("{}{}", dir, "/points");
    let scalars_path = format!("{}{}", dir, "/scalars");
    let (f1, f2) = if append {
        let file1 = File::options().append(true).create(true).open(points_path)?;
        let file2 = File::options().append(true).create(true).open(scalars_path)?;
        (file1, file2)                                                                              
    } else {
       let file1 = File::create(points_path)?;
       let file2 = File::create(scalars_path)?;
       (file1, file2)
    };
    points.serialize(&f1)?;
    scalars.serialize(&f2)?;
    Ok(())
}


type Point = bls377::G1Affine;
type Scalar = <bls377::Fr as PrimeField>::BigInt;

pub fn deserialize_input(
    dir: &str,
) -> Result<(
    Vec<Vec<Point>>,
    Vec<Vec<Scalar>>,
), HarnessError> { 
    let mut points_result = Vec::new();
    let mut scalars_result = Vec::new();
    let points_path = format!("{}{}", dir, "/points");
    let scalars_path = format!("{}{}", dir, "/scalars");
    let f1 = File::open(points_path)?;
    let f2 = File::open(scalars_path)?;

    loop {
        let points = Vec::<bls377::G1Affine>::deserialize(&f1);
        let scalars = Vec::<<bls377::Fr as PrimeField>::BigInt>::deserialize(&f2);

        let points = match points {
            Ok(x) => x,
            _ =>  { break; },
        };

        let scalars = match scalars {
            Ok(x) => x,
            _ => { break; },
        };

        points_result.push(points);
        scalars_result.push(scalars);
    }

    Ok((points_result, scalars_result))
}

pub fn benchmark_msm(
    output_dir: &str,
    points_vec: &Vec<Vec<Point>>, 
    scalars_vec: &Vec<Vec<Scalar>>,
    iterations: u32,
) -> Result<Vec<String>, HarnessError> {
    let output_path = format!("{}{}", output_dir, "/resulttimes.txt");
    let output_result_path = format!("{}{}", output_dir, "/result.txt");
    let mut output_file = File::create(output_path).expect("output file creation failed");
    let output_result_file = File::create(output_result_path).expect("output file creation failed");
    let num_vecs = cmp::min(points_vec.len(), scalars_vec.len());
    let mut result_vec = Vec::new();

    for idx in 0..num_vecs {
        let points = &points_vec[idx];
        let scalars = &scalars_vec[idx];

        let mut total_duration = Duration::ZERO;
        for i in 0..iterations {
            let start = Instant::now();
            let result = ark_ec::msm::VariableBaseMSM::multi_scalar_mul(&points[..], &scalars[..]);
            let time = start.elapsed();
            writeln!(output_file, "iteration {}: {:?}", i + 1, time)?;
            result.serialize(&output_result_file)?;
            total_duration += time;
        }
        let mean = total_duration / iterations;
        write!(output_file, "Mean across all iterations: {:?}", mean)?;
        println!(
            "Average time to execute MSM with {} points and {} scalars and {} iterations is: {:?}",
            points.len(),
            scalars.len(),
            iterations,
            mean
        );
        let d: String = DurationString::from(mean).into();
        result_vec.push(d);
    }
    Ok(result_vec)
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
    use std::ffi::CStr;

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

        let mut points_vec = Vec::new();
        let mut scalars_vec = Vec::new();
        points_vec.push(points);
        scalars_vec.push(scalars);

        let mean_time = benchmark_msm(&rust_dir, &points_vec, &scalars_vec, iters_val).unwrap();

        let output = env.new_string(&mean_time[0]).unwrap();

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_zprize_RustMSM_benchmarkMSMFile(
        env: JNIEnv,
        _: JClass,
        java_dir: JString,
        java_iters: JString,
    ) -> jstring {
        let dir = env.get_string(java_dir).expect("invalid string").as_ptr();
        let rust_dir = CStr::from_ptr(dir).to_str().expect("string invalid");

        let iters = env.get_string(java_iters).expect("invalid string").as_ptr();
        let rust_iters = CStr::from_ptr(iters).to_str().expect("string invalid");
        let iters_val: u32 = rust_iters.parse().unwrap();

        let (points, scalars) = deserialize_input(&rust_dir).unwrap();
        let mean_time = benchmark_msm(&rust_dir, &points, &scalars, iters_val).unwrap();

        let mut output_string = "".to_owned();
        for s in &mean_time {
            output_string.push_str(&s);
            output_string.push_str(", ");
        }
        let mut output_chars = output_string.chars();
        output_chars.next_back();
        output_chars.next_back();

        let output = env.new_string(&output_chars.as_str()).unwrap();

        output.into_inner()
    }
}
