enum Autocomplete {
    /// The browser is not permitted to automatically enter or select a value for this field. It is possible that the document or application provides its own autocomplete feature, or that security concerns require that the field's value not be automatically entered.
    Off = "off",
    /// The browser is allowed to automatically complete the input. No guidance is provided as to the type of data expected in the field, so the browser may use its own judgement.
    On = "on",
    /// The field expects the value to be a person's full name. Using "name" rather than breaking the name down into its components is generally preferred because it avoids dealing with the wide diversity of human names and how they are structured.
    Name = "name",
    /// The prefix or title, such as "Mrs.", "Mr.", "Miss", "Ms.", "Dr.", or "Mlle.".
    HonorificPrefix = "honorific-prefix",
    /// The given (or "first") name.
    GivenName = "given-name",
    /// The middle name.
    AdditionalName = "additional-name",
    /// The family (or "last") name.
    FamilyName = "family-name",
    /// The suffix, such as "Jr.", "B.Sc.", "PhD.", "MBASW", or "IV".
    HonorificSuffix = "honorific-suffix",
    /// A nickname or handle.
    Nickname = "nickname",
    /// An email address.
    Email = "email",
    /// A username or account name.
    Username = "username",
    /// A new password. When creating a new account or changing passwords, this should be used for an "Enter your new password" or "Confirm new password" field, as opposed to a general "Enter your current password" field that might be present. This may be used by the browser both to avoid accidentally filling in an existing password and to offer assistance in creating a secure password.
    NewPassword = "new-password",
    /// The user's current password.
    CurrentPassword = "current-password",
    /// A one-time password (OTP) for verifying user identity that is used as an additional factor in a sign-in flow. Most commonly this is a code received via some out-of-channel mechanism, such as SMS, email, or authenticator application.
    OneTimeCode = "one-time-code",
    /// A job title, or the title a person has within an organization, such as "Senior Technical Writer", "President", or "Assistant Troop Leader".
    OrganizationTitle = "organization-title",
    /// A company or organization name, such as "Acme Widget Company" or "Girl Scouts of America".
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
    /// A preferred language, given as a valid BCP 47 language tag.
    Language = "language",
    /// A gender identity (such as "Female", "Fa'afafine", "Hijra", "Male", "Nonbinary"), as freeform text without newlines.
    Sex = "sex",
}

enum AddressType {
    Standard,
    /// The street address to associate with the form of payment used. This can be combined with other tokens, such as "billing street-address" and "billing address-level2".
    Billing = "billing",
    /// The street address to send the product. This can be combined with other tokens, such as "shipping street-address" and "shipping address-level2".
    Shipping = "shipping",
}

enum Address {
    /// A street address. This can be multiple lines of text, and should fully identify the location of the address within its second administrative level (typically a city or town), but should not include the city name, ZIP or postal code, or country name.
    StreetAddress = "street-address",
    /// First individual line of the street address. These should only be present if the "street-address" is not present.
    AddressLine1 = "address-line1",
    /// Second individual line of the street address. These should only be present if the "street-address" is not present.
    AddressLine2 = "address-line2",
    /// Third individual line of the street address. These should only be present if the "street-address" is not present.
    AddressLine3 = "address-line3",
    /// The first administrative level in the address. This is typically the province in which the address is located. In the United States, this would be the state. In Switzerland, the canton. In the United Kingdom, the post town.
    AdderssLevel1 = "address-level1",
    /// The second administrative level, in addresses with at least two of them. In countries with two administrative levels, this would typically be the city, town, village, or other locality in which the address is located.
    AddressLevel2 = "address-level2",
    /// The third administrative level, in addresses with at least three administrative levels.
    AddressLevel3 = "address-level3",
    /// The finest-grained administrative level, in addresses which have four levels.
    AddressLevel4 = "address-level4",
    /// A country or territory code.
    Country = "country",
    /// A country or territory name.
    CountryName = "country-name",
    /// A postal code (in the United States, this is the ZIP code).
    PostalCode = "postal-code",
}

enum Telephone {
    /// A full telephone number, including the country code. If you need to break the phone number up into its components
    Tel = "tel",
    /// The country code, such as "1" for the United States, Canada, and other areas in North America and parts of the Caribbean.
    TelCountryCode = "tel-country-code",
    /// The entire phone number without the country code component, including a country-internal prefix. For the phone number "1-855-555-6502", this field's value would be "855-555-6502".
    TelNational = "tel-national",
    /// The area code, with any country-internal prefix applied if appropriate.
    TelAreaCode = "tel-area-code",
    /// The phone number without the country or area code. This can be split further into two parts, for phone numbers which have an exchange number and then a number within the exchange. For the phone number "555-6502", use "tel-local-prefix" for "555" and "tel-local-suffix" for "6502".
    TelLocal = "tel-local",
    /// A telephone extension code within the phone number, such as a room or suite number in a hotel or an office extension in a company.
    TelExtension = "tel-extension",
}
