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
    ) -> Contact {
        Contact {
            first_name,
            last_name,
            email,
            address,
            phone_number,
            ..Default::default()
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
