// Library root - clean organized module structure
// All modules organized into logical folders

// ============================================================================
// Core Framework
// ============================================================================
pub mod core; // config, error, ratelimit
pub mod shared; // errors, utils

// ============================================================================
// Infrastructure
// ============================================================================
pub mod infra; // redis, http_client, proxy, image_proxy
pub mod presentation; // API Handlers, DTOs

// ============================================================================
// Features
// ============================================================================
pub mod browser; // Browser tab pooling for scraping
pub mod events; // Event bus (pub/sub)
pub mod graceful; // Graceful shutdown with signals
pub mod health; // Health check endpoints
pub mod jobs; // Background job processing
pub mod middleware; // logging, request_id, cors, compression middleware
pub mod observability; // request ID, tracing
pub mod scheduler; // Cron jobs

// ============================================================================
// Data Layer (Legacy - Prefer src/core)
// ============================================================================
#[deprecated(note = "Legacy module for existing routes. Use crate::core instead.")]
pub mod entities; // SeaORM entities
#[deprecated(note = "Legacy module for existing routes. Use crate::core instead.")]
pub mod models; // Data models + types
#[deprecated(note = "Legacy module for existing routes. Use crate::core instead.")]
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
