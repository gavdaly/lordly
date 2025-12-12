use alloc::string::{String, ToString};
use core::fmt::{Display, Formatter, Result as FmtResult};
use core::result::Result;
use core::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationState {
    Dirty,
    Empty,
    Valid,
    Invalid(String),
}

impl<E: core::fmt::Display> From<Result<(), E>> for ValidationState {
    fn from(result: Result<(), E>) -> Self {
        match result {
            Ok(()) => Self::Valid,
            Err(e) => Self::Invalid(e.to_string()),
        }
    }
}

impl Display for ValidationState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let state_str = match self {
            Self::Empty => "empty",
            Self::Dirty => "dirty",
            Self::Valid => "valid",
            Self::Invalid(reason) => reason,
        };
        write!(f, "{state_str}")
    }
}

impl FromStr for ValidationState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "empty" => Ok(Self::Empty),
            "dirty" => Ok(Self::Dirty),
            "valid" => Ok(Self::Valid),
            e => Ok(Self::Invalid(String::from(e))),
        }
    }
}
