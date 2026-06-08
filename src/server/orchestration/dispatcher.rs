use serde::{Deserialize, Serialize};
use crate::orchestration::departments::types::{DepartmentType, ActionRisk};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    UpdateInventory,
    DraftCustomerMessage,
    CreateProduct,
    IssueRefund,
    UpdateBooking,
    Unknown(String),
}

impl From<&str> for ActionType {
    fn from(s: &str) -> Self {
        match s {
            "UpdateInventory" => ActionType::UpdateInventory,
            "DraftCustomerMessage" => ActionType::DraftCustomerMessage,
            "CreateProduct" => ActionType::CreateProduct,
            "IssueRefund" => ActionType::IssueRefund,
            "UpdateBooking" => ActionType::UpdateBooking,
            other => ActionType::Unknown(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPayload {
    pub action_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchRequest {
    pub tenant_id: String,
    pub department: DepartmentType,
    pub payload: ActionPayload,
    pub risk: ActionRisk,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchResponse {
    pub success: bool,
    pub action_id: Option<String>,
    pub error_message: Option<String>,
}

pub struct UnifiedActionDispatcher {
    orchestrator: Arc<DepartmentOrchestrator>,
}

impl UnifiedActionDispatcher {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }

    pub async fn dispatch(&self, request: DispatchRequest) -> DispatchResponse {
        if request.tenant_id.is_empty() {
            return DispatchResponse {
                success: false,
                action_id: None,
                error_message: Some("Tenant ID is required".to_string()),
            };
        }

        let action = ActionType::from(request.payload.action_type.as_str());

        // Basic RBAC/Scope validation
        if let ActionType::Unknown(ref unknown_type) = action {
            return DispatchResponse {
                success: false,
                action_id: None,
                error_message: Some(format!("Unknown action type: {}", unknown_type)),
            };
        }

        // Forward to the underlying orchestrator/department for execution
        match self.orchestrator.execute_action(
            request.department,
            request.description,
            request.tenant_id,
            request.risk,
            request.payload.data,
        ).await {
            Ok(approval_req) => DispatchResponse {
                success: true,
                action_id: Some(approval_req.id),
                error_message: None,
            },
            Err(e) => DispatchResponse {
                success: false,
                action_id: None,
                error_message: Some(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use crate::db::DbStore;

    async fn setup_db_for_test(tenant_id: &str) -> Arc<crate::db::DB> {
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        match &db.store {
            DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, 'Test Tenant', 'starter') ON CONFLICT (id) DO UPDATE SET tier = 'starter'")
                    .bind(tenant_id)
                    .execute(&db.pool)
                    .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES (?, 'Test Tenant', 'starter') ON CONFLICT (tenant_id) DO UPDATE SET tier = 'starter'")
                    .bind(tenant_id)
                    .execute(pool)
                    .await;
            }
        }
        db
    }

    #[tokio::test]
    async fn test_dispatcher_success() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let tenant_id = "test-tenant-dispatcher";
        let db = setup_db_for_test(tenant_id).await;

        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));
        let dispatcher = UnifiedActionDispatcher::new(orchestrator);

        let req = DispatchRequest {
            tenant_id: tenant_id.to_string(),
            department: DepartmentType::CustomerSuccess,
            payload: ActionPayload {
                action_type: "DraftCustomerMessage".to_string(),
                data: serde_json::json!({"customer": "Sarah", "message": "Vegan cake available"}),
            },
            risk: ActionRisk::DraftForReview,
            description: "Draft reply to Sarah".to_string(),
        };

        let res = dispatcher.dispatch(req).await;
        assert!(res.success, "Dispatch should succeed");
        assert!(res.action_id.is_some());
        assert!(res.error_message.is_none());
    }

    #[tokio::test]
    async fn test_dispatcher_unknown_action() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));
        let dispatcher = UnifiedActionDispatcher::new(orchestrator);

        let req = DispatchRequest {
            tenant_id: "test".to_string(),
            department: DepartmentType::Operations,
            payload: ActionPayload {
                action_type: "InvalidActionXYZ".to_string(),
                data: serde_json::json!({}),
            },
            risk: ActionRisk::AutoExecute,
            description: "Invalid".to_string(),
        };

        let res = dispatcher.dispatch(req).await;
        assert!(!res.success);
        assert!(res.error_message.unwrap().contains("Unknown action type: InvalidActionXYZ"));
    }

    #[tokio::test]
    async fn test_dispatcher_missing_tenant_id() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));
        let dispatcher = UnifiedActionDispatcher::new(orchestrator);

        let req = DispatchRequest {
            tenant_id: "".to_string(),
            department: DepartmentType::Operations,
            payload: ActionPayload {
                action_type: "UpdateInventory".to_string(),
                data: serde_json::json!({}),
            },
            risk: ActionRisk::AutoExecute,
            description: "Invalid".to_string(),
        };

        let res = dispatcher.dispatch(req).await;
        assert!(!res.success);
        assert!(res.error_message.unwrap().contains("Tenant ID is required"));
    }
}
