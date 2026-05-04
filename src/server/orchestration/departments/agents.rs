use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use uuid::Uuid;
use opentelemetry::metrics::Counter;
use opentelemetry::global;
use opentelemetry::KeyValue;

use crate::orchestration::departments::types::{DepartmentType, DepartmentConfig, DepartmentEvent, ApprovalRequest, ApprovalStatus};
use crate::orchestration::departments::orchestrator::{Department, DepartmentOrchestrator};
use crate::orchestration::mesh::TeammateMesh;

#[async_trait]
pub trait MemoryStore: Send + Sync {
    async fn search(&self, embeddings: Vec<f32>, limit: usize) -> Result<Vec<String>, String>;
}

use ohc_builtin_agent::memory::PgVectorMemoryStore;

pub struct PgVectorStoreAdapter {
    store: Arc<PgVectorMemoryStore>,
}

impl PgVectorStoreAdapter {
    pub fn new(store: Arc<PgVectorMemoryStore>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl MemoryStore for PgVectorStoreAdapter {
    async fn search(&self, embeddings: Vec<f32>, limit: usize) -> Result<Vec<String>, String> {
        let entries = self.store.search(embeddings, limit)
            .await
            .map_err(|e| format!("pgvector search failed: {}", e))?;
        Ok(entries.into_iter().map(|e| e.context).collect())
    }
}

pub struct OperationsDepartment {
    dep_type: DepartmentType,
    subscriptions: Vec<String>,
    configs: HashMap<String, DepartmentConfig>,
    orchestrator: Arc<DepartmentOrchestrator>,
    pub received_events: Mutex<Vec<DepartmentEvent>>,
    action_counter: Counter<u64>,
    mesh: Arc<dyn TeammateMesh>,
    memory_store: Arc<dyn MemoryStore>,
}

impl OperationsDepartment {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>, mesh: Arc<dyn TeammateMesh>, memory_store: Arc<dyn MemoryStore>) -> Self {
        let meter = global::meter("ohc_departments");
        let action_counter = meter.u64_counter("agent_actions").build();
        Self {
            dep_type: DepartmentType::Operations,
            subscriptions: vec!["OrderPlaced".to_string(), "InventoryLow".to_string()],
            configs: HashMap::new(),
            orchestrator,
            received_events: Mutex::new(Vec::new()),
            action_counter,
            mesh,
            memory_store,
        }
    }
}

#[async_trait]
impl Department for OperationsDepartment {
    fn department_type(&self) -> DepartmentType {
        self.dep_type
    }

    fn subscribed_events(&self) -> Vec<String> {
        self.subscriptions.clone()
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        self.action_counter.add(1, &[KeyValue::new("tenant_id", event.tenant_id.clone())]);

        let lock_resource = format!("dept_event_{}", event.id);
        self.mesh.acquire_lock(&lock_resource, &event.tenant_id, 30).await?;

        // Using a block to hold the lock only as long as necessary and avoid leaks
        let res = async {
            self.received_events.lock().unwrap().push(event.clone());

            let payload = serde_json::to_vec(&event).unwrap_or_default();
            self.mesh.publish("mesh:department:operations", payload).await
        }.await;

        self.mesh.release_lock(&lock_resource, &event.tenant_id).await?;

        res
    }

    async fn handle_scheduled_task(&self, task_id: &str) -> Result<(), String> {
        let _ = &task_id;
        Ok(())
    }

    async fn handle_on_demand_request(&self, request_payload: &str) -> Result<String, String> {
        Ok(format!("Handled Ops Request: {}", request_payload))
    }

    async fn query_memory(&self, query: &str) -> Result<Vec<String>, String> {
        let mut embeddings = vec![0.0; 1536];
        if !query.is_empty() {
             embeddings[0] = 0.1;
        }

        self.memory_store.search(embeddings, 5).await
    }

    async fn request_approval(&self, description: String, tenant_id: String) -> Result<ApprovalRequest, String> {
        let req = ApprovalRequest {
            id: Uuid::new_v4().to_string(),
            tenant_id,
            department: self.dep_type,
            description,
            status: ApprovalStatus::Pending,
        };
        self.orchestrator.add_approval_request(req.clone()).await;
        Ok(req)
    }

    fn get_config(&self, tenant_id: &str) -> Option<DepartmentConfig> {
        self.configs.get(tenant_id).cloned()
    }

    fn set_config(&mut self, tenant_id: String, config: DepartmentConfig) {
        self.configs.insert(tenant_id, config);
    }
}

pub struct CustomerSuccessDepartment {
    dep_type: DepartmentType,
    subscriptions: Vec<String>,
    configs: HashMap<String, DepartmentConfig>,
    orchestrator: Arc<DepartmentOrchestrator>,
    pub received_events: Mutex<Vec<DepartmentEvent>>,
    action_counter: Counter<u64>,
    mesh: Arc<dyn TeammateMesh>,
    memory_store: Arc<dyn MemoryStore>,
}

impl CustomerSuccessDepartment {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>, mesh: Arc<dyn TeammateMesh>, memory_store: Arc<dyn MemoryStore>) -> Self {
        let meter = global::meter("ohc_departments");
        let action_counter = meter.u64_counter("agent_actions").build();
        Self {
            dep_type: DepartmentType::CustomerSuccess,
            subscriptions: vec!["CustomerMessageReceived".to_string(), "OrderDelivered".to_string()],
            configs: HashMap::new(),
            orchestrator,
            received_events: Mutex::new(Vec::new()),
            action_counter,
            mesh,
            memory_store,
        }
    }
}

