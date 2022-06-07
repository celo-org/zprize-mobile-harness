# ZPrize MSM on Mobile Reference Test Harness

Example and reference testing harness for the "[Accelerating MSM on Mobile](https://www.zprize.io/prizes/accelerating-mobile-proving)" challenge in ZPrize 2022. 

## Build

### Android app

Building the Android app can be accomplished with Android Studio:
1. Download [Android Studio](https://developer.android.com/studio) and install the Android API 32 platform, NDK, and Android SDK Platform tools through the SDK Manager.
2. Open `android/Zprize` in Android Studio.
3. Use Android Studio's device manager to select either an emulator or physical device.
4. Install the app on that device using the Run > Run 'app' menu action.

<<<<<<< HEAD
To run `src/main.rs`, run `cargo run --release`. To generate new test vector files, 
=======
### Rust library
>>>>>>> ff4d5c8ed58ebcc986a099c2370f09638437ed07

Prebuilt libraries are available in this repository as `android/ZPrize/app/src/main/jniLibs/armeabi-v7a`.
When you modify the Rust libraries, build the modified library for inclusion in the Android app:
1. Download the [cross](https://github.com/cross-rs/cross) crate. 
2. Run `cross build --target armv7-linux-androideabi --release`
3. Copy `target/armv7-linux-androideabi/release/libcelo_zprize.so` to `android/ZPrize/app/src/main/jniLibs/armeabi-v7a/libmsm.so`.
4. Rebuild and install the app as described above.

## Test vectors

A number of static test vectors to check the basic correctness of the MSM implementation are
included, as described in the prize specification.

To modify the fixed test vectors, add the new test vector files to  `android/ZPrize/app/src/main/assets/points` and `android/ZPrize/app/src/main/assets/scalars`, then recompile the app. To generate new test vector files, first generate the vectors, then run `serialize_input` once per vector with the same directory specified, with the `append` option set to `true`. This process is demonstrated in `src/main.rs` for one vector. 

## Benchmarking on host

You can run a benchmark locally (on your computer) with run `cargo run --release`. The local
benchmark is defined in `src/main.rs`.

## Benchmarking on device

On the Android device, output files were stored to `data/data/com.example.zprize/files/`.
The output group elements of the MSM, one per iteration, are stored in `result.txt`.
The time per iteration is recorded in `resulttimes.txt`.
When running test vectors, the files will contain the results for each vector of inputs in sequence.
