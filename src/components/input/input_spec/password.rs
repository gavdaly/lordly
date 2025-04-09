use super::InputSpec;
use crate::check::Check;
use leptos::*;

enum Password {
    Current,
    New,
    Confirm,
    OneTimeCode,
}

impl InputSpec for Password {
    fn input_type() -> &'static str {
        "password"
    }
    fn autocomplete() -> &'static str {
        match self {
            Password::Current => "current-password",
            Password::New => "new-password",
            Password::Confirm => "new-password",
            Password::OneTimeCode => "one-time-code",
        }
    }
    fn aria_label() -> &'static str {
        "Password"
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
        Some(Callback::from(|value: String| {
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
