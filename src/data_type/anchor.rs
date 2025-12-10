use alloc::string::ToString;

use alloc::string::String;

use leptos::attr::{any_attribute::*, custom::*, *};
use core::fmt::Display;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Anchor {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl Anchor {
    fn as_str(&self) -> &str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

impl From<Anchor> for String {
    fn from(val: Anchor) -> Self {
        val.as_str().to_string()
    }
}

impl From<&str> for Anchor {
    fn from(s: &str) -> Self {
        match s {
            "top" => Self::Top,
            "right" => Self::Right,
            "bottom" => Self::Bottom,
            "left" => Self::Left,
            _ => panic!("Invalid `Anchor`"),
        }
    }
}

impl Display for Anchor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IntoAnyAttribute for Anchor {
    fn into_any_attr(self) -> AnyAttribute {
        custom_attribute("data-anchor", self.as_str()).into_any_attr()
    }
}

impl IntoAttributeValue for Anchor {
    type Output = String;
    fn into_attribute_value(self) -> Self::Output {
        self.to_string()
    }
}
