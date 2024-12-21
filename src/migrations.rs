use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(
    connection: &mut SqliteConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
