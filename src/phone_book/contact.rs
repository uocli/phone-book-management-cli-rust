use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};
/// Define a Contact struct with fields for first name, last name, email, address, and phone number.
pub struct Contact {
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) email: String,
    pub(crate) address: String,
    pub(crate) phone_number: String,
}
/// Contact struct implementation.
impl Contact {
    /// Creates a new `Contact` instance with the provided details.
    ///
    /// # Parameters
    ///
    /// * `first_name`: A `String` representing the first name of the contact.
    /// * `last_name`: A `String` representing the last name of the contact.
    /// * `email`: A `String` representing the email address of the contact.
    /// * `address`: A `String` representing the address of the contact.
    /// * `phone_number`: A `String` representing the phone number of the contact.
    ///
    /// # Returns
    ///
    /// A new `Contact` instance with the provided details. The phone number is standardized using the
    /// `standardize_phone_number` method.
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        address: String,
        phone_number: String,
    ) -> Self {
        let standard_phone_number = Self::standardize_phone_number(&phone_number);
        Self {
            first_name,
            last_name,
            email,
            address,
            phone_number: standard_phone_number,
            ..Default::default()
        }
    }
    /// Prints the contact information in a table format.
    ///
    /// This function creates a table and populates it with the contact details.
    /// The table is then printed to the console.
    ///
    /// # Parameters
    ///
    /// * `&self` - A reference to the current instance of `Contact`.
    ///
    /// # Return
    ///
    /// This function does not return any value. It prints the contact information to the console.
    pub fn print_contact(&self) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("First Name").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Last Name").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Email").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Address").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Phone Number").add_attribute(comfy_table::Attribute::Bold),
            ]);
        // Add contact details to the table
        table.add_row(vec![
            &self.first_name,
            &self.last_name,
            &self.email,
            &self.address,
            &self.phone_number,
        ]);
        // Print the contact information
        println!("{}", table);
    }
    /// Standardizes a phone number by removing non-numeric characters and formatting it as (XXX) XXX-XXXX.
    /// If the phone number does not have exactly 10 digits, it is returned as is.
    ///
    /// # Parameters
    ///
    /// * `phone_number`: A reference to a string representing the phone number to be standardized.
    ///
    /// # Returns
    ///
    /// A string representing the standardized phone number. If the input phone number does not have 10 digits,
    /// it is returned as is.
    fn standardize_phone_number(phone_number: &str) -> String {
        // Remove non-numeric characters
        let digits: String = phone_number.chars().filter(|c| c.is_digit(10)).collect();
        // Format the phone number
        if digits.len() == 10 {
            format!("({}) {}-{}", &digits[0..=2], &digits[3..=5], &digits[6..=9])
        } else {
            phone_number.to_string() // Return the original if it doesn't have 10 digits
        }
    }
}
/// Implements the `Default` trait for the `Contact` struct.
///
/// The `Default` trait provides a way to create a default instance of a type.
/// In this case, it creates a new `Contact` instance with empty strings for all fields.
impl Default for Contact {
    /// Creates a new `Contact` instance with default values.
    ///
    /// # Returns
    ///
    /// A new `Contact` instance with the following default values:
    /// - `first_name`: An empty string.
    /// - `last_name`: An empty string.
    /// - `email`: An empty string.
    /// - `address`: An empty string.
    /// - `phone_number`: An empty string.
    fn default() -> Self {
        Contact {
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            address: String::new(),
            phone_number: String::new(),
        }
    }
}
