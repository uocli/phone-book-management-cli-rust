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
    last_command: String,
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
            last_command: String::new(),
        }
    }
}
/**
 * PhoneBook struct implementation.
 */
impl PhoneBook {
    pub fn start() {
        let mut phone_book = Self::default();
        loop {
            phone_book.show_operations();
            println!("\nEnter an operation:");
            let mut operation = String::new();
            std::io::stdin().read_line(&mut operation).unwrap();
            let operation = operation.trim().to_uppercase();
            phone_book.last_command = operation.to_string();
            match operation.as_str() {
                "C" => {}
                "Q" => {}
                "F" => {}
                "U" => {}
                "D" => {}
                "E" => {
                    println!("Exiting the phone book...");
                    break;
                }
                "L" => {}
                "A" => {}
                "Z" => {}
                _ => println!("Invalid operation!"),
            }
        }
    }
    /**
     * Implement an add_contact method for the PhoneBook struct that takes a Contact struct as an argument and adds it to the contacts vector.
     */
    pub fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }
    /**
     * Implement a show_operations method for the PhoneBook struct that displays the available operations in a table format.
     */
    pub fn show_operations(&self) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("Option").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Description").add_attribute(comfy_table::Attribute::Bold),
            ]);
        // Add the operations as rows in the table
        for &(option, description) in OPERATIONS {
            table.add_row(vec![
                Cell::new(format!("{} | {}", option, option.to_lowercase())),
                Cell::new(description),
            ]);
        }
        // Add a footer to show the last command
        table.add_row(vec![
            Cell::new("Last Command:").add_attribute(comfy_table::Attribute::Bold),
            Cell::new(if self.last_command.is_empty() {
                "None"
            } else {
                self.last_command.as_str()
            })
            .fg(comfy_table::Color::Yellow),
        ]);
        // Print the table
        println!("{}", table);
    }
}
