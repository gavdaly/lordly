use leptos::attr::{any_attribute::{IntoAnyAttribute, AnyAttribute}, custom::custom_attribute, IntoAttributeValue};
use core::fmt::Display;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Color {
    #[default]
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

impl Color {
    pub fn as_str(&self) -> &'static str {
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

impl From<Color> for &'static str {
    fn from(val: Color) -> Self {
        val.as_str()
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
            _ => Color::Info,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IntoAnyAttribute for Color {
    fn into_any_attr(self) -> AnyAttribute {
        custom_attribute("data-color", self.as_str()).into_any_attr()
    }
}

impl IntoAttributeValue for Color {
    type Output = &'static str;
    fn into_attribute_value(self) -> Self::Output {
        self.as_str()
    }
}
