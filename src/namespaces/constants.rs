pub(crate) const UNREGISTERED_PREFIX: &str = "x_";
pub(crate) const EXTENSION_REVERSE_DOMAIN_PREFIX: &str = ".";
pub(crate) const FRAGMENT_DELIMITER: &str = "#";
pub(crate) const EXTENSION_TRANSLATION_PREFIX: &str = "$";
pub(crate) const SECTION_DELIMITER: &str = "/";

pub(crate) const RESERVED_INVALID_NAMESPACES: &[&str] = &["invalid", "x_invalid"];

pub(crate) const RESERVED_TEST_NAMESPACES: &[&str] = &["test", "x_test"];

pub(crate) const RESERVED_EXAMPLE_NAMESPACES: &[&str] = &["example", "x_example"];

/// All "registered" namespaces in the SSVC repository.
pub const REGISTERED_NAMESPACES: &[&str] =
    &["ssvc", "cvss", "cisa", "basic", "example", "test", "nist"];
