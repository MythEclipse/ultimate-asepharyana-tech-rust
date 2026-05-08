use std::sync::Arc;
use crate::domain::repositories::ImageRepository;
use crate::domain::entities::image::ImageCache;
use chrono::Utc;
use deadpool_redis::Pool as RedisPool;
use reqwest::Client;
use tracing::error;
use metrics::{counter, histogram};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::broadcast;
use crate::shared::utils::Cache;
use crate::shared::utils::cache_ttl::CACHE_TTL_IMAGE;
use crate::infra::http_client::http_client;

/// Default TTL for image cache in Redis (24 hours)
pub const IMAGE_CACHE_TTL: u64 = CACHE_TTL_IMAGE;
/// Redis key prefix for image cache
pub const IMAGE_CACHE_PREFIX: &str = "img_cache";
/// Redis key prefix for caching locks
pub const IMAGE_CACHE_LOCK_PREFIX: &str = "img_cache_lock";
/// Lock TTL (60 seconds)
pub const IMAGE_CACHE_LOCK_TTL: u64 = 60;

/// Picser API endpoints
pub const PICSER_API_ENDPOINTS: &[&str] = &[
    "https://picser-two.vercel.app/api/upload",
    "https://picser.asepharyana.tech/api/upload",
    "https://picser-mytheclipse8647-ahoqi9ef.leapcell.dev/api/upload",
    "https://picser.pages.dev/api/upload",
];

/// Global In-Flight Uploads Map
static IN_FLIGHT_UPLOADS: Lazy<DashMap<String, broadcast::Sender<Result<String, String>>>> =
    Lazy::new(DashMap::new);

#[derive(serde::Deserialize)]
pub struct PicserResponse {
    pub success: bool,
    pub url: Option<String>,
    pub urls: Option<PicserUrls>,
    pub github_url: Option<String>,
    pub error: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct PicserUrls {
    pub github: Option<String>,
    pub raw: Option<String>,
    pub jsdelivr: Option<String>,
    pub jsdelivr_commit: Option<String>,
}

pub struct CacheImageUseCase {
    repo: Arc<dyn ImageRepository>,
    redis: RedisPool,
    client: Client,
    semaphore: Option<Arc<tokio::sync::Semaphore>>,
}

impl CacheImageUseCase {
    pub fn new(repo: Arc<dyn ImageRepository>, redis: RedisPool) -> Self {
        Self {
            repo,
            redis,
            client: http_client().client().clone(),
            semaphore: None,
        }
    }

    pub fn with_semaphore(mut self, semaphore: Arc<tokio::sync::Semaphore>) -> Self {
        self.semaphore = Some(semaphore);
        self
    }

    pub async fn execute(&self, original_url: &str) -> Result<String, String> {
        self.execute_lazy(original_url).await
    }

