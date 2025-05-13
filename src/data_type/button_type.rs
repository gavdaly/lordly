use leptos::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ButtonType {
    #[default]
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

impl From<ButtonType> for String {
    fn from(val: ButtonType) -> Self {
        val.as_str().to_string()
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

// impl IntoAnyAttribute for ButtonType {
//     fn into_any_attr(self) -> leptos::attr::any_attribute::AnyAttribute {
//         self.as_str()
//     }
// }
