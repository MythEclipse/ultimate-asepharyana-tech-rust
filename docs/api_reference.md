# API Reference

The Rust backend exposes endpoints in a standardized format. Since the removal of Swagger/UI-Docs, this document serves as the primary source of truth for all active routes.

## 📦 Observability

| Endpoint | Method | Response |
| :--- | :--- | :--- |
| `/metrics` | `GET` | Prometheus formatted text |
| `/health` | `GET` | `{"status": "ok"}` |

## 🖼 Image Proxy & CDN

| Endpoint | Method | Request | Response |
| :--- | :--- | :--- | :--- |
| `/api/proxy/image-cache` | `POST` | `{"url": "...", "lazy": bool}` | `ImageCacheResponse` |
| `/api/proxy/image-cache` | `DELETE` | `{"url": "..."}` | `DeleteImageCacheResponse` |
| `/api/proxy/image-cache/batch` | `POST` | `{"urls": ["...", ...]}` | `ImageCacheBatchResponse` |
| `/api/proxy/image-cache/audit` | `POST` | `{"url": "..."}` | `AuditImageCacheResponse` |

## 📺 Anime Scraping (`anime2`)

All routes prefixed with `/api/anime2/`.

| Route | Method | Description |
| :--- | :--- | :--- |
| `/index` | `GET` | Home page for anime list |
| `/latest` | `GET` | Latest updated anime |
| `/ongoing_anime` | `GET` | Ongoing anime with pagination |
| `/complete_anime` | `GET` | Completed anime with pagination |
| `/detail/:id` | `GET` | Full anime detail and episode list |
| `/search?q=...` | `GET` | Search for anime by title |

## 📖 Komik Scraping

All routes prefixed with `/api/komik/`.

| Route | Method | Description |
| :--- | :--- | :--- |
| `/index` | `GET` | Latest uploaded comics |
| `/detail/:id` | `GET` | Comic details and chapter list |
| `/chapter/:id` | `GET` | Chapter images (returns CDN URLs) |
| `/search?q=...` | `GET` | Search for comics by title |

## 📄 Response Formats

- **Standard JSON API**: All responses are JSON unless specified.
- **Error Codes**:
  - `400 Bad Request`: Validation failure.
  - `404 Not Found`: Entity not found on source site.
  - `500 Internal Server Error`: Source site down or scraping error.
  - `503 Service Unavailable`: Rate limited or resource exhaustion.