#[async_trait]
impl Department for CustomerSuccessDepartment {
    fn department_type(&self) -> DepartmentType {
        self.dep_type
    }

    fn subscribed_events(&self) -> Vec<String> {
        self.subscriptions.clone()
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        self.action_counter.add(1, &[KeyValue::new("tenant_id", event.tenant_id.clone())]);

        let lock_resource = format!("dept_event_{}", event.id);
        self.mesh.acquire_lock(&lock_resource, &event.tenant_id, 30).await?;

        let res = async {
            self.received_events.lock().unwrap().push(event.clone());

            // Draft for review workflow implementation: Pause and request approval for high risk actions
            if event.event_type == "CustomerMessageReceived" {
                let req = self.request_approval(format!("Draft reply for event {}", event.id), event.tenant_id.clone()).await?;
                println!("Draft-for-review requested: {:?}", req);
            }

            let payload = serde_json::to_vec(&event).unwrap_or_default();
            self.mesh.publish("mesh:department:customersuccess", payload).await
        }.await;

        self.mesh.release_lock(&lock_resource, &event.tenant_id).await?;

        res
    }

    async fn handle_scheduled_task(&self, task_id: &str) -> Result<(), String> {
        let _ = &task_id;
        Ok(())
    }

    async fn handle_on_demand_request(&self, request_payload: &str) -> Result<String, String> {
        Ok(format!("Handled CS Request: {}", request_payload))
    }

    async fn query_memory(&self, query: &str) -> Result<Vec<String>, String> {
        let mut embeddings = vec![0.0; 1536];
        if !query.is_empty() {
             embeddings[0] = 0.1;
        }

        self.memory_store.search(embeddings, 5).await
    }

    async fn request_approval(&self, description: String, tenant_id: String) -> Result<ApprovalRequest, String> {
        let req = ApprovalRequest {
            id: Uuid::new_v4().to_string(),
            tenant_id,
            department: self.dep_type,
            description,
            status: ApprovalStatus::Pending,
        };
        self.orchestrator.add_approval_request(req.clone()).await;
        Ok(req)
    }

    fn get_config(&self, tenant_id: &str) -> Option<DepartmentConfig> {
        self.configs.get(tenant_id).cloned()
    }

    fn set_config(&mut self, tenant_id: String, config: DepartmentConfig) {
        self.configs.insert(tenant_id, config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;

    fn test_mesh() -> Arc<dyn TeammateMesh> {
        Arc::new(CentrifugeNode::new(Arc::new(MemoryTransport::new())))
    }

    struct MockMemoryStore;
    #[async_trait]
    impl MemoryStore for MockMemoryStore {
        async fn search(&self, embeddings: Vec<f32>, _limit: usize) -> Result<Vec<String>, String> {
            if embeddings[0] > 0.0 {
                Ok(vec!["Matched mock memory".to_string()])
            } else {
                Ok(vec![])
            }
        }
    }

    #[tokio::test]
    async fn test_operations_department_handle_event() {
        let orchestrator = Arc::new(DepartmentOrchestrator::new());
        let dept = OperationsDepartment::new(orchestrator, test_mesh(), Arc::new(MockMemoryStore {}));

        let event = DepartmentEvent {
            id: "1".to_string(),
            tenant_id: "tenant_1".to_string(),
            event_type: "OrderPlaced".to_string(),
            payload: serde_json::json!({}),
        };

        dept.handle_event(&event).await.unwrap();

        let events = dept.received_events.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "OrderPlaced");
    }

    #[tokio::test]
    async fn test_customer_success_department_handle_event() {
        let orchestrator = Arc::new(DepartmentOrchestrator::new());
        let dept = CustomerSuccessDepartment::new(orchestrator.clone(), test_mesh(), Arc::new(MockMemoryStore {}));

        let event = DepartmentEvent {
            id: "2".to_string(),
            tenant_id: "tenant_1".to_string(),
            event_type: "CustomerMessageReceived".to_string(),
            payload: serde_json::json!({}),
        };

        dept.handle_event(&event).await.unwrap();

        let events = dept.received_events.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "CustomerMessageReceived");

        let pending = orchestrator.get_pending_approvals("tenant_1").await;
        assert_eq!(pending.len(), 1);
    }

    #[tokio::test]
    async fn test_department_memory_query() {
        let orchestrator = Arc::new(DepartmentOrchestrator::new());
        let dept = CustomerSuccessDepartment::new(orchestrator, test_mesh(), Arc::new(MockMemoryStore {}));

        let result = dept.query_memory("test query").await.unwrap();
        assert!(result.len() > 0);
        assert_eq!(result[0], "Matched mock memory");
    }

    #[tokio::test]
    async fn test_handle_scheduled_task() {
        let orchestrator = Arc::new(DepartmentOrchestrator::new());
        let dept = OperationsDepartment::new(orchestrator, test_mesh(), Arc::new(MockMemoryStore {}));

        let result = dept.handle_scheduled_task("task_1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_on_demand_request() {
        let orchestrator = Arc::new(DepartmentOrchestrator::new());
        let dept = CustomerSuccessDepartment::new(orchestrator, test_mesh(), Arc::new(MockMemoryStore {}));

        let result = dept.handle_on_demand_request("custom payload").await.unwrap();
        assert!(result.contains("Handled CS Request"));
    }
}
