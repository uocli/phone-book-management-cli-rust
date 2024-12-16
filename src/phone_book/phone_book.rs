use crate::phone_book::contact::Contact;
/**
 * Define a PhoneBook struct with a field for a vector of Contact structs.
 */
#[derive(Debug)]
pub struct PhoneBook {
    pub contacts: Vec<Contact>,
    pub(crate) msg: String,
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
            let operation = Self::get_input("Enter an operation or ? for help: ").to_uppercase();
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
                "L" => phone_book.show_contacts(),
                "A" => {}
                "Z" => {}
                "?" => phone_book.show_operations(),
                _ => {
                    phone_book.msg = format!("Invalid operation: {}", operation);
                }
            }
        }
    }
}
