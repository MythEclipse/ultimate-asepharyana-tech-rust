use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Json, Router};
use std::sync::Arc;
use crate::routes::AppState;
use crate::core::use_cases::CacheImageUseCase;
use crate::infra::repositories::mysql_image_repository::MySqlImageRepository;
use crate::shared::errors::AppError;
use crate::core::types::ApiResponse;
use crate::scraping::urls::get_otakudesu_url;
use crate::helpers::{parse_html, Cache, fetch_html_with_retry, text_from_or, attr_from_or, selector, extract_slug, attr_from};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct OngoingAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub current_episode: String,
    pub anime_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct CompleteAnimeItem {
    pub title: String,
    pub slug: String,
    pub poster: String,
    pub episode_count: String,
    pub anime_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct AnimeData {
    pub ongoing_anime: Vec<OngoingAnimeItem>,
    pub complete_anime: Vec<CompleteAnimeItem>,
}

use crate::helpers::cache_ttl::CACHE_TTL_VERY_SHORT;
const CACHE_TTL: u64 = CACHE_TTL_VERY_SHORT;

#[utoipa::path(
    get,
    path = "/api/anime",
    tag = "anime",
    operation_id = "anime_index",
    responses(
        (status = 200, description = "Handles GET requests for the /api/anime endpoint.", body = serde_json::Value),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn anime_index(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let start_time = std::time::Instant::now();
    info!("Handling request for anime index");

    let cache = Cache::new(&app_state.redis_pool);

    let response = cache
        .get_or_set("anime:index", CACHE_TTL, || async {
            let mut data = fetch_anime_data()
                .await
                .map_err(|e| format!("Fetch error: {}", e))?;

            let repo = Arc::new(MySqlImageRepository::new((*app_state.db).clone()));
            let cache_image_use_case = CacheImageUseCase::new(repo, app_state.redis_pool.clone())
                .with_semaphore(app_state.image_processing_semaphore.clone());

            let mut all_posters = Vec::new();
            for item in &data.ongoing_anime {
                all_posters.push(item.poster.clone());
            }
            for item in &data.complete_anime {
                all_posters.push(item.poster.clone());
            }

            let mut cached_urls = Vec::new();
            for poster_url in all_posters {
                match cache_image_use_case.execute(&poster_url).await {
                    Ok(cdn_url) => cached_urls.push(cdn_url),
                    Err(_) => cached_urls.push(poster_url),
                }
            }

            let ongoing_len = data.ongoing_anime.len();
            for (i, item) in data.ongoing_anime.iter_mut().enumerate() {
                item.poster = cached_urls[i].clone();
            }
            for (i, item) in data.complete_anime.iter_mut().enumerate() {
                item.poster = cached_urls[ongoing_len + i].clone();
            }

            Ok(ApiResponse::success(data))
        })
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    info!("Anime index completed in {:?}", start_time.elapsed());
    Ok(Json(response))
}

async fn fetch_anime_data() -> Result<AnimeData, Box<dyn std::error::Error + Send + Sync>> {
    let ongoing_url = format!("{}/ongoing-anime/", get_otakudesu_url());
    let complete_url = format!("{}/complete-anime/", get_otakudesu_url());

    let (ongoing_html, complete_html) = tokio::join!(
        fetch_html_with_retry(&ongoing_url),
        fetch_html_with_retry(&complete_url)
    );

    let ongoing_html = ongoing_html?;
    let complete_html = complete_html?;

    let ongoing_anime =
        tokio::task::spawn_blocking(move || parse_ongoing_anime(&ongoing_html)).await??;
    let complete_anime =
        tokio::task::spawn_blocking(move || parse_complete_anime(&complete_html)).await??;

    Ok(AnimeData {
        ongoing_anime,
        complete_anime,
    })
}

fn parse_ongoing_anime(
    html: &str,
) -> Result<Vec<OngoingAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut ongoing_anime = Vec::new();

    let venz_selector = selector(".venz ul li").unwrap();
    let title_selector = selector(".thumbz h2.jdlflm").unwrap();
    let link_selector = selector("a").unwrap();
    let img_selector = selector("img").unwrap();
    let episode_selector = selector(".epz").unwrap();

    for element in document.select(&venz_selector) {
        let title = text_from_or(&element, &title_selector, "");
        let href = attr_from(&element, &link_selector, "href").unwrap_or_default();
        let slug = extract_slug(&href);
        let poster = attr_from_or(&element, &img_selector, "src", "");
        let current_episode = text_from_or(&element, &episode_selector, "N/A");
        let anime_url = attr_from_or(&element, &link_selector, "href", "");

        if !title.is_empty() {
            ongoing_anime.push(OngoingAnimeItem {
                title,
                slug,
                poster,
                current_episode,
                anime_url,
            });
        }
    }
    Ok(ongoing_anime)
}

fn parse_complete_anime(
    html: &str,
) -> Result<Vec<CompleteAnimeItem>, Box<dyn std::error::Error + Send + Sync>> {
    let document = parse_html(html);
    let mut complete_anime = Vec::new();

    let venz_selector = selector(".venz ul li").unwrap();
    let title_selector = selector(".thumbz h2.jdlflm").unwrap();
    let link_selector = selector("a").unwrap();
    let img_selector = selector("img").unwrap();
    let episode_selector = selector(".epz").unwrap();

    for element in document.select(&venz_selector) {
        let title = text_from_or(&element, &title_selector, "");
        let href = attr_from(&element, &link_selector, "href").unwrap_or_default();
        let slug = extract_slug(&href);
        let poster = attr_from_or(&element, &img_selector, "src", "");
        let episode_count = text_from_or(&element, &episode_selector, "N/A");
        let anime_url = attr_from_or(&element, &link_selector, "href", "");

        if !title.is_empty() {
            complete_anime.push(CompleteAnimeItem {
                title,
                slug,
                poster,
                episode_count,
                anime_url,
            });
        }
    }
    Ok(complete_anime)
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/api/anime", axum::routing::get(anime_index))
}
