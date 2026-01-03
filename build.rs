use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Project root
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Source file
    let src = manifest_dir.join("tests").join("ui").join("ok.ini");

    // Destination directory
    let dst_dir = manifest_dir
        .join("target")
        .join("tests")
        .join("trybuild")
        .join("rust_macro");

    // Destination file
    let dst = dst_dir.join("ok.ini");

    // Ensure destination directory exists
    fs::create_dir_all(&dst_dir).expect("failed to create target directory");

    // Copy file
    fs::copy(&src, &dst).expect("failed to copy ok.ini");

    // Re-run build script if source file changes
    println!("cargo:rerun-if-changed={}", src.display());
}
