use uuid::Uuid;

#[derive(Debug)]
pub struct PhoneBook {
    pub contacts: Vec<Contact>,
}

impl PhoneBook {
    pub fn new() -> PhoneBook {
        PhoneBook {
            contacts: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Contact {
    uuid: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    address: String,
    phone_number: String,
}

impl Contact {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        address: String,
        phone_number: String,
    ) -> Contact {
        Contact {
            uuid: Uuid::new_v4(),
            first_name,
            last_name,
            email,
            address,
            phone_number,
        }
    }
}
