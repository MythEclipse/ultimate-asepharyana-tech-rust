# RustExpress - Scraping & CDN Service

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust: 1.75+](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)

A specialized backend service for **web scraping, image proxying, and CDN caching**. Built with Axum and focusing on deterministic performance and low resource overhead.

## 🛠 Core Functionality

This service provides a technical foundation for:

1. **Automated Scraping**: Data extraction from Anime and Komik sources using static and dynamic engines.
2. **Image Proxying**: Managed proxying to bypass origin-server hotlinking protections.
3. **CDN Caching**: Automated image persistence to external CDN (Picser) storage via GitHub API.
4. **Audit & Repair**: Automated health checks to identify and re-upload inaccessible or corrupted CDN assets.

## 📚 Technical Documentation

- [Scraping Architecture](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/docs/scraping.md) - Detail on engines, concurrency, and data structures.
- [Image Proxy & CDN](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/docs/proxy_cdn.md) - Cache layers, request coalescing, and Picser implementation.
- [Observability & Metrics](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/docs/observability.md) - Prometheus integration and latency tracking schema.
- [API Reference](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/docs/api_reference.md) - Endpoint definitions and expected response formats.
- [Development Guide](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/docs/development.md) - Coding standards and maintenance protocols.

## 🤖 AI Assistant Guidelines

If you are an AI assistant (Gemini, Claude, GPT, etc.) working on this repository, please start by reading the following:

- **[AGENT.md](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/AGENT.md)**: Universal entry point for all agents.
- **[GEMINI.md](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/GEMINI.md)**: Internal technical overview and logic flows.
- **[CLAUDE.md](file:///mnt/code/bp3/ultimate-asepharyana.tech/apps/rust/CLAUDE.md)**: Tone and professionalism protocols.

## 🛠 Tech Stack

- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) (0.8.8) - Asynchronous Rust HTTP framework.
- **Database**: [SeaORM](https://www.sea-ql.org/SeaORM/) (MySQL) - Database abstraction layer.
- **Caching**: `deadpool-redis` & `redis` - In-memory data store for cache mapping.
- **Observability**: [Prometheus](https://prometheus.io/) - Performance instrumentation and histograms.
- **Scraping**: `scraper` (Static HTML) & `chromiumoxide` (Headless Chrome).

## 📊 Monitoring

The service exposes standardized Prometheus metrics for performance analysis.

- **Endpoint**: `GET /metrics`
- **Primary Metrics**:
  - `axum_http_requests_duration_seconds`: Response latency histogram.
  - `image_cache_hit_total`: Cache hit/miss distribution analysis.
  - `image_upload_duration_seconds`: External CDN upload timing.
  - `image_upload_failure_total`: Upload reliability tracking.

## 📁 Endpoints

| Endpoint | Method | Description |
| :--- | :--- | :--- |
| `/metrics` | `GET` | Prometheus metrics export |
| `/api/anime2/*` | `GET` | Anime scraping endpoints |
| `/api/komik/*` | `GET` | Komik scraping endpoints |
| `/api/proxy/image-cache` | `POST` | Process and return CDN URL |
| `/api/proxy/image-cache/audit` | `POST` | Health check for CDN assets |

## 📦 Run Instructions

```bash
# Start development server
cargo run

# Create optimized production build
cargo build --release
```

## 🏗 Maintenance Principles

- **Minimalist Architecture**: All non-essential modules (Auth, Social, GraphQL) are removed to maintain a lean binary.
- **Deterministic Build**: Controlled dependency tree to ensure consistent compile times and runtime behavior.
- **Direct Logic**: Prioritize native implementation over heavy external dependencies where possible.

## License

MIT License
