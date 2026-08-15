//! Namespace validation according to SSVC namespace specification.
//!
//! This module provides functionality to parse and validate SSVC namespaces
//! according to the structure defined in the documentation.

pub(crate) mod base_namespace;
pub(crate) mod constants;
pub(crate) mod errors;
pub(crate) mod extension;

use crate::namespaces::constants::{
    RESERVED_EXAMPLE_NAMESPACES, RESERVED_TEST_NAMESPACES, SECTION_DELIMITER,
};
use crate::namespaces::extension::{Extensions, parse_extensions};

pub use base_namespace::BaseNamespace;
pub use constants::REGISTERED_NAMESPACES;
pub use errors::NamespaceError;
pub use extension::Extension;

/// Represents the components of a parsed namespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedNamespace {
    pub base: BaseNamespace,
    pub extensions: Option<Extensions>,
}

impl std::fmt::Display for ParsedNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)?;
        if let Some(ref exts) = self.extensions {
            for ext in exts {
                write!(f, "{}{}", SECTION_DELIMITER, ext)?;
            }
        }
        Ok(())
    }
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

        let parts: Vec<&str> = namespace.split(SECTION_DELIMITER).collect();
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

    mod validate_namespace {
        use crate::{BaseNamespace, NamespaceError, validate_namespace};

        #[test]
        fn test_namespace_empty() {
            let result = validate_namespace("", false);
            assert_eq!(result, Err(NamespaceError::Empty));
        }

        #[test]
        fn test_namespace_too_short() {
            let result = validate_namespace("ab", false);
            assert_eq!(
                result.err(),
                Some(NamespaceError::LengthOutOfRange { length: 2 })
            );
        }

        #[test]
        fn test_namespace_too_long() {
            let long_ns = "a".repeat(1001);
            let result = validate_namespace(&long_ns, false);
            assert_eq!(
                result.err(),
                Some(NamespaceError::LengthOutOfRange { length: 1001 })
            );
        }

        #[test]
        fn test_multiple_extensions() {
            let result =
                validate_namespace("ssvc/de-DE/.example.org#ref1/.example.org#ref2", false);
            assert!(result.is_ok());
            let parsed = result.unwrap();
            assert!(parsed.extensions.is_some_and(|ext| ext.len() == 3));
        }

        #[test]
        fn test_unregistered_multiple_dots_in_domain() {
            let result = validate_namespace("x_com.example.subdomain#test", false);
            assert!(result.is_ok());
            let parsed = result.unwrap();
            match parsed.base {
                BaseNamespace::Unregistered {
                    reverse_domain,
                    fragment,
                } => {
                    assert_eq!(reverse_domain, "com.example.subdomain");
                    assert_eq!(fragment, "test");
                }
                _ => panic!("Expected unregistered namespace"),
            }
        }
    }
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

    mod display_round_trip {
        use crate::ParsedNamespace;
        use rstest::rstest;

        #[rstest]
        #[case::registered_without_fragment("ssvc", false)]
        #[case::registered_with_fragment("ssvc#fragment", false)]
        #[case::unregistered("x_com.example#fragment", false)]
        #[case::with_default_first_segment("ssvc/", false)]
        #[case::with_non_default_first_segment("ssvc/de-DE", false)]
        #[case::with_domain_extension("ssvc/de-DE/.example.organization#ref-arch-1", false)]
        #[case::with_domain_extension_and_lang_tag("ssvc//.example.organization#ref-arch-1/de-DE", false)]
        #[case::with_translation_with_fragment("ssvc//.example.organization#ref-arch-1$de-DE", false)]
        #[case::with_translation_without_fragment("ssvc//.example.organization$de-DE", false)]
        fn test_round_trip(#[case] namespace_str: &str, #[case] allow_test: bool) {
            let parsed = if allow_test {
                ParsedNamespace::parse_allow_test(namespace_str)
            } else {
                ParsedNamespace::parse(namespace_str)
            }.expect("Failed to parse");
            
            let displayed = parsed.to_string();
            assert_eq!(displayed, namespace_str, "Display should match original");
            
            let reparsed = if allow_test {
                ParsedNamespace::parse_allow_test(&displayed)
            } else {
                ParsedNamespace::parse(&displayed)
            }.expect("Failed to reparse");
            
            assert_eq!(parsed, reparsed, "Reparsed should equal original");
        }
    }
}
