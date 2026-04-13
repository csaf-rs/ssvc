//! Namespace validation according to SSVC namespace specification.
//!
//! This module provides functionality to parse and validate SSVC namespaces
//! according to the structure defined in the documentation.

/// Reserved namespace strings that must not be used in production.
const RESERVED_REGISTERED: &[&str] = &["example", "invalid"];
const RESERVED_UNREGISTERED: &[&str] = &["x_example", "x_invalid"];

/// Represents the components of a parsed namespace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedNamespace {
    pub base: BaseNamespace,
    pub extensions: Vec<Extension>,
}

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

/// A namespace extension segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extension {
    /// A BCP-47 language tag.
    Language(String),
    /// A reverse domain extension with an optional fragment.
    Domain {
        reverse_domain: String,
        fragment: Option<String>,
    },
    /// An unofficial translation.
    Translation {
        reverse_domain: String,
        fragment: Option<String>,
        language: String,
    },
}

impl ParsedNamespace {
    /// Parse a namespace string into its components, permitting "test" and "x_test" namespaces.
    pub fn parse_allow_test(namespace: &str) -> Result<Self, String> {
        Self::parse_internal(namespace, true)
    }

    /// Parse a namespace string into its components for "production" use.
    pub fn parse(namespace: &str) -> Result<Self, String> {
        Self::parse_internal(namespace, false)
    }

    pub fn parse_internal(namespace: &str, allow_test: bool) -> Result<Self, String> {
        if namespace.len() < 3 || namespace.len() > 1000 {
            return Err(format!(
                "Namespace length must be between 3 and 1000 characters, got {}",
                namespace.len()
            ));
        }

        let parts: Vec<&str> = namespace.split('/').collect();
        if parts.is_empty() {
            return Err("Namespace cannot be empty".to_string());
        }

        let base = Self::parse_base(parts[0], allow_test)?;
        let extensions = if parts.len() > 1 {
            Self::parse_extensions(&parts[1..])?
        } else {
            Vec::new()
        };

        Ok(ParsedNamespace { base, extensions })
    }

    fn parse_base(base: &str, allow_test: bool) -> Result<BaseNamespace, String> {
        if base.starts_with("x_") {
            Self::parse_unregistered_base(base, allow_test)
        } else {
            Self::parse_registered_base(base, allow_test)
        }
    }

    fn parse_registered_base(base: &str, allow_test: bool) -> Result<BaseNamespace, String> {
        if RESERVED_REGISTERED.contains(&base) {
            return Err(format!("Reserved namespace '{}' must not be used", base));
        }
        if !allow_test && base == "test" {
            return Err("Reserved namespace 'test' must not be used".to_string());
        }

        if let Some(hash_pos) = base.find('#') {
            let name = base[..hash_pos].to_string();
            let fragment = base[hash_pos + 1..].to_string();

            if fragment.is_empty() {
                return Err("Fragment cannot be empty after '#'".to_string());
            }

            Ok(BaseNamespace::Registered {
                name,
                fragment: Some(fragment),
            })
        } else {
            Ok(BaseNamespace::Registered {
                name: base.to_string(),
                fragment: None,
            })
        }
    }

    fn parse_unregistered_base(base: &str, allow_test: bool) -> Result<BaseNamespace, String> {
        let hash_pos = base.find('#').ok_or_else(|| {
            "Unregistered namespace must contain a fragment (format: x_domain#fragment)".to_string()
        })?;

        let reverse_domain = base[2..hash_pos].to_string();
        let fragment = base[hash_pos + 1..].to_string();

        if reverse_domain.is_empty() {
            return Err(
                "Reverse domain name cannot be empty in unregistered namespace".to_string(),
            );
        }
        if fragment.is_empty() {
            return Err("Fragment is required in unregistered namespace".to_string());
        }

        if !allow_test && reverse_domain == "test" {
            return Err("Reserved namespace 'x_test' must not be used".to_string());
        }
        // Check for reserved unregistered namespaces (check full x_domain, not just prefix)
        let full_unregistered = format!("x_{}", reverse_domain);
        for reserved in RESERVED_UNREGISTERED {
            if full_unregistered == *reserved {
                return Err(format!(
                    "Reserved unregistered namespace '{}' must not be used",
                    reserved
                ));
            }
        }

        Ok(BaseNamespace::Unregistered {
            reverse_domain,
            fragment,
        })
    }

