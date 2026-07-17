use std::fs;
use std::path::Path;

use super::utils;
use anyhow::Result;

/// Validates all decision point JSON files against the DecisionPoint schema.
///
/// # Errors
/// Returns an error if schema loading, file reading, or validation fails.
pub fn validate_decision_points() -> Result<()> {
    let decision_points_dir = "assets/ssvc_decision_points";
    let schema_path = "assets/DecisionPoint_2_0_0.schema.json";

    println!("cargo:rerun-if-changed={decision_points_dir}");
    println!("cargo:rerun-if-changed={schema_path}");

    let schema_file = fs::File::open(schema_path)?;
    let schema: serde_json::Value = serde_json::from_reader(schema_file)?;
    let validator =
        jsonschema::validator_for(&schema).map_err(|e| anyhow::anyhow!("Invalid schema: {}", e))?;

    let mut validator_closure = |path: &Path| {
        println!("cargo:rerun-if-changed={}", path.display());
        utils::validate_json_file(path, &validator, "DecisionPoint_2_0_0")
    };
    utils::walk_json_files(Path::new(decision_points_dir), &mut validator_closure)?;

    Ok(())
}
