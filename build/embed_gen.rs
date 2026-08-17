use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Validates the decision points and generates Rust code that contains the parsed decision point data as static structures.
pub fn generate_embedded_files() -> Result<()> {
    let assets_path = Path::new("assets/ssvc_decision_points");
    let out_path = Path::new("src/generated/ssvc/decision_point_assets.generated.rs");
    let schema_path = "assets/DecisionPoint_2_0_0.schema.json";

    // create a JSON schema validator
    println!("cargo:rerun-if-changed={schema_path}");
    let schema: Value = serde_json::from_reader(fs::File::open(schema_path)?)?;
    let validator =
        jsonschema::validator_for(&schema).map_err(|e| anyhow::anyhow!("Invalid schema: {}", e))?;

    // Parse and validate all JSON files
    let mut decision_points: HashMap<(String, String, String), (Value, String)> = HashMap::new();
    super::utils::walk_json_files(assets_path, &mut |path| {
        // validate if the file matches the schema
        let json_val = super::utils::validate_json_file(path, &validator, "DecisionPoint_2_0_0")?;

        // namespace and key are safe to unwrap as they are required by schema; version has a default of "0.0.1"
        // that the DecisionPoint deserialization applies
        let ns = json_val["namespace"].as_str().unwrap().to_string();
        let key = json_val["key"].as_str().unwrap().to_string();
        let ver = json_val["version"].as_str().unwrap_or("0.0.1").to_string();

        // check that the value keys are unique in a decision point
        validate_value_keys(&json_val, &ns, &key, &ver, path)?;

        // check for duplicate (namespace, key, version) triples across all imported decision points
        if let Some((_json_val, first_path)) = decision_points.insert(
            (ns.clone(), key.clone(), ver.clone()),
            (json_val, path.display().to_string()),
        ) {
            // we can safely unwrap here, as we are retrieving the collision causing this if block to run
            return Err(anyhow::anyhow!(
                "Duplicate decision point triple found: namespace='{}', key='{}', version='{}' (first occurrence: {}, second occurrence: {})",
                ns,
                key,
                ver,
                first_path,
                path.display()
            ));
        }
        Ok(())
    })?;
    generate_embedded_assets(decision_points, out_path)?;

    Ok(())
}

/// Generates rust code based on the previously aggregated decision point hashset
fn generate_embedded_assets(
    decision_points: HashMap<(String, String, String), (Value, String)>,
    out_path: &Path,
) -> Result<()> {
    // sort keys for deterministic output
    let mut sorted_keys: Vec<&(String, String, String)> = decision_points.keys().collect();
    sorted_keys.sort();

    // generate snippets for each decision point, including the hashmap key
    // we already have from the duplicate detection
    let decision_point_entries: Vec<TokenStream> = sorted_keys
        .iter()
        .map(|(ns, key, ver)| {
            let (json_val, _path) = &decision_points[&(ns.clone(), key.clone(), ver.clone())];
            let json_str = json_val.to_string();
            quote! {
                m.insert(
                    DecisionPointId {
                        namespace: BaseNamespace::parse_base(#ns, true).expect("The namespaces of an asset provided by CERTCC/SSVC does not match the SSVC namespace specification. This will fail during our test suite."),
                        key: #key.to_string(),
                        version: #ver.to_string(),
                    },
                    serde_json::from_str::<DecisionPoint>(#json_str).expect("This should not happen, as we have ensured that the JSON is schema compliant already."),
                );
            }
        })
        .collect();

    let count = decision_point_entries.len();

    // wrapper
    let tokens = quote! {
        use crate::assets::DecisionPointId;
        use crate::decision_point::DecisionPoint;
        use crate::BaseNamespace;
        use std::collections::HashMap;

        pub fn get_decision_points() -> HashMap<DecisionPointId, DecisionPoint> {
            let mut m = HashMap::with_capacity(#count);
            #(#decision_point_entries)*
            m
        }
    };

    // write to file
    let mut syn_file = syn::parse2::<syn::File>(tokens)?;
    super::utils::add_generated_code_header(&mut syn_file);
    super::utils::add_ignore_rustfmt(&mut syn_file);
    super::utils::add_ignore_clippy(&mut syn_file);

    let content = prettyplease::unparse(&syn_file);
    fs::write(out_path, content)?;

    Ok(())
}

/// Checks that all value keys within a decision point are unique.
fn validate_value_keys(
    json_val: &Value,
    namespace: &str,
    key: &str,
    version: &str,
    path: &Path,
) -> Result<()> {
    // safe to unwrap: values array is guaranteed by schema validation
    let values = json_val["values"].as_array().unwrap();
    let mut seen = HashSet::new();
    for value in values {
        let value_key = value["key"].as_str().unwrap();
        if !seen.insert(value_key) {
            return Err(anyhow::anyhow!(
                "Duplicate value key '{}' in decision point with namespace='{}', key='{}', version='{}' (file: {})",
                value_key,
                namespace,
                key,
                version,
                path.display()
            ));
        }
    }
    Ok(())
}
