use crate::namespaces::NamespaceError;
use crate::namespaces::constants::{
    FRAGMENT_DELIMITER, RESERVED_INVALID_NAMESPACES, RESERVED_TEST_NAMESPACES, UNREGISTERED_PREFIX,
};

/// The base namespace, either registered or unregistered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseNamespace {
    Registered {
        name: String,
        fragment: Option<String>,
    },
    Unregistered {
        reverse_domain: String,
        fragment: String,
    },
}

impl BaseNamespace {
    /// Dispatches parsing to either registered or unregistered base namespace parser
    /// based on whether the base starts with the unregistered prefix (`x_`).
    pub(super) fn parse_base(
        base: &str,
        allow_test: bool,
    ) -> Result<BaseNamespace, NamespaceError> {
        if base.starts_with(UNREGISTERED_PREFIX) {
            Self::parse_unregistered_base(base, allow_test)
        } else {
            Self::parse_registered_base(base, allow_test)
        }
    }

    /// Parses a registered base namespace.
    ///
    /// Validates against reserved namespaces and extracts any fragment delimited with `#'.
    ///
    /// # Arguments
    /// * `base` - The base namespace string to parse
    /// * `allow_test` - Whether reserved `test` namespaces are permitted
    ///
    /// # Returns
    /// A `BaseNamespace::Registered` variant, or a `NamespaceError` if validation fails
    fn parse_registered_base(
        base: &str,
        allow_test: bool,
    ) -> Result<BaseNamespace, NamespaceError> {
        // if there are fragments, separate them
        let (name, fragment) = if let Some((name, fragment)) = base.split_once(FRAGMENT_DELIMITER) {
            // fragment is present, check its not empty
            if fragment.is_empty() {
                return Err(NamespaceError::EmptyFragment);
            }
            (name, Some(fragment))
        } else {
            (base, None)
        };

        // check if base is not empty
        if name.is_empty() {
            return Err(NamespaceError::EmptyBase);
        }

        // reserved forbidden namespaces that must never be used
        if RESERVED_INVALID_NAMESPACES.contains(&name) {
            return Err(NamespaceError::ReservedForbiddenNamespace {
                namespace: name.to_string(),
            });
        }

        // reserved test namespaces that are only allowed if allow_test is true
        if RESERVED_TEST_NAMESPACES.contains(&name) && !allow_test {
            return Err(NamespaceError::ReservedTestNamespace {
                namespace: name.to_string(),
            });
        }

        Ok(BaseNamespace::Registered {
            name: name.to_string(),
            fragment: fragment.map(|f| f.to_string()),
        })
    }

    /// Parses an unregistered (prefixed with `x_`) base namespace.
    ///
    /// Unregistered namespaces must contain a fragment delimiter (`#`).
    /// Validates against reserved namespaces and extracts the reverse domain
    /// and fragment components.
    ///
    /// # Arguments
    /// * `base` - The unregistered base namespace string to parse
    /// * `allow_test` - Whether reserved test namespaces are permitted
    ///
    /// # Returns
    /// A `BaseNamespace::Unregistered` variant, or a `NamespaceError` if validation fails
    fn parse_unregistered_base(
        base: &str,
        allow_test: bool,
    ) -> Result<BaseNamespace, NamespaceError> {
        // unregistered namespaces need to contain a fragment (and therefore a fragment delimiter)
        let (prefixed_reverse_domain, fragment) = base
            .split_once(FRAGMENT_DELIMITER)
            .ok_or(NamespaceError::UnregisteredNamespaceMissingFragment)?;

        // reserved forbidden namespaces that must never be used
        if RESERVED_INVALID_NAMESPACES.contains(&prefixed_reverse_domain) {
            return Err(NamespaceError::ReservedForbiddenNamespace {
                namespace: prefixed_reverse_domain.to_string(),
            });
        }

        // reserved test namespaces that are only allowed if allow_test is true
        if RESERVED_TEST_NAMESPACES.contains(&prefixed_reverse_domain) && !allow_test {
            return Err(NamespaceError::ReservedTestNamespace {
                namespace: prefixed_reverse_domain.to_string(),
            });
        }

        // strip the "x_" prefix
        // unregistered namespaces must have a non-empty reverse domain
        let reverse_domain = &prefixed_reverse_domain[2..];
        if reverse_domain.is_empty() {
            return Err(NamespaceError::EmptyReverseDomain);
        }

        // unregistered namespaces must have a non-empty fragment
        if fragment.is_empty() {
            return Err(NamespaceError::EmptyFragment);
        }

        Ok(BaseNamespace::Unregistered {
            reverse_domain: reverse_domain.to_string(),
            fragment: fragment.to_string(),
        })
    }
}

