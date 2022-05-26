# celo-zprize

Example testing harness for the "[Accelerating Mobile Proving](https://www.zprize.io/prizes/accelerating-mobile-proving)" challenge in ZPrize 2022. 

To build, download [Android Studio](https://developer.android.com/studio) and open `android/Zprize/app`. Then use Android Studio's device manager to select either an
emulator or physical device, and install the app on that device using the "Run app" button.

To rebuild the Rust API, download the [cross](https://github.com/cross-rs/cross) crate. Then run `cross build --target armv7-linux-androideabi --release`,
then copy `target/armv7-linux-androideabi/release/libcelo_zprize.so` to `android/ZPrize/app/src/main/jniLibs/armeabi-v7a/libmsm.so`. Then reinstall the app as normal.

To modify the fixed test vectors, add the new test vector files to  `android/ZPrize/app/src/main/assets/`.

To run `src/main.rs`, run `cargo run --release`. 
