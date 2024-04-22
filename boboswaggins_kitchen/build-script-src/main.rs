use std::{env, path::Path};

mod rs_types;
mod json_database;

fn main() {
    let mut opts = built::Options::default();
    opts.set_dependencies(true);

    let src = env::var("CARGO_MANIFEST_DIR").unwrap();
    let built_dst = Path::new(&env::var("OUT_DIR").unwrap()).join("built.rs");
    let formula_dst = Path::new(&env::var("OUT_DIR").unwrap()).join("formulas.rs");

    built::write_built_file_with_opts(&opts, src.as_ref(), &built_dst)
        .expect("Failed to acquire build-time information");
}
