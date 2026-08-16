use ssvc::selection_list::SelectionList;
use ssvc::{ValidationError, ValidationResult, validate_selection_list};

#[test]
fn csaf_6_1_48_01_fails() {
    let json_data = r#"{
              "schemaVersion": "2.0.0",
              "selections": [
                {
                  "key": "MI",
                  "namespace": "ssvc",
                  "values": [
                    {
                      "key": "N",
                      "name": "None"
                    },
                    {
                      "key": "D",
                      "name": "Degraded"
                    }
                  ],
                  "version": "1.0.0"
                }
              ],
              "timestamp": "2024-01-24T10:00:00.000Z"
            }"#;

    let selection_list: SelectionList =
        serde_json::from_str(json_data).expect("SSVC SelectionList was invalid JSON");

    let result = validate_selection_list(&selection_list, false);

    assert_eq!(result, ValidationResult {
        success: false,
        errors: vec![
            ValidationError {
                message: "The SSVC decision point 'ssvc::Mission Impact' (version 1.0.0) doesn't have a value with key 'D'".to_string(),
                instance_path: Vec::from([
                    "selections".to_string(),
                    "0".to_string(),
                    "values".to_string(),
                    "1".to_string(),
                ]),
            },
        ],
    })
}

#[test]
fn csaf_6_1_48_01_fixed_passes() {
    let json_data = r#"{
              "schemaVersion": "2.0.0",
              "selections": [
                {
                  "key": "MI",
                  "namespace": "ssvc",
                  "values": [
                    {
                      "key": "N",
                      "name": "None"
                    }
                  ],
                  "version": "1.0.0"
                }
              ],
              "timestamp": "2024-01-24T10:00:00.000Z"
            }"#;

    let selection_list: SelectionList =
        serde_json::from_str(json_data).expect("SSVC SelectionList was invalid JSON");

    let result = validate_selection_list(&selection_list, false);

    assert!(result.success);
}

#[test]
fn nist_800_30_with_wrong_key_fails() {
    let json_data = r#"{
              "schemaVersion": "2.0.0",
              "selections": [
                {
                  "key": "P_5X",
                  "namespace": "nist#800-30",
                  "values": [
                    {
                      "key": "V",
                      "name": "Very Low"
                    }
                  ],
                  "version": "1.0.0"
                }
              ],
              "timestamp": "2024-01-24T10:00:00.000Z"
            }"#;

    let selection_list: SelectionList =
        serde_json::from_str(json_data).expect("SSVC SelectionList was invalid JSON");

    let result = validate_selection_list(&selection_list, false);

    assert_eq!(result, ValidationResult {
        success: false,
        errors: vec![
            ValidationError {
                message: "The SSVC decision point 'nist#800-30::Probability Scale in 5 weighted levels, ascending' (version 1.0.0) doesn't have a value with key 'V'".to_string(),
                instance_path: Vec::from([
                    "selections".to_string(),
                    "0".to_string(),
                    "values".to_string(),
                    "0".to_string(),
                ]),
            },
        ],
    })
}

#[test]
fn nist_800_30_passes() {
    let json_data = r#"{
              "schemaVersion": "2.0.0",
              "selections": [
                {
                  "key": "P_5X",
                  "namespace": "nist#800-30",
                  "values": [
                    {
                      "key": "VL",
                      "name": "Very Low"
                    }
                  ],
                  "version": "1.0.0"
                }
              ],
              "timestamp": "2024-01-24T10:00:00.000Z"
            }"#;

    let selection_list: SelectionList =
        serde_json::from_str(json_data).expect("SSVC SelectionList was invalid JSON");

    let result = validate_selection_list(&selection_list, false);

    assert!(result.success);
}
