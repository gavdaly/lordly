use super::InputSpec;
use crate::check::Check;
use leptos::*;

/// An organization or company name.
struct Organization;

/// Implementation of `InputSpec` for `Organization` type.
///
/// Provides specifications for organization name input fields:
/// - Uses "text" input type
/// - Sets appropriate autocomplete and aria-label
/// - Sets reasonable length constraints
impl InputSpec for Organization {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "organization"
    }
    fn aria_label() -> &'static str {
        "Organization name"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        Some(2)
    }
    fn validation() -> Option<Callback<String, Check<Organization>>> {
        None
    }
}
