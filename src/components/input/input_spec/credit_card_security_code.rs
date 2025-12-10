use alloc::string::String;

use super::InputSpec;
use crate::data_type::ValidationState;
use leptos::prelude::*;

/// A credit card security code (CVV/CVC).
pub struct CreditCardSecurityCode;

/// Implementation of `InputSpec` for `CreditCardSecurityCode` type.
///
/// Provides specifications for credit card security code input fields:
/// - Uses "text" input type
/// - Sets appropriate autocomplete and aria-label
/// - Configures numeric inputmode
/// - Validates security code format (3-4 digits)
impl InputSpec for CreditCardSecurityCode {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "cc-csc"
    }
    fn aria_label() -> &'static str {
        "Security code (CVV/CVC)"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^\d{3,4}$")
    }
    fn maxlength() -> Option<u32> {
        Some(4)
    }
    fn minlength() -> Option<u32> {
        Some(3)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}
