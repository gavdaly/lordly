use leptos::attr::{any_attribute::{IntoAnyAttribute, AnyAttribute}, custom::custom_attribute, IntoAttributeValue};
use core::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Anchor {
    Bottom,
    Left,
    #[default]
    Right,
    Top,
}

impl Anchor {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

impl From<Anchor> for &'static str {
    fn from(val: Anchor) -> Self {
        val.as_str()
    }
}

impl From<&str> for Anchor {
    fn from(s: &str) -> Self {
        match s {
            "top" => Self::Top,
            "bottom" => Self::Bottom,
            "left" => Self::Left,
            _ => Self::Right,
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
    type Output = &'static str;
    fn into_attribute_value(self) -> Self::Output {
        self.as_str()
    }
}
