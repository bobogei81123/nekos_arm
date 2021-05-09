fn main() {
    println!("cargo:rerun-if-changed=src/arch/aarch64/aarch64.ld");
    println!("cargo:rerun-if-changed=build.rs");
}
