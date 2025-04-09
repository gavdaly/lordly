use super::InputSpec;
use crate::check::Check;
use leptos::*;

/// A name on a credit card.
struct CreditCardName;

/// Implementation of `InputSpec` for `CreditCardName` type.
///
/// Provides specifications for credit card name input fields:
/// - Uses "text" input type
/// - Sets appropriate autocomplete and aria-label
/// - Configures text inputmode
/// - Sets reasonable length constraints
impl InputSpec for CreditCardName {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "cc-name"
    }
    fn aria_label() -> &'static str {
        "Name on card"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(50)
    }
    fn minlength() -> Option<u32> {
        Some(2)
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}
