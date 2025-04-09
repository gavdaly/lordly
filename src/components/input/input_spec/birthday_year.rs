use super::InputSpec;
use crate::check::Check;
use leptos::*;

/// A birthday year component.
struct BirthdayYear;

/// Implementation of `InputSpec` for `BirthdayYear` type.
///
/// Provides specifications for birthday year input fields:
/// - Uses "number" input type
/// - Sets appropriate autocomplete and aria-label
/// - Configures numeric inputmode
/// - Validates year within reasonable range
impl InputSpec for BirthdayYear {
    fn input_type() -> &'static str {
        "number"
    }
    fn autocomplete() -> &'static str {
        "bday-year"
    }
    fn aria_label() -> &'static str {
        "Year of birth"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^\d{4}$")
    }
    fn maxlength() -> Option<u32> {
        Some(4)
    }
    fn minlength() -> Option<u32> {
        Some(4)
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        Some(Callback::from(|value: String| {
            if let Ok(year) = value.parse::<u16>() {
                let current_year = 2023; // This should ideally be dynamic
                if year >= 1900 && year <= current_year {
                    return Check::Valid;
                }
            }
            Check::Invalid("Year must be a valid 4-digit year".into())
        }))
    }
}
