use crate::NamespaceError;

pub type Extensions = Vec<Extension>;

pub(crate) fn parse_extensions(parts: &[&str]) -> Result<Option<Extensions>, NamespaceError> {
    let mut extensions: Option<Extensions> = None;

    for (idx, part) in parts.iter().enumerate() {
        if idx == 0 {
            // First extension: must be a language tag or empty string,
            if part.is_empty() {
                // empty implies default language (en-US)
                extensions
                    .get_or_insert_default()
                    .push(Extension::EmptyDefaultLanguage);
            } else {
                extensions
                    .get_or_insert_default()
                    .push(Extension::parse_language_only_segment(part)?);
            }
        } else {
            extensions
                .get_or_insert_default()
                .push(Extension::parse_segment(part)?);
        }
    }

    Ok(extensions)
}

/// A namespace extension segment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extension {
    /// Represents the "default" language (en-US) when no language tag is provided at the first index.
    EmptyDefaultLanguage,
    /// A BCP-47 language tag.
    Language(String),
    /// A reverse domain extension with a fragment.
    Domain {
        reverse_domain: String,
        fragment: String,
    },
    /// A translation extension with a reverse domain, optional fragment, and language tag.
    Translation {
        reverse_domain: String,
        fragment: Option<String>,
        language: String,
    },
}

impl Extension {
    /// Parses an extension segment into the appropriate extension enum kind.
    ///
    /// This method determines the type of extension based on the segment format:
    /// - No leading dot: Treated as a BCP-47 language tag (e.g., "en-US", "fr-FR")
    /// - Leading dot without "$" delimiter: Domain extension (e.g., ".com.example#fragment")
    /// - Leading dot with "$" delimiter: Translation extension (e.g., ".com.example$en-US", ".com.example#fragment$en-US")
    ///
    /// # Arguments
    /// * `segment` - The extension segment string to parse
    ///
    /// # Returns
    /// * `Ok(Extension)` - The parsed extension (Language, Domain, or Translation)
    /// * `Err(NamespaceError)` - If the segment is invalid or malformed
    fn parse_segment(segment: &str) -> Result<Extension, NamespaceError> {
        // segments not starting with "." are Language only Segments, i.e. just a BCP-47 tags
        let Some(segment) = segment.strip_prefix('.') else {
            return Self::parse_language_only_segment(segment);
        };

        if let Some((domain_part, language)) = segment.split_once('$') {
            // Translation segment: .domain#fragment$lang or .domain$lang
            Self::parse_translation(domain_part, language)
        } else {
            // Domain extension segment: .domain#fragment
            Self::parse_domain_extension(segment)
        }
    }

    /// Parses a "Language Only" segment by validating the BCP-47 language tag.
    ///
    /// # Arguments
    /// * `segment` - The language tag string to parse and validate
    ///
    /// # Returns
    /// * `Ok(Extension::Language)` if the tag is valid
    /// * `Err(NamespaceError)` if the tag is empty or invalid
    fn parse_language_only_segment(segment: &str) -> Result<Extension, NamespaceError> {
        Self::validate_language_tag(segment)?;
        Ok(Extension::Language(segment.to_string()))
    }

    /// Parses a "Extension" segment (format: .reverse.domain#fragment`).
    ///
    /// This function assumes input from [Self::parse_segment], which
    /// a) validates that the segment started with "." and
    /// b) strips the leading "."
    ///
    /// # Arguments
    /// * `segment` - The domain extension string (without the leading ".")
    ///
    /// # Returns
    /// * `Ok(Extension::Domain)` with the parsed reverse domain and optional fragment
    /// * `Err(NamespaceError)` if the domain is empty or fragment is missing or empty
    fn parse_domain_extension(segment: &str) -> Result<Extension, NamespaceError> {
        // check if the fragment delimiter is present
        if let Some((reverse_domain, fragment)) = segment.split_once('#') {
            // check if reverse domain or fragment are empty
            if reverse_domain.is_empty() {
                return Err(NamespaceError::EmptyReverseDomain);
            }
            if fragment.is_empty() {
                return Err(NamespaceError::EmptyFragment);
            }

            Ok(Extension::Domain {
                reverse_domain: reverse_domain.to_string(),
                fragment: fragment.to_string(),
            })
        } else {
            Err(NamespaceError::ExtensionSegmentMissingFragment)
        }
    }

