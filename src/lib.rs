mod assets;
mod namespaces;
mod validation;

mod generated {
    pub mod ssvc {
        pub mod decision_point;
        pub mod selection_list;
    }
}

// public API
// validation schemas
pub use assets::{DECISION_POINT_SCHEMA, SELECTION_LIST_SCHEMA};
// generated structs
pub use generated::ssvc::{decision_point, selection_list};
// registered namespaces list, namespace validation function and assoc. structs
pub use namespaces::{REGISTERED_NAMESPACES, validate_namespace, ParsedNamespace, BaseNamespace, Extension, NamespaceError};
// selection list validation function and assoc. structs
pub use validation::{validate_selection_list, ValidationError, ValidationResult};

#[cfg(feature = "wasm")]
pub mod wasm;
