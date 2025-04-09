use super::InputSpec;
use crate::check::Check;
use leptos::*;
use std::fmt;

/// Defines different components of a telephone number
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Telephone {
    /// A full telephone number, including the country code.
    Tel,
    /// The country code, such as "1" for the United States, Canada, and other areas in North America and parts of the Caribbean.
    TelCountryCode,
    /// The entire phone number without the country code component, including a country-internal prefix.
    /// For the phone number "1-855-555-6502", this field's value would be "855-555-6502".
    TelNational,
    /// A telephone extension code within the phone number, such as a room or suite number in a hotel or an office extension in a company.
    TelExtension,
}

impl fmt::Display for Telephone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tel => write!(f, "tel"),
            Self::TelCountryCode => write!(f, "tel-country-code"),
            Self::TelNational => write!(f, "tel-national"),
            Self::TelExtension => write!(f, "tel-extension"),
        }
    }
}

/// A full phone number, including country code.
pub struct Phone;

/// Implementation of `InputSpec` for full `Phone` type.
///
/// Provides specifications for telephone input fields:
/// - Uses "tel" input type and autocomplete
/// - Sets appropriate aria-label as "Phone number"
/// - Configures telephone-specific inputmode
/// - Validates phone numbers using a regex pattern
impl InputSpec for Phone {
    fn input_type() -> &'static str {
        "tel"
    }
    fn autocomplete() -> &'static str {
        "tel"
    }
    fn aria_label() -> &'static str {
        "Phone number"
    }
    fn input_mode() -> &'static str {
        "tel"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^(?:\+\d\s*)?(?:\(?\d{3}\)?)(?:[- ]?\d{3})(?:[- ]?\d{4})$")
    }
    fn maxlength() -> Option<u32> {
        Some(20) // Increased to accommodate international formats
    }
    fn minlength() -> Option<u32> {
        Some(10) // Minimum for most standard formats
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}

/// A country code component of a phone number (e.g., "1" for US/Canada).
pub struct TelCountryCode;

/// Implementation of `InputSpec` for telephone country code.
impl InputSpec for TelCountryCode {
    fn input_type() -> &'static str {
        "tel"
    }
    fn autocomplete() -> &'static str {
        "tel-country-code"
    }
    fn aria_label() -> &'static str {
        "Country code"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^\+?\d{1,4}$")
    }
    fn maxlength() -> Option<u32> {
        Some(4) // Country codes are 1-4 digits
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}

/// The national part of a phone number without country code
pub struct TelNational;

/// Implementation of `InputSpec` for the national part of a telephone number.
impl InputSpec for TelNational {
    fn input_type() -> &'static str {
        "tel"
    }
    fn autocomplete() -> &'static str {
        "tel-national"
    }
    fn aria_label() -> &'static str {
        "Phone number (without country code)"
    }
    fn input_mode() -> &'static str {
        "tel"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^(?:\(?\d{3}\)?)(?:[- ]?\d{3})(?:[- ]?\d{4})$") // US format
    }
    fn maxlength() -> Option<u32> {
        Some(15) // Allow for formatting characters
    }
    fn minlength() -> Option<u32> {
        Some(7) // Minimum for standard formats without country code
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        Some(Callback::from(|value: String| {
            // This validation focuses on finding sufficient digits
            let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.len() >= 7 {
                Check::Valid
            } else {
                Check::Invalid("Please enter a valid phone number without country code".into())
            }
        }))
    }
}

/// A telephone extension
pub struct TelExtension;

/// Implementation of `InputSpec` for telephone extension.
impl InputSpec for TelExtension {
    fn input_type() -> &'static str {
        "tel"
    }
    fn autocomplete() -> &'static str {
        "tel-extension"
    }
    fn aria_label() -> &'static str {
        "Extension"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^\d{1,10}$")
    }
    fn maxlength() -> Option<u32> {
        Some(10) // Extensions are typically shorter
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}
