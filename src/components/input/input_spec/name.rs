use super::InputSpec;
use crate::check::Check;
use leptos::prelude::*;

/// A name, as a full name.
struct Name;

impl InputSpec for Name {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "name"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn aria_label() -> &'static str {
        "Name"
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
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}
