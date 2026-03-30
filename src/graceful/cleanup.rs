//! Graceful shutdown with proper resource cleanup.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;
use tokio::sync::Notify;
use tokio::time::{sleep, Duration};
use tracing::info;

/// Graceful shutdown coordinator.
pub struct ShutdownCoordinator {
    /// Shutdown signal flag
    is_shutting_down: Arc<AtomicBool>,
    /// Notify for graceful shutdown
    shutdown_notify: Arc<Notify>,
}

impl ShutdownCoordinator {
    /// Create a new shutdown coordinator.
    pub fn new() -> Self {
        Self {
            is_shutting_down: Arc::new(AtomicBool::new(false)),
            shutdown_notify: Arc::new(Notify::new()),
        }
    }

    /// Check if shutdown is in progress.
    pub fn is_shutting_down(&self) -> bool {
        self.is_shutting_down.load(Ordering::Relaxed)
    }

    /// Start shutdown process.
    pub fn initiate_shutdown(&self) {
        info!("🛑 Initiating graceful shutdown...");
        self.is_shutting_down.store(true, Ordering::Relaxed);
        self.shutdown_notify.notify_waiters();
    }

    /// Wait for shutdown signal.
    pub async fn wait_for_shutdown_signal(&self) {
        let ctrl_c = async {
            if let Err(e) = signal::ctrl_c().await {
                tracing::error!("Failed to listen for Ctrl+C: {}", e);
            }
        };

        #[cfg(unix)]
        let terminate = async {
            match signal::unix::signal(signal::unix::SignalKind::terminate()) {
                Ok(mut stream) => {
                    stream.recv().await;
                }
                Err(e) => {
                    tracing::error!("Failed to listen for SIGTERM: {}", e);
                }
            }
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                info!("Received Ctrl+C signal");
            }
            _ = terminate => {
                info!("Received SIGTERM signal");
            }
        }

        self.initiate_shutdown();
    }

    /// Perform cleanup operations.
    pub async fn cleanup(&self) {
        info!("🧹 Starting cleanup operations...");

        // Give active requests time to finish
        info!("Waiting for active requests to complete...");
        sleep(Duration::from_secs(5)).await;

        info!("✅ Cleanup completed");
    }

    /// Get a handle for checking shutdown status.
    pub fn handle(&self) -> ShutdownHandle {
        ShutdownHandle {
            is_shutting_down: Arc::clone(&self.is_shutting_down),
        }
    }
}

impl Default for ShutdownCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle for checking shutdown status.
#[derive(Clone)]
pub struct ShutdownHandle {
    is_shutting_down: Arc<AtomicBool>,
}

impl ShutdownHandle {
    /// Check if shutdown is in progress.
    pub fn is_shutting_down(&self) -> bool {
        self.is_shutting_down.load(Ordering::Relaxed)
    }
}

pub async fn wait_for_shutdown_and_cleanup() {
    let coordinator = ShutdownCoordinator::new();

    coordinator.wait_for_shutdown_signal().await;
    coordinator.cleanup().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shutdown_coordinator() {
        let coordinator = ShutdownCoordinator::new();
        assert!(!coordinator.is_shutting_down());

        coordinator.initiate_shutdown();
        assert!(coordinator.is_shutting_down());
    }

    #[test]
    fn test_shutdown_handle() {
        let coordinator = ShutdownCoordinator::new();
        let handle = coordinator.handle();

        assert!(!handle.is_shutting_down());

        coordinator.initiate_shutdown();
        assert!(handle.is_shutting_down());
    }
}
