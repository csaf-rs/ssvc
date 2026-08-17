#[path = "build/embed_gen.rs"]
mod embed_gen;
#[path = "build/typegen.rs"]
mod typegen;
#[path = "build/utils.rs"]
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build/typegen.rs");
    println!("cargo:rerun-if-changed=build/utils.rs");
    println!("cargo:rerun-if-changed=build/embed_gen.rs");
    println!("cargo:rerun-if-changed=assets/ssvc_decision_points");

    // Generate embedded asset files
    embed_gen::generate_embedded_files()?;

    // Generate types from JSON schemas
    typegen::build_all_schemas()?;

    Ok(())
}