#[cfg(test)]
mod test_registered {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_parse_registered_forbidden_namespace() {
        let result = BaseNamespace::parse_base("invalid", true);
        assert!(matches!(
            result,
            Err(NamespaceError::ReservedForbiddenNamespace { .. })
        ));
    }

    #[rstest]
    #[case("test", true, true)]
    #[case("test", false, false)]
    fn test_parse_registered_test_namespace(
        #[case] base: &str,
        #[case] allow_test: bool,
        #[case] should_succeed: bool,
    ) {
        let result = BaseNamespace::parse_base(base, allow_test);
        if should_succeed {
            assert!(matches!(result, Ok(BaseNamespace::Registered { .. })));
        } else {
            assert!(matches!(
                result,
                Err(NamespaceError::ReservedTestNamespace { .. })
            ));
        }
    }

    #[test]
    fn test_parse_registered_empty_base() {
        let result = BaseNamespace::parse_base("#some-fragment", true);
        println!("result: {:?}", result);
        assert!(matches!(result, Err(NamespaceError::EmptyBase)));
    }

    #[test]
    fn test_parse_registered_empty_fragment() {
        let result = BaseNamespace::parse_base("test#", true);
        assert!(matches!(result, Err(NamespaceError::EmptyFragment)));
    }

    #[test]
    fn test_parse_registered_with_fragment() {
        let result = BaseNamespace::parse_base("test#some-fragment", true);
        assert!(matches!(
            result,
            Ok(BaseNamespace::Registered {
                fragment: Some(_),
                ..
            })
        ));
    }

    #[test]
    fn test_parse_registered_without_fragment() {
        let result = BaseNamespace::parse_base("test", true);
        assert!(matches!(
            result,
            Ok(BaseNamespace::Registered { fragment: None, .. })
        ));
    }
}

#[cfg(test)]
mod test_unregistered {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_parse_unregistered_missing_fragment() {
        let result = BaseNamespace::parse_base("x_com.example", true);
        assert!(matches!(
            result,
            Err(NamespaceError::UnregisteredNamespaceMissingFragment)
        ));
    }

    #[test]
    fn test_parse_unregistered_forbidden_namespace() {
        let result = BaseNamespace::parse_base("x_invalid#fragment", true);
        assert!(matches!(
            result,
            Err(NamespaceError::ReservedForbiddenNamespace { .. })
        ));
    }

    #[rstest]
    #[case("x_test", true, true)]
    #[case("x_test", false, false)]
    fn test_parse_unregistered_test_namespace(
        #[case] base: &str,
        #[case] allow_test: bool,
        #[case] should_succeed: bool,
    ) {
        let test_base = format!("{}#fragment", base);
        let result = BaseNamespace::parse_base(&test_base, allow_test);
        if should_succeed {
            assert!(matches!(result, Ok(BaseNamespace::Unregistered { .. })));
        } else {
            assert!(matches!(
                result,
                Err(NamespaceError::ReservedTestNamespace { .. })
            ));
        }
    }

    #[test]
    fn test_parse_unregistered_empty_domain() {
        let result = BaseNamespace::parse_base("x_#fragment", true);
        assert!(matches!(result, Err(NamespaceError::EmptyReverseDomain)));
    }

    #[test]
    fn test_parse_unregistered_empty_fragment() {
        let result = BaseNamespace::parse_base("x_com.example#", true);
        assert!(matches!(result, Err(NamespaceError::EmptyFragment)));
    }

    #[test]
    fn test_parse_unregistered_with_fragment() {
        let result = BaseNamespace::parse_base("x_com.example#my-fragment", true);
        assert!(matches!(result, Ok(BaseNamespace::Unregistered { .. })));
    }
}
