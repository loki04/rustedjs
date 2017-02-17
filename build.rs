extern crate cmake;

fn main() {
    use cmake::Config;

    let dst = Config::new("jerryscript")
                     .define("ENABLE_STATIC_LINK","OFF")
                     .define("ENABLE_LTO","OFF")
                     .define("ENABLE_ALL_IN_ONE","ON")
                     .define("ENABLE_STRIP", "OFF")
                     .build();

    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
    println!("cargo:rustc-link-lib=static=jerry-core");
}
