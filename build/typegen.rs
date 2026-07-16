use std::fs;
use std::path::Path;
use typify::{TypeSpace, TypeSpaceSettings};

use super::utils;

pub const SCHEMA_TARGETS: &[(&str, &str)] = &[
    (
        "assets/SelectionList_2_0_0.schema.json",
        "src/generated/ssvc/selection_list.rs",
    ),
    (
        "assets/DecisionPoint_2_0_0.schema.json",
        "src/generated/ssvc/decision_point.rs",
    ),
];

/// Generates Rust types from all schemas defined in `SCHEMA_TARGETS`.
///
/// # Errors
/// Returns an error if schema reading, type generation, or file writing fails.
pub fn build_all_schemas() -> Result<(), Box<dyn std::error::Error>> {
    for (file_path, target_path) in SCHEMA_TARGETS {
        println!("cargo:rerun-if-changed={file_path}");
        build_from_schema(file_path, target_path)?;
    }
    Ok(())
}

/// Generates Rust types from a JSON schema file.
///
/// # Arguments
/// * `file_path` - Path to the JSON schema file
/// * `target_path` - Path where generated Rust code will be written
///
/// # Errors
/// Returns an error if schema reading, type generation, or file writing fails.
pub fn build_from_schema(
    file_path: &str,
    target_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let schema = serde_json::from_reader(file)?;

    let mut type_space = TypeSpace::new(
        TypeSpaceSettings::default()
            .with_struct_builder(true)
            .with_derive("PartialEq".into())
            .with_derive("Eq".into()),
    );
    type_space.add_root_schema(schema)?;

    let mut syn_file = syn::parse2::<syn::File>(type_space.to_stream())?;
    utils::add_generated_code_header(&mut syn_file);
    utils::add_ignore_rustfmt(&mut syn_file);
    utils::add_ignore_clippy(&mut syn_file);

    let content = prettyplease::unparse(&syn_file);

    let out_file = Path::new(&target_path).to_path_buf();
    if let Some(parent) = out_file.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(out_file, content)?;

    Ok(())
}
