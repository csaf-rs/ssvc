use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::sync::LazyLock;
use rust_embed::RustEmbed;
use crate::decision_point::DecisionPoint;

/// The embedded Decision Point JSON Schema content.
pub static SELECTION_LIST_SCHEMA: &str = include_str!("../assets/SelectionList_2_0_0.schema.json");

/// The embedded Decision Point JSON Schema content.
pub static DECISION_POINT_SCHEMA: &str = include_str!("../assets/DecisionPoint_2_0_0.schema.json");

#[derive(RustEmbed)]
#[folder = "assets/ssvc_decision_points/"]
#[include = "*.json"]
struct SsvcDecisionPointJsonFiles;

type SsvcDecisionPointsMap = HashMap<(String, String, String), DecisionPoint>;
/// Recursively loads all decision point JSON descriptions from `../ssvc/data/json/decision_points`.
/// Entries are stored in a `HashMap` indexed by their respective (name, version) tuple for lookup.
pub static SSVC_DECISION_POINTS: LazyLock<SsvcDecisionPointsMap> = LazyLock::new(|| {
    let mut decision_points = HashMap::new();

    for filename in SsvcDecisionPointJsonFiles::iter() {
        if let Some(file) = SsvcDecisionPointJsonFiles::get(&filename) {
            let content = std::str::from_utf8(&file.data).unwrap();
            match serde_json::from_str::<DecisionPoint>(content) {
                Ok(dp) => {
                    let key = (
                        dp.namespace.deref().to_owned(),
                        dp.key.deref().to_owned(),
                        dp.version.deref().to_owned(),
                    );
                    decision_points.insert(key, dp);
                },
                Err(err) => eprintln!("Warning: Failed to parse decision point from file {filename}: {err}"),
            }
        }
    }

    decision_points
});

type SsvcDecisionPointsLookupMap = HashMap<(String, String, String), HashMap<String, i32>>;
/// Derives lookup maps for all observed SSVC decision points that can be used
/// to verify the order of values within the respective decision points.
pub static DP_VAL_KEYS_LOOKUP: LazyLock<SsvcDecisionPointsLookupMap> = LazyLock::new(|| {
    let mut lookups = HashMap::new();

    for (key, dp) in SSVC_DECISION_POINTS.iter() {
        let mut lookup_map = HashMap::new();
        for (i, v) in dp.values.iter().enumerate() {
            lookup_map.insert(v.key.deref().to_owned(), i as i32);
        }
        lookups.insert(key.clone(), lookup_map);
    }

    lookups
});

/// Collects all "registered" namespaces from known decision points. We assume that each namespace
/// that occurs in at least one decision point in the SSVC repository is a "registered" namespace.
pub static REGISTERED_SSVC_NAMESPACES: LazyLock<HashSet<String>> = LazyLock::new(|| {
    let mut namespaces = HashSet::new();

    for (namespace, _, _) in SSVC_DECISION_POINTS.keys() {
        namespaces.insert(namespace.to_owned());
    }

    namespaces
});