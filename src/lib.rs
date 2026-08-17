mod assets;
mod namespaces;
mod validation;

mod generated {
    pub mod ssvc {
        #[path = "./decision_point_assets.generated.rs"]
        pub mod decision_point_assets;

        pub mod decision_point;
        pub mod selection_list;
    }
}

// public API
// validation schemas
pub use assets::{AVAILABLE_REGISTERED_NAMESPACES, DECISION_POINT_SCHEMA, SELECTION_LIST_SCHEMA};
// generated structs
pub use generated::ssvc::{decision_point, selection_list};
// registered namespaces list, namespace validation function and assoc. structs
pub use namespaces::{
    BaseNamespace, Extension, NamespaceError, ParsedNamespace, REGISTERED_NAMESPACES,
    validate_namespace,
};
// selection list validation function and assoc. structs
pub use validation::{ValidationError, ValidationResult, validate_selection_list};

#[cfg(feature = "wasm")]
pub mod wasm;
