use std::ops::Deref;
use crate::assets::{DP_VAL_KEYS_LOOKUP, REGISTERED_SSVC_NAMESPACES, SSVC_DECISION_POINTS};
use crate::namespaces::{validate_namespace, BaseNamespace};
use crate::selection_list::SelectionList;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ValidationResult {
    pub success: bool,
    pub errors: Vec<ValidationError>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ValidationError {
    pub message: String,
    #[serde(rename = "instancePath")]
    pub instance_path: Vec<String>,
}

/// Helper function for validating field matches between selection and base
/// decision points.
/// 
/// Returns a ValidationError if the selection field is Some and does not match
/// the base field, otherwise returns None.
fn validate_field_match<S, B>(
    selection_field: &Option<S>,
    base_field: &B,
    field_name: &str,
    ext_namespace: &str,
    base_namespace: &str,
    context: &str,
    i_s: usize,
    path_suffix: &[&str],
) -> Option<ValidationError>
where
    S: Deref<Target = String>,
    B: Deref<Target = String>,
{
    if let Some(sel_field) = selection_field {
        if sel_field.deref() != base_field.deref() {
            let mut instance_path = vec![
                "selections".to_string(),
                i_s.to_string(),
            ];
            instance_path.extend(path_suffix.iter().map(|s| s.to_string()));
            instance_path.push(field_name.to_string());

            return Some(ValidationError {
                message: format!(
                    "Extension namespace '{}' {} {} '{}' does not match base namespace '{}' {} '{}'",
                    ext_namespace,
                    context,
                    field_name,
                    sel_field.deref(),
                    base_namespace,
                    field_name,
                    base_field.deref()
                ),
                instance_path,
            });
        }
    }
    None
}

/// Main validation function for SSVC SelectionList. Validates that all
/// selections with registered SSVC namespaces conform to the structure of their
/// corresponding decision points, including extension rules.
/// 
/// # Arguments
/// * `selection_list` - The SelectionList to validate
/// * `allow_test_namespaces` - Whether to allow namespaces with "test"
///   extensions
pub fn validate_selection_list(
    selection_list: &SelectionList,
    allow_test_namespaces: bool,
) -> ValidationResult {
    let mut errors: Vec<ValidationError> = Vec::new();

    for (i_s, selection) in selection_list.selections.iter().enumerate() {
        // Parse and validate namespace structure
        let parsed_ns = match validate_namespace(selection.namespace.deref(), allow_test_namespaces) {
            Ok(ns) => ns,
            Err(err) => {
                errors.push(ValidationError {
                    message: format!("Invalid SSVC namespace: {}", err),
                    instance_path: vec![
                        "selections".to_string(),
                        i_s.to_string(),
                        "namespace".to_string()
                    ],
                });
                continue;
            }
        };

        // Extract base namespace name (without extensions and fragment)
        let base_name = match parsed_ns.base {
            BaseNamespace::Registered { name, .. } => name,
            // Skip unregistered namespaces - they are not validated against known decision points
            BaseNamespace::Unregistered { .. } => continue,
        };

        // Skip if the base namespace is not explicitly registered in SSVC
        if !REGISTERED_SSVC_NAMESPACES.contains(base_name.as_str()) {
            continue;
        }

        // Look up the decision point using base namespace (without extensions).
        // This implements the extension validation rule: extensions apply to
        // registered base namespaces and must follow the same decision point structure.
        let s_key = selection.key.deref().to_owned();
        let version = selection.version.deref().to_owned();
        let dp_key = (base_name.clone(), s_key.clone(), version.clone());

        match SSVC_DECISION_POINTS.get(&dp_key) {
            Some(dp) => {
                // Get value indices of decision point from base namespace
                let reference_indices = DP_VAL_KEYS_LOOKUP.get(&dp_key).unwrap();

                // Validate extension rules:
                // - Extensions can limit values (subset) but not add new ones
                // - Extensions cannot change the order of values
                // - Extensions can translate/refine name and definition but not change key
                // All validation is done against the base namespace decision point

                // If the namespace has no extensions, validate that name and definition match the base (if provided)
                if parsed_ns.extensions.is_empty() {
                    if let Some(error) = validate_field_match(
                        &selection.name,
                        &dp.name,
                        "name",
                        selection.namespace.deref(),
                        &base_name,
                        "decision point",
                        i_s,
                        &[],
                    ) {
                        errors.push(error);
                    }

                    if let Some(error) = validate_field_match(
                        &selection.definition,
                        &dp.definition,
                        "definition",
                        selection.namespace.deref(),
                        &base_name,
                        "decision point",
                        i_s,
                        &[],
                    ) {
                        errors.push(error);
                    }
                }

                let mut last_index: i32 = -1;

                // Check if all values exist in the base decision point and are correctly ordered.
                for (i_val, sel_val) in selection.values.iter().enumerate() {
                    let v_key = sel_val.key.deref();
                    match reference_indices.get(v_key) {
                        None => {
                            // The value is not found in the base decision point
                            errors.push(ValidationError {
                                message: format!(
                                    "The SSVC decision point '{}::{}' (version {}) doesn't have a value with key '{}'",
                                    selection.namespace.deref(),
                                    dp.name.deref(),
                                    version,
                                    v_key
                                ),
                                instance_path: vec![
                                    "selections".to_string(),
                                    i_s.to_string(),
                                    "values".to_string(),
                                    i_val.to_string(),
                                ],
                            });
                            continue;
                        },
                        Some(i_dp_val) => {
                            // Verify order is maintained (subset must preserve order from base)
                            if last_index > *i_dp_val {
                                errors.push(ValidationError {
                                    message: format!(
                                        "The values for SSVC decision point '{}::{}' (version {}) are not in correct order",
                                        selection.namespace.deref(),
                                        dp.name.deref(),
                                        version
                                    ),
                                    instance_path: vec![
                                        "selections".to_string(),
                                        i_s.to_string(),
                                        "values".to_string(),
                                        i_val.to_string(),
                                    ],
                                });
                                continue;
                            }
                            last_index = *i_dp_val;

                            // If the namespace has no extensions, validate value name and definition match base (if provided)
                            if parsed_ns.extensions.is_empty() {
                                let base_val = &dp.values[*i_dp_val as usize];
                                let context = format!("value '{}'", v_key);

                                if let Some(error) = validate_field_match(
                                    &sel_val.name,
                                    &base_val.name,
                                    "name",
                                    selection.namespace.deref(),
                                    &base_name,
                                    &context,
                                    i_s,
                                    &["values", &i_val.to_string()],
                                ) {
                                    errors.push(error);
                                }

                                if let Some(error) = validate_field_match(
                                    &sel_val.definition,
                                    &base_val.definition,
                                    "definition",
                                    selection.namespace.deref(),
                                    &base_name,
                                    &context,
                                    i_s,
                                    &["values", &i_val.to_string()],
                                ) {
                                    errors.push(error);
                                }
                            }
                        },
                    }
                }
            },
            None => {
                errors.push(ValidationError {
                    message: format!(
                        "Unknown SSVC decision point '{}::{}' with version '{}'",
                        selection.namespace.deref(),
                        s_key,
                        version
                    ),
                    instance_path: vec!["selections".to_string(), i_s.to_string()],
                });
                continue;
            },
        }
    }

    ValidationResult {
        success: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::ValidationError;

    #[test]
    fn validation_error_is_serde_serializable() {
        let error = ValidationError {
            message: "example".to_string(),
            instance_path: vec!["selections".to_string(), "0".to_string()],
        };

        let json = serde_json::to_value(&error).expect("serialize ValidationError");
        assert_eq!(json.get("message").and_then(|v| v.as_str()), Some("example"));
        assert_eq!(json.get("instancePath").and_then(|v| v.as_array()).map(|a| a.len()), Some(2));

        let roundtrip: ValidationError = serde_json::from_value(json).expect("deserialize ValidationError");
        assert_eq!(roundtrip, error);
    }
}