//! Preset validation patterns for common use cases

use crate::validation::{ValidationOperator, ValidationBuilder};

/// Common validation presets
pub struct Presets;

impl Presets {
    /// Email validation with comprehensive rules
    pub fn email() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .max_length(254) // RFC 5321 limit
            .build()
    }

    /// URL validation (HTTP/HTTPS)
    pub fn url() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^https?://[^\s/$.?#].[^\s]*$")
            .max_length(2048) // Reasonable URL limit
            .build()
    }

    /// UUID validation (v4)
    pub fn uuid() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .exact_length(36)
            .matches(r"^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$")
            .build()
    }

    /// Phone number validation (international format)
    pub fn phone() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^\+?[1-9]\d{1,14}$")
            .min_length(7)
            .max_length(17)
            .build()
    }

    /// Strong password validation
    pub fn strong_password() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .min_length(8)
            .max_length(128)
            .matches(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]+$")
            .build()
    }

    /// Medium password validation (less strict)
    pub fn medium_password() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .min_length(6)
            .max_length(128)
            .matches(r"^(?=.*[a-zA-Z])(?=.*\d)[A-Za-z\d@$!%*?&]+$")
            .build()
    }

    /// Username validation
    pub fn username() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .min_length(3)
            .max_length(32)
            .matches(r"^[a-zA-Z0-9_-]+$")
            .not_in_list(vec!["admin", "root", "system", "null", "undefined", "test"])
            .build()
    }

    /// API key validation
    pub fn api_key(prefix: &str) -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .starts_with(prefix)
            .min_length(20)
            .max_length(255)
            .matches(r"^[A-Za-z0-9_-]+$")
            .build()
    }

    /// JWT token validation
    pub fn jwt_token() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^[A-Za-z0-9-_]+\.[A-Za-z0-9-_]+\.[A-Za-z0-9-_]*$")
            .min_length(50)
            .build()
    }

    /// Positive integer validation
    pub fn positive_integer() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .positive()
            .build()
    }

    /// Non-negative integer validation (including zero)
    pub fn non_negative_integer() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .greater_than_or_equal(0)
            .build()
    }

    /// Percentage validation (0-100)
    pub fn percentage() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .between(0, 100)
            .build()
    }

    /// Port number validation
    pub fn port_number() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .between(1, 65535)
            .build()
    }

    /// Color hex code validation
    pub fn hex_color() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$")
            .build()
    }

    /// Domain name validation
    pub fn domain_name() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)*[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?$")
            .max_length(253)
            .build()
    }

    /// IP address validation (IPv4)
    pub fn ipv4() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$")
            .build()
    }

    /// IP address validation (IPv6)
    pub fn ipv6() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$")
            .build()
    }

    /// Semantic version validation
    pub fn semver() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(-((0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(\.(0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(\+([0-9a-zA-Z-]+(\.[0-9a-zA-Z-]+)*))?$")
            .build()
    }

    /// Credit card number validation (Luhn algorithm pattern)
    pub fn credit_card() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^[0-9]{13,19}$")
            .build()
    }

    /// Social security number validation (US format)
    pub fn ssn() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^\d{3}-\d{2}-\d{4}$")
            .build()
    }

    /// Slug validation (URL-friendly string)
    pub fn slug() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .min_length(1)
            .max_length(100)
            .matches(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
            .build()
    }

    /// File path validation (Unix-style)
    pub fn file_path() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^(/[^/\0]+)+/?$")
            .max_length(4096)
            .build()
    }

    /// HTML tag validation
    pub fn html_tag() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^[a-zA-Z][a-zA-Z0-9]*$")
            .min_length(1)
            .max_length(20)
            .build()
    }

    /// ISO 8601 date validation
    pub fn iso_date() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^\d{4}-\d{2}-\d{2}$")
            .build()
    }

    /// ISO 8601 datetime validation
    pub fn iso_datetime() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d{3})?(?:Z|[+-]\d{2}:\d{2})$")
            .build()
    }

    /// Time validation (HH:MM format)
    pub fn time_hhmm() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^([01]?[0-9]|2[0-3]):[0-5][0-9]$")
            .build()
    }

    /// Base64 validation
    pub fn base64() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .matches(r"^[A-Za-z0-9+/]*={0,2}$")
            .build()
    }

    /// MongoDB ObjectId validation
    pub fn mongodb_objectid() -> ValidationOperator {
        ValidationBuilder::new()
            .required()
            .exact_length(24)
            .matches(r"^[0-9a-fA-F]{24}$")
            .build()
    }
}

// Convenience trait for easy access to presets
pub trait ValidationPresets {
    fn email() -> ValidationOperator { Presets::email() }
    fn url() -> ValidationOperator { Presets::url() }
    fn uuid() -> ValidationOperator { Presets::uuid() }
    fn phone() -> ValidationOperator { Presets::phone() }
    fn strong_password() -> ValidationOperator { Presets::strong_password() }
    fn medium_password() -> ValidationOperator { Presets::medium_password() }
    fn username() -> ValidationOperator { Presets::username() }
    fn api_key(prefix: &str) -> ValidationOperator { Presets::api_key(prefix) }
    fn jwt_token() -> ValidationOperator { Presets::jwt_token() }
    fn positive_integer() -> ValidationOperator { Presets::positive_integer() }
    fn non_negative_integer() -> ValidationOperator { Presets::non_negative_integer() }
    fn percentage() -> ValidationOperator { Presets::percentage() }
    fn port_number() -> ValidationOperator { Presets::port_number() }
    fn hex_color() -> ValidationOperator { Presets::hex_color() }
    fn domain_name() -> ValidationOperator { Presets::domain_name() }
    fn ipv4() -> ValidationOperator { Presets::ipv4() }
    fn ipv6() -> ValidationOperator { Presets::ipv6() }
    fn semver() -> ValidationOperator { Presets::semver() }
    fn credit_card() -> ValidationOperator { Presets::credit_card() }
    fn ssn() -> ValidationOperator { Presets::ssn() }
    fn slug() -> ValidationOperator { Presets::slug() }
    fn file_path() -> ValidationOperator { Presets::file_path() }
    fn html_tag() -> ValidationOperator { Presets::html_tag() }
    fn iso_date() -> ValidationOperator { Presets::iso_date() }
    fn iso_datetime() -> ValidationOperator { Presets::iso_datetime() }
    fn time_hhmm() -> ValidationOperator { Presets::time_hhmm() }
    fn base64() -> ValidationOperator { Presets::base64() }
    fn mongodb_objectid() -> ValidationOperator { Presets::mongodb_objectid() }
}

// Implement the trait for ValidationOperator to enable ValidationOperator::email()
impl ValidationPresets for ValidationOperator {}