use super::InputSpec;
use crate::check::Check;
use leptos::prelude::*;

/// A job title or position within an organization.
struct OrganizationTitle;

/// Implementation of `InputSpec` for `OrganizationTitle` type.
///
/// Provides specifications for organization title input fields:
/// - Uses "text" input type
/// - Sets appropriate autocomplete and aria-label
/// - Sets reasonable length constraints
impl InputSpec for OrganizationTitle {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "organization-title"
    }
    fn aria_label() -> &'static str {
        "Job title"
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
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}
