//! Item Service — Task/item management
//!
//! Provides CRUD operations for items. Default implementation uses in-memory storage.
//! Can be swapped for database-backed implementation (SQLx, etc.)

use serde::{Deserialize, Serialize};
use std::sync::RwLock;

/// Item data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub done: bool,
}

/// Item service trait — defines operations for item management
pub trait ItemService: Send + Sync {
    fn list_all(&self) -> Vec<Item>;
    fn get_by_id(&self, id: u32) -> Option<Item>;
    fn create(&self, title: String, description: String) -> Item;
    fn toggle_done(&self, id: u32) -> Option<Item>;
    fn delete(&self, id: u32) -> bool;
}

/// In-memory item storage (good for prototyping, tests)
pub struct InMemoryItemService {
    items: RwLock<Vec<Item>>,
    next_id: RwLock<u32>,
}

impl InMemoryItemService {
    pub fn new() -> Self {
        // Seed with example data
        let items = vec![
            Item { id: 1, title: "Set up project".into(), description: "Scaffold Axum + HTMX boilerplate".into(), done: true },
            Item { id: 2, title: "Add database".into(), description: "Integrate SQLite or Postgres".into(), done: false },
            Item { id: 3, title: "Deploy".into(), description: "Containerize and ship to production".into(), done: false },
        ];

        Self {
            items: RwLock::new(items),
            next_id: RwLock::new(4),
        }
    }
}

impl Default for InMemoryItemService {
    fn default() -> Self {
        Self::new()
    }
}

impl ItemService for InMemoryItemService {
    fn list_all(&self) -> Vec<Item> {
        self.items.read().unwrap().clone()
    }

    fn get_by_id(&self, id: u32) -> Option<Item> {
        self.items.read().unwrap().iter().find(|i| i.id == id).cloned()
    }

    fn create(&self, title: String, description: String) -> Item {
        let mut next_id = self.next_id.write().unwrap();
        let item = Item {
            id: *next_id,
            title,
            description,
            done: false,
        };
        *next_id += 1;

        self.items.write().unwrap().push(item.clone());
        item
    }

    fn toggle_done(&self, id: u32) -> Option<Item> {
        let mut items = self.items.write().unwrap();
        if let Some(item) = items.iter_mut().find(|i| i.id == id) {
            item.done = !item.done;
            Some(item.clone())
        } else {
            None
        }
    }

    fn delete(&self, id: u32) -> bool {
        let mut items = self.items.write().unwrap();
        let len_before = items.len();
        items.retain(|i| i.id != id);
        items.len() < len_before
    }
}

// ============================================================================
// SQLx Implementation (optional — when "database" feature is enabled)
// ============================================================================

#[cfg(feature = "database")]
pub mod db {
    use super::*;
    use sqlx::{Pool, Sqlite};

    pub struct SqliteItemService {
        pool: Pool<Sqlite>,
    }

    impl SqliteItemService {
        pub fn new(pool: Pool<Sqlite>) -> Self {
            Self { pool }
        }
    }

    // TODO: Implement ItemService for SqliteItemService
    // This is a placeholder showing where database integration would go
}
