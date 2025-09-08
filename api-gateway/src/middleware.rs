use axum::{
    http::{Request, HeaderValue},
    middleware::Next,
    response::Response,
    body::Body,
};
use tracing::info;

// Security headers middleware
pub async fn security_headers(
    request: Request<Body>,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    
    let mut response = response;
    let headers = response.headers_mut();
    
    // Add security headers equivalent to helmet middleware
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
    headers.insert("Strict-Transport-Security", HeaderValue::from_static("max-age=31536000; includeSubDomains"));
    headers.insert("Referrer-Policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
    headers.insert("Content-Security-Policy", HeaderValue::from_static("default-src 'self'"));
    
    response
}

// Request logging middleware (equivalent to morgan)
pub async fn request_logging(
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let user_agent = request
        .headers()
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("-")
        .to_string();
    
    let start = std::time::Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed();
    
    let status = response.status();
    
    info!(
        "{} {} {} - {} - {:?}",
        method,
        uri,
        status.as_u16(),
        user_agent,
        duration
    );
    
    response
}
