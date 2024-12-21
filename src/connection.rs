use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

/// Establishes a connection to the SQLite database using the `DATABASE_URL` environment variable.
///
/// # Panics
///
/// This function will panic if the `DATABASE_URL` environment variable is not set.
///
/// # Returns
///
/// Returns a `SqliteConnection` instance representing the established connection to the database.
pub fn establish_connection() -> SqliteConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
