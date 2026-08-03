use thiserror::Error;

/// Errors that can occur during namespace parsing and validation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NamespaceError {
    #[error("Namespace cannot be empty")]
    Empty,

    #[error("Namespace length must be between 3 and 1000 characters, got {length}")]
    LengthOutOfRange { length: usize },

    #[error("Reverse domain name cannot be empty")]
    EmptyReverseDomain,

    #[error("Fragment cannot be empty after '#'")]
    EmptyFragment,

    /// "invalid" or "x_invalid"
    #[error("Reserved forbidden namespace '{namespace}' must not be used")]
    ReservedForbiddenNamespace { namespace: String },

    /// "test" or "x_test", when allow_test is false
    #[error("Reserved test namespace '{namespace}' must not be used")]
    ReservedTestNamespace { namespace: String },

    #[error("Unregistered namespace must contain a fragment")]
    UnregisteredNamespaceMissingFragment,

    #[error("Extension Segments must contain a fragment")]
    ExtensionSegmentMissingFragment,

    #[error("Language tag cannot be empty in translation")]
    EmptyLanguageTag,

    #[error("Invalid language tag '{tag}': must be a valid BCP 47 language tag")]
    InvalidLanguageTag { tag: String },
}
