//! Lookups for assets embedded in this library.
//!
//! This module provides lookups to access the underlying decision point JSON files published by the SSVC repository.

use crate::decision_point::DecisionPoint;
use rust_embed::RustEmbed;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::LazyLock;

/// The embedded Decision Point JSON Schema content.
pub static SELECTION_LIST_SCHEMA: &str = include_str!("../assets/SelectionList_2_0_0.schema.json");

/// The embedded Decision Point JSON Schema content.
pub static DECISION_POINT_SCHEMA: &str = include_str!("../assets/DecisionPoint_2_0_0.schema.json");

/// Recursively loads all decision point JSON descriptions from `../ssvc/data/json/decision_points`.
#[derive(RustEmbed)]
#[folder = "assets/ssvc_decision_points/"]
#[include = "*.json"]
struct SsvcDecisionPointJsonFiles;

/// A unique identifier for a decision point composed of namespace, key, and version.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DecisionPointId {
    pub namespace: String,
    pub key: String,
    pub version: String,
}

type SsvcDecisionPointsMap = HashMap<DecisionPointId, DecisionPoint>;
/// Derives a lookup of all available decision points.
/// Entries are stored in a `HashMap` indexed by their respective DecisionPointId (namespace, key, version) for lookup.
pub static DECISION_POINTS: LazyLock<SsvcDecisionPointsMap> = LazyLock::new(|| {
    let mut decision_points = HashMap::new();

    for filename in SsvcDecisionPointJsonFiles::iter() {
        if let Some(file) = SsvcDecisionPointJsonFiles::get(&filename) {
            let content = std::str::from_utf8(&file.data).unwrap();
            match serde_json::from_str::<DecisionPoint>(content) {
                Ok(dp) => {
                    let key = DecisionPointId {
                        namespace: dp.namespace.deref().to_owned(),
                        key: dp.key.deref().to_owned(),
                        version: dp.version.deref().to_owned(),
                    };
                    decision_points.insert(key, dp);
                }
                Err(err) => {
                    // TODO: This should be cought or at least pre-validation at compile time.
                    eprintln!("Warning: Failed to parse decision point from file {filename}: {err}")
                }
            }
        }
    }

    decision_points
});

type SsvcDecisionPointsLookupMap = HashMap<DecisionPointId, HashMap<String, i32>>;
/// Derives a lookup mapping the known decision points to their decision point value keys and their respective position.
/// Used to verify the correct order of values within the respective decision points.
pub static DP_VALUE_KEY_ORDER: LazyLock<SsvcDecisionPointsLookupMap> = LazyLock::new(|| {
    let mut lookups = HashMap::new();

    for (decision_point_id, dp) in DECISION_POINTS.iter() {
        let mut lookup_map = HashMap::new();
        for (i, v) in dp.values.iter().enumerate() {
            lookup_map.insert(v.key.deref().to_owned(), i as i32);
        }
        lookups.insert(decision_point_id.clone(), lookup_map);
    }

    lookups
});

/// Derives all "registered" namespaces from the decision points. We assume that each namespace
/// that occurs in at least one decision point in the SSVC repository is a "registered" namespace.
pub static REGISTERED_NAMESPACES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut namespaces = HashSet::new();

    for decision_point_id in DECISION_POINTS.keys() {
        namespaces.insert(decision_point_id.namespace.to_owned());
    }

    namespaces
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssvc_exploitation_1_0_0_lookup() {
        let key = DecisionPointId {
            namespace: "ssvc".to_string(),
            key: "E".to_string(),
            version: "1.0.0".to_string(),
        };

        let decision_point = DECISION_POINTS.get(&key);
        assert!(
            decision_point.is_some(),
            "SSVC Exploitation 1.0.0 should be found in DECISION_POINTS lookup"
        );

        let dp = decision_point.unwrap();
        assert_eq!(dp.namespace.deref(), "ssvc");
        assert_eq!(dp.key.deref(), "E");
        assert_eq!(dp.version.deref(), "1.0.0");
        assert_eq!(dp.name.deref(), "Exploitation");
        assert_eq!(dp.values.len(), 3);
    }

    #[test]
    fn test_ssvc_exploitation_1_0_0_value_keys_order() {
        let key = DecisionPointId {
            namespace: "ssvc".to_string(),
            key: "E".to_string(),
            version: "1.0.0".to_string(),
        };

        let value_order = DP_VALUE_KEY_ORDER.get(&key);
        assert!(
            value_order.is_some(),
            "SSVC Exploitation 1.0.0 should be found in DP_VALUE_KEY_ORDER lookup"
        );

        let order_map = value_order.unwrap();
        assert_eq!(
            order_map.get("N"),
            Some(&0),
            "Value with key 'N' should be at position 0"
        );
        assert_eq!(
            order_map.get("P"),
            Some(&1),
            "Value with key 'P' should be at position 1"
        );
        assert_eq!(
            order_map.get("A"),
            Some(&2),
            "Value with key 'A' should be at position 2"
        );
    }

    #[test]
    fn test_ssvc_namespace_registered() {
        assert!(
            REGISTERED_NAMESPACES.contains("ssvc"),
            "ssvc namespace should be in REGISTERED_NAMESPACES"
        );
    }
}
