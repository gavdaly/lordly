use super::InputSpec;
use crate::check::Check;
use leptos::*;

/// A language preference.
struct Language;

/// Implementation of `InputSpec` for `Language` type.
///
/// Provides specifications for language selection input fields:
/// - Uses "text" input type (often used with datalist)
/// - Sets appropriate autocomplete and aria-label
/// - No specific validation pattern
impl InputSpec for Language {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "language"
    }
    fn aria_label() -> &'static str {
        "Preferred language"
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
        None
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}
