use crate::phone_book::contact::Contact;
/**
 * Define a PhoneBook struct with a field for a vector of Contact structs.
 */
pub struct PhoneBook {
    pub contacts: Vec<Contact>,
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
        println!("Welcome to the Phone Book!");
        Self {
            contacts: Vec::new(),
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
            let operation = Self::get_input("Enter an operation or ? for help: ").to_uppercase();
            match operation.as_str() {
                "C" => phone_book.create_contact(),
                "Q" => phone_book.search_contact(),
                "F" => phone_book.load_contacts_from_csv(),
                "U" => phone_book.update_contact(),
                "D" => phone_book.delete_contact(),
                "E" => {
                    let confirmation = Self::get_input("Are you sure you want to exit? (y/n): ");
                    if confirmation == "y" {
                        println!("Exiting the phone book...");
                        break;
                    }
                }
                "L" => phone_book.list_contacts(),
                "A" => phone_book.list_contacts_in_ascending_order(),
                "Z" => phone_book.list_contacts_in_descending_order(),
                "?" => phone_book.show_operations(),
                _ => println!("Invalid operation: {}", operation),
            }
        }
    }
}
