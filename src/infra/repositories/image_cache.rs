use async_trait::async_trait;
use crate::domain::repositories::image_cache::ImageCacheRepository;
use crate::infra::persistence::entities::image_cache;
use crate::shared::utils::Cache;
use chrono::Utc;
use deadpool_redis::Pool as RedisPool;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

pub struct SeaOrmImageCacheRepository {
    db: Arc<DatabaseConnection>,
    redis: RedisPool,
}

impl SeaOrmImageCacheRepository {
    pub fn new(db: Arc<DatabaseConnection>, redis: RedisPool) -> Self {
        Self { db, redis }
    }
}

#[async_trait]
impl ImageCacheRepository for SeaOrmImageCacheRepository {
    async fn get_from_redis(&self, key: &str) -> Option<String> {
        Cache::new(&self.redis).get::<String>(key).await
    }

    async fn set_in_redis(&self, key: &str, value: &str, ttl: u64) -> Result<(), String> {
        Cache::new(&self.redis)
            .set_with_ttl(key, &value, ttl)
            .await
            .map_err(|e| e.to_string())
    }

    async fn get_from_db(&self, original_url: &str) -> Result<Option<String>, String> {
        let entry = image_cache::Entity::find()
            .filter(image_cache::Column::OriginalUrl.eq(original_url))
            .one(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())?;
        Ok(entry.map(|m| m.cdn_url))
    }

    async fn save_to_db(&self, original_url: &str, cdn_url: &str) -> Result<(), String> {
        let model = image_cache::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            original_url: Set(original_url.to_string()),
            cdn_url: Set(cdn_url.to_string()),
            created_at: Set(Utc::now()),
            expires_at: Set(None),
        };

        model
            .insert(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn find_original_from_cdn(&self, cdn_url: &str) -> Result<Option<String>, String> {
        let entry = image_cache::Entity::find()
            .filter(image_cache::Column::CdnUrl.eq(cdn_url))
            .one(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())?;
        Ok(entry.map(|m| m.original_url))
    }

    async fn delete_from_db(&self, original_url: &str) -> Result<(), String> {
        image_cache::Entity::delete_many()
            .filter(image_cache::Column::OriginalUrl.eq(original_url))
            .exec(self.db.as_ref())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn delete_from_redis(&self, key: &str) -> Result<(), String> {
        Cache::new(&self.redis).delete(key).await.map_err(|e| e.to_string())
    }

    async fn get_lock(&self, key: &str) -> bool {
        Cache::new(&self.redis).get::<bool>(key).await.is_some()
    }

    async fn set_lock(&self, key: &str, ttl: u64) -> Result<(), String> {
        Cache::new(&self.redis)
            .set_with_ttl(key, &true, ttl)
            .await
            .map_err(|e| e.to_string())
    }

    async fn release_lock(&self, key: &str) -> Result<(), String> {
        self.delete_from_redis(key).await
    }

    async fn invalidate_api_caches(&self, patterns: Vec<&str>) -> Result<(), String> {
        use deadpool_redis::redis::{cmd, AsyncCommands};

        let mut conn = self.redis.get().await.map_err(|e| e.to_string())?;

        for pattern in patterns {
            let mut cursor: u64 = 0;
            loop {
                let (new_cursor, keys): (u64, Vec<String>) = cmd("SCAN")
                    .arg(cursor)
                    .arg("MATCH")
                    .arg(pattern)
                    .arg("COUNT")
                    .arg(100)
                    .query_async(&mut *conn)
                    .await
                    .map_err(|e| e.to_string())?;

                if !keys.is_empty() {
                    let _: usize = conn.del(&keys).await.map_err(|e| e.to_string())?;
                }

                cursor = new_cursor;
                if cursor == 0 {
                    break;
                }
            }
        }
        Ok(())
    }
}
