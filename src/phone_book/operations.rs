use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Write;

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};
use csv::ReaderBuilder;
use diesel::prelude::*;

use crate::connection::establish_connection;
use crate::phone_book::contact::Contact;
use crate::phone_book::phone_book::PhoneBook;
use crate::schema::contacts;

/// Define a list of operations available in the phone book.
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
    ('?', "Show available operations"),
];

impl PhoneBook {
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
    ///
    /// # Notes
    ///
    /// - First name and phone number are required fields. If either is left empty,
    ///   the contact creation is cancelled, and an appropriate message is set.
    /// - Last name, email, and address are optional fields.
    pub fn create_contact(&mut self) {
        let first_name = Self::get_input("Enter first name (required): ");
        if first_name.is_empty() {
            println!("First name is required. Contact creation cancelled.");
            return;
        }
        let last_name = Self::get_input("Enter last name (optional): ");
        let phone_number = Self::get_input("Enter phone number (required): ");
        if phone_number.is_empty() {
            println!("Phone number is required. Contact creation cancelled.");
            return;
        }
        let email = Self::get_input("Enter email (optional): ");
        let address = Self::get_input("Enter address (optional): ");
        let new_contact = Contact::new(first_name, last_name, email, address, phone_number);
        new_contact.print_contact();
        self.add_contact(new_contact);
        println!("Contact created successfully!");
    }
    /// Lists the contacts in the phone book in the specified order.
    ///
    /// # Parameters
    ///
    /// * `self` - A mutable reference to the `PhoneBook` instance.
    /// * `order` - A string slice representing the order in which to list the contacts.
    ///               It can be either "asc" for ascending order or "desc" for descending order.
    ///
    /// # Return
    ///
    /// This function does not return any value. It prints the table of contacts to the console.
    pub fn list_contacts_in_order(&mut self, order: &str) {
        let mut connection = establish_connection();
        let contacts_result = match order {
            "asc" => contacts::table
                .order(contacts::first_name.asc())
                .load::<Contact>(&mut connection),
            "desc" => contacts::table
                .order(contacts::first_name.desc())
                .load::<Contact>(&mut connection),
            "" => contacts::table.load::<Contact>(&mut connection),
            _ => {
                println!("Invalid order parameter. Please use 'asc' or 'desc'.");
                return;
            }
        };
        match contacts_result {
            Ok(contacts) => {
                self.contacts = contacts;
                Self::print_contacts(self.contacts.iter().collect());
            }
            Err(err) => {
                println!("Error fetching contacts from the database: {}", err);
            }
        }
    }
    /// Displays a list of stored contacts in the phone book.
    ///
    /// This function iterates through the contacts stored in the phone book and displays them in a formatted table.
    /// If no contacts are found, it prints a message indicating that no contacts were found.
    ///
    /// # Parameters
    ///
    /// * `contacts` - A slice of `Contact` instances representing the contacts to be displayed.
    ///
    /// # Return
    ///
    /// This function does not return any value. It prints the table to the console.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut phone_book = PhoneBook::new();
    /// phone_book.add_contact(Contact::new("John", "Doe", "john@example.com", "123 Main St", "1234567890"));
    /// PhoneBook::print_contacts(&phone_book.contacts);
    /// ```
    fn print_contacts(contacts: Vec<&Contact>) {
        if contacts.is_empty() {
            println!("No contacts found.");
            return;
        }
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("#").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("First Name").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Last Name").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Phone Number").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Email").add_attribute(comfy_table::Attribute::Bold),
                Cell::new("Address").add_attribute(comfy_table::Attribute::Bold),
            ]);

        for (index, contact) in contacts.iter().enumerate() {
            table.add_row(vec![
                Cell::new(format!("{}", index + 1)),
                Cell::new(&contact.first_name),
                Cell::new(&contact.last_name),
                Cell::new(&contact.phone),
                Cell::new(&contact.email),
                Cell::new(&contact.address),
            ]);
        }
        println!("{}", table);
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
    pub(crate) fn get_input(prompt: &str) -> String {
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
        use crate::schema::contacts;
        let mut connection = establish_connection();
        diesel::insert_into(contacts::table)
            .values(&contact)
            .execute(&mut connection)
            .expect("Error saving new contact");
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
        // Print the table
        println!("{}", table);
    }
    /// Deletes a contact from the phone book's contacts list based on the provided index.
    ///
    /// This method takes an index as an argument and removes the contact at that index from the `contacts` vector.
    /// If the index is out of bounds, it prints an error message.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the `PhoneBook` instance.
    /// * `index` - An integer representing the index of the contact to be deleted.
    ///
    /// # Return
    ///
    /// This function does not return any value. The `contacts` vector of the `PhoneBook` instance is updated.
    pub fn delete_contact(&mut self) {
        let index_result =
            Self::get_input("Enter the index of the contact to delete: ").parse::<usize>();
        if index_result.is_err() {
            println!("Invalid contact index!");
            return;
        }
        let index = index_result.unwrap();
        if index < 1 || index > self.contacts.len() {
            println!("Invalid contact index!");
            return;
        }
        self.contacts[index - 1].print_contact();
        let confirm = Self::get_input("Are you sure you want to delete this contact? (y/n): ");
        if confirm == "y" {
            self.contacts.remove(index - 1);
            println!("Contact at index {} deleted successfully.", index);
        } else {
            println!("Contact deletion cancelled.");
        }
    }
    /// Updates a contact in the phone book's contacts list based on the provided index.
    ///
    /// This function prompts the user to enter the index of the contact to update, validates the input,
    /// and then asks for new contact details. If the index is valid and all required fields are provided,
    /// the contact at the specified index is updated with the new details.
    ///
    /// # Parameters
    ///
    /// * `self` - A mutable reference to the `PhoneBook` instance.
    ///
    /// # Return
    ///
    /// This function does not return any value. The `contacts` vector of the `PhoneBook` instance is updated.
    pub(crate) fn update_contact(&mut self) {
        let index_result =
            Self::get_input("Enter the index of the contact to update: ").parse::<usize>();
        if index_result.is_err() {
            println!("Invalid contact index!");
            return;
        }
        let index = index_result.unwrap();
        if index < 1 || index > self.contacts.len() {
            println!("Invalid contact index!");
            return;
        }
        self.contacts[index - 1].print_contact();
        println!("Updating contact details...");
        let new_first_name = Self::get_input("Enter new first name: ");
        if new_first_name.is_empty() {
            println!("First name is required. Contact update cancelled.");
            return;
        }
        let new_last_name = Self::get_input("Enter new last name: ");
        let new_phone_number = Self::get_input("Enter new phone number: ");
        if new_phone_number.is_empty() {
            println!("Phone number is required. Contact update cancelled.");
            return;
        }
        let new_email = Self::get_input("Enter new email: ");
        let new_address = Self::get_input("Enter new address: ");
        let updated_contact = Contact::new(
            new_first_name,
            new_last_name,
            new_email,
            new_address,
            new_phone_number,
        );
        self.contacts[index - 1] = updated_contact;
        println!("Contact updated successfully!");
    }
    /// Searches for contacts in the phone book based on a given search query.
    ///
    /// This function takes a search query as input, converts it to lowercase, and then iterates through
    /// the list of contacts in the phone book. If any contact's first name, last name, email, address,
    /// or phone number contains the search query, the contact is added to a new vector of found contacts.
    ///
    /// If no contacts are found matching the search query, a message is printed to the console indicating
    /// that no contacts were found. Otherwise, the found contacts are displayed using the `print_contacts`
    /// function.
    ///
    /// # Parameters
    ///
    /// * `&self` - A reference to the `PhoneBook` instance.
    ///
    /// # Return
    ///
    /// This function does not return any value. It prints the search results to the console.
    pub(crate) fn search_contact(&self) {
        let query = Self::get_input("Enter a search query: ").to_lowercase();
        let mut found_contacts: Vec<&Contact> = Vec::new();
        for contact in &self.contacts {
            if contact.first_name.to_lowercase().contains(&query)
                || contact.last_name.to_lowercase().contains(&query)
                || contact.email.to_lowercase().contains(&query)
                || contact.address.to_lowercase().contains(&query)
                || contact.phone.to_lowercase().contains(&query)
            {
                found_contacts.push(contact);
            }
        }
        if found_contacts.is_empty() {
            println!("No contacts found matching the search query.");
        } else {
            Self::print_contacts(found_contacts);
        }
    }
    /// Loads contacts from a CSV file into the phone book.
    ///
    /// # Parameters
    ///
    /// * `self` - A mutable reference to the `PhoneBook` instance.
    ///
    /// # Functionality
    ///
    /// 1. Prompts the user to enter the name of the CSV file to load contacts from.
    /// 2. Opens the CSV file. If the file cannot be opened, an error message is printed and the function returns.
    /// 3. Creates a CSV reader.
    /// 4. Reads the header row of the CSV file. If the header row cannot be read, an error message is printed and the function returns.
    /// 5. Gets the indices of the required columns based on the header.
    /// 6. Iterates through the CSV records and creates `Contact` instances.
    /// 7. Adds each created `Contact` instance to the `contacts` vector of the `PhoneBook` instance.
    /// 8. Prints a success message indicating that the contacts have been loaded successfully from the CSV file.
    pub(crate) fn load_contacts_from_csv(&mut self) {
        let file_name = Self::get_input("Enter the name of the CSV file to load contacts from: ");
        // Open the CSV file
        let file = match File::open(&file_name) {
            Ok(file) => file,
            Err(err) => {
                println!("Error opening file: {}", err);
                return;
            }
        };
        // Create a CSV reader
        let mut reader = ReaderBuilder::new().from_reader(BufReader::new(file));
        // Read the header row
        let header_row = match reader.headers() {
            Ok(header_row) => header_row,
            Err(err) => {
                println!("Error reading header row: {}", err);
                return;
            }
        };
        // Get the indices of the required columns based on the header
        let first_name_index = header_row.iter().position(|header| header == "first_name");
        let last_name_index = header_row.iter().position(|header| header == "last_name");
        let email_index = header_row.iter().position(|header| header == "email");
        let address_index = header_row.iter().position(|header| header == "address");
        let phone_number_index = header_row.iter().position(|header| header == "phone");
        // Iterate through the CSV records and create Contact instances
        for record in reader.records() {
            match record {
                Ok(record) => {
                    let mut contact = Contact::default();
                    if let Some(first_name_index) = first_name_index {
                        contact.first_name = record[first_name_index].to_string();
                    }
                    if let Some(last_name_index) = last_name_index {
                        contact.last_name = record[last_name_index].to_string();
                    }
                    if let Some(email_index) = email_index {
                        contact.email = record[email_index].to_string();
                    }
                    if let Some(address_index) = address_index {
                        contact.address = record[address_index].to_string();
                    }
                    if let Some(phone_number_index) = phone_number_index {
                        contact.phone = record[phone_number_index].to_string();
                    }
                    self.contacts.push(contact);
                }
                Err(err) => {
                    println!("Error reading record: {}", err);
                }
            }
        }
        println!("Contacts loaded successfully from file '{}'.", file_name);
    }
}
