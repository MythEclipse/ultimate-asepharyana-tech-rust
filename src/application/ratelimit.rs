//! Rate limiter middleware using the governor crate.
//!
//! Provides token-bucket based rate limiting with configurable limits.
//! Default: 20 requests per IP address, burst 50.

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use governor::{
    clock::DefaultClock,
    state::keyed::DefaultKeyedStateStore,
    Quota, RateLimiter as GovernorRateLimiter,
};
use once_cell::sync::Lazy;
use serde_json::json;
use std::{num::NonZeroU32, sync::Arc, time::Duration};
use tracing::warn;

/// Global IP-keyed rate limiter instance.
/// Configured for 20 requests per second with a burst of 50.
static IP_LIMITER: Lazy<Arc<GovernorRateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>>> =
    Lazy::new(|| {
        let quota = Quota::with_period(Duration::from_millis(50))
            .unwrap()
            .allow_burst(NonZeroU32::new(50).unwrap());

        Arc::new(GovernorRateLimiter::keyed(quota))
    });

/// Rate limiter configuration.
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Requests per second
    pub requests_per_second: u32,
    /// Burst size (max requests that can be made instantly)
    pub burst_size: u32,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 20,
            burst_size: 50,
        }
    }
}

/// Create a custom rate limiter with specific configuration.
pub fn create_rate_limiter(
    config: RateLimiterConfig,
) -> Arc<GovernorRateLimiter<String, DefaultKeyedStateStore<String>, DefaultClock>> {
    let period_ms = 1000 / config.requests_per_second.max(1);
    let quota = Quota::with_period(Duration::from_millis(period_ms as u64))
        .unwrap()
        .allow_burst(NonZeroU32::new(config.burst_size.max(1)).unwrap());

    Arc::new(GovernorRateLimiter::keyed(quota))
}

fn extract_ip(req: &Request) -> String {
    if let Some(ip) = req.headers().get("X-Forwarded-For") {
        if let Ok(ip_str) = ip.to_str() {
            if let Some(first_ip) = ip_str.split(',').next() {
                return first_ip.trim().to_string();
            }
        }
    }
    if let Some(ip) = req.headers().get("X-Real-IP") {
        if let Ok(ip_str) = ip.to_str() {
            return ip_str.trim().to_string();
        }
    }
    "unknown".to_string()
}

/// Rate limiting middleware using the IP-keyed limiter.
///
/// Returns 429 Too Many Requests if the limit is exceeded.
pub async fn rate_limit_middleware(req: Request, next: Next) -> Response {
    let client_ip = extract_ip(&req);

    match IP_LIMITER.check_key(&client_ip) {
        Ok(_) => next.run(req).await,
        Err(_) => {
            warn!("Rate limit exceeded for IP: {}", client_ip);
            (
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "error": "Too many requests",
                    "code": "RATE_LIMIT_EXCEEDED",
                    "retry_after_ms": 1000
                })),
            )
                .into_response()
        }
    }
}
