//! Page Handlers — serve full HTML pages
//!
//! Uses the define_page! macro for zero-cost dual-mode rendering:
//! - Debug: minijinja hot-reloads templates from disk  
//! - Release: askama compiles templates into the binary

use axum::response::IntoResponse;

// Define pages using the macro — one line per page instead of ~20!
crate::define_page!(HomePage, "pages/home.html", { current_page: &'static str });
crate::define_page!(AboutPage, "pages/about.html", { current_page: &'static str });
crate::define_page!(DemoPage, "pages/demo.html", { current_page: &'static str });

// =============================================================================
// Page Handlers — thin wrappers that delegate to templates
// =============================================================================

pub async fn home_page() -> impl IntoResponse {
    HomePage {
        current_page: "home",
    }
    .render_response()
}

pub async fn about_page() -> impl IntoResponse {
    AboutPage {
        current_page: "about",
    }
    .render_response()
}

pub async fn demo_page() -> impl IntoResponse {
    DemoPage {
        current_page: "demo",
    }
    .render_response()
}
