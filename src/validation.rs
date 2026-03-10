use std::ops::Deref;
use crate::assets::{DP_VAL_KEYS_LOOKUP, REGISTERED_SSVC_NAMESPACES, SSVC_DECISION_POINTS};
use crate::namespaces::{validate_namespace, validate_namespace_allow_test, BaseNamespace, ParsedNamespace};
use crate::selection_list::SelectionList;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsvcError {
    pub message: String,
    pub instance_path: Vec<String>,
}

/// Helper function for validating field matches between selection and base decision points.
fn validate_field_match<S, B>(
    selection_field: &Option<S>,
    base_field: &B,
    field_name: &str,
    ext_namespace: &str,
    base_namespace: &str,
    context: &str,
    i_s: usize,
    path_suffix: &[&str],
) -> Result<(), SsvcError>
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

            return Err(SsvcError {
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
    Ok(())
}

pub fn validate_selection_list(selection_list: &SelectionList) -> Result<(), Vec<SsvcError>> {
    validate_selection_list_internal(selection_list, validate_namespace)
}

pub fn validate_selection_list_allow_test(selection_list: &SelectionList) -> Result<(), Vec<SsvcError>> {
    validate_selection_list_internal(selection_list, validate_namespace_allow_test)
}

fn validate_selection_list_internal(
    selection_list: &SelectionList,
    namespace_validation_fn: fn(&str) -> Result<ParsedNamespace, String>,
) -> Result<(), Vec<SsvcError>> {
    for (i_s, selection) in selection_list.selections.iter().enumerate() {
        // Parse and validate namespace structure
        let parsed_ns = match namespace_validation_fn(selection.namespace.deref()) {
            Ok(ns) => ns,
            Err(err) => {
                return Err(vec![SsvcError {
                    message: format!("Invalid SSVC namespace: {}", err),
                    instance_path: vec![
                        "selections".to_string(),
                        i_s.to_string(),
                        "namespace".to_string()
                    ],
                }]);
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
                    validate_field_match(
                        &selection.name,
                        &dp.name,
                        "name",
                        selection.namespace.deref(),
                        &base_name,
                        "decision point",
                        i_s,
                        &[],
                    ).map_err(|e| vec![e])?;

                    validate_field_match(
                        &selection.definition,
                        &dp.definition,
                        "definition",
                        selection.namespace.deref(),
                        &base_name,
                        "decision point",
                        i_s,
                        &[],
                    ).map_err(|e| vec![e])?;
                }

                let mut last_index: i32 = -1;

                // Check if all values exist in the base decision point and are correctly ordered.
                for (i_val, sel_val) in selection.values.iter().enumerate() {
                    let v_key = sel_val.key.deref();
                    match reference_indices.get(v_key) {
                        None => {
                            // The value is not found in the base decision point
                            return Err(vec![SsvcError {
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
                            }]);
                        },
                        Some(i_dp_val) => {
                            // Verify order is maintained (subset must preserve order from base)
                            if last_index > *i_dp_val {
                                return Err(vec![SsvcError {
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
                                }]);
                            }
                            last_index = *i_dp_val;

                            // If the namespace has no extensions, validate value name and definition match base (if provided)
                            if parsed_ns.extensions.is_empty() {
                                let base_val = &dp.values[*i_dp_val as usize];
                                let context = format!("value '{}'", v_key);

                                validate_field_match(
                                    &sel_val.name,
                                    &base_val.name,
                                    "name",
                                    selection.namespace.deref(),
                                    &base_name,
                                    &context,
                                    i_s,
                                    &["values", &i_val.to_string()],
                                ).map_err(|e| vec![e])?;

                                validate_field_match(
                                    &sel_val.definition,
                                    &base_val.definition,
                                    "definition",
                                    selection.namespace.deref(),
                                    &base_name,
                                    &context,
                                    i_s,
                                    &["values", &i_val.to_string()],
                                ).map_err(|e| vec![e])?;
                            }
                        },
                    }
                }
            },
            None => {
                return Err(vec![SsvcError {
                    message: format!(
                        "Unknown SSVC decision point '{}::{}' with version '{}'",
                        selection.namespace.deref(),
                        s_key,
                        version
                    ),
                    instance_path: vec!["selections".to_string(), i_s.to_string()],
                }]);
            },
        }
    }
    Ok(())
}