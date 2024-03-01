use leptos::{Attribute, IntoAttribute};
use std::fmt::Display;

pub enum Style {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

impl Style {
    fn as_str(&self) -> &str {
        match self {
            Style::Primary => "primary",
            Style::Secondary => "secondary",
            Style::Success => "success",
            Style::Danger => "danger",
            Style::Warning => "warning",
            Style::Info => "info",
        }
    }
}

impl Into<String> for Style {
    fn into(self) -> String {
        self.as_str().to_string()
    }
}

impl From<&str> for Style {
    fn from(s: &str) -> Self {
        match s {
            "primary" => Style::Primary,
            "secondary" => Style::Secondary,
            "success" => Style::Success,
            "danger" => Style::Danger,
            "warning" => Style::Warning,
            "info" => Style::Info,
            _ => Style::Info,
        }
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IntoAttribute for Style {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.as_str().to_string().into())
    }
    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        Box::new(self).into_attribute()
    }
}

impl Default for Style {
    fn default() -> Self {
        Style::Info
    }
}
