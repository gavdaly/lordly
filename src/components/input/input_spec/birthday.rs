use super::InputSpec;
use crate::check::Check;
use leptos::*;

/// A complete birth date.
struct Birthday;

/// Implementation of `InputSpec` for `Birthday` type.
///
/// Provides specifications for birthday input fields:
/// - Uses "date" input type
/// - Sets appropriate autocomplete and aria-label
/// - No specific pattern as date input handles format
impl InputSpec for Birthday {
    fn input_type() -> &'static str {
        "date"
    }
    fn autocomplete() -> &'static str {
        "bday"
    }
    fn aria_label() -> &'static str {
        "Birth date"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        None // Date input provides its own format
    }
    fn maxlength() -> Option<u32> {
        None // Date input handles this
    }
    fn minlength() -> Option<u32> {
        None // Date input handles this
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        Some(Callback::from(|_value: String| {
            if true {
                Check::Valid
            } else {
                Check::Invalid("Invalid birth date".into())
            }
        }))
    }
}
