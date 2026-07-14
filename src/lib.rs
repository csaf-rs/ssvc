pub mod assets;
pub mod namespaces;
pub mod validation;

pub mod generated {
    pub mod ssvc {
        pub mod decision_point;
        pub mod selection_list;
    }
}

pub use generated::ssvc::{decision_point, selection_list};
pub use validation::{validate_selection_list, ValidationError, ValidationResult};

#[cfg(feature = "wasm")]
pub mod wasm;
