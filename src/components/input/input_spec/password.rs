use alloc::string::String;

use super::InputSpec;
use crate::data_type::ValidationState;
use leptos::prelude::*;

pub struct CurrentPassword;
pub struct NewPassword;
pub struct ConfirmPassword;
pub struct OneTimeCode;

fn is_valid_password(val: &str) -> bool {
    let has_len = val.len() >= 8;
    let has_lower = val.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = val.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = val.chars().any(|c| c.is_ascii_digit());
    let has_special = val.chars().any(|c| "@$!%*?&".contains(c));
    let all_allowed = val.chars().all(|c| c.is_ascii_alphanumeric() || "@$!%*?&".contains(c));
    
    has_len && has_lower && has_upper && has_digit && has_special && all_allowed
}

impl InputSpec for CurrentPassword {
    fn input_type() -> &'static str {
        "password"
    }
    fn autocomplete() -> &'static str {
        "current-password"
    }
    fn aria_label() -> &'static str {
        "Current Password"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$")
    }
    fn maxlength() -> Option<u32> {
        None
    }
    fn minlength() -> Option<u32> {
        Some(8)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        Some(Callback::new(|value: String| {
            if is_valid_password(&value) {
                ValidationState::Valid
            } else {
                ValidationState::Invalid(String::from("Invalid password"))
            }
        }))
    }
}

impl InputSpec for NewPassword {
    fn input_type() -> &'static str {
        "password"
    }
    fn autocomplete() -> &'static str {
        "new-password"
    }
    fn aria_label() -> &'static str {
        "New Password"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$")
    }
    fn maxlength() -> Option<u32> {
        None
    }
    fn minlength() -> Option<u32> {
        Some(8)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        Some(Callback::new(|value: String| {
            if is_valid_password(&value) {
                ValidationState::Valid
            } else {
                ValidationState::Invalid(String::from("Invalid password"))
            }
        }))
    }
}

impl InputSpec for ConfirmPassword {
    fn input_type() -> &'static str {
        "password"
    }
    fn autocomplete() -> &'static str {
        "new-password"
    }
    fn aria_label() -> &'static str {
        "Confirm Password"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$")
    }
    fn maxlength() -> Option<u32> {
        None
    }
    fn minlength() -> Option<u32> {
        Some(8)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        Some(Callback::new(|value: String| {
            if is_valid_password(&value) {
                ValidationState::Valid
            } else {
                ValidationState::Invalid(String::from("Invalid password"))
            }
        }))
    }
}

impl InputSpec for OneTimeCode {
    fn input_type() -> &'static str {
        "password"
    }
    fn autocomplete() -> &'static str {
        "one-time-code"
    }
    fn aria_label() -> &'static str {
        "One-Time Code"
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
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}
