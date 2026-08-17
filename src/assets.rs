//! Lookups for assets embedded in this library.
//!
//! This module provides lookups to access the underlying decision point JSON files published by the SSVC repository.

use crate::BaseNamespace;
use crate::decision_point::DecisionPoint;
use crate::generated::ssvc::decision_point_assets;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::LazyLock;

/// The embedded Decision Point JSON Schema content.
pub static SELECTION_LIST_SCHEMA: &str = include_str!("../assets/SelectionList_2_0_0.schema.json");

/// The embedded Decision Point JSON Schema content.
pub static DECISION_POINT_SCHEMA: &str = include_str!("../assets/DecisionPoint_2_0_0.schema.json");

/// A unique identifier for a decision point composed of namespace, key, and version.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct DecisionPointId {
    pub namespace: BaseNamespace,
    pub key: String,
    pub version: String,
}

type SsvcDecisionPointsMap = HashMap<DecisionPointId, DecisionPoint>;
/// Derives a lookup of all available decision points.
/// Entries are stored in a `HashMap` indexed by their respective DecisionPointId (namespace, key, version) for lookup.
pub static DECISION_POINTS: LazyLock<SsvcDecisionPointsMap> =
    LazyLock::new(decision_point_assets::get_decision_points);

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

/// Collects all available registered namespaces from the decision points. These are the namespaces our
/// library "knows" and can validate against.
pub static AVAILABLE_REGISTERED_NAMESPACES: LazyLock<HashSet<BaseNamespace>> =
    LazyLock::new(|| {
        let mut namespaces = HashSet::new();

        for decision_point_id in DECISION_POINTS.keys().filter(|decision_point_id| {
            matches!(
                decision_point_id.namespace,
                BaseNamespace::Registered { .. }
            )
        }) {
            namespaces.insert(decision_point_id.namespace.to_owned());
        }

        namespaces
    });

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_ssvc_exploitation_1_0_0_lookup() {
        let key = DecisionPointId {
            namespace: BaseNamespace::parse_base("ssvc", false).unwrap(),
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
            namespace: BaseNamespace::parse_base("ssvc", false).unwrap(),
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

    #[rstest]
    #[case::ssvc_is_available("ssvc", true)]
    #[case::example_is_not_available("example", false)]
    fn test_available_registered_namespaces_lookup(
        #[case] namespace: &str,
        #[case] expected: bool,
    ) {
        let parsed_namespaces = BaseNamespace::parse_base(namespace, false).unwrap();
        assert_eq!(
            AVAILABLE_REGISTERED_NAMESPACES.contains(&parsed_namespaces),
            expected
        );
    }
}
