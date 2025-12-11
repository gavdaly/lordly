use leptos::attr::{
    any_attribute::{AnyAttribute, IntoAnyAttribute},
    custom::custom_attribute,
    IntoAttributeValue,
};
use alloc::borrow::ToOwned as _;
use alloc::string::String;
use core::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Shape {
    Circular,
    Pill,
    Rounded,
    #[default]
    Square,
}

impl Shape {
    const fn as_str(&self) -> &'static str {
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
        val.as_str().to_owned()
    }
}

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        match s {
            "pill" => Self::Pill,
            "rounded" => Self::Rounded,
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
    type Output = &'static str;
    fn into_attribute_value(self) -> Self::Output {
        self.as_str()
    }
}
