use super::InputSpec;
use crate::check::Check;
use leptos::prelude::*;

pub struct CurrentPassword;
pub struct NewPassword;
pub struct ConfirmPassword;
pub struct OneTimeCode;

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
    fn validation() -> Option<Callback<String, Check<String>>> {
        Some(Callback::new(|value: String| {
            let pattern = regex::Regex::new(
                r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$",
            )
            .unwrap();
            if pattern.is_match(&value) {
                Check::Valid
            } else {
                Check::Invalid("Invalid password".into())
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
    fn validation() -> Option<Callback<String, Check<String>>> {
        Some(Callback::new(|value: String| {
            let pattern = regex::Regex::new(
                r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$",
            )
            .unwrap();
            if pattern.is_match(&value) {
                Check::Valid
            } else {
                Check::Invalid("Invalid password".into())
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
    fn validation() -> Option<Callback<String, Check<String>>> {
        Some(Callback::new(|value: String| {
            let pattern = regex::Regex::new(
                r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$",
            )
            .unwrap();
            if pattern.is_match(&value) {
                Check::Valid
            } else {
                Check::Invalid("Invalid password".into())
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
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}
