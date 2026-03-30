use axum::{
    extract::{Query, State},
    response::Response,
    Router,
};
use http::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

use crate::infra::proxy::fetch_with_proxy;
use crate::routes::AppState;
use crate::core::error::AppError;


#[derive(Debug, Deserialize)]
pub struct ProxyParams {
    url: String,
}

/// Handles GET requests for the proxy endpoint.
pub async fn fetch_with_proxy_only(
    _: State<Arc<AppState>>,
    Query(params): Query<ProxyParams>,
) -> Result<Response, AppError> {
    let slug = params.url;
    match fetch_with_proxy(&slug).await {
        Ok(fetch_result) => {
            let mut response_builder = Response::builder().status(StatusCode::OK);

            if let Some(content_type) = fetch_result.content_type {
                response_builder = response_builder.header("Content-Type", content_type);
            }

            Ok(response_builder.body(fetch_result.data.into())?)
        }
        Err(e) => {
            eprintln!("Proxy fetch error: {:?}", e);
            Err(AppError::Other(format!(
                "Failed to fetch URL via proxy: {}",
                e
            )))
        }
    }
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route("/api/proxy/croxy", axum::routing::get(fetch_with_proxy_only))
}