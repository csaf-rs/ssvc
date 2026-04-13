//! WASM bindings for SSVC
//!
//! This module provides WebAssembly bindings for the SSVC library, allowing it to be used in web applications.

use wasm_bindgen::prelude::*;
use crate::selection_list::SelectionList;
use crate::validation::validate_selection_list;


/// Initialize panic hook for better error messages in the browser console
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(js_name = validateSelectionList)]
pub fn validate_selection_list_from_string(
    json_str: &str,
    allow_test_namespaces: bool,
) -> Result<JsValue, JsValue> {
    let selection_list: SelectionList = serde_json::from_str(json_str)
        .map_err(|e| JsValue::from_str(&format!("Invalid SelectionList JSON: {e}")))?;

    let result = validate_selection_list(&selection_list, allow_test_namespaces);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = validateSelectionListValue)]
pub fn validate_selection_list_from_jsvalue(
    json_value: JsValue,
    allow_test_namespaces: bool,
) -> Result<JsValue, JsValue> {
    let selection_list: SelectionList = serde_wasm_bindgen::from_value(json_value)
        .map_err(|e| JsValue::from_str(&format!("Invalid SelectionList JSON value: {e}")))?;

    let result = validate_selection_list(&selection_list, allow_test_namespaces);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;
    use serde::Serialize;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn validate_selection_list_reports_validation_error() {
        let json = r#"{
            "schemaVersion": "2.0.0",
            "timestamp": "2025-01-01T00:00:00Z",
            "selections": [
                {
                    "key": "A",
                    "namespace": "example",
                    "version": "1.0.0",
                    "values": [
                        {"key": "B"}
                    ]
                }
            ]
        }"#;

        let result = validate_selection_list_from_string(json, false);
        assert!(result.is_ok());

        let value: serde_json::Value =
            serde_wasm_bindgen::from_value(result.unwrap()).expect("result should be valid JSON");

        assert_eq!(value.get("success").and_then(|v| v.as_bool()), Some(false));

        let errors = value
            .get("errors")
            .and_then(|v| v.as_array())
            .expect("errors should be an array");
        assert!(!errors.is_empty(), "expected at least one validation error");

        let first_error = &errors[0];
        let message = first_error
            .get("message")
            .and_then(|v| v.as_str())
            .expect("error message should be a string");
        assert!(message.contains("Invalid SSVC namespace"));

        let instance_path = first_error
            .get("instancePath")
            .and_then(|v| v.as_array())
            .expect("instancePath should be an array");
        assert_eq!(instance_path.len(), 3);
        assert_eq!(instance_path[0], "selections");
        assert_eq!(instance_path[1], "0");
        assert_eq!(instance_path[2], "namespace");
    }

    #[wasm_bindgen_test]
    fn validate_selection_list_accepts_js_value_input() {
        let value = serde_json::json!({
            "schemaVersion": "2.0.0",
            "timestamp": "2025-01-01T00:00:00Z",
            "selections": [
                {
                    "key": "A",
                    "namespace": "example",
                    "version": "1.0.0",
                    "values": [{"key": "B"}]
                }
            ]
        });

        let js_input = value
            .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
            .expect("input should convert to JSON-compatible JsValue");
        let result = validate_selection_list_from_jsvalue(js_input, false);
        assert!(result.is_ok());

        let output: serde_json::Value =
            serde_wasm_bindgen::from_value(result.unwrap()).expect("output should be valid JSON");
        assert_eq!(output.get("success").and_then(|v| v.as_bool()), Some(false));
    }
}
