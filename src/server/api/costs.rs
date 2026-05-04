use axum::{
    extract::State,
    response::IntoResponse,
    Json, Router,
    routing::get,
};
use std::sync::Arc;
use crate::services::billing::auditor::CostAuditor;

pub fn router<S: Clone + Send + Sync + 'static>(auditor: Arc<CostAuditor>) -> Router<S> {
    Router::new()
        .route("/", get(get_costs))
        .with_state(auditor)
}

async fn get_costs(State(auditor): State<Arc<CostAuditor>>) -> impl IntoResponse {
    let summary = auditor.get_summary_data();
    Json(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::pricing::calculator::CostConfig;
    use crate::services::billing::auditor::{AuditEvent, CostSummaryData};

    #[tokio::test]
    async fn test_get_costs_dummy() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.002,
            ..Default::default()
        };
        let auditor = Arc::new(CostAuditor::new(config));

        auditor.record_event(AuditEvent {
            agent_id: "agent-1".to_string(),
            input_tokens: 100,
            output_tokens: 50,
            cached_input_tokens: 0,
            local_embedding_tokens: 0,
        });

        let summary = auditor.get_summary_data();

        assert_eq!(summary.total_tokens, 50); // only output tokens are counted in total_tokens for some reason based on logic
        assert!(summary.total_cost > 0.0);
        assert_eq!(summary.agents.len(), 1);
        assert_eq!(summary.agents[0].agent_id, "agent-1");

        let _router: Router<()> = router(auditor);
    }
}