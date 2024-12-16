use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};
use uuid::Uuid;

/**
 * Define a Contact struct with fields for a UUID, first name, last name, email, address, and phone number.
 */
#[derive(Debug)]
pub struct Contact {
    uuid: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    address: String,
    phone_number: String,
}
/**
 * Contact struct implementation.
 */
impl Contact {
    /**
     * Automatically generate a UUID for each contact when one is created.
     */
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
    /**
     * Implement a show_contact method for the Contact struct that displays the contact information in a table format.
     */
    pub fn show_contact(&self) {
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

        table.add_row(vec![
            &self.first_name,
            &self.last_name,
            &self.email,
            &self.address,
            &self.phone_number,
        ]);

        println!("{}", table);
    }
    /**
     * Implement a standardize_phone_number method for the Contact struct that takes a phone number as a string and returns a standardized version of the phone number.
     */
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
/**
 * Implement the Default trait for the Contact struct.
 */
impl Default for Contact {
    /**
     * Implement the Default trait for the Contact struct.
     */
    fn default() -> Self {
        Contact {
            uuid: Uuid::new_v4(),
            first_name: String::new(),
            last_name: String::new(),
            email: String::new(),
            address: String::new(),
            phone_number: String::new(),
        }
    }
}
