//! Scheduler implementation using tokio-cron-scheduler.

use async_trait::async_trait;
use deadpool_redis::redis::AsyncCommands;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;

/// Trait for scheduled tasks.
#[async_trait]
pub trait ScheduledTask: Send + Sync {
    /// Task name for logging.
    fn name(&self) -> &'static str;

    /// Cron expression (e.g., "0 * * * * *" for every minute).
    fn schedule(&self) -> &'static str;

    /// Execute the task.
    async fn run(&self);
}

/// Scheduler for running cron jobs.
pub struct Scheduler {
    inner: JobScheduler,
}

impl Scheduler {
    /// Create a new scheduler.
    pub async fn new() -> anyhow::Result<Self> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self { inner: scheduler })
    }

    /// Add a task to the scheduler.
    pub async fn add<T: ScheduledTask + 'static>(&self, task: T) -> anyhow::Result<()> {
        let task = Arc::new(task);
        let task_name = task.name();
        let schedule = task.schedule();

        let job = Job::new_async(schedule, move |_uuid, _lock| {
            let task = Arc::clone(&task);
            Box::pin(async move {
                info!("Running scheduled task: {}", task.name());
                task.run().await;
            })
        })?;

        self.inner.add(job).await?;
        info!("Scheduled task '{}' with cron: {}", task_name, schedule);
        Ok(())
    }

    /// Add a simple job with a closure.
    pub async fn add_job<F, Fut>(
        &self,
        name: &'static str,
        schedule: &str,
        f: F,
    ) -> anyhow::Result<()>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let f = Arc::new(f);
        let job = Job::new_async(schedule, move |_uuid, _lock| {
            let f = Arc::clone(&f);
            Box::pin(async move {
                info!("Running scheduled job: {}", name);
                f().await;
            })
        })?;

        self.inner.add(job).await?;
        info!("Scheduled job '{}' with cron: {}", name, schedule);
        Ok(())
    }

    /// Start the scheduler.
    pub async fn start(&self) -> anyhow::Result<()> {
        info!("Starting scheduler");
        self.inner.start().await?;
        Ok(())
    }

    /// Stop the scheduler.
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        info!("Shutting down scheduler");
        self.inner.shutdown().await?;
        Ok(())
    }
}

// Real scheduled tasks with actual implementations

// Helper for Redis SCAN processing without blocking the core event loop
async fn scan_and_clean(
    conn: &mut deadpool_redis::Connection,
    pattern: &str,
    fix_ttl: bool,
) -> usize {
    let mut cursor: u64 = 0;
    let mut cleaned = 0;

    loop {
        let result: (u64, Vec<String>) = match deadpool_redis::redis::cmd("SCAN")
            .cursor_arg(cursor)
            .arg("MATCH")
            .arg(pattern)
            .arg("COUNT")
            .arg(1000)
            .query_async(&mut **conn)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("SCAN failed for {}: {}", pattern, e);
                break;
            }
        };

        cursor = result.0;
        let keys = result.1;

        for key in keys {
            let ttl: i64 = match conn.ttl(&key).await {
                Ok(t) => t,
                Err(_) => continue,
            };

            if ttl == -2 {
                cleaned += 1;
            } else if ttl == -1 && fix_ttl {
                let _: () = conn.expire(&key, 86400).await.unwrap_or(());
            }
        }

        if cursor == 0 {
            break;
        }
    }
    cleaned
}

/// Cleanup expired sessions from Redis.
/// Runs every hour to remove expired session tokens.
pub struct CleanupExpiredSessions;

#[async_trait]
impl ScheduledTask for CleanupExpiredSessions {
    fn name(&self) -> &'static str {
        "cleanup_expired_sessions"
    }

    fn schedule(&self) -> &'static str {
        // Every hour at minute 0
        "0 0 * * * *"
    }

    async fn run(&self) {
        use crate::infra::redis::REDIS_POOL;

        info!("Cleaning up expired sessions...");

        match REDIS_POOL.get().await {
            Ok(mut conn) => {
                let cleaned = scan_and_clean(&mut conn, "session:*", true).await;
                info!(
                    "Session cleanup complete: {} expired sessions found",
                    cleaned
                );
            }
            Err(e) => {
                tracing::error!("Failed to connect to Redis for session cleanup: {}", e);
            }
        }
    }
}

/// Cleanup expired tokens (JWT blacklist, verification tokens, etc.)
pub struct CleanupExpiredTokens;

#[async_trait]
impl ScheduledTask for CleanupExpiredTokens {
    fn name(&self) -> &'static str {
        "cleanup_expired_tokens"
    }

    fn schedule(&self) -> &'static str {
        // Every 6 hours
        "0 0 */6 * * *"
    }

    async fn run(&self) {
        use crate::infra::redis::REDIS_POOL;

        info!("Cleaning up expired tokens...");

        match REDIS_POOL.get().await {
            Ok(mut conn) => {
                let mut cleaned = 0;
                cleaned += scan_and_clean(&mut conn, "jwt_blacklist:*", false).await;
                cleaned += scan_and_clean(&mut conn, "verify:*", false).await;

                info!("Token cleanup complete: {} expired tokens found", cleaned);
            }
            Err(e) => {
                tracing::error!("Failed to connect to Redis for token cleanup: {}", e);
            }
        }
    }
}

/// Log application metrics periodically.
pub struct LogMetrics;

#[async_trait]
impl ScheduledTask for LogMetrics {
    fn name(&self) -> &'static str {
        "log_metrics"
    }

    fn schedule(&self) -> &'static str {
        // Every 5 minutes
        "0 */5 * * * *"
    }

    async fn run(&self) {
        use crate::infra::redis::REDIS_POOL;

        // Get Redis pool stats
        let redis_stats = match REDIS_POOL.status() {
            s => format!("size={}, available={}", s.size, s.available),
        };

        info!("📊 Metrics - Redis pool: {}", redis_stats);
    }
}
