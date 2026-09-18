use ssvc::selection_list::SelectionList;

/// Test based on CSAF 2.1 6-1-46-01
#[test]
fn test_6_1_46_01_deserialize_fails() {
    let json_data = r#"{
  "schemaVersion": "2.0.0",
  "timestamp": "2024-01-24T10:00:00.000Z"
    }"#;
    let selection_list: Result<SelectionList, _> = serde_json::from_str(json_data);
    assert!(selection_list.is_err());
    assert!(
        selection_list
            .unwrap_err()
            .to_string()
            .contains("missing field `selections`")
    );
}

/// Test based on CSAF 2.1 6-1-46-02
#[test]
fn test_6_1_46_02_deserialize_fails() {
    let json_data = r#"{
  "schemaVersion": "2.0.0",
  "selections": [
    {
      "name": "Attack Complexity",
      "namespace": "cvss",
      "values": [
        {
          "name": "Low"
        }
      ],
      "version": "3.0.1"
    }
  ],
  "target_ids": [
    "CVE-1900-0001"
  ],
  "timestamp": "2024-01-24T10:00:00.000Z"
}"#;
    let selection_list: Result<SelectionList, _> = serde_json::from_str(json_data);
    assert!(selection_list.is_err());
    assert!(
        selection_list
            .unwrap_err()
            .to_string()
            .contains("missing field `key`")
    );
}

/// Test based on CSAF 2.1 6-1-46-11
#[test]
fn test_6_1_46_11_deserialize_ok() {
    let json_data = r#"{
  "schemaVersion": "2.0.0",
  "selections": [
    {
      "key": "MI",
      "namespace": "ssvc",
      "values": [
        {
          "key": "N"
        }
      ],
      "version": "1.0.0"
    }
  ],
  "timestamp": "2024-01-24T10:00:00.000Z"
}"#;
    let selection_list: Result<SelectionList, _> = serde_json::from_str(json_data);
    assert!(selection_list.is_ok());
}

/// Test based on CSAF 2.1 6-1-46-12
#[test]
fn test_6_1_46_12_deserialize_ok() {
    let json_data = r#"{
  "schemaVersion": "2.0.0",
  "selections": [
    {
      "key": "AC",
      "name": "Attack Complexity",
      "namespace": "cvss",
      "values": [
        {
          "key": "L",
          "name": "Low"
        }
      ],
      "version": "3.0.1"
    }
  ],
  "target_ids": [
    "CVE-1900-0001"
  ],
  "timestamp": "2024-01-24T10:00:00.000Z"
}"#;
    let selection_list: Result<SelectionList, _> = serde_json::from_str(json_data);
    assert!(selection_list.is_ok());
}
