use alloc::vec::Vec;

use alloc::string::String;

use super::InputSpec;
use crate::data_type::ValidationState;
use leptos::prelude::*;

/// A credit card expiration date.
pub struct CreditCardExpiration;

/// Implementation of `InputSpec` for `CreditCardExpiration` type.
///
/// Provides specifications for credit card expiration input fields:
/// - Uses "text" input type
/// - Sets appropriate autocomplete and aria-label
/// - Configures numeric inputmode
/// - Validates MM/YY format
impl InputSpec for CreditCardExpiration {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "cc-exp"
    }
    fn aria_label() -> &'static str {
        "Credit card expiration date"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^(0[1-9]|1[0-2])\/([0-9]{2})$")
    }
    fn maxlength() -> Option<u32> {
        Some(5)
    }
    fn minlength() -> Option<u32> {
        Some(5)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        Some(Callback::new(|value: String| {
            // Basic MM/YY validation - more complex validation would check against current date
            if value.len() == 5 && value.chars().nth(2) == Some('/') {
                let parts: Vec<&str> = value.split('/').collect();
                if parts.len() == 2 {
                    if let (Ok(month), Ok(_year)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>())
                    {
                        if month >= 1 && month <= 12 {
                            return ValidationState::Valid;
                        }
                    }
                }
            }
            ValidationState::Invalid("Use MM/YY format".into())
        }))
    }
}
