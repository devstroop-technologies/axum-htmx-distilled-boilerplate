//! HTMX Partial Handlers — return HTML fragments for dynamic updates
//!
//! These handlers return *fragments* of HTML, not full pages.
//! HTMX swaps them into the existing page for SPA-like interactivity.

use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use serde::Deserialize;
use std::sync::Arc;

use crate::models::AppState;
use crate::services::items::Item;

// =============================================================================
// Partial Templates — using the macro for dual-mode rendering
// =============================================================================

crate::define_partial!(StatusCardPartial, "partials/status_card.html", {
    status: String,
    uptime: String,
    version: String
});

crate::define_partial!(ItemListPartial, "partials/item_list.html", {
    items: Vec<Item>
});

// =============================================================================
// Partial Handlers
// =============================================================================

/// Status card partial — shows server health on the dashboard
pub async fn status_card(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let health = state.services.health.get_status();

    StatusCardPartial {
        status: health.status,
        uptime: health.uptime_formatted,
        version: health.version,
    }
    .render_response()
}

/// Item list partial — returns a list of items as an HTML fragment
pub async fn item_list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let items = state.services.items.list_all();
    ItemListPartial { items }.render_response()
}

/// Greeting partial — demonstrates HTMX form submission returning a fragment
pub async fn greeting(Query(params): Query<GreetingQuery>) -> impl IntoResponse {
    let name = params.name.unwrap_or_else(|| "World".to_string());
    Html(format!(
        r#"<div class="alert alert-success">
            <div class="alert-title"><i class="bi bi-check-circle"></i> <strong>Hello, {}!</strong></div>
            <div class="alert-body">This fragment was loaded via HTMX.</div>
        </div>"#,
        html_escape::encode_text(&name)
    ))
}

#[derive(Deserialize)]
pub struct GreetingQuery {
    pub name: Option<String>,
}
