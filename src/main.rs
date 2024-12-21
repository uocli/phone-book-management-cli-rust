mod connection;
mod migrations;
mod phone_book;
mod schema;

use phone_book::phone_book::PhoneBook;

fn main() {
    PhoneBook::start();
}
