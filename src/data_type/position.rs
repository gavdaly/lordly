use leptos::attr::{any_attribute::{IntoAnyAttribute, AnyAttribute}, custom::custom_attribute, IntoAttributeValue};
use core::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Position {
    BottomCenter,
    BottomLeft,
    BottomRight,
    MiddleLeft,
    MiddleRight,
    #[default]
    TopCenter,
    TopLeft,
    TopRight,
}

impl Position {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::TopLeft => "top-left",
            Self::TopCenter => "top-center",
            Self::TopRight => "top-right",
            Self::MiddleLeft => "middle-left",
            Self::MiddleRight => "middle-right",
            Self::BottomLeft => "bottom-left",
            Self::BottomCenter => "bottom-center",
            Self::BottomRight => "bottom-right",
        }
    }
}

#[allow(clippy::fallible_impl_from, clippy::panic)]
impl From<&str> for Position {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "top-left" | "tl" | "nw" => Self::TopLeft,
            "top-right" | "tr" | "ne" => Self::TopRight,
            "middle-left" | "ml" | "l" | "w" => Self::MiddleLeft,
            "middle-right" | "mr" | "r" | "e" => Self::MiddleRight,
            "bottom-left" | "bl" | "sw" => Self::BottomLeft,
            "bottom-center" | "bc" | "s" => Self::BottomCenter,
            "bottom-right" | "br" | "se" => Self::BottomRight,
            _ => Self::TopCenter,
        }
    }
}

/// Converts a `Position` to a string.
impl Display for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IntoAnyAttribute for Position {
    fn into_any_attr(self) -> AnyAttribute {
        custom_attribute("data-position", self.as_str()).into_any_attr()
    }
}

impl IntoAttributeValue for Position {
    type Output = &'static str;
    fn into_attribute_value(self) -> Self::Output {
        self.as_str()
    }
}
