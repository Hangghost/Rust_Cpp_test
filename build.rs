fn main() -> miette::Result<()> {
    // It's necessary to use an absolute path here because the
    // C++ codegen and the macro codegen appears to be run from different
    // working directories.
    let path = std::path::PathBuf::from("FiltFilt/src");
    let path2 = std::path::PathBuf::from("src");
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path, &path2]).build()?;
    b.flag_if_supported("-std=c++14")
        .compile("demo_auto");
    println!("cargo:rerun-if-changed=src/main.rs");

    // Add instructions to link to any C++ libraries you need.
    Ok(())
}