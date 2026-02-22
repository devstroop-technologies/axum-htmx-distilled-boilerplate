use crate::services::Services;

/// Shared application state passed to handlers via Axum's State extractor
#[derive(Clone)]
pub struct AppState {
    pub services: Services,
}

impl AppState {
    pub fn new(services: Services) -> Self {
        Self { services }
    }
}