    pub async fn execute_lazy(&self, original_url: &str) -> Result<String, String> {
        let hash = self.url_hash(original_url);
        let cache_key = format!("{}:{}", IMAGE_CACHE_PREFIX, hash);
        let _lock_key = format!("{}:{}", IMAGE_CACHE_LOCK_PREFIX, hash);

        // 1. Redis Check
        let redis_cache = Cache::new(&self.redis);
        counter!("image_cache_requests_total").increment(1);

        if let Some(cached_url) = redis_cache.get::<String>(&cache_key).await {
            counter!("image_cache_hit_total", "source" => "redis").increment(1);

            // Verify in background
            let repo = self.repo.clone();
            let redis = self.redis.clone();
            let url = original_url.to_string();
            let cdn = cached_url.clone();
            let client = self.client.clone();
            let sem = self.semaphore.clone();

            tokio::spawn(async move {
                let verify_client = client.clone();
                match verify_client.get(&cdn).send().await {
                    Ok(resp) if resp.status().is_success() => {}, // OK
                    _ => {
                        error!("CDN URL {} broken, purging and re-fetching", cdn);
                        let _ = repo.delete_by_original_url(&url).await;
                        let redis_c = Cache::new(&redis);
                        let hash = url_hash_internal(url.as_str());
                        let _ = redis_c.delete(format!("{}:{}", IMAGE_CACHE_PREFIX, hash).as_str()).await;

                        // Trigger re-upload
                        let uc = CacheImageUseCase {
                            repo,
                            redis,
                            client,
                            semaphore: sem,
                        };
                        let _ = uc.do_cache(url.as_str(), &format!("{}:{}", IMAGE_CACHE_PREFIX, hash), &format!("{}:{}", IMAGE_CACHE_LOCK_PREFIX, hash)).await;
                    }
                }
            });

            return Ok(cached_url);
        }

        // 2. Coalescing
        let (tx, is_leader) = {
            use dashmap::mapref::entry::Entry;
            match IN_FLIGHT_UPLOADS.entry(original_url.to_string()) {
                Entry::Occupied(entry) => (entry.get().clone(), false),
                Entry::Vacant(entry) => {
                    let (tx, _) = broadcast::channel(1);
                    entry.insert(tx.clone());
                    (tx, true)
                }
            }
        };

        if !is_leader {
            // If already in flight, return original immediately (proactive)
            return Ok(crate::application::services::images::cache::to_wp_cdn(original_url));
        }

        // Leader triggers do_cache in background and returns original immediately
        let repo = self.repo.clone();
        let redis = self.redis.clone();
        let url = original_url.to_string();
        let client = self.client.clone();
        let sem = self.semaphore.clone();

        tokio::spawn(async move {
            let hash = url_hash_internal(url.as_str());
            let uc = CacheImageUseCase { repo, redis, client, semaphore: sem };
            let _ = uc.do_cache(url.as_str(), &format!("{}:{}", IMAGE_CACHE_PREFIX, hash), &format!("{}:{}", IMAGE_CACHE_LOCK_PREFIX, hash)).await;
            let _ = tx.send(Ok(url.clone())); // Dummy send to clear coalescing
        });

        IN_FLIGHT_UPLOADS.remove(original_url);

        Ok(crate::application::services::images::cache::to_wp_cdn(original_url))
    }

    async fn do_cache(&self, original_url: &str, cache_key: &str, lock_key: &str) -> Result<String, String> {
        let redis_cache = Cache::new(&self.redis);

        // 3. Repo
        if let Ok(Some(image)) = self.repo.find_by_original_url(original_url).await {
            let _ = redis_cache.set_with_ttl(cache_key, &image.cdn_url, IMAGE_CACHE_TTL).await;
            counter!("image_cache_hit_total", "source" => "db").increment(1);
            return Ok(image.cdn_url);
        }

        // 4. Lock
        if redis_cache.get::<bool>(lock_key).await.is_some() {
            return Err(format!("URL {} is already being cached", original_url));
        }
        let _ = redis_cache.set_with_ttl(lock_key, &true, IMAGE_CACHE_LOCK_TTL).await;

        // 5. Work
        counter!("image_cache_miss_total").increment(1);
        let start = std::time::Instant::now();

        let _permit = if let Some(sem) = &self.semaphore {
            Some(sem.acquire().await.map_err(|e| e.to_string())?)
        } else {
            None
        };

        let work_result = self.upload_and_verify(original_url).await;

        if let Ok(cdn_url) = &work_result {
            let image = ImageCache {
                id: uuid::Uuid::new_v4().to_string(),
                original_url: original_url.to_string(),
                cdn_url: cdn_url.clone(),
                created_at: Utc::now(),
                expires_at: None,
            };

            if let Err(e) = self.repo.save(image).await {
                error!("CacheImageUseCase: failed to save to repo: {}", e);
            }

            let _ = redis_cache.set_with_ttl(cache_key, cdn_url, IMAGE_CACHE_TTL).await;
            let _ = self.invalidate_api_caches().await;

            histogram!("image_upload_duration_seconds").record(start.elapsed().as_secs_f64());
            counter!("image_upload_success_total").increment(1);
        }

        let _ = redis_cache.delete(lock_key).await;
        work_result
    }

