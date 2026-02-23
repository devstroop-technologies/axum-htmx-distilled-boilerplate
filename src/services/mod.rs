//! Service Layer — Business logic abstraction
//!
//! Services encapsulate business logic and data access, keeping handlers thin.
//! Uses trait-based design for testability and flexibility.

use std::sync::Arc;

pub mod csrf;
pub mod health;
pub mod items;
pub mod session;

pub use csrf::CsrfSecret;
pub use health::HealthService;
pub use items::ItemService;
pub use session::{InMemorySessionStore, SessionStore};

use crate::db::Db;

/// Application services container — injected into handlers via State
#[derive(Clone)]
pub struct Services {
    pub health: Arc<dyn HealthService>,
    pub items: Arc<dyn ItemService>,
    pub sessions: Arc<dyn SessionStore>,
    pub csrf: CsrfSecret,
}

impl Services {
    /// Create services with SQLite-backed item storage
    pub fn new_with_db(start_time: std::time::SystemTime, db: Db) -> Self {
        Self {
            health: Arc::new(health::DefaultHealthService::new(start_time)),
            items: Arc::new(items::SqliteItemService::new(db)),
            sessions: Arc::new(InMemorySessionStore::new()),
            csrf: CsrfSecret::generate(),
        }
    }

    /// Create services with in-memory implementations (fallback / tests)
    pub fn new_default(start_time: std::time::SystemTime) -> Self {
        Self {
            health: Arc::new(health::DefaultHealthService::new(start_time)),
            items: Arc::new(items::InMemoryItemService::new()),
            sessions: Arc::new(InMemorySessionStore::new()),
            csrf: CsrfSecret::generate(),
        }
    }
}
