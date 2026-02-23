//! Axum + HTMX Full-Stack Boilerplate
//!
//! ## Architecture
//!
//! - **Backend**: Axum web framework with layered middleware
//! - **Frontend**: Server-rendered HTML with HTMX for SPA-like navigation
//! - **Templates**: Askama (compiled) in release, minijinja (hot-reload) in debug
//! - **Styling**: Minimal CSS design system (no framework dependencies)
//! - **Interactivity**: HTMX + vanilla JS (zero framework overhead)
//!
//! ## How It Works
//!
//! 1. Full HTML pages are served on initial navigation (GET /, /about, etc.)
//! 2. HTMX fetches HTML *partials* (fragments) for dynamic content updates
//! 3. REST API endpoints return JSON for programmatic access
//! 4. Both page templates and partials share the same design system

pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
#[macro_use]
pub mod render;
pub mod services;
pub mod utils;

pub use config::AppConfig;
pub use error::{AppError, AppResult};
