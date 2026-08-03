//! Namespace validation according to SSVC namespace specification.
//!
//! This module provides functionality to parse and validate SSVC namespaces
//! according to the structure defined in the documentation.

pub(crate) mod assets;
pub(crate) mod base_namespace;
pub(crate) mod errors;
pub(crate) mod extension;

use crate::namespaces::assets::{RESERVED_EXAMPLE_NAMESPACES, RESERVED_TEST_NAMESPACES};
use crate::namespaces::extension::{Extensions, parse_extensions};

pub use assets::{REGISTERED_NAMESPACES};
pub use base_namespace::BaseNamespace;
pub use errors::NamespaceError;
pub use extension::Extension;

/// Represents the components of a parsed namespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedNamespace {
    pub base: BaseNamespace,
    pub extensions: Option<Extensions>,
}

impl ParsedNamespace {
    /// Parse a namespace string into its components, permitting "test" and "x_test" namespaces.
    pub fn parse_allow_test(namespace: &str) -> Result<Self, NamespaceError> {
        Self::parse_internal(namespace, true)
    }

    /// Parse a namespace string into its components for "production" use.
    /// "test" and "x_test" are forbidden.
    pub fn parse(namespace: &str) -> Result<Self, NamespaceError> {
        Self::parse_internal(namespace, false)
    }

    /// Parse a namespace string into its components.
    fn parse_internal(namespace: &str, allow_test: bool) -> Result<Self, NamespaceError> {
        if namespace.is_empty() {
            return Err(NamespaceError::Empty);
        }

        let len = namespace.len();
        if !(3..=1000).contains(&len) {
            return Err(NamespaceError::LengthOutOfRange { length: len });
        }

        let parts: Vec<&str> = namespace.split('/').collect();
        if parts.is_empty() {
            return Err(NamespaceError::Empty);
        }

        let base = BaseNamespace::parse_base(parts[0], allow_test)?;
        // check if there are extensions, if so, parse them
        let extensions = if parts.len() > 1 {
            parse_extensions(&parts[1..])?
        } else {
            None
        };

        Ok(ParsedNamespace { base, extensions })
    }

    /// Get the base namespace name (without fragment).
    pub fn base_name(&self) -> &str {
        match &self.base {
            BaseNamespace::Registered { name, .. } => name,
            BaseNamespace::Unregistered { reverse_domain, .. } => reverse_domain,
        }
    }

    pub fn is_test(&self) -> bool {
        RESERVED_TEST_NAMESPACES.contains(&self.base_name())
    }

    pub fn is_example(&self) -> bool {
        RESERVED_EXAMPLE_NAMESPACES.contains(&self.base_name())
    }

    /// Check if this is an unregistered namespace.
    pub fn is_unregistered(&self) -> bool {
        matches!(self.base, BaseNamespace::Unregistered { .. })
    }

    /// Check if this is a registered namespace.
    pub fn is_registered(&self) -> bool {
        matches!(self.base, BaseNamespace::Registered { .. })
    }
}

/// Validate a namespace string according to SSVC namespace rules.
///
/// This function parses and validates the namespace structure but does not
/// check if registered namespaces are actually registered in the system (that
/// check happens during selection list validation).
///
/// # Arguments
/// * `namespace` - The namespace string to validate
/// * `allow_test_namespaces` - Whether to allow namespaces with "test" extensions
pub fn validate_namespace(
    namespace: &str,
    allow_test_namespaces: bool,
) -> Result<ParsedNamespace, NamespaceError> {
    if allow_test_namespaces {
        ParsedNamespace::parse_allow_test(namespace)
    } else {
        ParsedNamespace::parse(namespace)
    }
}

#[cfg(test)]
mod tests {
    mod csaf_6_2_34_tests {
        use crate::validate_namespace;

        #[test]
        fn test_case_01_unregistered_namespace() {
            let result = validate_namespace(
                "x_example.unregistered#some-yet-unknown-or-maybe-private-namespace",
                false,
            );
            assert!(
                result.is_ok(),
                "Parsing should succeed for valid unregistered namespace"
            );
            let parsed = result.unwrap();
            assert!(parsed.is_unregistered());
        }

        #[test]
        fn test_case_02_unregistered_reserved_test_namespace_with_extension() {
            let result = validate_namespace(
                "x_test#also-unregistered-namespace//.example.other-test#some-extension",
                false,
            );
            assert!(
                result.is_err(),
                "x_test is reserved for testing and should fail without allow_test"
            );
        }

        #[test]
        fn test_case_03_invalid_namespace() {
            let result = validate_namespace("invalid", false);
            assert!(
                result.is_err(),
                "Forbidden namespace 'invalid' must not be used"
            );
        }

        #[test]
        fn test_case_11_registered_namespace() {
            let result = validate_namespace("ssvc", false);
            assert!(
                result.is_ok(),
                "Registered namespace 'ssvc' should be valid"
            );
        }

        #[test]
        fn test_case_12_registered_namespace_with_extension() {
            let result = validate_namespace("ssvc//.example.other-test#some-extension", false);
            assert!(
                result.is_ok(),
                "Registered namespace with extension should be valid"
            );
        }

        #[test]
        fn test_case_13_example_reserved_namespace() {
            let result = validate_namespace("example", false);
            assert!(
                result.is_ok(),
                "Reserved namespace 'example' for documentation should be valid"
            );
        }
    }
}
