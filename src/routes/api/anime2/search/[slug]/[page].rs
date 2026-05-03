use axum::Router;
use std::sync::Arc;
use crate::routes::AppState;
use crate::helpers::api_response::{internal_err, ApiResult, ApiResponse};
use crate::helpers::{fetch_html_with_retry, parse_html, Cache};
use axum::extract::{Path, State};

use serde_json::json;
use tracing::info;

// Import shared models and parsers
use crate::models::anime2::{PaginationWithStringPages, SearchAnimeItem};
use crate::scraping::anime2 as parsers;
use crate::scraping::anime::cache as cache_utils;


const CACHE_TTL: u64 = 300; // 5 minutes

// Removed SearchQuery struct as it is now path-based




#[utoipa::path(




    get,




    path = "/api/anime2/search/{slug}/{page}",




    tag = "anime2",




    operation_id = "anime2_search_slug_page",




    responses(




        (status = 200, description = "Handles GET requests for the /api/anime2/search/[slug]/[page] endpoint.", body = serde_json::Value),




        (status = 500, description = "Internal Server Error", body = String)




    )




)]




pub async fn page(
    State(app_state): State<Arc<AppState>>,
    Path((slug, page)): Path<(String, u32)>,
) -> ApiResult<Vec<SearchAnimeItem>> {
    let query = slug;
    info!("Starting search for query: {}, page: {}", query, page);

    let cache_key = format!("anime2:search:{}:{}", query, page);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let url = if page == 1 {
                format!("https://alqanime.si/?s={}", urlencoding::encode(&query))
            } else {
                format!("https://alqanime.si/page/{}/?s={}", page, urlencoding::encode(&query))
            };
            let (data, pagination) = fetch_and_parse_search(&url, page)
                .await
                .map_err(|e| e.to_string())?;

            // Trigger lazy batch caching using shared utility
            cache_utils::cache_posters(&app_state, &data).await;

            // We return original data for speed on cold start

            Ok(ApiResponse::success_with_meta(
                data,
                json!({ "pagination": pagination, "status": "Ok" }),
            ))
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(response)
}

async fn fetch_and_parse_search(
    url: &str,
    page: u32,
) -> Result<(Vec<SearchAnimeItem>, PaginationWithStringPages), Box<dyn std::error::Error + Send + Sync>> {
    let html = fetch_html_with_retry(url).await?;
    let (data, pagination) = tokio::task::spawn_blocking(move || {
        parse_search_document(&html, page)
    })
    .await??;

    Ok((data, pagination))
}

fn parse_search_document(
    html: &str,
    page: u32,
) -> Result<(Vec<SearchAnimeItem>, PaginationWithStringPages), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);

    // Parse anime items using shared parser
    let data = parsers::parse_search_anime(html)?;

    // Parse pagination using shared parser
    let current_page = page;
    let pagination = parsers::parse_pagination_with_string(&document, current_page)?;

    Ok((data, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route("/api/anime2/search/{slug}/{page}.rs", axum::routing::get(page))
}