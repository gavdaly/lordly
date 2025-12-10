use alloc::string::String;

use super::InputSpec;
use crate::data_type::ValidationState;
use leptos::prelude::*;

/// A credit card number.
pub struct CreditCardNumber;

/// Implementation of `InputSpec` for `CreditCardNumber` type.
///
/// Provides specifications for credit card number input fields:
/// - Uses "text" input type with appropriate autocomplete
/// - Sets appropriate aria-label
/// - Configures numeric inputmode
/// - Validates card numbers using a pattern
/// - Sets standard length constraints
impl InputSpec for CreditCardNumber {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "cc-number"
    }
    fn aria_label() -> &'static str {
        "Credit card number"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^[0-9]{13,19}$")
    }
    fn maxlength() -> Option<u32> {
        Some(19)
    }
    fn minlength() -> Option<u32> {
        Some(13)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}
