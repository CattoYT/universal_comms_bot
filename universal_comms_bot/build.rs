use std::env;

fn main() {
    // no clue if this works
    let lib_dir = env::current_dir().unwrap().join("libs");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
}
