//! Scheduler implementation using tokio-cron-scheduler.

use async_trait::async_trait;
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
                tracing::debug!("Running scheduled task: {}", task.name());
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
// Real scheduled tasks with actual implementations
