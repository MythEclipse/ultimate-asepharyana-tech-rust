//! Observability utilities: metrics, tracing, request ID, and documentation.

pub mod request_id;
pub mod metrics;
pub mod openapi;
pub mod openapi_generated;

pub use request_id::RequestId;
pub use metrics::setup_metrics;
