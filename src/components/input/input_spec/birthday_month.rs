use super::InputSpec;
use crate::check::Check;
use leptos::prelude::*;

/// A birthday month component.
struct BirthdayMonth;

/// Implementation of `InputSpec` for `BirthdayMonth` type.
///
/// Provides specifications for birthday month input fields:
/// - Uses "number" input type
/// - Sets appropriate autocomplete and aria-label
/// - Configures numeric inputmode
/// - Validates months (1-12)
impl InputSpec for BirthdayMonth {
    fn input_type() -> &'static str {
        "number"
    }
    fn autocomplete() -> &'static str {
        "bday-month"
    }
    fn aria_label() -> &'static str {
        "Month of birth"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^([1-9]|1[0-2])$")
    }
    fn maxlength() -> Option<u32> {
        Some(2)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        Some(Callback::new(|value: String| {
            if let Ok(month) = value.parse::<u8>() {
                if month >= 1 && month <= 12 {
                    return Check::Valid;
                }
            }
            Check::Invalid("Month must be between 1-12".into())
        }))
    }
}
