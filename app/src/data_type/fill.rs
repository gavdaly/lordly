use leptos::{Attribute, IntoAttribute};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Fill {
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

impl Into<String> for Fill {
    fn into(self) -> String {
        self.as_str().to_string()
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

impl IntoAttribute for Fill {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.as_str().to_string().into())
    }
    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        Box::new(self).into_attribute()
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self::Solid
    }
}
