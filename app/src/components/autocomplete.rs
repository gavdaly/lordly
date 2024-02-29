enum Autocomplete {
    Off = "off",
    On = "on",
    Name = "name",
    HonorificPrefix = "honorific-prefix",
    GivenName = "given-name",
    AdditionalName = "additional-name",
    FamilyName = "family-name",
    Nickname = "nickname",
    Email = "email",
    Username = "username",
    NewPassword = "new-password",
    CurrentPassword = "current-password",
    OneTimeCode = "one-time-code",
    OrganizationTitle = "organization-title",
    Organization = "organization",
    /// The full name as printed on or associated with a payment instrument such as a credit card. Using a full name field is preferred, typically, over breaking the name into pieces.
    CreditCardName = "cc-name",
    /// A given (first) name as given on a payment instrument like a credit card.
    CreditCardGivenName = "cc-given-name",
    /// A middle name as given on a payment instrument or credit card.
    CreditCardAdditionalName = "cc-additional-name",
    /// A family name, as given on a credit card.
    CreditCardFamilyName = "cc-family-name",
    /// A credit card number or other number identifying a payment method, such as an account number.
    CreditCardNumber = "cc-number",
    /// A payment method expiration date, typically in the form "MM/YY" or "MM/YYYY".
    CreditCardExpiration = "cc-exp",
    /// The month in which the payment method expires.
    CreditCardExpirationMonth = "cc-exp-month",
    /// The year in which the payment method expires.
    CreditCardExpirationYear = "cc-exp-year",
    /// The security code for the payment instrument; on credit cards, this is the 3-digit verification number on the back of the card.
    CreditCardSecurityCode = "cc-csc",
    /// The type of payment instrument (such as "Visa" or "Master Card").
    CreditCardType = "cc-type",
    /// The currency in which the transaction is to take place.
    TransactionCurrency = "transaction-currency",
    /// The amount, given in the currency specified by "transaction-currency", of the transaction, for a payment form.
    TransactionAmount = "transaction-amount",
    /// A birth date, as a full date.
    Birthday = "bday",
    /// The day of the month of a birth date.
    BirthdayDay = "bday-day",
    /// The month of the year of a birth date.
    BirthdayMonth = "bday-month",
    /// The year of a birth date.
    BirthdayYear = "bday-year",
    /// A URL for an instant messaging protocol endpoint, such as "xmpp:username@example.net".
    Impp = "impp",
    /// A URL, such as a home page or company website address as appropriate given the context of the other fields in the form.
    Url = "url",
    /// The URL of an image representing the person, company, or contact information given in the other fields in the form.
    Photo = "photo",
}

enum Address {
    Standard = "",
    /// The street address to associate with the form of payment used. This can be combined with other tokens, such as "billing street-address" and "billing address-level2".
    Billing = "billing",
    /// The street address to send the product. This can be combined with other tokens, such as "shipping street-address" and "shipping address-level2".
    Shipping = "shipping",
}

/*
"street-address"
A street address. This can be multiple lines of text, and should fully identify the location of the address within its second administrative level (typically a city or town), but should not include the city name, ZIP or postal code, or country name.


"address-line1", "address-line2", "address-line3"
Each individual line of the street address. These should only be present if the "street-address" is not present.

"address-level4"
The finest-grained administrative level, in addresses which have four levels.

"address-level3"
The third administrative level, in addresses with at least three administrative levels.

"address-level2"
The second administrative level, in addresses with at least two of them. In countries with two administrative levels, this would typically be the city, town, village, or other locality in which the address is located.

"address-level1"
The first administrative level in the address. This is typically the province in which the address is located. In the United States, this would be the state. In Switzerland, the canton. In the United Kingdom, the post town.

"country"
A country or territory code.

"country-name"
A country or territory name.

"postal-code"
A postal code (in the United States, this is the ZIP code).



"language"
A preferred language, given as a valid BCP 47 language tag.



"sex"
A gender identity (such as "Female", "Fa'afafine", "Hijra", "Male", "Nonbinary"), as freeform text without newlines.

"tel"
A full telephone number, including the country code. If you need to break the phone number up into its components, you can use these values for those fields:

"tel-country-code"
The country code, such as "1" for the United States, Canada, and other areas in North America and parts of the Caribbean.

"tel-national"
The entire phone number without the country code component, including a country-internal prefix. For the phone number "1-855-555-6502", this field's value would be "855-555-6502".

"tel-area-code"
The area code, with any country-internal prefix applied if appropriate.

"tel-local"
The phone number without the country or area code. This can be split further into two parts, for phone numbers which have an exchange number and then a number within the exchange. For the phone number "555-6502", use "tel-local-prefix" for "555" and "tel-local-suffix" for "6502".

"tel-extension"
A telephone extension code within the phone number, such as a room or suite number in a hotel or an office extension in a company.

*/