    fn parse_extensions(parts: &[&str]) -> Result<Vec<Extension>, String> {
        let mut extensions = Vec::new();

        for (idx, part) in parts.iter().enumerate() {
            if idx == 0 {
                // First extension: must be a language tag or empty string
                if part.is_empty() {
                    // Empty string implies default language (en-US)
                    continue;
                } else {
                    // TODO: Validate BCP-47 language tag
                    // For now, we accept any non-empty string as a language tag
                    extensions.push(Extension::Language(part.to_string()));
                }
            } else {
                extensions.push(Self::parse_extension_segment(part)?);
            }
        }

        Ok(extensions)
    }

    fn parse_extension_segment(segment: &str) -> Result<Extension, String> {
        if segment.contains('$') {
            // Translation segment: .domain#fragment$lang or .domain$lang
            Self::parse_translation(segment)
        } else if segment.starts_with('.') {
            // Domain extension segment
            Self::parse_domain_extension(segment)
        } else {
            // Language tag
            // TODO: Validate BCP-47 language tag
            Ok(Extension::Language(segment.to_string()))
        }
    }

    fn parse_domain_extension(segment: &str) -> Result<Extension, String> {
        if !segment.starts_with('.') {
            return Err("Domain extension must start with '.'".to_string());
        }

        let content = &segment[1..];
        if let Some(hash_pos) = content.find('#') {
            let reverse_domain = content[..hash_pos].to_string();
            let fragment = content[hash_pos + 1..].to_string();

            if reverse_domain.is_empty() {
                return Err("Reverse domain cannot be empty in extension".to_string());
            }
            if fragment.is_empty() {
                return Err("Fragment cannot be empty after '#' in extension".to_string());
            }

            Ok(Extension::Domain {
                reverse_domain,
                fragment: Some(fragment),
            })
        } else {
            if content.is_empty() {
                return Err("Domain extension cannot be empty".to_string());
            }
            Ok(Extension::Domain {
                reverse_domain: content.to_string(),
                fragment: None,
            })
        }
    }

    fn parse_translation(segment: &str) -> Result<Extension, String> {
        if !segment.starts_with('.') {
            return Err("Translation segment must start with '.'".to_string());
        }

        let content = &segment[1..];
        let dollar_pos = content.find('$').ok_or_else(|| {
            "Translation segment must contain '$' before language tag".to_string()
        })?;

        let domain_part = &content[..dollar_pos];

        // TODO: Validate BCP-47 language tag
        let language = content[dollar_pos + 1..].to_string();
        if language.is_empty() {
            return Err("Language tag cannot be empty in translation".to_string());
        }

        if let Some(hash_pos) = domain_part.find('#') {
            let reverse_domain = domain_part[..hash_pos].to_string();
            let fragment = domain_part[hash_pos + 1..].to_string();

            if reverse_domain.is_empty() {
                return Err("Reverse domain cannot be empty in translation".to_string());
            }
            if fragment.is_empty() {
                return Err("Fragment cannot be empty after '#' in translation".to_string());
            }

            Ok(Extension::Translation {
                reverse_domain,
                fragment: Some(fragment),
                language,
            })
        } else {
            if domain_part.is_empty() {
                return Err("Domain part cannot be empty in translation".to_string());
            }
            Ok(Extension::Translation {
                reverse_domain: domain_part.to_string(),
                fragment: None,
                language,
            })
        }
    }

