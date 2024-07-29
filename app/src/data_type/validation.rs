#[derive(Clone)]
pub enum ValidationState {
    Valid,
    Dirty,
    Invalid(String),
}
