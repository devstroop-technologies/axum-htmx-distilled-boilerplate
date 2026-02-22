use std::sync::Arc;
use std::time::SystemTime;

use axum::{middleware, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use app::{
    config::AppConfig,
    handlers::{api::health, partials, templates},
    middleware as mw,
    models::AppState,
    services::Services,
    utils::logging,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config
    let config = AppConfig::load().unwrap_or_else(|e| {
        eprintln!("Config error: {}, using defaults", e);
        AppConfig::default()
    });

    // Init logging
    logging::init_logging(&config.logging.level)?;

    info!("Starting axum-htmx-app v{}", env!("CARGO_PKG_VERSION"));

    // Initialize services
    let services = Services::new_default(SystemTime::now());

    // Shared state with services
    let state = Arc::new(AppState::new(services));

    // CORS
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    // OpenAPI
    #[derive(OpenApi)]
    #[openapi(
        paths(health::health_check),
        components(schemas(health::HealthResponse)),
        tags((name = "Health", description = "Health check endpoints")),
        info(title = "Axum HTMX App", version = "0.1.0")
    )]
    struct ApiDoc;

    // ── Routes ──────────────────────────────────────────────────────────

    // API routes (JSON)
    let api_routes = Router::new()
        .route("/health", get(health::health_check))
        .with_state(state.clone());

    // HTMX partial routes (HTML fragments)
    let partial_routes = Router::new()
        .route("/partials/status-card", get(partials::status_card))
        .route("/partials/item-list", get(partials::item_list))
        .route("/partials/greeting", get(partials::greeting))
        .with_state(state.clone());

    // Page routes (full HTML)
    let app = Router::new()
        .route("/", get(templates::home_page))
        .route("/about", get(templates::about_page))
        .route("/demo", get(templates::demo_page))
        .nest("/api", api_routes)
        .merge(partial_routes)
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        // Swagger UI
        .merge(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // Middleware (applied bottom-up)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(middleware::from_fn(mw::request_logger))
                .layer(middleware::from_fn(mw::security_headers))
                .layer(cors),
        );

    // ── Start ───────────────────────────────────────────────────────────

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Listening on http://{}", addr);
    info!("Swagger UI at http://{}/api-docs/", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.ok();
            info!("Shutting down...");
        })
        .await?;

    Ok(())
}