    /// Parses a translation extension segment combining domain (with optional fragment) and language (format: .reverse.domain$language` or `.reverse.domain#fragment$language`)
    ///
    /// This function assumes input from [Self::parse_segment], which
    /// a) validates that the segment started with "." and
    /// b) strips the leading "." and
    /// c) validates that the segment contains the "$" delimiter
    /// d) splits the segment into domain and language parts using the "$" delimiter
    ///
    /// # Arguments
    /// * `domain_part` - The domain part of the extension (may include a fragment)
    /// * `language` - The BCP-47 language tag for the translation
    ///
    /// # Returns
    /// * `Ok(Extension::Translation)` with the parsed domain, fragment, and language
    /// * `Err(NamespaceError)` if any component is invalid or empty
    fn parse_translation(domain_part: &str, language: &str) -> Result<Extension, NamespaceError> {
        // validate the language tag
        Self::validate_language_tag(language)?;

        // check if the domain part contains a fragment
        if let Some((reverse_domain, fragment)) = domain_part.split_once('#') {
            // check if reverse domain or fragment are empty
            if reverse_domain.is_empty() {
                return Err(NamespaceError::EmptyReverseDomain);
            }
            if fragment.is_empty() {
                return Err(NamespaceError::EmptyFragment);
            }

            Ok(Extension::Translation {
                reverse_domain: reverse_domain.to_string(),
                fragment: Some(fragment.to_string()),
                language: language.to_string(),
            })
        } else {
            // check if reverse domain is empty
            if domain_part.is_empty() {
                return Err(NamespaceError::EmptyReverseDomain);
            }

            Ok(Extension::Translation {
                reverse_domain: domain_part.to_string(),
                fragment: None,
                language: language.to_string(),
            })
        }
    }

    /// Mock impl until lang tag validation has been moved into a separate crate from csaf-rs
    /// This will be removed in the future
    /// TODO: Fix this after lang tag lib extraction
    pub(crate) fn validate_language_tag(tag: &str) -> Result<(), NamespaceError> {
        if tag.is_empty() {
            return Err(NamespaceError::EmptyLanguageTag);
        }

        Ok(())
    }

