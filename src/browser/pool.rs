//! Browser pool for managing a single remote browser with multiple tabs.
//!
//! This pool maintains a connection to a remote Chrome DevTools Protocol (CDP)
//! endpoint and provides tabs on-demand for scraping. Tabs are returned to the
//! pool after use. Requires EXTERNAL_BROWSERLESS_WS or CHROME_REMOTE_WS to be set.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::{Mutex, Semaphore};
use tracing::{debug, info, warn};
use serde_json::json;

/// Configuration for the browser pool.
#[derive(Debug, Clone)]
pub struct BrowserPoolConfig {
    /// Remote Chrome DevTools Protocol WebSocket URL (required).
    /// Must be set via `EXTERNAL_BROWSERLESS_WS` or `CHROME_REMOTE_WS` environment variable.
    /// Takes precedence from: EXTERNAL_BROWSERLESS_WS → CHROME_REMOTE_WS → None
    pub remote_websocket_url: String,
    /// Maximum number of concurrent tabs
    pub max_tabs: usize,
}

impl Default for BrowserPoolConfig {
    fn default() -> Self {
        // Priority (highest → lowest):
        //   1. EXTERNAL_BROWSERLESS_WS  — explicit operator override, always wins
        //   2. CHROME_REMOTE_WS         — may be injected by Coolify/Docker networking;
        //      rejected when it resolves to the unroutable Docker alias
        let external = std::env::var("EXTERNAL_BROWSERLESS_WS").ok();
        let chrome_remote = std::env::var("CHROME_REMOTE_WS").ok();

        let remote_websocket_url = if let Some(ext) = external {
            tracing::info!("🌐 Browser: using EXTERNAL_BROWSERLESS_WS");
            ext
        } else if let Some(ref cr) = chrome_remote {
            if cr == "ws://browserless:3000" {
                // Coolify Docker alias — unroutable outside overlay network.
                // Fail-fast: remote browser is required.
                panic!(
                    "CHROME_REMOTE_WS is set to the unroutable Docker alias \"ws://browserless:3000\" \
                     and EXTERNAL_BROWSERLESS_WS is unset. Remote browser connectivity is required."
                );
            } else {
                tracing::info!("🌐 Browser: using CHROME_REMOTE_WS");
                cr.clone()
            }
        } else {
            panic!(
                "Remote browser URL not configured. Set EXTERNAL_BROWSERLESS_WS or CHROME_REMOTE_WS environment variable."
            );
        };

        Self {
            remote_websocket_url,
            max_tabs: 10,
        }
    }
}

/// A pool of browser tabs backed by a single remote browser instance.
///
/// # Example
///
/// ```ignore
/// use rustexpress::browser::{BrowserPool, BrowserPoolConfig};
///
/// let pool = BrowserPool::new(BrowserPoolConfig::default()).await?;
/// let tab = pool.get_tab().await?;
/// tab.goto("https://example.com").await?;
/// let html = tab.content().await?;
/// ```
pub struct BrowserPool {
    /// Available (idle) tab IDs
    available_tabs: Mutex<Vec<String>>,
    /// Semaphore to limit concurrent tabs
    semaphore: Arc<Semaphore>,
    /// Configuration
    config: BrowserPoolConfig,
    /// Counter for generating unique tab IDs
    tab_counter: AtomicU64,
}

impl BrowserPool {
    /// Create a new browser pool connecting to a remote CDP endpoint.
    ///
    /// The pool will attempt to connect to the remote browser URL specified in
    /// BrowserPoolConfig. If the connection fails, an error is returned.
    pub async fn new(config: BrowserPoolConfig) -> anyhow::Result<Arc<Self>> {
        info!("Initializing browser pool (remote: {}) with max {} tabs",
              config.remote_websocket_url, config.max_tabs);

        // Verify connection to remote browser with a simple health check
        Self::verify_remote_connection(&config).await?;

        let pool = Arc::new(Self {
            available_tabs: Mutex::new(Vec::new()),
            semaphore: Arc::new(Semaphore::new(config.max_tabs)),
            config: config.clone(),
            tab_counter: AtomicU64::new(0),
        });

        info!("Browser pool initialized (remote-only)");
        Ok(pool)
    }

