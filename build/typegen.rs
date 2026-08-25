use std::fs;
use std::path::Path;
use typify::{TypeSpace, TypeSpaceSettings};

use super::utils;
use anyhow::Result;

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
pub fn build_all_schemas() -> Result<()> {
    for (file_path, target_path) in SCHEMA_TARGETS {
        println!("cargo:rerun-if-changed={file_path}");
        // Only the SelectionList schema is part of the wasm-exposed API surface,
        // so only its generated types need `Tsify` derives.
        let with_tsify = *target_path == "src/generated/ssvc/selection_list.rs";
        build_from_schema(file_path, target_path, with_tsify)?;
    }
    Ok(())
}

/// Generates Rust types from a JSON schema file.
///
/// # Arguments
/// * `file_path` - Path to the JSON schema file
/// * `target_path` - Path where generated Rust code will be written
/// * `with_tsify` - Whether to add `Tsify` derives to generated structs/enums
///
/// # Errors
/// Returns an error if schema reading, type generation, or file writing fails.
pub fn build_from_schema(file_path: &str, target_path: &str, with_tsify: bool) -> Result<()> {
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
    if with_tsify {
        utils::add_tsify_derive(&mut syn_file);
        utils::add_tsify_datetime_type_override(&mut syn_file);
    }

    let content = prettyplease::unparse(&syn_file);

    let out_file = Path::new(&target_path).to_path_buf();
    if let Some(parent) = out_file.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(out_file, content)?;

    Ok(())
}
