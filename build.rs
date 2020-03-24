use cbindgen;
use std::env;

fn main() -> Result<(), cbindgen::Error> {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)?
      .write_to_file("bindings.h");

    Ok(())
}