    async fn upload_and_verify(&self, original_url: &str) -> Result<String, String> {
        let cdn_url = self.upload_to_picser(original_url).await?;

        for attempt in 1..=10 {
            match self.verify_cdn_url(&cdn_url).await {
                Ok(true) => return Ok(cdn_url),
                _ => {
                    if attempt < 10 {
                        tokio::time::sleep(std::time::Duration::from_secs(attempt)).await;
                    }
                }
            }
        }

        Err("CDN verification failed".to_string())
    }

    async fn upload_to_picser(&self, original_url: &str) -> Result<String, String> {
        let bytes = self.client.get(original_url).send().await
            .map_err(|e| e.to_string())?
            .bytes().await
            .map_err(|e| e.to_string())?;

        if !infer::get(&bytes).map(|k| k.mime_type().starts_with("image/")).unwrap_or(false) {
            return Err("Invalid image data".to_string());
        }

        let filename = original_url.split('/').last()
            .and_then(|s| s.split('?').next())
            .filter(|s| !s.is_empty() && s.contains('.'))
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}.jpg", self.url_hash(original_url)));

        for api_url in PICSER_API_ENDPOINTS {
            match tokio::time::timeout(std::time::Duration::from_secs(30), self.perform_upload(api_url, &bytes, &filename)).await {
                Ok(Ok(resp)) => if let Ok(url) = self.extract_cdn_url(resp) { return Ok(url); },
                _ => continue,
            }
        }

        counter!("image_upload_failure_total").increment(1);
        Err("All upload attempts failed".to_string())
    }

    async fn perform_upload(&self, api_url: &str, bytes: &[u8], filename: &str) -> Result<PicserResponse, String> {
        let part = reqwest::multipart::Part::bytes(bytes.to_vec())
            .file_name(filename.to_string())
            .mime_str("image/jpeg")
            .map_err(|e| e.to_string())?;

        let form = reqwest::multipart::Form::new().part("file", part);

        let resp = self.client.post(api_url).multipart(form).send().await
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("Upload failed: {}", resp.status()));
        }

        resp.json::<PicserResponse>().await.map_err(|e| e.to_string())
    }

    async fn verify_cdn_url(&self, cdn_url: &str) -> Result<bool, String> {
        let resp = self.client.get(cdn_url).send().await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() { return Ok(false); }
        let bytes = resp.bytes().await.map_err(|e| e.to_string())?;
        Ok(infer::get(&bytes).map(|k| k.mime_type().starts_with("image/")).unwrap_or(false))
    }

    fn extract_cdn_url(&self, resp: PicserResponse) -> Result<String, String> {
        if let Some(urls) = resp.urls {
            if let Some(url) = urls.jsdelivr_commit { return Ok(url); }
            if let Some(url) = urls.jsdelivr { return Ok(url); }
            if let Some(url) = urls.raw { return Ok(url); }
            if let Some(url) = urls.github { return Ok(url); }
        }
        if let Some(url) = resp.url { return Ok(url); }
        if let Some(url) = resp.github_url { return Ok(url); }
        Err("No CDN URL found".to_string())
    }

    pub fn url_hash(&self, url: &str) -> String {
        url_hash_internal(url)
    }

    async fn invalidate_api_caches(&self) -> Result<(), String> {
        use deadpool_redis::redis::{cmd, AsyncCommands};
        let mut conn = self.redis.get().await.map_err(|e| e.to_string())?;
        let patterns = vec!["anime:*", "anime2:*", "komik:*"];
        for pattern in patterns {
            let mut cursor: u64 = 0;
            loop {
                let (new_cursor, keys): (u64, Vec<String>) = cmd("SCAN")
                    .arg(cursor).arg("MATCH").arg(pattern).arg("COUNT").arg(100)
                    .query_async(&mut *conn).await.map_err(|e| e.to_string())?;
                if !keys.is_empty() { let _: usize = conn.del(&keys).await.map_err(|e| e.to_string())?; }
                cursor = new_cursor;
                if cursor == 0 { break; }
            }
        }
        Ok(())
    }
}

fn url_hash_internal(url: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    hex::encode(&hasher.finalize()[..16])
}
