# Axum + HTMX Boilerplate

Full-stack Rust web application with SPA-like capabilities — no JavaScript framework required.

## Architecture

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Backend** | [Axum](https://github.com/tokio-rs/axum) | Async web framework on Tokio |
| **Frontend** | [HTMX](https://htmx.org) | SPA-like navigation via HTML fragments |
| **Reactivity** | Vanilla JS | Zero-dependency client-side interactivity |
| **Styling** | Custom CSS (~5KB) | Minimal design system with dark mode |
| **Templates (dev)** | [minijinja](https://github.com/mitsuhiko/minijinja) | Hot-reload from disk |
| **Templates (release)** | [askama](https://github.com/djc/askama) | Compiled into binary, zero overhead |
| **API docs** | [utoipa](https://github.com/juhaku/utoipa) + Swagger UI | OpenAPI spec + interactive docs |

## Design Principles

- **Minimal dependencies**: ~19KB JS (HTMX only) vs ~85KB (HTMX + Alpine + Bootstrap)
- **DRY templates**: `define_page!` / `define_partial!` macros eliminate duplication
- **Service layer**: Trait-based abstractions for testability
- **HTMX-aware errors**: Auto-routed to toast notifications with proper HTTP codes

## How It Works

1. **Full pages** served on navigation (`GET /`, `/about`, `/demo`)
2. **HTMX partials** fetched as HTML fragments (`GET /partials/status-card`)
3. HTMX swaps fragments into the DOM — no full page reload
4. **REST API** returns JSON (`GET /api/health`)
5. Templates hot-reload in `cargo run` (debug), compiled in `cargo build --release`

## Quick Start

```bash
cargo run
# → http://localhost:3001
# → http://localhost:3001/api-docs/  (Swagger UI)
```

## Project Structure

```
src/
  bin/main.rs            # Entry point — router, middleware, server
  lib.rs                 # Library root with macro exports
  config.rs              # TOML configuration loader
  error.rs               # Error types with HTMX-aware HTML responses
  render.rs              # define_page! / define_partial! macros
  handlers/
    templates.rs         # Full-page handlers (3 lines each!)
    partials.rs          # HTMX partial handlers (HTML fragments)
    api/health.rs        # JSON REST API endpoints
  services/
    mod.rs               # Service container
    health.rs            # Health status service
    items.rs             # Item CRUD service (in-memory / DB-ready)
  middleware/mod.rs      # Security headers, request logging
  models/mod.rs          # Shared application state
  utils/
    logging.rs           # Tracing/logging setup
    templates.rs         # minijinja hot-reload helper

templates/
  base.html              # Layout: sidebar, header, theme toggle
  pages/                 # Full page templates
  partials/              # HTML fragment templates
  components/            # Design tokens

static/
  css/app.css            # Minimal CSS framework (~5KB)
  css/bootstrap-icons.*  # Icon font CSS
  fonts/                 # Bootstrap Icons webfonts (woff/woff2)
  js/htmx.min.js         # HTMX library (~14KB gzip)
  favicon.svg            # Replaceable placeholder

config/app.toml          # Application config
```

## Configuration

Edit `config/app.toml` or use environment variables with `APP__` prefix:

```bash
APP__SERVER__PORT=8080 cargo run
```

## Adding a New Page

1. Create `templates/pages/mypage.html` extending `base.html`
2. Add to `src/handlers/templates.rs`:
   ```rust
   crate::define_page!(MyPage, "pages/mypage.html", { current_page: &'static str });
   
   pub async fn my_page() -> impl IntoResponse {
       MyPage { current_page: "mypage" }.render_response()
   }
   ```
3. Add route in `src/bin/main.rs`: `.route("/mypage", get(templates::my_page))`

## Adding a New Partial

1. Create `templates/partials/my_partial.html`
2. Add to `src/handlers/partials.rs`:
   ```rust
   crate::define_partial!(MyPartial, "partials/my_partial.html", { data: String });
   
   pub async fn my_partial() -> impl IntoResponse {
       MyPartial { data: "hello".into() }.render_response()
   }
   ```
3. Add route: `.route("/partials/my-partial", get(partials::my_partial))`
4. Use in templates: `hx-get="/partials/my-partial"`
