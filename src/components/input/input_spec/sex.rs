use super::InputSpec;
use crate::check::Check;
use leptos::*;

/// A gender identity (such as "Female", "Fa'afafine", "Hijra", "Male", "Nonbinary"), as freeform text without newlines.
struct Sex;

/// Implementation of `InputSpec` for `Sex` type.
///
/// Provides specifications for sex selection input fields:
/// - Uses "text" input type (often used with select/radio)
/// - Sets appropriate autocomplete and aria-label
impl InputSpec for Sex {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "sex"
    }
    fn aria_label() -> &'static str {
        "sex"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        None
    }
    fn minlength() -> Option<u32> {
        None
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}
