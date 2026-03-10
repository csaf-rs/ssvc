pub mod assets;
pub mod validation;
pub mod namespaces;

pub mod generated {
    pub mod ssvc {
        pub mod decision_point;
        pub mod selection_list;
    }
}

pub use generated::ssvc::{decision_point, selection_list};
