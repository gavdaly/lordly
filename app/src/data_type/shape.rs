use leptos::{Attribute, IntoAttribute};
use std::fmt::Display;

pub enum Shape {
    Pill,
    Rounded,
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

impl Into<String> for Shape {
    fn into(self) -> String {
        self.as_str().to_string()
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

impl IntoAttribute for Shape {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.as_str().to_string().into())
    }
    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        Box::new(self).into_attribute()
    }
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Square
    }
}
