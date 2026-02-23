pub mod partials;
pub mod templates;

/// Lightweight health check — no auth, no session, no template rendering
pub async fn healthz() -> &'static str {
    "ok"
}
