// Library root - clean organized module structure
// All modules organized into logical folders

// ============================================================================
// Core Framework
// ============================================================================
pub mod core; // config, error, ratelimit

// ============================================================================
// Infrastructure
// ============================================================================
pub mod infra; // redis, http_client, proxy, image_proxy

// ============================================================================
// Features
// ============================================================================
pub mod browser; // Browser tab pooling for scraping
pub mod circuit_breaker; // Circuit breaker for external services
pub mod events; // Event bus (pub/sub)
pub mod graceful; // Graceful shutdown with signals
pub mod health; // Health check endpoints
pub mod helpers; // Utility helpers (string, datetime, file)
pub mod jobs; // Background job processing
pub mod middleware; // logging, request_id, cors, compression middleware
pub mod observability; // Metrics, request ID, tracing
pub mod scheduler; // Cron jobs

// ============================================================================
// Data Layer
// ============================================================================
pub mod entities; // SeaORM entities
pub mod models; // Data models + types
pub mod services; // Domain services

// ============================================================================
// Application-Specific (Scraping)
// ============================================================================
pub mod scraping; // URLs, CDN, base URLs

// ============================================================================
// Build & Routes
// ============================================================================
#[path = "../build_utils/mod.rs"]
pub mod build_utils;
pub mod routes;
pub mod bootstrap;
