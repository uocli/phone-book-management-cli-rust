use crate::contact::Contact;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};

/**
 * Define a list of operations available in the phone book.
 */
const OPERATIONS: &[(char, &str)] = &[
    ('C', "Create"),
    ('Q', "Fuzzy Query"),
    ('F', "Upload contacts from a CSV file"),
    ('U', "Update"),
    ('D', "Delete"),
    ('E', "Exit"),
    ('L', "List in original order based on creation time"),
    ('A', "List in ascending order"),
    ('Z', "List in descending order"),
];
/**
 * Define a PhoneBook struct with a field for a vector of Contact structs.
 */
#[derive(Debug)]
pub struct PhoneBook {
    pub contacts: Vec<Contact>,
}
/**
 * Implement the Default trait for the PhoneBook struct.
 */
impl Default for PhoneBook {
    /**
     * Implement the Default trait for the PhoneBook struct.
     */
    fn default() -> Self {
        Self {
            contacts: Vec::new(),
        }
    }
}
/**
 * PhoneBook struct implementation.
 */
impl PhoneBook {
    /**
     * Implement a new method for the PhoneBook struct that creates a new PhoneBook instance with an empty vector of contacts.
     */
    pub fn new() -> Self {
        Self::default()
    }
    /**
     * Implement an add_contact method for the PhoneBook struct that takes a Contact struct as an argument and adds it to the contacts vector.
     */
    pub fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }
    /**
     * Display the list of operations available in the phone book as a table.
     */
    pub fn show_operations() {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("Option").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Description").add_attribute(comfy_table::Attribute::Bold),
            ]);

        for &(option, description) in OPERATIONS {
            table.add_row(vec![Cell::new(option), Cell::new(description)]);
        }

        println!("{}", table);
    }
}
