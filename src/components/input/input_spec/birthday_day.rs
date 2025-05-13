use super::InputSpec;
use crate::check::Check;
use leptos::prelude::*;

/// A birthday day component.
struct BirthdayDay;

/// Implementation of `InputSpec` for `BirthdayDay` type.
///
/// Provides specifications for birthday day input fields:
/// - Uses "number" input type
/// - Sets appropriate autocomplete and aria-label
/// - Configures numeric inputmode
/// - Validates days (1-31)
impl InputSpec for BirthdayDay {
    fn input_type() -> &'static str {
        "number"
    }
    fn autocomplete() -> &'static str {
        "bday-day"
    }
    fn aria_label() -> &'static str {
        "Day of birth"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^([1-9]|[12]\d|3[01])$")
    }
    fn maxlength() -> Option<u32> {
        Some(2)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        Some(Callback::new(|value: String| {
            if let Ok(day) = value.parse::<u8>() {
                if day >= 1 && day <= 31 {
                    return Check::Valid;
                }
            }
            Check::Invalid("Day must be between 1-31".into())
        }))
    }
}
