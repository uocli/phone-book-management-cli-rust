use crate::contact::Contact;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};
use std::io;
use std::io::Write;

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
    msg: String,
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
            msg: String::new(),
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
            let operation = Self::get_input("Enter an operation: ").to_uppercase();
            phone_book.msg = format!("Last command: {}", operation);
            match operation.as_str() {
                "C" => phone_book.create_contact(),
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
                _ => {
                    phone_book.msg = format!("Invalid operation: {}", operation);
                }
            }
        }
    }
    /**
     * Implement a create_contact method for the PhoneBook struct that prompts the user to enter contact information and creates a new Contact struct.
     */
    pub fn create_contact(&mut self) {
        println!("Creating a new contact...");
        let first_name = Self::get_input("Enter first name (required): ");
        if first_name.is_empty() {
            self.msg = "First name is required. Contact creation cancelled.".to_string();
            return;
        }
        let last_name = Self::get_input("Enter last name (optional): ");
        let phone_number = Self::get_input("Enter phone number (required): ");
        if phone_number.is_empty() {
            self.msg = "Phone number is required. Contact creation cancelled.".to_string();
            return;
        }
        let email = Self::get_input("Enter email (optional): ");
        let address = Self::get_input("Enter address (optional): ");
        let new_contact = Contact::new(first_name, last_name, email, address, phone_number);
        self.add_contact(new_contact);
        self.msg = "Contact created successfully!".to_string();
    }

    fn get_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
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
            Cell::new("Message:").add_attribute(comfy_table::Attribute::Bold),
            Cell::new(if self.msg.is_empty() {
                "None"
            } else {
                self.msg.as_str()
            })
            .fg(comfy_table::Color::Yellow),
        ]);
        // Print the table
        println!("{}", table);
    }
}
