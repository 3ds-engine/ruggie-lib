fn main() {
    // link libcitro2d and libcitro3d
    println!("cargo:rustc-link-lib=citro3d");
    println!("cargo:rustc-link-lib=citro2d");
}
