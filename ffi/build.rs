fn main() {
    println!("cargo:rustc-link-search=native=c");
    println!("cargo:rustc-link-lib=static=math");
}