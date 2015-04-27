extern crate compiletest;

use std::path::PathBuf;

#[test]
fn compile_test() {
    let mut config = compiletest::default_config();
    config.target_rustcflags = Some("-L target/debug/".to_string());
    config.mode = "compile-fail".parse().ok().expect("Invalid mode");
    config.src_base = PathBuf::from("tests/compile-fail");

    compiletest::run_tests(&config);
}
