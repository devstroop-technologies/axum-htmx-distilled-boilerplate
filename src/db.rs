//! Database initialization and pool management
//!
//! Uses SQLx with SQLite. The pool is created once at startup and shared
//! across all handlers via AppState.

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tracing::info;

/// Type alias for the database connection pool
pub type Db = SqlitePool;

/// Initialize the SQLite connection pool and run migrations.
///
/// The `database_url` should be a SQLite connection string, e.g.:
/// - `sqlite://data.db?mode=rwc` (file-based, auto-create)
/// - `sqlite::memory:` (in-memory, useful for tests)
pub async fn init_pool(database_url: &str) -> Result<Db, sqlx::Error> {
    info!("Connecting to database: {}", database_url);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Run embedded migrations at startup
    sqlx::migrate!("./migrations").run(&pool).await?;

    info!("Database migrations applied successfully");

    Ok(pool)
}
