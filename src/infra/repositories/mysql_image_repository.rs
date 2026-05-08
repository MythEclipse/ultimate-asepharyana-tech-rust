use async_trait::async_trait;
use sea_orm::*;
use crate::core::models::image::ImageCache as DomainImageCache;
use crate::core::repositories::image_repository::ImageRepository;
use crate::entities::image_cache::{Entity as ImageCacheEntity, ActiveModel as ImageCacheActiveModel, Column};
use crate::shared::errors::AppError;

use std::sync::Arc;

pub struct MySqlImageRepository {
    db: Arc<DatabaseConnection>,
}

impl MySqlImageRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ImageRepository for MySqlImageRepository {
    async fn find_by_original_url(&self, url: &str) -> Result<Option<DomainImageCache>, AppError> {
        let model = ImageCacheEntity::find()
            .filter(Column::OriginalUrl.eq(url))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(model.map(|m| DomainImageCache {
            id: m.id,
            original_url: m.original_url,
            cdn_url: m.cdn_url,
            created_at: m.created_at,
            expires_at: m.expires_at,
        }))
    }

    async fn save(&self, image: DomainImageCache) -> Result<(), AppError> {
        let active_model = ImageCacheActiveModel {
            id: Set(image.id),
            original_url: Set(image.original_url),
            cdn_url: Set(image.cdn_url),
            created_at: Set(image.created_at),
            expires_at: Set(image.expires_at),
        };

        ImageCacheEntity::insert(active_model)
            .on_conflict(
                sea_query::OnConflict::column(Column::OriginalUrl)
                    .update_columns([Column::CdnUrl, Column::CreatedAt, Column::ExpiresAt])
                    .to_owned(),
            )
            .exec(self.db.as_ref())
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_original_url(&self, url: &str) -> Result<(), AppError> {
        ImageCacheEntity::delete_many()
            .filter(Column::OriginalUrl.eq(url))
            .exec(self.db.as_ref())
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }
}
