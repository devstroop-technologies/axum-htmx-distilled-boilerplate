//! Unified template rendering with zero-cost release builds
//!
//! This module eliminates code duplication by providing macros that generate
//! both the askama struct (release) and minijinja renderer (debug) from a single definition.

/// Macro to define a page template that works in both debug and release mode.
/// - Debug: hot-reloads from disk via minijinja
/// - Release: compiled into the binary via askama
///
/// # Example
/// ```ignore
/// define_page!(HomePage, "pages/home.html", { current_page: &'static str });
/// ```
#[macro_export]
macro_rules! define_page {
    ($name:ident, $path:literal, { $($field:ident : $ty:ty),* $(,)? }) => {
        // Release: compiled askama template
        #[cfg(not(debug_assertions))]
        #[derive(askama::Template)]
        #[template(path = $path)]
        pub struct $name {
            $(pub $field: $ty,)*
        }

        // Debug: runtime rendering struct (matches askama struct shape)
        #[cfg(debug_assertions)]
        pub struct $name {
            $(pub $field: $ty,)*
        }

        impl $name {
            pub fn render_response(self) -> axum::response::Html<String> {
                #[cfg(not(debug_assertions))]
                {
                    use askama::Template;
                    axum::response::Html(self.render().unwrap_or_else(|e| {
                        format!("<h1>Template Error</h1><pre>{}</pre>", e)
                    }))
                }

                #[cfg(debug_assertions)]
                {
                    use $crate::utils::templates::render_template;
                    use serde_json::json;

                    let ctx = json!({ $(stringify!($field): self.$field,)* });
                    match render_template($path, ctx) {
                        Ok(html) => axum::response::Html(html),
                        Err(e) => axum::response::Html(format!(
                            r#"<html><body style="font-family:monospace;padding:2rem">
                            <h1 style="color:#ef4444">Template Error</h1>
                            <pre style="background:#1e1e1e;color:#f8f8f2;padding:1rem;border-radius:8px;overflow-x:auto">{}</pre>
                            <p>Fix the template and refresh.</p>
                            </body></html>"#,
                            e
                        )),
                    }
                }
            }
        }
    };
}

/// Macro to define a partial template (HTML fragment for HTMX).
/// Same dual-mode behavior as define_page.
#[macro_export]
macro_rules! define_partial {
    ($name:ident, $path:literal, { $($field:ident : $ty:ty),* $(,)? }) => {
        #[cfg(not(debug_assertions))]
        #[derive(askama::Template)]
        #[template(path = $path)]
        pub struct $name {
            $(pub $field: $ty,)*
        }

        #[cfg(debug_assertions)]
        pub struct $name {
            $(pub $field: $ty,)*
        }

        impl $name {
            pub fn render_response(self) -> axum::response::Html<String> {
                #[cfg(not(debug_assertions))]
                {
                    use askama::Template;
                    axum::response::Html(self.render().unwrap_or_else(|e| {
                        format!(r#"<div class="alert alert-danger">Template error: {}</div>"#, e)
                    }))
                }

                #[cfg(debug_assertions)]
                {
                    use $crate::utils::templates::render_template;
                    use serde_json::json;

                    let ctx = json!({ $(stringify!($field): self.$field,)* });
                    match render_template($path, ctx) {
                        Ok(html) => axum::response::Html(html),
                        Err(e) => axum::response::Html(format!(
                            r#"<div class="alert alert-danger"><strong>Template Error:</strong> {}</div>"#, e
                        )),
                    }
                }
            }
        }
    };
}
