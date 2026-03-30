# Scraping Architecture

RustExpress utilizes a scraping engine for data extraction from Anime and Komik sources.

## 🛠 Engines

1. **Static HTML Scraper (`scraper`)**:
   - Used for server-side rendered (SSR) content.
   - Low resource footprint.
   - CSS selectors for data extraction.
2. **Headless Browser (`chromiumoxide`)**:
   - Used for dynamic SPA sites and client-side rendering.
   - Configurable viewport and user-agent.
   - Resource reuse via a managed **Browser Pool**.

## 🚀 Concurrency & Rate Limiting

- **Throttling**: Scraping is rate-limited by domain to prevent IP blacklisting.
- **Async Workers**: Parallel scraping tasks are processed using tokio tasks.
- **Retries**: Configurable retry logic for network timeouts and 5xx errors.

## 📂 Data Normalization

Extractors map results to standard domain models (e.g., `AnimeResult`, `EpisodeInfo`) to provide consistent API responses across different providers.
