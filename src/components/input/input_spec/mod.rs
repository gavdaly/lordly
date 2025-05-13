use leptos::prelude::*;

mod address;
mod birthday;
mod birthday_day;
mod birthday_month;
mod birthday_year;
mod credit_card_expiration;
mod credit_card_name;
mod credit_card_number;
mod credit_card_security_code;
mod email;
mod language;
mod name;
mod nickname;
mod organization;
mod organization_title;
mod password;
mod phone;
mod photo;
mod sex;
mod text;
mod transaction_amount;
mod transaction_currency;
mod url;

use crate::check::Check;

// Export all input spec types
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

// Trait to provide default HTML attributes.
pub trait InputSpec {
    fn input_type() -> &'static str;
    fn autocomplete() -> &'static str;
    fn aria_label() -> &'static str;
    fn input_mode() -> &'static str;
    fn pattern() -> Option<&'static str>;
    fn maxlength() -> Option<u32>;
    fn minlength() -> Option<u32>;
    fn validation() -> Option<Callback<String, Check<String>>>;
}
