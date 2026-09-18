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

    // On docs.rs builds the filesystem is read-only. The generated files are
    // already committed, so we can skip code generation entirely.
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    // Generate types from JSON schemas
    typegen::build_all_schemas()?;

    // Validate all decision point JSON files
    validation::validate_decision_points()?;

    Ok(())
}
