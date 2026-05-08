// Library root - clean organized module structure
// All modules organized into logical folders

// ============================================================================
// Core Framework
// ============================================================================
pub mod domain;
pub mod application;
pub mod shared;
pub mod infra;
pub mod presentation;

pub mod browser;
pub mod events;
pub mod graceful;
pub mod health;
pub mod jobs;
pub mod middleware;
pub mod observability;
pub mod scheduler;
pub mod scraping;

#[path = "../build_utils/mod.rs"]
pub mod build_utils;
pub mod bootstrap;

