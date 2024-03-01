use leptos::{Attribute, IntoAttribute};
use std::fmt::Display;

pub enum ButtonType {
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    fn as_str(&self) -> &str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}

impl Into<String> for ButtonType {
    fn into(self) -> String {
        self.as_str().to_string()
    }
}

impl From<&str> for ButtonType {
    fn from(s: &str) -> Self {
        match s {
            "button" => Self::Button,
            "subit" => Self::Submit,
            "reset" => Self::Reset,
            _ => Self::Button,
        }
    }
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IntoAttribute for ButtonType {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.as_str().to_string().into())
    }
    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        Box::new(self).into_attribute()
    }
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Button
    }
}
