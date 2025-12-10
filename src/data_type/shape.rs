use alloc::string::ToString;

use alloc::string::String;

use leptos::attr::{any_attribute::*, custom::*, *};
use core::fmt::Display;

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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IntoAnyAttribute for Shape {
    fn into_any_attr(self) -> AnyAttribute {
        custom_attribute("data-shape", self.as_str()).into_any_attr()
    }
}

impl IntoAttributeValue for Shape {
    type Output = String;
    fn into_attribute_value(self) -> Self::Output {
        self.to_string()
    }
}
