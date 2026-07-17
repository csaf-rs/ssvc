#[path = "build/typegen.rs"]
mod typegen;
#[path = "build/utils.rs"]
mod utils;
#[path = "build/validation.rs"]
mod validation;

use anyhow::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build/typegen.rs");
    println!("cargo:rerun-if-changed=build/utils.rs");
    println!("cargo:rerun-if-changed=build/validation.rs");

    // Generate types from JSON schemas
    typegen::build_all_schemas()?;

    // Validate all decision point JSON files
    validation::validate_decision_points()?;

    Ok(())
}
