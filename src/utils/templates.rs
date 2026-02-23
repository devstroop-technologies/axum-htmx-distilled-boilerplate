use serde::Serialize;
#[cfg(debug_assertions)]
use minijinja::Environment;

/// Render a template from disk (debug mode hot-reload).
/// In release mode, askama compiled templates are used instead.
#[cfg(debug_assertions)]
pub fn render_template<T: Serialize>(name: &str, context: T) -> Result<String, String> {
    let mut env = Environment::new();
    env.set_loader(minijinja::path_loader("templates"));

    let template = env
        .get_template(name)
        .map_err(|e| format!("Template load error: {}", e))?;
    template
        .render(context)
        .map_err(|e| format!("Template render error: {}", e))
}

#[cfg(not(debug_assertions))]
pub fn render_template<T: Serialize>(_name: &str, _context: T) -> Result<String, String> {
    Err("Runtime templates not available in release mode".to_string())
}
