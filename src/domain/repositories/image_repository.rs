use async_trait::async_trait;
use crate::domain::entities::image::ImageCache;
use crate::shared::errors::AppError;

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn find_by_original_url(&self, url: &str) -> Result<Option<ImageCache>, AppError>;
    async fn save(&self, image: ImageCache) -> Result<(), AppError>;
    async fn delete_by_original_url(&self, url: &str) -> Result<(), AppError>;
}
