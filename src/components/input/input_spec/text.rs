use super::InputSpec;
use crate::check::Check;
use leptos::*;

/// A generic text input.
struct Text;

/// Implementation of `InputSpec` for `Text` type.
///
/// Provides specifications for generic text input fields:
/// - Uses "text" input type
/// - Sets appropriate aria-label
/// - Sets reasonable length constraints
impl InputSpec for Text {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "off"
    }
    fn aria_label() -> &'static str {
        "Text"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(255)
    }
    fn minlength() -> Option<u32> {
        None
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        Some(Callback::from(|_value| {
            // Basic validation - could be customized per use case
            Check::Valid
        }))
    }
}
