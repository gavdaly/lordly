use alloc::string::String;

use leptos::prelude::*;

pub mod address;
pub mod birthday;
pub mod birthday_day;
pub mod birthday_month;
pub mod birthday_year;
pub mod credit_card_expiration;
pub mod credit_card_name;
pub mod credit_card_number;
pub mod credit_card_security_code;
pub mod email;
pub mod language;
pub mod name;
pub mod nickname;
pub mod organization;
pub mod organization_title;
pub mod password;
pub mod phone;
pub mod photo;
pub mod sex;
pub mod text;
pub mod transaction_amount;
pub mod transaction_currency;
pub mod url;

pub use address::*;
pub use birthday::*;
pub use birthday_day::*;
pub use birthday_month::*;
pub use birthday_year::*;
pub use credit_card_expiration::*;
pub use credit_card_name::*;
pub use credit_card_number::*;
pub use credit_card_security_code::*;
pub use email::*;
pub use language::*;
pub use name::*;
pub use nickname::*;
pub use organization::*;
pub use organization_title::*;
pub use password::*;
pub use phone::*;
pub use photo::*;
pub use sex::*;
pub use text::*;
pub use transaction_amount::*;
pub use transaction_currency::*;
pub use url::*;

use crate::data_type::ValidationState;

// Export all input spec types

// Trait to provide default HTML attributes.
pub trait InputSpec {
    fn input_type() -> &'static str;
    fn autocomplete() -> &'static str;
    fn aria_label() -> &'static str;
    fn input_mode() -> &'static str;
    fn pattern() -> Option<&'static str>;
    fn maxlength() -> Option<u32>;
    fn minlength() -> Option<u32>;
    fn validation() -> Option<Callback<String, ValidationState>>;
}
