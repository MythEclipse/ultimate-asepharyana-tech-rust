//! Core utilities - configuration, errors, JWT, rate limiting.
//!
//! This module groups essential framework components.

pub mod config;
pub mod types;
pub mod error;
pub mod ratelimit;

pub use config::CONFIG;

pub use ratelimit::rate_limit_middleware;

// Re-export AppError from utils for backward compatibility
pub use self::error::AppError;
