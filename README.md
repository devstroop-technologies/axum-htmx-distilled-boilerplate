# Hardened Axum + HTMX Web Application

A security-first, full-stack Rust web application. No JSON API. No external dependencies. No JavaScript framework. Suitable for high-security deployments, internal tools, or Tor hidden services.

## Security Model

| Threat | Mitigation |
|---|---|
| XSS | Strict CSP, no inline scripts, SRI on all JS files |
| CSRF | Per-session HMAC tokens, auto-sent via HTMX headers |
| Clickjacking | `X-Frame-Options: DENY`, `frame-ancestors 'none'` |
| Supply chain | Zero npm, zero CDN — all assets vendored locally |
| API abuse | No JSON API exists — only HTML pages and fragments |
| Template injection | Askama compiles templates at build time |
| Session theft | HttpOnly + SameSite=Strict cookies, server-side sessions |
| Fingerprinting | No server header, no referrer, no DNS prefetch |
| DNS leaks | `X-DNS-Prefetch-Control: off`, `Referrer-Policy: no-referrer` |

## Stack

| Layer | Tech | Role |
|---|---|---|
| Runtime | [Tokio](https://tokio.rs) | Async runtime |
| Framework | [Axum](https://github.com/tokio-rs/axum) 0.7 | HTTP routing, middleware, state |
| Interactivity | [HTMX](https://htmx.org) | Swap HTML fragments — single vendored file, SRI-pinned |
| Templates (dev) | [MiniJinja](https://github.com/mitsuhiko/minijinja) | Hot-reload from disk |
| Templates (release) | [Askama](https://github.com/djc/askama) | Compiled into the binary at build time |
| Styling | Custom CSS (~5 KB) | Dark-mode-ready design system, no framework |
| Error Handling | [thiserror](https://github.com/dtolnay/thiserror) + [anyhow](https://github.com/dtolnay/anyhow) | Typed errors with HTMX-aware HTML responses |

## Quick Start

```bash
cargo run
```

Open [http://localhost:8000](http://localhost:8000).

## How It Works

```
Browser                 Server
  │                       │
  │  GET /about           │   full HTML page (server-rendered)
  │──────────────────────▶│   + Set-Cookie: __Host-sid=... (HttpOnly)
  │◀──────────────────────│   + X-CSRF-Token: ...
  │                       │
  │  GET /partials/       │   HTML fragment (HTMX swap)
  │  status-card          │   + X-CSRF-Token header sent
  │──────────────────────▶│
  │◀──────────────────────│
  │                       │
  │  POST /submit         │   CSRF validated, session verified
  │  X-CSRF-Token: ...    │   responds with HTML fragment
  │──────────────────────▶│
  │◀──────────────────────│
```

Two response modes:

1. **Pages** — full HTML documents served on navigation (`/`, `/about`, `/demo`).
2. **Partials** — HTML fragments fetched by HTMX and swapped into the DOM (`/partials/status-card`, `/partials/item-list`, `/partials/greeting`).

Templates hot-reload during development (`cargo run`) and compile into the binary in release builds (`cargo build --release`).

## Project Layout

```
├── config/
│   └── app.toml                 # Server, logging & env settings
├── src/
│   ├── bin/main.rs              # Entry point: router, middleware, server
│   ├── lib.rs                   # Crate root, module declarations
│   ├── config.rs                # TOML config loader (env override support)
│   ├── error.rs                 # AppError — HTMX-aware error responses
│   ├── render.rs                # define_page! / define_partial! macros
│   ├── handlers/
│   │   ├── templates.rs         # Full-page route handlers (with CSRF)
│   │   └── partials.rs          # HTMX fragment handlers
│   ├── services/
│   │   ├── mod.rs               # Service container (DI via Arc<dyn Trait>)
│   │   ├── csrf.rs              # CSRF token generation + HMAC validation
│   │   ├── session.rs           # Server-side session management
│   │   ├── health.rs            # Health check service
│   │   └── items.rs             # Item CRUD (in-memory, DB-ready)
│   ├── middleware/mod.rs        # Security headers, CSRF, sessions, logging
│   ├── models/mod.rs            # Shared AppState
│   └── utils/
│       ├── logging.rs           # tracing/tracing-subscriber init
│       └── templates.rs         # MiniJinja hot-reload helper
├── templates/
│   ├── base.html                # Root layout (sidebar, header, theme toggle)
│   ├── pages/                   # Full-page templates
│   ├── partials/                # Fragment templates
│   └── components/              # Reusable design tokens
└── static/
    ├── css/app.css              # ~5 KB design system
    ├── css/bootstrap-icons.*    # Vendored icon font styles
    ├── fonts/                   # Vendored Bootstrap Icons woff/woff2
    ├── js/htmx.min.js           # Vendored HTMX (~14 KB gzip), SRI-pinned
    └── js/app.js                # Minimal UI (~30 lines), SRI-pinned
```

## Configuration

Default settings live in `config/app.toml`:

```toml
[server]
host = "0.0.0.0"
port = 8000

[logging]
level = "info"
```

Override any value with environment variables using the `APP__` prefix and `__` as the nesting separator:

```bash
APP__SERVER__PORT=8080 cargo run
```

## Adding a Page

1. Create a template at `templates/pages/mypage.html` (extend `base.html`).
2. Define the struct and handler in `src/handlers/templates.rs`:

```rust
crate::define_page!(MyPage, "pages/mypage.html", { current_page: &'static str, csrf_token: String });

pub async fn my_page(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let sid = get_session_id(&headers).unwrap_or_default();
    let csrf_token = state.services.csrf.generate_token(&sid);
    MyPage { current_page: "mypage", csrf_token }.render_response()
}
```

3. Register the route in `src/bin/main.rs`:

```rust
.route("/mypage", get(templates::my_page))
```

## Adding a Partial

1. Create a template at `templates/partials/widget.html`.
2. Define the struct and handler in `src/handlers/partials.rs`:

```rust
crate::define_partial!(Widget, "partials/widget.html", { label: String });

pub async fn widget() -> impl IntoResponse {
    Widget { label: "hello".into() }.render_response()
}
```

3. Register the route in `src/bin/main.rs`:

```rust
.route("/partials/widget", get(partials::widget))
```

4. Trigger from any template:

```html
<div hx-get="/partials/widget" hx-swap="innerHTML"></div>
```

## Key Design Decisions

- **`define_page!` / `define_partial!` macros** — a single declaration generates both the Askama compiled template (release) and the MiniJinja hot-reloading template (debug), eliminating boilerplate.
- **Trait-based service layer** — services are injected as `Arc<dyn Trait>`, making it straightforward to swap in-memory implementations for database-backed ones or test doubles.
- **HTMX-aware error handling** — `AppError` renders HTML fragments with `HX-Retarget` and `HX-Reswap` headers so errors automatically appear in a toast/notification area.
- **CSRF on every mutation** — per-session HMAC-SHA256 tokens are rotated on each page load and sent automatically by HTMX via `hx-headers` on the `<body>` tag.
- **SRI integrity hashes** — both `htmx.min.js` and `app.js` have `integrity` attributes. If a single byte changes, the browser refuses to execute them.
- **Strict CSP** — `script-src` only allows self + SRI hashes. No `unsafe-inline`, no `unsafe-eval`. Even injected `<script>` tags are blocked.
- **Zero external dependencies** — no CDN calls, no remote fonts, no analytics. The entire app is self-contained. Works fully offline or on .onion.
- **Minimal JS footprint** — two JS files: HTMX (~14 KB gzipped) and app.js (~30 lines). Both vendored, both SRI-pinned, both fully auditable.

## Tor Hidden Service Deployment

This app is optimized for Tor deployment:

- No external resource requests (fonts, CDN, analytics)
- `Referrer-Policy: no-referrer` prevents referrer leaks
- `X-DNS-Prefetch-Control: off` prevents DNS leaks
- No `target="_blank"` links (prevents `window.opener` attacks)
- Server header stripped (no tech fingerprinting)
- `Cross-Origin-*` policies set to `same-origin`

To deploy as a Tor hidden service, configure your `torrc`:

```
HiddenServiceDir /var/lib/tor/myapp/
HiddenServicePort 80 127.0.0.1:8000
```

## Optional Features

Enable SQLite support via the `database` feature flag:

```bash
cargo run --features database
```

This pulls in [SQLx](https://github.com/launchbadge/sqlx) with the `runtime-tokio` and `sqlite` drivers.

## License

MIT