    /// Verify connection to the remote browser.
    async fn verify_remote_connection(config: &BrowserPoolConfig) -> anyhow::Result<()> {
        let client = reqwest::Client::new();

        // Try a simple health check endpoint
        let response = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            client
                .get(format!("{}/health", config.remote_websocket_url))
                .send(),
        )
        .await;

        match response {
            Ok(Ok(r)) if r.status().is_success() => {
                info!("Remote browser health check passed");
                Ok(())
            }
            Ok(Ok(r)) => {
                warn!("Remote browser health check returned: {}", r.status());
                // Don't fail hard - might work despite health check
                Ok(())
            }
            Ok(Err(e)) => {
                Err(anyhow::anyhow!("Failed to connect to remote browser: {}", e))
            }
            Err(_) => {
                Err(anyhow::anyhow!("Remote browser connection timeout"))
            }
        }
    }

    /// Get a tab from the pool.
    ///
    /// This will reuse an existing idle tab or create a new one.
    /// The returned `PooledTab` automatically returns to the pool when dropped.
    pub async fn get_tab(self: &Arc<Self>) -> anyhow::Result<PooledTab> {
        // Acquire semaphore permit (limits concurrent tabs)
        let permit = self.semaphore.clone().acquire_owned().await?;

        // Try to reuse an existing tab ID, or generate a new one
        let tab_id = {
            let mut tabs = self.available_tabs.lock().await;
            tabs.pop().unwrap_or_else(|| {
                let id = self.tab_counter.fetch_add(1, Ordering::SeqCst);
                format!("tab-{}", id)
            })
        };

        debug!("Allocated tab: {}", tab_id);

        Ok(PooledTab {
            tab_id,
            cdp_url: self.config.remote_websocket_url.clone(),
            pool: Arc::clone(self),
            _permit: permit,
        })
    }

    /// Get the number of available (idle) tabs in the pool.
    pub async fn available_count(&self) -> usize {
        self.available_tabs.lock().await.len()
    }

    /// Close the browser pool.
    pub async fn close(&self) -> anyhow::Result<()> {
        info!("Closing browser pool");
        self.available_tabs.lock().await.clear();
        Ok(())
    }
}

/// A tab borrowed from the pool (remote CDP-based).
///
/// This is a lightweight wrapper that communicates with a remote Chrome instance
/// via HTTP calls to the browser service. The tab is returned to the pool when dropped.
pub struct PooledTab {
    /// Unique identifier for this tab
    pub tab_id: String,
    /// Remote CDP/browser service URL
    cdp_url: String,
    /// Pool reference for returning on drop
    pool: Arc<BrowserPool>,
    /// Semaphore permit (released on drop)
    _permit: tokio::sync::OwnedSemaphorePermit,
}

impl PooledTab {
    /// Navigate to a URL.
    pub async fn goto(&self, url: &str) -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let timeout = std::time::Duration::from_secs(15);

