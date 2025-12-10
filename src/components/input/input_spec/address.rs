

use alloc::string::String;

use super::InputSpec;
use leptos::prelude::*;
use crate::data_type::ValidationState;
use core::fmt;

/// Defines the type of address (Standard, Billing, Shipping)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    /// Standard address without special designation
    Standard,
    /// The street address to associate with the form of payment used.
    /// This can be combined with other tokens, such as "billing street-address" and "billing address-level2".
    Billing,
    /// The street address to send the product.
    /// This can be combined with other tokens, such as "shipping street-address" and "shipping address-level2".
    Shipping,
}

impl fmt::Display for AddressType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, ""),
            Self::Billing => write!(f, "billing"),
            Self::Shipping => write!(f, "shipping"),
        }
    }
}

/// Defines the different address components/fields
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressComponent {
    /// A street address. This can be multiple lines of text, and should fully identify the location of the address
    /// within its second administrative level (typically a city or town), but should not include the city name,
    /// ZIP or postal code, or country name.
    StreetAddress,
    /// First individual line of the street address. These should only be present if the "street-address" is not present.
    AddressLine1,
    /// Second individual line of the street address. These should only be present if the "street-address" is not present.
    AddressLine2,
    /// Third individual line of the street address. These should only be present if the "street-address" is not present.
    AddressLine3,
    /// The first administrative level in the address. This is typically the province in which the address is located.
    /// In the United States, this would be the state. In Switzerland, the canton. In the United Kingdom, the post town.
    AddressLevel1,
    /// The second administrative level, in addresses with at least two of them. In countries with two administrative levels,
    /// this would typically be the city, town, village, or other locality in which the address is located.
    AddressLevel2,
    /// The third administrative level, in addresses with at least three administrative levels.
    AddressLevel3,
    /// The finest-grained administrative level, in addresses which have four levels.
    AddressLevel4,
    /// A country or territory code.
    Country,
    /// A country or territory name.
    CountryName,
    /// A postal code (in the United States, this is the ZIP code).
    PostalCode,
}

impl fmt::Display for AddressComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StreetAddress => write!(f, "street-address"),
            Self::AddressLine1 => write!(f, "address-line1"),
            Self::AddressLine2 => write!(f, "address-line2"),
            Self::AddressLine3 => write!(f, "address-line3"),
            Self::AddressLevel1 => write!(f, "address-level1"),
            Self::AddressLevel2 => write!(f, "address-level2"),
            Self::AddressLevel3 => write!(f, "address-level3"),
            Self::AddressLevel4 => write!(f, "address-level4"),
            Self::Country => write!(f, "country"),
            Self::CountryName => write!(f, "country-name"),
            Self::PostalCode => write!(f, "postal-code"),
        }
    }
}

/// A struct representing a standard address field
#[derive(Debug, Clone, Copy)]
pub struct AddressField {
    /// The type of address (standard, billing, shipping)
    pub address_type: AddressType,
    /// The component of the address (street, city, etc.)
    pub component: AddressComponent,
}

/// Base implementation for standard street address
pub struct StreetAddress;

