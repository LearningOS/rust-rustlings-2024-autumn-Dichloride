//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // For tests7, set up an environment variable `TEST_FOO`
    // and assign it the current UNIX timestamp.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Use println! to instruct Cargo to set this environment variable.
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // For tests8, enable the "pass" feature so that the test returns early.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