    /// Get the base namespace name (without fragment).
    pub fn base_name(&self) -> &str {
        match &self.base {
            BaseNamespace::Registered { name, .. } => name,
            BaseNamespace::Unregistered { reverse_domain, .. } => reverse_domain,
        }
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
) -> Result<ParsedNamespace, String> {
    if allow_test_namespaces {
        ParsedNamespace::parse_allow_test(namespace)
    } else {
        ParsedNamespace::parse(namespace)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_registered_namespace() {
        let result = validate_namespace("ssvc", false);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert!(parsed.is_registered());
        assert_eq!(parsed.base_name(), "ssvc");
    }

    #[test]
    fn test_registered_namespace_with_fragment() {
        let result = validate_namespace("nist#800-30", false);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert!(parsed.is_registered());
        match parsed.base {
            BaseNamespace::Registered { name, fragment } => {
                assert_eq!(name, "nist");
                assert_eq!(fragment, Some("800-30".to_string()));
            }
            _ => panic!("Expected registered namespace"),
        }
    }

    #[test]
    fn test_unregistered_namespace() {
        let result = validate_namespace("x_example.test#test", false);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert!(parsed.is_unregistered());
    }

    #[test]
    fn test_unregistered_namespace_missing_fragment() {
        let result = validate_namespace("x_example.test", false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("fragment"));
    }

    #[test]
    fn test_reserved_registered_namespace() {
        assert!(validate_namespace("example", false).is_err());
        assert!(validate_namespace("test", false).is_err());
        assert!(validate_namespace("invalid", false).is_err());
    }

    #[test]
    fn test_reserved_unregistered_namespace() {
        assert!(validate_namespace("x_example#test", false).is_err());
        assert!(validate_namespace("x_test#foo", false).is_err());
        assert!(validate_namespace("x_invalid#bar", false).is_err());
    }

    #[test]
    fn test_namespace_with_language_extension() {
        let result = validate_namespace("ssvc/de-DE", false);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.extensions.len(), 1);
        match &parsed.extensions[0] {
            Extension::Language(lang) => assert_eq!(lang, "de-DE"),
            _ => panic!("Expected language extension"),
        }
    }

    #[test]
    fn test_namespace_with_empty_first_extension() {
        let result = validate_namespace("ssvc//.example.org#ref", false);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.extensions.len(), 1);
        match &parsed.extensions[0] {
            Extension::Domain {
                reverse_domain,
                fragment,
            } => {
                assert_eq!(reverse_domain, "example.org");
                assert_eq!(fragment, &Some("ref".to_string()));
            }
            _ => panic!("Expected domain extension"),
        }
    }

    #[test]
    fn test_complex_namespace_with_extensions() {
        let result = validate_namespace("ssvc/de-DE/.example.organization#ref-arch-1", false);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.extensions.len(), 2);
    }

    #[test]
    fn test_translation_extension() {
        let result = validate_namespace("ssvc//.example.isao#constituency/.example.isao$pl-PL", false);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.extensions.len(), 2);
        match &parsed.extensions[1] {
            Extension::Translation {
                reverse_domain,
                fragment,
                language,
            } => {
                assert_eq!(reverse_domain, "example.isao");
                assert_eq!(fragment, &None);
                assert_eq!(language, "pl-PL");
            }
            _ => panic!("Expected translation extension"),
        }
    }

    #[test]
    fn test_namespace_too_short() {
        let result = validate_namespace("ab", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_namespace_too_long() {
        let long_ns = "a".repeat(1001);
        let result = validate_namespace(&long_ns, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_example_from_docs() {
        // Examples from documentation
        assert!(validate_namespace("ssvc", false).is_ok());
        assert!(validate_namespace("cisa", false).is_ok());
        assert!(
            validate_namespace(
                "x_example.test#test//.example.test#private-extension",
                false
            )
            .is_ok()
        );
        assert!(
            validate_namespace("ssvc/de-DE/.example.organization#reference-arch-1", false).is_ok()
        );
    }

    #[test]
    fn test_cvss_namespace_example() {
        let result = validate_namespace("cvss", false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nist_with_fragment() {
        let result = validate_namespace("nist#800-30", false);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        match parsed.base {
            BaseNamespace::Registered { name, fragment } => {
                assert_eq!(name, "nist");
                assert_eq!(fragment.unwrap(), "800-30");
            }
            _ => panic!("Expected registered namespace"),
        }
    }

    #[test]
    fn test_multiple_extensions() {
        let result = validate_namespace("ssvc/de-DE/.example.org#ref1/.example.org#ref2", false);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.extensions.len(), 3);
    }

    #[test]
    fn test_empty_fragment_error() {
        let result = validate_namespace("ssvc#", false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_translation_without_domain() {
        let result = validate_namespace("ssvc//.$de-DE", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_domain_extension_without_dot() {
        let result = validate_namespace("ssvc//example.org", false);
        assert!(result.is_ok());
        // This is treated as a language tag, not a domain extension
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