    pub fn is_language(&self) -> bool {
        matches!(self, Extension::Language(..) | Extension::EmptyDefaultLanguage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_extensions {
        use super::*;

        #[test]
        fn no_extension() {
            // No extensions provided (this should not happen, as the caller ensures
            // that there are extensions to be parsed.
            // But even if that pre-check is removed, the parser should not return an empty array,
            // which would indicate that the first extensions was empty, i.e. the default langauge.
            let result = parse_extensions(&[]);
            assert_eq!(result, Ok(None));
        }

        #[test]
        fn only_empty_extension() {
            // One empty object (represents default language)
            let result = parse_extensions(&[""]);
            assert_eq!(result, Ok(Some(vec![Extension::EmptyDefaultLanguage])));
        }

        #[test]
        fn first_extension_with_language_tag() {
            // First extension with fr-FR as language tag
            let result = parse_extensions(&["fr-FR"]);
            assert_eq!(
                result,
                Ok(Some(vec![Extension::Language("fr-FR".to_string())]))
            );
        }

        #[test]
        fn empty_first_extension_with_domain_extension() {
            // Empty first extension (default language) followed by a domain extension
            let result = parse_extensions(&["", ".com.example#fragment"]);
            assert_eq!(
                result,
                Ok(Some(vec![
                    Extension::EmptyDefaultLanguage,
                    Extension::Domain {
                        reverse_domain: "com.example".to_string(),
                        fragment: "fragment".to_string(),
                    }
                ]))
            );
        }

        #[test]
        fn language_domain_and_translation() {
            // fr-FR language, a domain extension, and a translation
            let result = parse_extensions(&["fr-FR", ".org.example#fragment", ".com.vendor$de-DE"]);
            assert_eq!(
                result,
                Ok(Some(vec![
                    Extension::Language("fr-FR".to_string()),
                    Extension::Domain {
                        reverse_domain: "org.example".to_string(),
                        fragment: "fragment".to_string(),
                    },
                    Extension::Translation {
                        reverse_domain: "com.vendor".to_string(),
                        fragment: None,
                        language: "de-DE".to_string(),
                    },
                ]))
            );
        }

        // TODO: Missing tests for a non-language tag being supplied as first, tag
        // right now, everything passes, including other "kinds" of extension
    }

    // Tests for parse_domain_extension via parse_segment
    mod parse_domain_extension {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case(".com.example#fragment", "com.example", "fragment")]
        // This is questionable behavior
        #[case(".com.example#frag#extra", "com.example", "frag#extra")]
        fn valid_domain_extensions(
            #[case] input: &str,
            #[case] expected_domain: &str,
            #[case] expected_fragment: &str,
        ) {
            let result = Extension::parse_segment(input);
            assert_eq!(
                result,
                Ok(Extension::Domain {
                    reverse_domain: expected_domain.to_string(),
                    fragment: expected_fragment.to_string(),
                })
            );
        }

        #[rstest]
        #[case::fragment_missing(".com.example", NamespaceError::ExtensionSegmentMissingFragment)]
        #[case::reverse_domain_empty(".#fragment", NamespaceError::EmptyReverseDomain)]
        #[case::fragment_empty(".com.example#", NamespaceError::EmptyFragment)]
        #[case::both_empty(".#", NamespaceError::EmptyReverseDomain)]
        // This is questionable behavior
        #[case::both_empty(".", NamespaceError::ExtensionSegmentMissingFragment)]
        fn error_cases(#[case] input: &str, #[case] expected_error: NamespaceError) {
            let result = Extension::parse_segment(input);
            assert_eq!(result, Err(expected_error));
        }
    }

    // Tests for parse_translation via parse_segment
    mod parse_translation {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case(
            ".com.example#fragment$de-DE",
            "com.example",
            Some("fragment"),
            "de-DE"
        )]
        #[case(".com.example$en-US", "com.example", None, "en-US")]
        // This is questionable behavior
        #[case(
            ".com.example#frag#extra$en-US",
            "com.example",
            Some("frag#extra"),
            "en-US"
        )]
        fn valid_translation_segments(
            #[case] input: &str,
            #[case] expected_domain: &str,
            #[case] expected_fragment: Option<&str>,
            #[case] expected_language: &str,
        ) {
            let result = Extension::parse_segment(input);
            assert_eq!(
                result,
                Ok(Extension::Translation {
                    reverse_domain: expected_domain.to_string(),
                    fragment: expected_fragment.map(|s| s.to_string()),
                    language: expected_language.to_string(),
                })
            );
        }

        #[rstest]
        #[case::empty_language(".com.example$", NamespaceError::EmptyLanguageTag)]
        #[case::empty_domain(".#frag$en", NamespaceError::EmptyReverseDomain)]
        #[case::empty_fragment(".com.example#$en", NamespaceError::EmptyFragment)]
        // This is questionable behavior
        #[case::empty_domain_empty_fragment(".#$en", NamespaceError::EmptyReverseDomain)]
        #[case::empty_domain_empty_language(".$", NamespaceError::EmptyLanguageTag)]
        #[case::empty_domain_empty_fragment_empty_language(".#$", NamespaceError::EmptyLanguageTag)]
        fn error_cases(#[case] input: &str, #[case] expected_error: NamespaceError) {
            let result = Extension::parse_segment(input);
            assert_eq!(result, Err(expected_error));
        }
    }
}
