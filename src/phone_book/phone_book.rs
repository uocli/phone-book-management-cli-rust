use crate::phone_book::contact::Contact;
use crate::phone_book::operations::OPERATIONS;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};
use std::io;
use std::io::Write;
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
    /// Creates a new `PhoneBook` instance with default values.
    ///
    /// This function implements the `Default` trait for the `PhoneBook` struct.
    /// It initializes a new `PhoneBook` with an empty contacts list and an empty message.
    ///
    /// # Returns
    ///
    /// A new `PhoneBook` instance with:
    /// - `contacts`: An empty vector of `Contact` structs.
    /// - `msg`: An empty `String` for storing messages.
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
    /// Starts the phone book application and handles user interactions.
    ///
    /// This function initializes a new `PhoneBook` instance, displays the available operations,
    /// and processes user input to perform various operations on the phone book.
    ///
    /// # Examples
    ///
    /// ```
    /// use phone_book_management_cli_rust::phone_book::PhoneBook;
    ///
    /// PhoneBook::start();
    /// ```
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
    /// Creates a new contact by prompting the user for contact information.
    ///
    /// This method interactively asks the user to input various details for a new contact,
    /// including first name, last name, phone number, email, and address. It then creates
    /// a new `Contact` struct with this information and adds it to the phone book.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - A mutable reference to the `PhoneBook` instance.
    ///
    /// # Effects
    ///
    /// - Prompts the user for contact information.
    /// - Creates a new `Contact` instance if all required fields are provided.
    /// - Adds the new contact to the phone book's contacts list.
    /// - Updates the `msg` field of the `PhoneBook` with the result of the operation.
    ///
    /// # Notes
    ///
    /// - First name and phone number are required fields. If either is left empty,
    ///   the contact creation is cancelled, and an appropriate message is set.
    /// - Last name, email, and address are optional fields.
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

    /// Prompts the user for input and returns the entered string.
    ///
    /// This function prints a prompt to the console, waits for user input,
    /// and returns the input as a trimmed string.
    ///
    /// # Arguments
    ///
    /// * `prompt` - A string slice that holds the text to be displayed as the input prompt.
    ///
    /// # Returns
    ///
    /// A `String` containing the user's input, with leading and trailing whitespace removed.
    ///
    /// # Examples
    ///
    /// ```
    /// let name = get_input("Enter your name: ");
    /// println!("Hello, {}!", name);
    /// ```
    fn get_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
    /// Adds a new contact to the phone book's contacts list.
    ///
    /// This method takes a `Contact` struct as an argument and adds it to the `contacts` vector.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the `PhoneBook` instance.
    /// * `contact` - A `Contact` struct representing the new contact to be added.
    ///
    /// # Return
    ///
    /// This function does not return any value. The `contacts` vector of the `PhoneBook` instance is updated.
    pub fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }
    /// Displays the available operations in a table format for the `PhoneBook` struct.
    ///
    /// This function creates a new `Table` instance, sets the table header, adds rows for each operation,
    /// and adds a footer to display the last command. It then prints the table to the console.
    ///
    /// # Arguments
    ///
    /// * `&self` - self A reference to the `PhoneBook` instance.
    ///
    /// # Return
    ///
    /// This function does not return any value. It prints the table to the console.
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
