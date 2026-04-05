//! Request/Response logging middleware.
//!
//! Provides structured logging for HTTP requests with timing and correlation.
//!
//! # Example
//!
//! ```ignore
//! use rustexpress::middleware::logging::{LoggingConfig, with_logging};
//! use axum::Router;
//!
//! let app = Router::new()
//!     .route("/api/test", get(handler))
//!     .layer(axum::middleware::from_fn(with_logging(LoggingConfig::default())));
//! ```

use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use uuid::Uuid;

/// Logging configuration.
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Log request headers
    pub log_headers: bool,
    /// Log request body (be careful with sensitive data)
    pub log_body: bool,
    /// Log response body
    pub log_response_body: bool,
    /// Maximum body size to log (bytes)
    pub max_body_size: usize,
    /// Paths to exclude from logging
    pub exclude_paths: HashSet<String>,
    /// Log level for successful requests
    pub success_level: LogLevel,
    /// Log level for client errors (4xx)
    pub client_error_level: LogLevel,
    /// Log level for server errors (5xx)
    pub server_error_level: LogLevel,
}

/// Log level enum.
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        let mut exclude = HashSet::new();
        exclude.insert("/health".to_string());
        exclude.insert("/metrics".to_string());
        exclude.insert("/favicon.ico".to_string());

        Self {
            log_headers: false,
            log_body: false,
            log_response_body: false,
            max_body_size: 1024,
            exclude_paths: exclude,
            success_level: LogLevel::Info,
            client_error_level: LogLevel::Warn,
            server_error_level: LogLevel::Error,
        }
    }
}

impl LoggingConfig {
    /// Create a new logging config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable header logging.
    pub fn with_headers(mut self) -> Self {
        self.log_headers = true;
        self
    }

    /// Add a path to exclude from logging.
    pub fn exclude_path(mut self, path: &str) -> Self {
        self.exclude_paths.insert(path.to_string());
        self
    }
}

/// Request ID extension for correlation.
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

impl RequestId {
    /// Generate a new request ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Get the request ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

/// Logging middleware.
pub async fn logging_middleware(config: Arc<LoggingConfig>, req: Request, next: Next) -> Response {
    let path = req.uri().path();

    if config.exclude_paths.contains(path) {
        return next.run(req).await;
    }

    let request_id = req
        .headers()
        .get("X-Request-ID")
        .and_then(|h| h.to_str().ok())
        .map(|s| RequestId(s.to_string()))
        .unwrap_or_else(RequestId::new);

    let method = req.method().clone();
    let uri = req.uri().clone();
    let path = uri.path().to_string();
    let query = uri.query().map(str::to_owned);

    let headers_log = if config.log_headers {
        let headers: Vec<String> = req
            .headers()
            .iter()
            .filter(|(name, _)| !is_sensitive_header(name.as_str()))
            .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap_or("<binary>")))
            .collect();
        Some(headers)
    } else {
        None
    };

    let mut req = req;
    req.extensions_mut().insert(request_id.clone());

    let start = Instant::now();

    tracing::info!(
        request_id = %request_id.0,
        method = %method,
        path = %path,
        query = ?query,
        "→ Request started"
    );

    if let Some(ref headers) = headers_log {
        tracing::debug!(request_id = %request_id.0, headers = ?headers, "Request headers");
    }

    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    match status.as_u16() {
        100..=399 => log_response(
            config.success_level,
            &request_id,
            status,
            duration,
            &method,
            &path,
        ),
        400..=499 => log_response(
            config.client_error_level,
            &request_id,
            status,
            duration,
            &method,
            &path,
        ),
        _ => log_response(
            config.server_error_level,
            &request_id,
            status,
            duration,
            &method,
            &path,
        ),
    }

    response
}

fn is_sensitive_header(name: &str) -> bool {
    name.eq_ignore_ascii_case("authorization")
        || name.eq_ignore_ascii_case("cookie")
        || name.eq_ignore_ascii_case("x-api-key")
}

fn log_response(
    level: LogLevel,
    request_id: &RequestId,
    status: StatusCode,
    duration: std::time::Duration,
    method: &axum::http::Method,
    path: &str,
) {
    let duration_ms = duration.as_millis();

    match level {
        LogLevel::Trace => tracing::trace!(
            request_id = %request_id.0,
            status = %status,
            duration_ms = %duration_ms,
            method = %method,
            path = %path,
            "← Response completed"
        ),
        LogLevel::Debug => tracing::debug!(
            request_id = %request_id.0,
            status = %status,
            duration_ms = %duration_ms,
            method = %method,
            path = %path,
            "← Response completed"
        ),
        LogLevel::Info => tracing::info!(
            request_id = %request_id.0,
            status = %status,
            duration_ms = %duration_ms,
            method = %method,
            path = %path,
            "← Response completed"
        ),
        LogLevel::Warn => tracing::warn!(
            request_id = %request_id.0,
            status = %status,
            duration_ms = %duration_ms,
            method = %method,
            path = %path,
            "← Response completed"
        ),
        LogLevel::Error => tracing::error!(
            request_id = %request_id.0,
            status = %status,
            duration_ms = %duration_ms,
            method = %method,
            path = %path,
            "← Response completed"
        ),
    }
}

/// Create logging middleware.
pub fn with_logging(
    config: LoggingConfig,
) -> impl Fn(
    Request<Body>,
    Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>
       + Clone
       + Send {
    let config = Arc::new(config);
    move |req: Request<Body>, next: Next| {
        let config = config.clone();
        Box::pin(async move { logging_middleware(config, req, next).await })
    }
}

/// Extractor for RequestId in route handlers.
impl<S> axum::extract::FromRequestParts<S> for RequestId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<RequestId>().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "RequestId not found. Did you add logging middleware?",
        ))
    }
}
