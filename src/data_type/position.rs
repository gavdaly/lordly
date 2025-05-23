use leptos::{
    attr::{any_attribute::*, custom::*, *},
    prelude::*,
};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Position {
    TopLeft,
    #[default]
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Position {
    /// Returns the string representation of a `Position`.
    fn as_str(&self) -> &str {
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

/// Converts a `Position` to a string.
impl From<Position> for String {
    fn from(val: Position) -> Self {
        val.as_str().to_string()
    }
}

/// Converts a string to a `Position`.
impl From<&str> for Position {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "top-left" | "tl" | "nw" => Self::TopLeft,
            "top-center" | "tc" | "n" => Self::TopCenter,
            "top-right" | "tr" | "ne" => Self::TopRight,
            "middle-left" | "ml" | "l" | "w" => Self::MiddleLeft,
            "middle-right" | "mr" | "r" | "e" => Self::MiddleRight,
            "bottom-left" | "bl" | "sw" => Self::BottomLeft,
            "bottom-center" | "bc" | "s" => Self::BottomCenter,
            "bottom-right" | "br" | "se" => Self::BottomRight,
            _ => panic!("Invalid `Position`"),
        }
    }
}

/// Converts a `Position` to a string.
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IntoAnyAttribute for Position {
    fn into_any_attr(self) -> AnyAttribute {
        custom_attribute("data-position", self.as_str()).into_any_attr()
    }
}

impl IntoAttributeValue for Position {
    type Output = String;
    fn into_attribute_value(self) -> Self::Output {
        self.to_string()
    }
}
