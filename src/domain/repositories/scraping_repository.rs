use async_trait::async_trait;
use crate::shared::errors::AppError;

#[async_trait]
pub trait ScrapingRepository: Send + Sync {
    async fn fetch_html(&self, url: &str) -> Result<String, AppError>;
}