/// Implementation of `InputSpec` for the standard street address type.
///
/// Provides specifications for street address input fields:
/// - Uses "text" input type
/// - Sets appropriate aria-label
/// - Sets common validations
impl InputSpec for StreetAddress {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "street-address"
    }
    fn aria_label() -> &'static str {
        "Street address"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None // Address formats vary widely, so no specific pattern
    }
    fn maxlength() -> Option<u32> {
        Some(100) // Reasonable limit for address lines
    }
    fn minlength() -> Option<u32> {
        Some(5) // Most addresses should be at least a few chars
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

/// Billing street address type.
pub struct BillingStreetAddress;

/// Implementation of `InputSpec` for billing street address type.
///
/// Provides specifications for billing street address input fields:
/// - Uses "text" input type
/// - Sets autocomplete to "billing street-address"
/// - Sets appropriate aria-label
impl InputSpec for BillingStreetAddress {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "billing street-address"
    }
    fn aria_label() -> &'static str {
        "Billing street address"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        Some(5)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

/// Shipping street address type.
pub struct ShippingStreetAddress;

/// Implementation of `InputSpec` for shipping street address type.
///
/// Provides specifications for shipping street address input fields:
/// - Uses "text" input type
/// - Sets autocomplete to "shipping street-address"
/// - Sets appropriate aria-label
impl InputSpec for ShippingStreetAddress {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "shipping street-address"
    }
    fn aria_label() -> &'static str {
        "Shipping street address"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        Some(5)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

// AddressLine1 implementations
pub struct AddressLine1;
pub struct BillingAddressLine1;
pub struct ShippingAddressLine1;

impl InputSpec for AddressLine1 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "address-line1"
    }
    fn aria_label() -> &'static str {
        "Address line 1"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        None
    }
    fn minlength() -> Option<u32> {
        None
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

impl InputSpec for BillingAddressLine1 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "billing address-line1"
    }
    fn aria_label() -> &'static str {
        "Billing address line 1"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

impl InputSpec for ShippingAddressLine1 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "shipping address-line1"
    }
    fn aria_label() -> &'static str {
        "Shipping address line 1"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

// AddressLine2 implementations
pub struct AddressLine2;
pub struct BillingAddressLine2;
pub struct ShippingAddressLine2;

impl InputSpec for AddressLine2 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "address-line2"
    }
    fn aria_label() -> &'static str {
        "Address line 2"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        None // Line 2 is optional
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None // No specific validation needed for line 2
    }
}

impl InputSpec for BillingAddressLine2 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "billing address-line2"
    }
    fn aria_label() -> &'static str {
        "Billing address line 2"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        None // Line 2 is optional
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None // No specific validation needed for line 2
    }
}

impl InputSpec for ShippingAddressLine2 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "shipping address-line2"
    }
    fn aria_label() -> &'static str {
        "Shipping address line 2"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        None // Line 2 is optional
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None // No specific validation needed for line 2
    }
}

// AddressLine3 implementations
pub struct AddressLine3;
pub struct BillingAddressLine3;
pub struct ShippingAddressLine3;

impl InputSpec for AddressLine3 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "address-line3"
    }
    fn aria_label() -> &'static str {
        "Address line 3"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        None // Line 3 is optional
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None // No specific validation needed for line 3
    }
}

impl InputSpec for BillingAddressLine3 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "billing address-line3"
    }
    fn aria_label() -> &'static str {
        "Billing address line 3"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        None // Line 3 is optional
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None // No specific validation needed for line 3
    }
}

impl InputSpec for ShippingAddressLine3 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "shipping address-line3"
    }
    fn aria_label() -> &'static str {
        "Shipping address line 3"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(100)
    }
    fn minlength() -> Option<u32> {
        None // Line 3 is optional
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None // No specific validation needed for line 3
    }
}

// AddressLevel1 implementations (State/Province/Region)
pub struct AddressLevel1;
pub struct BillingAddressLevel1;
pub struct ShippingAddressLevel1;

impl InputSpec for AddressLevel1 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "address-level1"
    }
    fn aria_label() -> &'static str {
        "State or province"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(50)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String,  ValidationState>> {
        None
    }
}

impl InputSpec for BillingAddressLevel1 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "billing address-level1"
    }
    fn aria_label() -> &'static str {
        "Billing state or province"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(50)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

impl InputSpec for ShippingAddressLevel1 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "shipping address-level1"
    }
    fn aria_label() -> &'static str {
        "Shipping state or province"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(50)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

// AddressLevel2 implementations (City/Town/Village)
pub struct AddressLevel2;
pub struct BillingAddressLevel2;
pub struct ShippingAddressLevel2;

impl InputSpec for AddressLevel2 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "address-level2"
    }
    fn aria_label() -> &'static str {
        "City"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(50)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

impl InputSpec for BillingAddressLevel2 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "billing address-level2"
    }
    fn aria_label() -> &'static str {
        "Billing city"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(50)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

