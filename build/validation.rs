use std::collections::{HashMap, HashSet};
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

    println!("cargo:rerun-if-changed={schema_path}");

    let schema_file = fs::File::open(schema_path)?;
    let schema: serde_json::Value = serde_json::from_reader(schema_file)?;
    let validator =
        jsonschema::validator_for(&schema).map_err(|e| anyhow::anyhow!("Invalid schema: {}", e))?;

    let mut seen_triples = HashMap::new();

    let mut validator_closure = |path: &Path| {
        println!("cargo:rerun-if-changed={}", path.display());
        let json = utils::validate_json_file(path, &validator, "DecisionPoint_2_0_0")?;

        // Check for duplicate (namespace, key, version) triples
        // namespace and key are safe to unwrap as they are required by schema; version has a default of "0.0.1"
        let namespace = json["namespace"].as_str().unwrap().to_string();
        let key = json["key"].as_str().unwrap().to_string();
        let version = json["version"]
            .as_str()
            .unwrap_or("0.0.1")
            .to_string();

        let triple = (namespace, key, version);
        if let Some(original_path) = seen_triples.get(&triple) {
            return Err(anyhow::anyhow!(
                "Duplicate decision point triple found: namespace='{}', key='{}', version='{}'\n  first occurrence: {}\n  offending occurrence: {}",
                triple.0,
                triple.1,
                triple.2,
                original_path,
                path.display()
            ));
        }
        seen_triples.insert(triple.clone(), path.display().to_string());

        // Check that decision point value keys are unique
        // Safe to unwrap as these are guaranteed by schema validation
        let values = json["values"].as_array().unwrap();
        let mut seen_value_keys = HashSet::new();
        for value in values.iter() {
            let value_key = value["key"].as_str().unwrap();
            if !seen_value_keys.insert(value_key) {
                return Err(anyhow::anyhow!(
                    "Duplicate value key '{}' in decision point {}:{}:{} (file: {})",
                    value_key,
                    triple.0,
                    triple.1,
                    triple.2,
                    path.display()
                ));
            }
        }

        Ok(())
    };
    utils::walk_json_files(Path::new(decision_points_dir), &mut validator_closure)?;

    Ok(())
}
