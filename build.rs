fn main() {
    println!("cargo:rerun-if-changes=lds/virt.lds");
    println!("cargo:rerun-if-changes=build.rs");
}
