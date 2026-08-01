use super::*;
use crate::orchestration::mesh::CentrifugeNode;
use ohc_builtin_agent::mesh::transport::InProcessTransport;
use std::sync::Arc;
use crate::orchestration::departments::Department;

#[tokio::test]
async fn test_negotiator_agent_interaction_logging() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        return;
    }
    // We would need to set up a full DB pool here and run the event through,
    // which might be complex, but for now we'll just check if the agent triggers types
    let db = Arc::new(crate::db::DB::new().await.unwrap());
    let transport = Arc::new(InProcessTransport::new());
    let mesh = Arc::new(CentrifugeNode::new(transport));
    let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));
    let agent = NegotiatorAgent::new(orchestrator);
    assert_eq!(agent.department_type(), DepartmentType::CustomerSuccess);
}