        let response = tokio::time::timeout(
            timeout,
            client
                .post(format!("{}/goto", self.cdp_url))
                .json(&json!({ "url": url }))
                .send(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("Timeout navigating to {}", url))?
        .map_err(|e| anyhow::anyhow!("Failed to navigate to {}: {}", url, e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to navigate to {}: {}",
                url,
                response.status()
            ))
        }
    }

    /// Get the page content (HTML).
    pub async fn content(&self) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let timeout = std::time::Duration::from_secs(15);

        let response = tokio::time::timeout(
            timeout,
            client.post(format!("{}/content", self.cdp_url)).send(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("Timeout getting page content"))?
        .map_err(|e| anyhow::anyhow!("Failed to get page content: {}", e))?;

        response
            .text()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read response body: {}", e))
    }

    /// Execute JavaScript and return the result.
    pub async fn evaluate<T: serde::de::DeserializeOwned>(
        &self,
        expression: &str,
    ) -> anyhow::Result<T> {
        let client = reqwest::Client::new();
        let timeout = std::time::Duration::from_secs(15);

        let response = tokio::time::timeout(
            timeout,
            client
                .post(format!("{}/evaluate", self.cdp_url))
                .json(&json!({ "expression": expression }))
                .send(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("Timeout evaluating JS"))?
        .map_err(|e| anyhow::anyhow!("Failed to evaluate JS: {}", e))?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to parse JS result: {}", e))?;

        serde_json::from_value(data).map_err(|e| anyhow::anyhow!("Invalid JS result: {}", e))
    }

    /// Wait for a selector to appear.
    pub async fn wait_for_selector(&self, selector: &str) -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let timeout = std::time::Duration::from_secs(15);

        let response = tokio::time::timeout(
            timeout,
            client
                .post(format!("{}/waitForSelector", self.cdp_url))
                .json(&json!({ "selector": selector }))
                .send(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("Timeout waiting for selector '{}'", selector))?
        .map_err(|e| anyhow::anyhow!("Selector '{}' error: {}", selector, e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Selector '{}' not found: {}",
                selector,
                response.status()
            ))
        }
    }

    /// Click an element by selector.
    pub async fn click(&self, selector: &str) -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let timeout = std::time::Duration::from_secs(15);

        let response = tokio::time::timeout(
            timeout,
            client
                .post(format!("{}/click", self.cdp_url))
                .json(&json!({ "selector": selector }))
                .send(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("Timeout clicking element"))?
        .map_err(|e| anyhow::anyhow!("Click error: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to click '{}': {}",
                selector,
                response.status()
            ))
        }
    }

    /// Type text into an element.
    pub async fn type_text(&self, selector: &str, text: &str) -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let timeout = std::time::Duration::from_secs(15);

        let response = tokio::time::timeout(
            timeout,
            client
                .post(format!("{}/type", self.cdp_url))
                .json(&json!({ "selector": selector, "text": text }))
                .send(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("Timeout typing text"))?
        .map_err(|e| anyhow::anyhow!("Type error: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to type into '{}': {}",
                selector,
                response.status()
            ))
        }
    }

    /// Take a screenshot as PNG bytes.
    pub async fn screenshot(&self) -> anyhow::Result<Vec<u8>> {
        let client = reqwest::Client::new();
        let timeout = std::time::Duration::from_secs(15);

        let response = tokio::time::timeout(
            timeout,
            client
                .post(format!("{}/screenshot", self.cdp_url))
                .json(&json!({ "fullPage": true }))
                .send(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("Timeout taking screenshot"))?
        .map_err(|e| anyhow::anyhow!("Failed to take screenshot: {}", e))?;

        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| anyhow::anyhow!("Failed to read screenshot bytes: {}", e))
    }

    /// Get the current URL.
    pub async fn url(&self) -> anyhow::Result<String> {
        let client = reqwest::Client::new();
        let timeout = std::time::Duration::from_secs(10);

        let response = tokio::time::timeout(
            timeout,
            client.post(format!("{}/url", self.cdp_url)).send(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("Timeout getting URL"))?
        .map_err(|e| anyhow::anyhow!("Failed to get URL: {}", e))?;

        response
            .text()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read URL: {}", e))
    }
}

impl Drop for PooledTab {
    fn drop(&mut self) {
        let pool = Arc::clone(&self.pool);
        let tab_id = self.tab_id.clone();

        let rt = tokio::runtime::Handle::try_current();
        if let Ok(handle) = rt {
            handle.spawn(async move {
                // Return tab ID to available pool for reuse
                pool.available_tabs.lock().await.push(tab_id);
            });
        } else {
            warn!("Cannot return tab: no tokio runtime available");
        }
    }
}

// Global browser pool instance
use once_cell::sync::OnceCell;

static BROWSER_POOL: OnceCell<Arc<BrowserPool>> = OnceCell::new();

/// Initialize the global browser pool.
/// Call this once at application startup.
pub async fn init_browser_pool(config: BrowserPoolConfig) -> anyhow::Result<()> {
    let pool = BrowserPool::new(config).await?;
    BROWSER_POOL
        .set(pool)
        .map_err(|_| anyhow::anyhow!("Browser pool already initialized"))?;
    Ok(())
}

/// Get the global browser pool.
/// Returns None if not initialized.
pub fn get_browser_pool() -> Option<Arc<BrowserPool>> {
    BROWSER_POOL.get().cloned()
}
