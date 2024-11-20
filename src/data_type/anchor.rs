use leptos::{Attribute, IntoAttribute};
use std::fmt::Display;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IntoAttribute for Anchor {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.as_str().to_string().into())
    }
    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        Box::new(self).into_attribute()
    }
}
