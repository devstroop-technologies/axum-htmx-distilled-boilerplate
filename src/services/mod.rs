//! Service Layer — Business logic abstraction
//!
//! Services encapsulate business logic and data access, keeping handlers thin.
//! Uses trait-based design for testability and flexibility.

use std::sync::Arc;

pub mod health;
pub mod items;

pub use health::HealthService;
pub use items::ItemService;

/// Application services container — injected into handlers via State
#[derive(Clone)]
pub struct Services {
    pub health: Arc<dyn HealthService>,
    pub items: Arc<dyn ItemService>,
}

impl Services {
    /// Create services with in-memory implementations (default)
    pub fn new_default(start_time: std::time::SystemTime) -> Self {
        Self {
            health: Arc::new(health::DefaultHealthService::new(start_time)),
            items: Arc::new(items::InMemoryItemService::new()),
        }
    }
}
