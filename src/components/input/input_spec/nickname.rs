use super::InputSpec;
use crate::check::Check;
use leptos::*;

/// A nickname, as a short name.
struct Nickname;

impl InputSpec for Nickname {
    fn autocomplete() -> &'static str {
        "nickname"
    }
    fn input_type() -> &'static str {
        "text"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn aria_label() -> &'static str {
        "Nickname"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        None
    }
    fn minlength() -> Option<u32> {
        Some(5)
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}
