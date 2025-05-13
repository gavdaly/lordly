use leptos::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    #[default]
    Info,
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Color::Primary => "primary",
            Color::Secondary => "secondary",
            Color::Success => "success",
            Color::Danger => "danger",
            Color::Warning => "warning",
            Color::Info => "info",
        }
    }
}

impl From<Color> for String {
    fn from(val: Color) -> Self {
        val.as_str().to_string()
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s {
            "primary" => Color::Primary,
            "secondary" => Color::Secondary,
            "success" => Color::Success,
            "danger" => Color::Danger,
            "warning" => Color::Warning,
            "info" => Color::Info,
            _ => Color::Info,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// impl IntoAnyAttribute for Color {
//     fn into_any_attr(self) -> leptos::attr::any_attribute::AnyAttribute {
//         self.as_str()
//     }
// }
