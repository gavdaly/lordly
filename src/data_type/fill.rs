use leptos::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Fill {
    #[default]
    Solid,
    Ghost,
    Text,
}

impl Fill {
    fn as_str(&self) -> &str {
        match self {
            Self::Solid => "solid",
            Self::Ghost => "ghost",
            Self::Text => "text",
        }
    }
}

impl From<Fill> for String {
    fn from(val: Fill) -> Self {
        val.as_str().to_string()
    }
}

impl From<&str> for Fill {
    fn from(s: &str) -> Self {
        match s {
            "solid" => Self::Solid,
            "gost" => Self::Ghost,
            "text" => Self::Text,
            _ => Self::Solid,
        }
    }
}

impl Display for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// impl IntoAnyAttribute for Fill {
//     fn into_any_attr(self) -> leptos::attr::any_attribute::AnyAttribute {
//         self.as_str()
//     }
// }
