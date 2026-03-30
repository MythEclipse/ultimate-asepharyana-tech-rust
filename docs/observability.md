 
# Performance Observability (Prometheus)


The service is instrumented using the `metrics` and `axum-prometheus` crates.

## 📊 Prometheus Endpoint
- **URL**: `GET /metrics`
- **Format**: Prometheus text-based.

## 🚀 Metrics Indicators

### 1. HTTP Request Latency
- **Metric**: `axum_http_requests_duration_seconds`
- **Type**: Histogram
- **Description**: Total time for request-response cycle.
- **Labels**: `method`, `path`, `status`.

### 2. Image Cache Metrics
- **Metric**: `image_cache_hit_total`
- **Labels**: `source` (`redis`, `db`).
- **Metric**: `image_cache_miss_total`
- **Description**: Total new images added to cache.

### 3. CDN Metrics
- **Metric**: `image_upload_duration_seconds`
- **Type**: Histogram
- **Description**: Time spent communicating with Picser/GitHub.
- **Metric**: `image_upload_success_total` / `image_upload_failure_total`.

### 4. Scraping Activity
- **Metric**: `axum_http_requests_total`
- **Labels**: `path` (e.g., `/api/anime2/*`).

## 📈 Dashboarding
To visualize:
1.  Configure service as a **Prometheus Data Source** in Grafana.
2.  Query `axum_http_requests_duration_seconds` for latency analysis.
3.  **Sample Query** (P95 Latency):
    ```promql
    histogram_quantile(0.95, sum(rate(axum_http_requests_duration_seconds_bucket[5m])) by (le, path))
    ```
