use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Schema};
use tracing::{info, error};
use crate::entities::{image_cache, user, posts, likes, comments, chat_room};

pub async fn init(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    info!("🚀 Initializing database schema...");
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);

    match backend {
        DbBackend::MySql => {
            // List of entities to initialize
            let tables = vec![
                ("ImageCache", schema.create_table_from_entity(image_cache::Entity).if_not_exists().to_owned()),
                ("User", schema.create_table_from_entity(user::Entity).if_not_exists().to_owned()),
                ("Posts", schema.create_table_from_entity(posts::Entity).if_not_exists().to_owned()),
                ("Likes", schema.create_table_from_entity(likes::Entity).if_not_exists().to_owned()),
                ("Comments", schema.create_table_from_entity(comments::Entity).if_not_exists().to_owned()),
                ("ChatRoom", schema.create_table_from_entity(chat_room::Entity).if_not_exists().to_owned()),
            ];

            for (name, stmt) in tables {
                match db.execute(backend.build(&stmt)).await {
                    Ok(_) => info!("   ✓ Table '{}' checked/created", name),
                    Err(e) => {
                        error!("   [!] Failed to create table '{}': {}", name, e);
                        return Err(e);
                    }
                }
            }

            // Create bookmarks table (Legacy raw SQL)
            let sql = r#"
                CREATE TABLE IF NOT EXISTS bookmarks (
                    id VARCHAR(255) NOT NULL PRIMARY KEY,
                    user_id VARCHAR(255) NOT NULL,
                    content_type VARCHAR(50) NOT NULL,
                    slug VARCHAR(255) NOT NULL,
                    title VARCHAR(255) NOT NULL,
                    poster VARCHAR(512) NOT NULL,
                    created_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                    INDEX idx_bookmarks_user_id (user_id),
                    INDEX idx_bookmarks_content (content_type, slug)
                ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
            "#;
            
            match db.execute(Statement::from_string(backend, sql)).await {
                Ok(_) => info!("   ✓ Table 'bookmarks' checked/created"),
                Err(e) => {
                    error!("   [!] Failed to create legacy table 'bookmarks': {}", e);
                    return Err(e);
                }
            }

            // Ensure CDN URL index exists on ImageCache
            let index_sql = "CREATE INDEX idx_image_cache_cdn_url ON ImageCache (cdn_url)";
            match db.execute(Statement::from_string(backend, index_sql)).await {
                Ok(_) => info!("   ✓ Index 'idx_image_cache_cdn_url' ensured"),
                Err(e) => {
                    let err_str = e.to_string();
                    // Ignore "Duplicate key name" error (1061 in MySQL)
                    if err_str.contains("1061") {
                        info!("   ✓ Index 'idx_image_cache_cdn_url' already exists");
                    } else {
                        error!("   [!] Failed to create index on ImageCache: {}", e);
                        // Don't return error here to allow app to start even if indexing fails temporarily
                    }
                }
            }
            
            info!("✅ Database schema initialization complete.");
        }
        _ => {
            info!("ℹ️ Skipping schema init for non-MySQL backend");
        }
    }
    Ok(())
}
