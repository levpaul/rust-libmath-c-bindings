fn main() {
    println!("cargo:rustc-env=LD_LIBRARY_PATH=/opt/lib");
    println!("cargo:rustc-env=LIBRARY_PATH=/opt/lib");
    println!("cargo:rustc-link-lib=math");
}
