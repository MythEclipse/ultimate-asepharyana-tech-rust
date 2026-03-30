//! Prometheus metrics exporter setup.

use axum_prometheus::PrometheusMetricLayer;
use metrics_exporter_prometheus::PrometheusHandle;

/// Returns the prometheus metrics layer and a handle to render the metrics.
pub fn setup_metrics() -> (PrometheusMetricLayer<'static>, PrometheusHandle) {
    PrometheusMetricLayer::pair()
}
