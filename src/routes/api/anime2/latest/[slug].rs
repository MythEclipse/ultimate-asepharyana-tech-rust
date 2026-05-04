use axum::Router;
use std::sync::Arc;
use crate::routes::AppState;
use crate::helpers::api_response::{internal_err, ApiResult, ApiResponse};
use crate::helpers::{fetch_html_with_retry, Cache};
use axum::extract::{Path, State};

use serde_json::json;
use tracing::info;

// Import shared models and parsers
use crate::models::anime2::{LatestAnimeItem, Pagination};
use crate::scraping::anime2 as parsers;
use crate::scraping::anime::cache as cache_utils;


// Removed LatestQuery struct as it is now path-based

const CACHE_TTL: u64 = 120;




#[utoipa::path(




    get,




    path = "/api/anime2/latest/{slug}",




    tag = "anime2",




    operation_id = "anime2_latest_slug",




    responses(




        (status = 200, description = "Retrieves details for a specific latest by slug.", body = serde_json::Value),




        (status = 500, description = "Internal Server Error", body = String)




    )




)]




pub async fn latest(
    State(app_state): State<Arc<AppState>>,
    Path(slug): Path<String>,
) -> ApiResult<Vec<LatestAnimeItem>> {
    let page = slug.parse::<u32>().unwrap_or(1);
    info!("anime2 latest request, page: {}", page);

    let cache_key = format!("anime2:latest:{}", page);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (data, pagination) = fetch_latest_anime(page).await.map_err(|e| e.to_string())?;

            // Use shared cache utility for poster caching
            let updated_data = cache_utils::cache_and_update_posters(&app_state, data).await;

            Ok(ApiResponse::success_with_meta(
                updated_data,
                json!({ "pagination": pagination }),
            ))
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(response)
}

async fn fetch_latest_anime(
    page: u32,
) -> Result<(Vec<LatestAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://alqanime.si/anime/page/{}/?status=&type=&order=latest",
        page
    );

    let html = fetch_html_with_retry(&url)
        .await
        .map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_latest_page(&html, page)).await??;

    Ok((anime_list, pagination))
}

fn parse_latest_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<LatestAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = crate::helpers::parse_html(html);

    // Use shared parser for anime items
    let anime_list = parsers::parse_latest_anime(html)?;

    // Use shared parser for pagination
    let pagination = parsers::parse_pagination(&document, current_page)?;

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route("/api/anime2/latest/{slug}", axum::routing::get(latest))
}