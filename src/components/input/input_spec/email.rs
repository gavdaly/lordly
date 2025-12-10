use alloc::string::String;

use super::InputSpec;
use crate::data_type::ValidationState;
use leptos::prelude::*;

/// An email address.
pub struct Email;

/// Implementation of `InputSpec` for `Email` type.
///
/// Provides specifications for email input fields:
/// - Uses "email" input type and autocomplete
/// - Sets appropriate aria-label as "Email address"
/// - Configures email-specific inputmode
/// - Validates email addresses using a regex pattern
/// - Sets character length constraints (50 characters)
impl InputSpec for Email {
    fn input_type() -> &'static str {
        "email"
    }
    fn autocomplete() -> &'static str {
        "email"
    }
    fn aria_label() -> &'static str {
        "Email address"
    }
    fn input_mode() -> &'static str {
        "email"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
    }
    fn maxlength() -> Option<u32> {
        Some(50)
    }
    fn minlength() -> Option<u32> {
        Some(5)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        Some(Callback::new(|value: String| {
            if value.contains('@') {
                ValidationState::Valid
            } else {
                ValidationState::Invalid("Invalid email address".into())
            }
        }))
    }
}
