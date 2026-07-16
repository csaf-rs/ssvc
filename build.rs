#[path = "build/typegen.rs"]
mod typegen;
#[path = "build/utils.rs"]
mod utils;
#[path = "build/validation.rs"]
mod validation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    // Generate types from JSON schemas
    typegen::build_all_schemas()?;

    // Validate all decision point JSON files
    validation::validate_decision_points()?;

    Ok(())
}
