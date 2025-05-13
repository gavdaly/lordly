use leptos::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Shape {
    Pill,
    Rounded,
    #[default]
    Square,
    Circular,
}

impl Shape {
    fn as_str(&self) -> &str {
        match self {
            Self::Pill => "pill",
            Self::Rounded => "rounded",
            Self::Square => "square",
            Self::Circular => "circular",
        }
    }
}

impl From<Shape> for String {
    fn from(val: Shape) -> Self {
        val.as_str().to_string()
    }
}

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        match s {
            "pill" => Self::Pill,
            "rounded" => Self::Rounded,
            "square" => Self::Square,
            "circular" => Self::Circular,
            _ => Self::Square,
        }
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// impl IntoAnyAttribute for Shape {
//     fn into_any_attr(self) -> leptos::attr::any_attribute::AnyAttribute {
//         self.as_str()
//     }
// }
