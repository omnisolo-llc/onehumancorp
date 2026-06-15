use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    routing::get,
    Router,
    Json,
};
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use ::server_common::Claims;

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_drafts))
        .with_state(orchestrator)
}

async fn list_drafts(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!([]))).into_response(),
    };

    let approvals = orchestrator.get_pending_approvals(&tenant_id, None, 50).await;

    // Filter to only include drafts for inbox messages
    let drafts: Vec<_> = approvals.into_iter()
        .filter(|a| {
            a.department == crate::orchestration::departments::types::DepartmentType::CustomerSuccess
            && a.payload.as_ref().and_then(|p| p.get("feature_type")).and_then(|v| v.as_str()) == Some("ambassador_reply")
            && a.payload.as_ref().and_then(|p| p.get("inbox_message_id")).is_some()
        })
        .collect();

    (StatusCode::OK, Json(drafts)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use crate::db::DB;

    #[tokio::test]
    async fn test_inbox_drafts_filtering() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let db = Arc::new(DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));

        // Create a dummy draft specifically for inbox
        let payload = serde_json::json!({
            "feature_type": "ambassador_reply",
            "inbox_message_id": "test_msg_id_123",
            "generated_response": "Hello there!",
        });

        let approval = orchestrator.execute_action(
            DepartmentType::CustomerSuccess,
            "Draft email for review".to_string(),
            "default".to_string(),
            ActionRisk::DraftForReview,
            payload,
        ).await;

        assert!(approval.is_ok());

        // Call the list_drafts internal logic (conceptually what list_drafts does)
        let approvals = orchestrator.get_pending_approvals("default", None, 50).await;

        let drafts: Vec<_> = approvals.into_iter()
            .filter(|a| {
                a.department == DepartmentType::CustomerSuccess
                && a.payload.as_ref().and_then(|p| p.get("feature_type")).and_then(|v| v.as_str()) == Some("ambassador_reply")
                && a.payload.as_ref().and_then(|p| p.get("inbox_message_id")).is_some()
            })
            .collect();

        assert!(!drafts.is_empty(), "Expected at least one inbox draft");

        let first = &drafts[0];
        assert_eq!(first.department, DepartmentType::CustomerSuccess);
        let p = first.payload.as_ref().unwrap();
        assert_eq!(p.get("feature_type").unwrap().as_str().unwrap(), "ambassador_reply");
        assert_eq!(p.get("inbox_message_id").unwrap().as_str().unwrap(), "test_msg_id_123");
    }
}
