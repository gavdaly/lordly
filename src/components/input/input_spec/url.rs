use alloc::string::String;

use super::InputSpec;
use crate::data_type::ValidationState;
use leptos::prelude::*;

/// A URL, such as a home page or company website address as appropriate given the context of the other fields in the form.
pub struct Url;

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
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}
