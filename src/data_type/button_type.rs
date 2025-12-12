use leptos::attr::{any_attribute::{IntoAnyAttribute, AnyAttribute}, custom::custom_attribute, IntoAttributeValue};
use core::fmt::Display;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}

impl From<ButtonType> for &'static str {
    fn from(val: ButtonType) -> Self {
        val.as_str()
    }
}

impl From<&str> for ButtonType {
    fn from(s: &str) -> Self {
        match s {
            "submit" => Self::Submit,
            "reset" => Self::Reset,
            _ => Self::Button,
        }
    }
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl IntoAnyAttribute for ButtonType {
    fn into_any_attr(self) -> AnyAttribute {
        custom_attribute("type", self.as_str()).into_any_attr()
    }
}

impl IntoAttributeValue for ButtonType {
    type Output = &'static str;
    fn into_attribute_value(self) -> Self::Output {
        self.as_str()
    }
}
