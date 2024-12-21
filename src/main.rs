mod phone_book;
mod schema;
use dotenv::dotenv;
use phone_book::phone_book::PhoneBook;

fn main() {
    dotenv().ok();
    PhoneBook::start();
}
