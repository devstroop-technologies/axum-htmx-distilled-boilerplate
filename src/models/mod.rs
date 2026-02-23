use crate::db::Db;
use crate::services::Services;

/// Shared application state passed to handlers via Axum's State extractor
#[derive(Clone)]
pub struct AppState {
    pub services: Services,
    pub db: Db,
}

impl AppState {
    pub fn new(services: Services, db: Db) -> Self {
        Self { services, db }
    }
}
