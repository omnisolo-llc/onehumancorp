use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use ohc_builtin_agent::mesh::transport::MeshTransport;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshMetrics {
    pub active_agents: usize,
    pub messages_sent: u64,
    pub lock_contention_rate: f64,
    pub sync_backlog: usize,
    pub latency_ms: f64,
    pub errors_last_hour: usize,
}

impl Default for MeshMetrics {
    fn default() -> Self {
        Self {
            active_agents: 0,
            messages_sent: 0,
            lock_contention_rate: 0.0,
            sync_backlog: 0,
            latency_ms: 0.0,
            errors_last_hour: 0,
        }
    }
}

pub async fn mesh_report_ui_handler(
    State(transport): State<Arc<dyn MeshTransport>>,
) -> impl IntoResponse {
    let mut metrics = MeshMetrics::default();

    if let Ok(transport_metrics) = transport.get_metrics().await {
        metrics.active_agents = transport_metrics.active_agents;
        metrics.messages_sent = transport_metrics.messages_sent;
        metrics.lock_contention_rate = transport_metrics.lock_contention_rate;
        metrics.errors_last_hour = transport_metrics.errors_last_hour;
        metrics.latency_ms = transport_metrics.avg_latency_ms;
        metrics.sync_backlog = transport_metrics.subscriptions_active; // Approximated
    } else if let Ok(active_agents) = transport.get_active_agents().await {
        metrics.active_agents = active_agents.len();
    }

    let html = crate::api::mesh_report_view::generate_mesh_report_html(&metrics);
    Html(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;

    #[tokio::test]
    async fn test_handler_wiring() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let _resp = mesh_report_ui_handler(State(transport)).await;
    }
}
