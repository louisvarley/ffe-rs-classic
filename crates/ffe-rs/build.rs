fn main() {
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        // The simulation/render loop's debug stack frame exceeds Windows' 1 MiB
        // executable default. Keep the larger reserve on this binary only.
        println!("cargo:rustc-link-arg-bin=ffe-rs=/STACK:8388608");
    }
}
