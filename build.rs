fn main() {
    println!("cargo:rerun-if-changed=kernel.ld");
    println!("cargo:rerun-if-changed=build.rs");
}
