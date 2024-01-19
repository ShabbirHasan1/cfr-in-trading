use std::env;

fn main() {
    let root_dir = env!("CARGO_MANIFEST_DIR");
    println!("cargo:rustc-link-lib=dylib=pyemb");
    println!("cargo:rustc-link-search=native={}/py", root_dir);
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}/py", root_dir);
}
