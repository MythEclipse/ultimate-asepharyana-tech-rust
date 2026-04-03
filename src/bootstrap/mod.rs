use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

use axum::Router;
use sea_orm::{Database, DatabaseConnection, ConnectionTrait, Statement, DbBackend};
use tower_http::compression::{CompressionLayer, CompressionLevel};
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

use crate::core::config::CONFIG;
use crate::infra::redis::REDIS_POOL;
use crate::routes::AppState;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::observability::openapi::ApiDoc;

pub struct Application {
    pub port: u16,
    router: Router,
    listener: TcpListener,
}

impl Application {
    pub async fn build() -> anyhow::Result<Self> {
        // Initialize tracing
        let filter = &CONFIG.log_level;
        if std::env::var("RUST_LOG").is_err() {
            tracing_subscriber::fmt()
                .with_env_filter(EnvFilter::new(filter))
                .init();
        }

        tracing::info!("🚀 RustExpress starting up...");
        tracing::info!("   Environment: {}", CONFIG.environment);

        // Log thread configuration
        let worker_threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        tracing::info!(
            "   Tokio Worker Threads: (Defaulting to CPU cores: {})",
            worker_threads
        );

        // Redis
        let _ = REDIS_POOL.get().await;

        // Browser Pool
        tracing::info!("Initializing browser pool...");
        let browser_config = crate::browser::BrowserPoolConfig::default();
        match crate::browser::pool::init_browser_pool(browser_config).await {
            Ok(_) => tracing::info!("✓ Browser pool initialized"),
            Err(e) => tracing::error!("⚠️ Failed to initialize browser pool: {}", e),
        }

        // Ensure database exists (MySQL only)
        if CONFIG.database_url.starts_with("mysql") {
            if let Err(e) = Self::ensure_database_exists().await {
                tracing::warn!("⚠️ Failed to ensure database exists: {}. Attempting connection anyway...", e);
            }
        }

        // Database
        let mut opt = sea_orm::ConnectOptions::new(CONFIG.database_url.clone());
        opt.max_connections(20)
            .min_connections(1)
            .connect_timeout(std::time::Duration::from_secs(CONFIG.db.connect_timeout_seconds))
            .idle_timeout(std::time::Duration::from_secs(CONFIG.db.idle_timeout_seconds))
            .acquire_timeout(std::time::Duration::from_secs(CONFIG.db.acquire_timeout_seconds))
            .max_lifetime(std::time::Duration::from_secs(CONFIG.db.max_lifetime_seconds))
            .sqlx_logging(CONFIG.log_level == "debug");

        let db = Database::connect(opt).await
            .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;
        tracing::info!("✓ SeaORM database connection established");

        // Schema & Seeding
        if let Err(e) = crate::infra::db_setup::init(&db).await {
            tracing::error!("Failed to init DB schema: {}", e);
        }
        // Schema & Seeding
        if let Err(e) = crate::infra::db_setup::init(&db).await {
            tracing::error!("Failed to init DB schema: {}", e);
        }

        // App State components
        let db_arc = Arc::new(db);
        let image_processing_semaphore =
            Arc::new(tokio::sync::Semaphore::new(CONFIG.image_processing_concurrency));
        let event_bus = Arc::new(crate::events::bus::EventBus::new());

        let app_state = Arc::new(AppState {
            redis_pool: REDIS_POOL.clone(),
            db: db_arc.clone(),

            image_processing_semaphore,
            event_bus: event_bus.clone(),
        });

        // Prometheus Metrics
        let (prometheus_layer, metric_handle) = crate::observability::metrics::setup_metrics();

        // Scheduler
        Self::init_scheduler(db_arc.clone()).await?;

        // OpenApi merging
        let mut openapi = ApiDoc::openapi();
        openapi.merge(crate::observability::openapi_generated::GeneratedApiDoc::openapi());

        // Router
        let app = crate::routes::api::register_routes(Router::new())
            .route("/metrics", axum::routing::get(move || async move { metric_handle.render() }))
            .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
            .with_state(app_state.clone())
            .layer(prometheus_layer)
            .layer(CompressionLayer::new().quality(CompressionLevel::Fastest))
            .layer(CorsLayer::permissive());

        // Listener
        let port = CONFIG.server_port;
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("Server listening on {}", listener.local_addr()?);

        Ok(Self {
            port,
            router: app,
            listener,
        })
    }

    async fn init_scheduler(db: Arc<DatabaseConnection>) -> anyhow::Result<()> {
        let scheduler = crate::scheduler::Scheduler::new()
            .await
            .expect("Failed to create scheduler");

        let cache_cleanup = crate::scheduler::CleanupOldCache::new(db);
        scheduler
            .add(cache_cleanup)
            .await
            .expect("Failed to add cache cleanup");

        scheduler.start().await.expect("Failed to start scheduler");
        tracing::info!("✓ Scheduler started");
        Ok(())
    }

    async fn ensure_database_exists() -> anyhow::Result<()> {
        let db_url = &CONFIG.database_url;
        let parsed = url::Url::parse(db_url)
            .map_err(|e| anyhow::anyhow!("Failed to parse DATABASE_URL: {}", e))?;
        
        // Extract database name and base server URL
        let db_name = parsed.path().trim_start_matches('/');
        if db_name.is_empty() {
            return Ok(());
        }

        let mut server_url = parsed.clone();
        server_url.set_path(""); // Remove database name from path
        let server_url_str = server_url.to_string();

        tracing::info!("🔍 Checking/creating database '{}'...", db_name);
        
        let mut opt = sea_orm::ConnectOptions::new(server_url_str);
        opt.connect_timeout(std::time::Duration::from_secs(5))
           .sqlx_logging(false);

        let db = Database::connect(opt).await
            .map_err(|e| anyhow::anyhow!("Failed to connect to database server: {}", e))?;

        // Create table character set handling
        let sql = format!("CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci", db_name);
        
        db.execute(Statement::from_string(DbBackend::MySql, sql)).await
            .map_err(|e| anyhow::anyhow!("Failed to execute 'CREATE DATABASE': {}", e))?;
            
        tracing::info!("✓ Database '{}' ensured", db_name);
        Ok(())
    }

    pub async fn run(self) -> std::io::Result<()> {
        axum::serve(self.listener, self.router.into_make_service()).await
    }
}
