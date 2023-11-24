use std::error::Error;

use diesel::pg::Pg;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

type SyncError = dyn Error + Send + Sync;

pub fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<(), Box<SyncError>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
