//! HTTP Middleware

use axum::{extract::Request, middleware::Next, response::Response};

/// Security headers middleware — adds standard security headers to all responses
pub async fn security_headers(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    headers.insert("x-content-type-options", "nosniff".parse().unwrap());
    headers.insert("x-frame-options", "DENY".parse().unwrap());
    headers.insert("x-xss-protection", "1; mode=block".parse().unwrap());
    headers.insert(
        "referrer-policy",
        "strict-origin-when-cross-origin".parse().unwrap(),
    );
    headers.insert(
        "content-security-policy",
        "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'"
            .parse()
            .unwrap(),
    );

    response
}

/// Request logging middleware — logs method, path, status and duration
pub async fn request_logger(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let start = std::time::Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed();
    tracing::info!(
        method = %method,
        path = %path,
        status = response.status().as_u16(),
        duration_ms = duration.as_millis() as u64,
        "request"
    );

    response
}
