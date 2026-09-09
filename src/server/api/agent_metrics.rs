//! Tenant-filtered measurements for completed agent requests.
use crate::hub::Hub;
use axum::{
    Json, Router,
    extract::{Extension, State},
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
};
use chrono::{DateTime, Utc};
use futures_util::{Stream, stream};
use serde::Serialize;
use server_common::Claims;
use std::{collections::HashMap, convert::Infallible, sync::Arc, time::Duration};

#[derive(Clone, Default)]
pub struct ExecutionMeasurements {
    pub messages_processed: u64,
    elapsed_ms: f64,
    errors: u64,
    pub last_active_at: Option<DateTime<Utc>>,
}
impl ExecutionMeasurements {
    pub(crate) fn record(&mut self, duration: Duration, failed: bool) {
        self.messages_processed += 1;
        self.elapsed_ms += duration.as_secs_f64() * 1000.0;
        self.errors += u64::from(failed);
        self.last_active_at = Some(Utc::now());
    }
    pub fn average_ms(&self) -> Option<f64> {
        (self.messages_processed > 0).then(|| self.elapsed_ms / self.messages_processed as f64)
    }
    pub fn error_rate(&self) -> Option<f64> {
        (self.messages_processed > 0).then(|| self.errors as f64 / self.messages_processed as f64)
    }
}
#[derive(Serialize)]
pub struct AgentMetrics {
    agent_id: String,
    messages_processed: u64,
    avg_response_time_ms: Option<f64>,
    active_connections: usize,
    memory_usage_bytes: Option<u64>,
    last_active_at: Option<DateTime<Utc>>,
    error_rate: Option<f64>,
    cost_accumulated: f64,
}
async fn snapshot(hub: &Hub, org: &str) -> Vec<AgentMetrics> {
    let costs: HashMap<_, _> = hub
        .get_cost_auditor()
        .get_agent_costs_snapshot()
        .into_iter()
        .map(|(id, cost, _, _, _, _)| (id, cost))
        .collect();
    let mut rows = Vec::new();
    for agent in hub.get_agents_by_org(org).await {
        let (sample, connections) = hub.agent_measurements(&agent.id).await;
        rows.push(AgentMetrics {
            cost_accumulated: costs.get(&agent.id).copied().unwrap_or_default(),
            memory_usage_bytes: crate::telemetry::sandbox_memory_sample(&agent.id),
            agent_id: agent.id,
            messages_processed: sample.messages_processed,
            avg_response_time_ms: sample.average_ms(),
            active_connections: connections,
            last_active_at: sample.last_active_at,
            error_rate: sample.error_rate(),
        });
    }
    rows.sort_by(|a, b| a.agent_id.cmp(&b.agent_id));
    rows
}
pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/api/v1/agents/metrics", get(get_metrics))
        .route("/api/v1/agents/metrics/stream", get(stream_metrics))
        .with_state(hub)
}
async fn get_metrics(
    State(hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<AgentMetrics>>, StatusCode> {
    let org = claims
        .organization_id
        .filter(|org| !org.is_empty())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(Json(snapshot(&hub, &org).await))
}
async fn stream_metrics(
    State(hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let org = claims
        .organization_id
        .filter(|org| !org.is_empty())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let interval = tokio::time::interval(Duration::from_secs(5));
    let events = stream::unfold(
        (hub, org, interval, claims.exp),
        |(hub, org, mut interval, expiry)| async move {
            interval.tick().await;
            if Utc::now().timestamp() >= expiry {
                return None;
            }
            let data = snapshot(&hub, &org).await;
            let event = Event::default()
                .event("metrics")
                .json_data(data)
                .expect("finite metrics serialize");
            Some((Ok(event), (hub, org, interval, expiry)))
        },
    );
    Ok(Sse::new(events).keep_alive(KeepAlive::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measurements_report_actual_duration_and_error_rate() {
        let mut sample = ExecutionMeasurements::default();
        sample.record(Duration::from_millis(100), false);
        sample.record(Duration::from_millis(300), true);
        assert_eq!(sample.messages_processed, 2);
        assert_eq!(sample.average_ms(), Some(200.0));
        assert_eq!(sample.error_rate(), Some(0.5));
        assert!(sample.last_active_at.is_some());
    }

    #[test]
    fn absent_measurements_are_unknown_instead_of_fabricated() {
        let sample = ExecutionMeasurements::default();
        assert_eq!(sample.average_ms(), None);
        assert_eq!(sample.error_rate(), None);
        assert_eq!(sample.last_active_at, None);
    }
}
