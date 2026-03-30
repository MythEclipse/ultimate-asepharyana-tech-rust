use utoipa::OpenApi;
use crate::observability::openapi_generated::GeneratedApiDoc;

/// Bridge for the auto-generated OpenAPI documentation.
/// This allows merging manual schema definitions with the auto-discovered handlers and schemas.
#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/", api = GeneratedApiDoc)
    ),
    info(
        title = "RustExpress API",
        version = "1.0.0",
        description = "High-performance scraping and CDN microservice"
    )
)]
pub struct ApiDoc;
