use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement, Schema};
use tracing::{info, error};
use crate::entities::image_cache;

pub async fn init(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    info!("🚀 Initializing database schema...");
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);

    match backend {
        DbBackend::MySql => {
            // List of entities to initialize
            let tables = vec![(
                "ImageCache",
                schema
                    .create_table_from_entity(image_cache::Entity)
                    .if_not_exists()
                    .to_owned(),
            )];

            for (name, stmt) in tables {
                match db.execute(backend.build(&stmt)).await {
                    Ok(_) => info!("   ✓ Table '{}' checked/created", name),
                    Err(e) => {
                        error!("   [!] Failed to create table '{}': {}", name, e);
                        return Err(e);
                    }
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
