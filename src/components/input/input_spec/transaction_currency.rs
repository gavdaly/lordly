use alloc::string::String;

use super::InputSpec;
use crate::data_type::ValidationState;
use leptos::prelude::*;

/// A currency selection for transactions.
pub struct TransactionCurrency;

/// Implementation of `InputSpec` for `TransactionCurrency` type.
///
/// Provides specifications for transaction currency input fields:
/// - Uses "text" input type (often used with select)
/// - Sets appropriate aria-label
impl InputSpec for TransactionCurrency {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "transaction-currency"
    }
    fn aria_label() -> &'static str {
        "Currency"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(3)
    }
    fn minlength() -> Option<u32> {
        Some(3)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}
