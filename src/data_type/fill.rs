use leptos::attr::{any_attribute::{IntoAnyAttribute, AnyAttribute}, custom::custom_attribute, IntoAttributeValue};
use core::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Fill {
    Ghost,
    #[default]
    Solid,
    Text,
}

impl Fill {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Ghost => "ghost",
            Self::Text => "text",
        }
    }
}

impl From<Fill> for &'static str {
    fn from(val: Fill) -> Self {
        val.as_str()
    }
}

impl From<&str> for Fill {
    fn from(s: &str) -> Self {
        match s {
            "ghost" => Self::Ghost,
            "text" => Self::Text,
            _ => Self::Solid,
        }
    }
}

impl Display for Fill {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IntoAnyAttribute for Fill {
    fn into_any_attr(self) -> AnyAttribute {
        custom_attribute("data-fill", self.as_str()).into_any_attr()
    }
}

impl IntoAttributeValue for Fill {
    type Output = &'static str;
    fn into_attribute_value(self) -> Self::Output {
        self.as_str()
    }
}
