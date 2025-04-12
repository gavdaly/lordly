use super::InputSpec;
use crate::check::Check;
use leptos::*;

/// A URL, such as a home page or company website address as appropriate given the context of the other fields in the form.
struct Url;

impl InputSpec for Url {
    fn input_mode() -> &'static str {
        "url"
    }
    fn autocomplete() -> &'static str {
        "url"
    }
    fn aria_label() -> &'static str {
        "Url"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^https?://")
    }
    fn input_type() -> &'static str {
        "url"
    }
    fn maxlength() -> Option<u32> {
        Some(2048)
    }
    fn minlength() -> Option<u32> {
        Some(10)
    }
    fn validation() -> Option<Callback<String, Check<String>>> {
        None
    }
}
