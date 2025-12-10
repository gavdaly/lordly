use alloc::format;

use alloc::string::String;

use core::fmt::{Display, Formatter, Result as FmtResult};
use core::result::Result;
use core::str::FromStr;

#[derive(Clone)]
pub enum ValidationState {
    Empty,
    Valid,
    Dirty,
    Invalid(String),
}

impl FromStr for ValidationState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "empty" => Ok(ValidationState::Empty),
            "dirty" => Ok(ValidationState::Dirty),
            "valid" => Ok(ValidationState::Valid),
            "invalid" => Ok(ValidationState::Invalid("".into())),
            _ => Err(format!("Invalid state: {}", s)),
        }
    }
}

impl Display for ValidationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let state_str = match self {
            ValidationState::Empty => "empty",
            ValidationState::Dirty => "dirty",
            ValidationState::Valid => "valid",
            ValidationState::Invalid(reason) => &reason,
        };
        write!(f, "{}", state_str)
    }
}
