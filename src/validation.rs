use std::ops::Deref;
use crate::assets::{DP_VAL_KEYS_LOOKUP, REGISTERED_SSVC_NAMESPACES, SSVC_DECISION_POINTS};
use crate::selection_list::SelectionList;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsvcError {
    pub message: String,
    pub instance_path: Vec<String>,
}

fn create_unknown_value_error(
    namespace: &str,
    dp_name: &str,
    version: &str,
    value_key: &str,
    i_s: usize,
    i_val: usize,
) -> SsvcError {
    SsvcError {
        message: format!(
            "The SSVC decision point '{namespace}::{dp_name}' (version {version}) doesn't have a value with key '{value_key}'"
        ),
        instance_path: vec![
            "selections".to_string(),
            i_s.to_string(),
            "values".to_string(),
            i_val.to_string(),
        ],
    }
}

fn create_incorrect_order_error(
    namespace: &str,
    dp_name: &str,
    version: &str,
    i_s: usize,
    i_val: usize,
) -> SsvcError {
    SsvcError {
        message: format!(
            "The values for SSVC decision point '{namespace}::{dp_name}' (version {version}) are not in correct order"
        ),
        instance_path: vec![
            "selections".to_string(),
            i_s.to_string(),
            "values".to_string(),
            i_val.to_string(),
        ],
    }
}

fn create_unknown_decision_point_error(
    namespace: &str,
    key: &str,
    version: &str,
    i_s: usize,
) -> SsvcError {
    SsvcError {
        message: format!("Unknown SSVC decision point '{namespace}::{key}' with version '{version}'"),
        instance_path: vec!["selections".to_string(), i_s.to_string()],
    }
}

pub fn validate_selection_list(selection_list: SelectionList) -> Result<(), Vec<SsvcError>> {
    for (i_s, selection) in selection_list.selections.iter().enumerate() {
        // Skip this test for unregistered namespaces
        if !REGISTERED_SSVC_NAMESPACES.contains(selection.namespace.deref()) {
            continue;
        }

        // Create the key for lookup in CSAF_SSVC_DECISION_POINTS
        let (namespace, s_key, version) = (
            selection.namespace.deref().to_owned(),
            selection.key.deref().to_owned(),
            selection.version.deref().to_owned(),
        );
        let dp_key = (namespace.clone(), s_key.clone(), version.clone());
        match SSVC_DECISION_POINTS.get(&dp_key) {
            Some(dp) => {
                // Get value indices of decision point
                let reference_indices = DP_VAL_KEYS_LOOKUP.get(&dp_key).unwrap();
                // Index of last-seen value
                let mut last_index: i32 = -1;
                // Check if all values exist and are correctly ordered
                for (i_val, v_key) in selection.values.iter().map(|v| v.key.deref()).enumerate()
                {
                    match reference_indices.get(v_key) {
                        None => {
                            return Err(vec![create_unknown_value_error(
                                &namespace,
                                dp.name.deref(),
                                &version,
                                v_key,
                                i_s,
                                i_val,
                            )]);
                        },
                        Some(i_dp_val) => {
                            if last_index > *i_dp_val {
                                return Err(vec![create_incorrect_order_error(
                                    &namespace,
                                    dp.name.deref(),
                                    &version,
                                    i_s,
                                    i_val,
                                )]);
                            } else {
                                last_index = *i_dp_val;
                            }
                        },
                    }
                }
            },
            None => {
                return Err(vec![create_unknown_decision_point_error(
                    &namespace, &s_key, &version, i_s,
                )]);
            },
        }
    }
    Ok(())
}