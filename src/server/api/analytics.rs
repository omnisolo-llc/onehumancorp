use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use std::sync::Arc;

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<crate::hub::Hub>) -> Router<S> {
    Router::new()
        .route("/get", post(move || {
            let hub_clone = hub.clone();
            async move {
                get_analytics(State(hub_clone)).await
            }
        }))
}

#[derive(serde::Serialize)]
pub struct AnalyticsSummaryResponse {
    pub human_agent_ratio: f64,
    pub total_agents: i32,
    pub total_humans: i32,
    pub audit_fidelity_pct: f64,
    pub resumption_latency_ms: i32,
    pub pending_approvals: i32,
    pub active_handoffs: i32,
    pub token_velocity: i64,
}

async fn get_analytics(
    State(hub): State<Arc<crate::hub::Hub>>,
) -> Result<Json<AnalyticsSummaryResponse>, axum::http::StatusCode> {
    let hub1 = hub.clone();
    let hub2 = hub.clone();
    let hub3 = hub.clone();
    let (agents_res, meetings_res, summary_res) = tokio::join!(
        tokio::task::spawn_blocking(move || hub1.get_agents()),
        tokio::task::spawn_blocking(move || hub2.get_meetings()),
        tokio::task::spawn_blocking(move || hub3.tracker().summary("system"))
    );
    let agents = agents_res.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let meetings = meetings_res.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let summary = summary_res.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut total_msgs = 0;
    let mut audited_msgs = 0;
    let mut agent_set = std::collections::HashSet::new();
    for a in agents.iter() {
        agent_set.insert(a.id.clone());
    }

    for m in meetings.iter() {
        for msg in &m.transcript {
            total_msgs += 1;
            if agent_set.contains(&msg.from_agent) {
                audited_msgs += 1;
            }
        }
    }

    let audit_fidelity_pct = if total_msgs > 0 {
        (audited_msgs as f64 / total_msgs as f64) * 100.0
    } else {
        100.0
    };

    let total_agents = agents.len() as i32;

    let total_humans: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&hub.pool)
        .await
        .unwrap_or(0);

    let pending_approvals: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM swarm_tasks WHERE status = 'PENDING'")
        .fetch_one(&hub.pool)
        .await
        .unwrap_or(0);

    let active_handoffs: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_missions WHERE status = 'active'")
        .fetch_one(&hub.pool)
        .await
        .unwrap_or(0);

    let human_agent_ratio = if total_humans > 0 {
        total_agents as f64 / total_humans as f64
    } else {
        0.0
    };

    Ok(Json(AnalyticsSummaryResponse {
        human_agent_ratio,
        total_agents,
        total_humans: total_humans as i32,
        audit_fidelity_pct,
        resumption_latency_ms: 0,
        pending_approvals: pending_approvals as i32,
        active_handoffs: active_handoffs as i32,
        token_velocity: summary.total_tokens,
    }))
}