impl InputSpec for ShippingAddressLevel2 {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "shipping address-level2"
    }
    fn aria_label() -> &'static str {
        "Shipping city"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(50)
    }
    fn minlength() -> Option<u32> {
        Some(1)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

// PostalCode implementations
pub struct PostalCode;
pub struct BillingPostalCode;
pub struct ShippingPostalCode;

impl InputSpec for PostalCode {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "postal-code"
    }
    fn aria_label() -> &'static str {
        "Postal code"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        None // Postal code formats vary widely by country
    }
    fn maxlength() -> Option<u32> {
        Some(20)
    }
    fn minlength() -> Option<u32> {
        Some(3)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

impl InputSpec for BillingPostalCode {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "billing postal-code"
    }
    fn aria_label() -> &'static str {
        "Billing postal code"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        None // Postal code formats vary widely by country
    }
    fn maxlength() -> Option<u32> {
        Some(20)
    }
    fn minlength() -> Option<u32> {
        Some(3)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        Some(Callback::new(|value: String| {
            if value.trim().len() >= 3 {
                ValidationState::Valid
            } else {
                ValidationState::Invalid("Please enter a valid billing postal code")
            }
        }))
    }
}

impl InputSpec for ShippingPostalCode {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "shipping postal-code"
    }
    fn aria_label() -> &'static str {
        "Shipping postal code"
    }
    fn input_mode() -> &'static str {
        "numeric"
    }
    fn pattern() -> Option<&'static str> {
        None // Postal code formats vary widely by country
    }
    fn maxlength() -> Option<u32> {
        Some(20)
    }
    fn minlength() -> Option<u32> {
        Some(3)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

// Country implementations
pub struct Country;
pub struct BillingCountry;
pub struct ShippingCountry;

impl InputSpec for Country {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "country"
    }
    fn aria_label() -> &'static str {
        "Country code"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^[A-Z]{2}$") // ISO 3166-1 alpha-2 country codes (e.g., US, CA, GB)
    }
    fn maxlength() -> Option<u32> {
        Some(2)
    }
    fn minlength() -> Option<u32> {
        Some(2)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

impl InputSpec for BillingCountry {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "billing country"
    }
    fn aria_label() -> &'static str {
        "Billing country code"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^[A-Z]{2}$") // ISO 3166-1 alpha-2 country codes
    }
    fn maxlength() -> Option<u32> {
        Some(2)
    }
    fn minlength() -> Option<u32> {
        Some(2)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

impl InputSpec for ShippingCountry {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "shipping country"
    }
    fn aria_label() -> &'static str {
        "Shipping country code"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        Some(r"^[A-Z]{2}$") // ISO 3166-1 alpha-2 country codes
    }
    fn maxlength() -> Option<u32> {
        Some(2)
    }
    fn minlength() -> Option<u32> {
        Some(2)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

// CountryName implementations
pub struct CountryName;
pub struct BillingCountryName;
pub struct ShippingCountryName;

impl InputSpec for CountryName {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "country-name"
    }
    fn aria_label() -> &'static str {
        "Country name"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(56) // Length of the longest country name
    }
    fn minlength() -> Option<u32> {
        Some(2)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

impl InputSpec for BillingCountryName {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "billing country-name"
    }
    fn aria_label() -> &'static str {
        "Billing country name"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(56)
    }
    fn minlength() -> Option<u32> {
        Some(2)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}

impl InputSpec for ShippingCountryName {
    fn input_type() -> &'static str {
        "text"
    }
    fn autocomplete() -> &'static str {
        "shipping country-name"
    }
    fn aria_label() -> &'static str {
        "Shipping country name"
    }
    fn input_mode() -> &'static str {
        "text"
    }
    fn pattern() -> Option<&'static str> {
        None
    }
    fn maxlength() -> Option<u32> {
        Some(56)
    }
    fn minlength() -> Option<u32> {
        Some(2)
    }
    fn validation() -> Option<Callback<String, ValidationState>> {
        None
    }
}
