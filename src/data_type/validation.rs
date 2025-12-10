use core::fmt::{Display, Formatter, Result as FmtResult};
use core::result::Result;
use core::str::FromStr;

#[derive(Clone, Copy)]
pub enum ValidationState {
    Empty,
    Valid,
    Dirty,
    Invalid(&'static str),
}

impl FromStr for ValidationState {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s { // Case-sensitive match generally preferred in no_alloc if we can't lower
            "empty" => Ok(ValidationState::Empty),
            "dirty" => Ok(ValidationState::Dirty),
            "valid" => Ok(ValidationState::Valid),
            "invalid" => Ok(ValidationState::Invalid("unknown")),
            _ => Err("Invalid state"),
        }
    }
}

impl Display for ValidationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let state_str = match self {
            ValidationState::Empty => "empty",
            ValidationState::Dirty => "dirty",
            ValidationState::Valid => "valid",
            ValidationState::Invalid(reason) => reason,
        };
        write!(f, "{}", state_str)
    }
}
