use axum::Router;
use std::sync::Arc;
use crate::routes::AppState;
use crate::helpers::api_response::{internal_err, ApiResult, ApiResponse};
use crate::helpers::{fetch_html_with_retry, parse_html, Cache};
use axum::extract::{Path, State};

use serde::Deserialize;
use utoipa::ToSchema;
use serde_json::json;
use tracing::info;

// Import shared models and parsers
use crate::models::anime2::{GenreAnimeItem, Pagination};
use crate::scraping::anime2 as parsers;


#[derive(Deserialize, ToSchema)]
pub struct GenreQuery {
    pub page: Option<u32>,
    pub status: Option<String>,
    pub order: Option<String>,
}

const CACHE_TTL: u64 = 300;
















#[utoipa::path(
















    get,
















    path = "/api/anime2/genre/{slug}",
















    tag = "anime2",
















    operation_id = "anime2_genre_slug_index",
















    responses(
















        (status = 200, description = "Retrieves details for a specific genre by slug.", body = serde_json::Value),
















        (status = 500, description = "Internal Server Error", body = String)
















    )
















)]
















pub async fn slug(
    State(app_state): State<Arc<AppState>>,
    Path(genre_slug): Path<String>,
) -> ApiResult<Vec<GenreAnimeItem>> {
    let page = 1u32;
    let status = String::new();
    let order = "update".to_string();

    info!(
        "anime2 genre request: {}, page: {}, status: {}, order: {}",
        genre_slug, page, status, order
    );

    let cache_key = format!("anime2:genre:{}:{}:{}:{}", genre_slug, page, status, order);
    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set(&cache_key, CACHE_TTL, || async {
            let (data, pagination) =
                fetch_genre_anime(&genre_slug, page, &status, &order)
                    .await
                    .map_err(|e: Box<dyn std::error::Error + Send + Sync>| e.to_string())?;

            // Convert all poster URLs to CDN URLs concurrently
            let posters: Vec<String> = data.iter().map(|i| i.poster.clone()).collect();
            crate::services::images::cache::cache_image_urls_batch_lazy(
                app_state.db.clone(),
                &app_state.redis_pool,
                posters,
                Some(app_state.image_processing_semaphore.clone()),
            )
            .await;

            Ok(ApiResponse::success_with_meta(
                data,
                json!({
                    "pagination": pagination,
                    "genre": genre_slug,
                    "status": "Ok"
                }),
            ))
        })
        .await
        .map_err(|e| internal_err(&e))?;

    Ok(response)
}

async fn fetch_genre_anime(
    genre_slug: &str,
    page: u32,
    status: &str,
    order: &str,
) -> Result<(Vec<GenreAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let mut url = if page > 1 {
        format!(
            "https://alqanime.si/anime/page/{}/?genre[]={}",
            page, genre_slug
        )
    } else {
        format!("https://alqanime.si/anime/?genre[]={}", genre_slug)
    };

    if !status.is_empty() {
        url.push_str(&format!("&status={}", status));
    }
    url.push_str(&format!("&order={}", order));

    let html = fetch_html_with_retry(&url).await.map_err(|e| format!("Failed to fetch HTML: {}", e))?;

    let (anime_list, pagination) =
        tokio::task::spawn_blocking(move || parse_genre_page(&html, page)).await??;

    Ok((anime_list, pagination))
}

fn parse_genre_page(
    html: &str,
    current_page: u32,
) -> Result<(Vec<GenreAnimeItem>, Pagination), Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);

    // Parse anime items using shared parser
    let anime_list = parsers::parse_genre_anime(html)?;

    // Parse pagination using shared parser
    let pagination = parsers::parse_pagination(&document, current_page)?;

    Ok((anime_list, pagination))
}

pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.route("/api/anime2/genre/{slug}/index.rs", axum::routing::get(slug))
}