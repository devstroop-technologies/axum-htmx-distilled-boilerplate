use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize tracing/logging based on config
pub fn init_logging(log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filter = if log_level.contains('=') {
        log_level.to_string()
    } else {
        format!("app={},tower_http=debug", log_level)
    };

    let env_filter =
        EnvFilter::try_new(&filter).unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_level(true),
        )
        .init();

    Ok(())
}
