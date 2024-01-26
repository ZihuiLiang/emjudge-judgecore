fn main() {
    pkg_config::Config::new().probe("libcgroup").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
