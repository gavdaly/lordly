use alloc::string::String;

use super::InputSpec;
use crate::data_type::ValidationState;
use leptos::prelude::*;

/// A monetary transaction amount.
pub struct TransactionAmount;

/// Implementation of `InputSpec` for `TransactionAmount` type.
///
/// Provides specifications for monetary amount input fields:
/// - Uses "number" input type
/// - Sets appropriate aria-label
/// - Configures decimal inputmode
/// - Sets constraints for reasonable monetary values
impl InputSpec for TransactionAmount {
    fn input_type() -> &'static str {
        "number"
    }
    fn autocomplete() -> &'static str {
        "transaction-amount"
    }
    fn aria_label() -> &'static str {
        "Transaction amount"
    }
    fn input_mode() -> &'static str {
        "decimal"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^\d+(\.\d{1,2})?$")
    }
    fn maxlength() -> Option<u32> {
        Some(16)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}
