use async_trait::async_trait;

#[async_trait]
pub trait ImageCacheRepository: Send + Sync {
    async fn get_from_redis(&self, key: &str) -> Option<String>;
    async fn set_in_redis(&self, key: &str, value: &str, ttl: u64) -> Result<(), String>;
    async fn get_from_db(&self, original_url: &str) -> Result<Option<String>, String>;
    async fn save_to_db(&self, original_url: &str, cdn_url: &str) -> Result<(), String>;
    async fn find_original_from_cdn(&self, cdn_url: &str) -> Result<Option<String>, String>;
    async fn delete_from_db(&self, original_url: &str) -> Result<(), String>;
    async fn delete_from_redis(&self, key: &str) -> Result<(), String>;
    async fn get_lock(&self, key: &str) -> bool;
    async fn set_lock(&self, key: &str, ttl: u64) -> Result<(), String>;
    async fn release_lock(&self, key: &str) -> Result<(), String>;
    async fn invalidate_api_caches(&self, patterns: Vec<&str>) -> Result<(), String>;
}
