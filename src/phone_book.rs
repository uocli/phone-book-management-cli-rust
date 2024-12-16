use crate::contact::Contact;
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
}
