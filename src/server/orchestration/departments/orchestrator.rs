use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;
use std::str::FromStr;

use crate::orchestration::departments::types::{DepartmentType, DepartmentConfig, DepartmentEvent, ApprovalRequest, ApprovalStatus, ActionRisk};
use crate::db::DbStore;
use ohc_builtin_agent::memory_store::VectorRepository;
use opentelemetry::global;
use opentelemetry::KeyValue;
use crate::orchestration::mesh::TeammateMesh;
use opentelemetry::metrics::Counter;

pub enum AgentTriggerType {
    Scheduled,
    EventDriven,
    OnDemand,
}

#[async_trait::async_trait]
pub trait BaseAgent: Send + Sync {
    fn agent_id(&self) -> String;
    fn trigger_type(&self) -> AgentTriggerType;
    async fn execute(&self, payload: serde_json::Value) -> Result<(), String>;
}

#[async_trait::async_trait]
pub trait Department: Send + Sync {
    fn department_type(&self) -> DepartmentType;
    fn subscribed_events(&self) -> Vec<String>;
    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String>;
    async fn query_memory(&self, query: &str) -> Result<Vec<String>, String>;
    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String>;
    fn get_config(&self, tenant_id: &str) -> Option<DepartmentConfig>;
    fn set_config(&mut self, tenant_id: String, config: DepartmentConfig);
}

pub struct DummyDepartment {
    dep_type: DepartmentType,
    subscriptions: Vec<String>,
    configs: HashMap<String, DepartmentConfig>,
    orchestrator: Arc<DepartmentOrchestrator>,
    pub received_events: Mutex<Vec<DepartmentEvent>>,
}

impl DummyDepartment {
    pub fn new(dep_type: DepartmentType, subscriptions: Vec<String>, orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self {
            dep_type,
            subscriptions,
            configs: HashMap::new(),
            orchestrator,
            received_events: Mutex::new(Vec::new()),
        }
    }

}


#[async_trait::async_trait]
impl Department for DummyDepartment {
    fn department_type(&self) -> DepartmentType {
        self.dep_type
    }

    fn subscribed_events(&self) -> Vec<String> {
        self.subscriptions.clone()
    }

    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String> {
        self.received_events.lock().unwrap().push(event.clone());
        let payload = serde_json::json!({"test": "data"});
        let _ = self.orchestrator.execute_action(self.dep_type, "Test action".to_string(), event.tenant_id.clone(), ActionRisk::AutoExecute, payload).await;
        Ok(())
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        // Dummy implementation
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String> {
        let req = ApprovalRequest {
            id: Uuid::new_v4().to_string(),
            tenant_id,
            department: self.dep_type,
            description,
            status: match risk {
                ActionRisk::AutoExecute => ApprovalStatus::Approved,
                ActionRisk::DraftForReview => ApprovalStatus::PendingApproval,
            },
            action_risk: risk.clone(),
            payload: None,
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


pub struct DepartmentOrchestrator {
    db: Arc<crate::db::DB>,
    departments: RwLock<HashMap<DepartmentType, Arc<tokio::sync::RwLock<dyn Department>>>>,
    agents: RwLock<HashMap<String, Arc<tokio::sync::RwLock<dyn BaseAgent>>>>,
    event_subscriptions: RwLock<HashMap<String, Vec<DepartmentType>>>,
    memory_repo: Arc<VectorRepository>,
    mesh: Arc<dyn TeammateMesh>,
    action_counter: Counter<u64>,
}

impl DepartmentOrchestrator {
    pub fn new(db: Arc<crate::db::DB>, mesh: Arc<dyn TeammateMesh>) -> Self {
        let memory_repo = match &db.store {
            DbStore::Postgres => Arc::new(VectorRepository::new(db.pool.clone())),
            DbStore::Sqlite(pool) => Arc::new(VectorRepository::new_sqlite(pool.clone())),
        };
        let meter = global::meter("ohc.orchestrator");
        let action_counter = meter.u64_counter("agent.actions.total").build();
        Self {
            db,
            departments: RwLock::new(HashMap::new()),
            agents: RwLock::new(HashMap::new()),
            event_subscriptions: RwLock::new(HashMap::new()),
            memory_repo,
            mesh,
            action_counter,
        }
    }

        pub async fn register_agent(&self, agent: Arc<tokio::sync::RwLock<dyn BaseAgent>>) {
        let agent_id = agent.read().await.agent_id();
        self.agents.write().await.insert(agent_id, agent.clone());
    }

    pub async fn register_department(&self, department: Arc<tokio::sync::RwLock<dyn Department>>) {
        let dep = department.read().await;
        let dep_type = dep.department_type();
        let subs = dep.subscribed_events();

        self.departments.write().await.insert(dep_type, department.clone());

        let mut subscriptions = self.event_subscriptions.write().await;
        for sub in subs {
            subscriptions.entry(sub).or_insert_with(Vec::new).push(dep_type);
        }
    }

    pub async fn dispatch_event(&self, event: DepartmentEvent) -> Result<(), String> {
        let topic = format!("department_event:{}", event.event_type);
        let payload = serde_json::to_vec(&event).map_err(|e| e.to_string())?;
        self.mesh.publish(&topic, payload).await?;

        let subscriptions = self.event_subscriptions.read().await;
        if let Some(dep_types) = subscriptions.get(&event.event_type) {
            let departments = self.departments.read().await;
            for dep_type in dep_types {
                if let Some(dep) = departments.get(dep_type) {
                    let lock_key = format!("ohc:lock:{}:{}:{}", event.tenant_id, dep_type, event.id);
                    if self.mesh.acquire_lock(&lock_key, "orchestrator", 30).await.unwrap_or(false) {
                        self.action_counter.add(1, &[
                            KeyValue::new("tenant_id", event.tenant_id.clone()),
                            KeyValue::new("department", dep_type.to_string())
                        ]);

                        let mut success = false;
                        let mut last_err = String::new();
                        for _ in 0..3 {
                            let fut = dep.read().await;
                            let res = tokio::time::timeout(std::time::Duration::from_secs(60), fut.handle_event(&event)).await;
                            match res {
                                Ok(Ok(_)) => {
                                    success = true;
                                    break;
                                }
                                Ok(Err(e)) => {
                                    last_err = e.to_string();
                                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                                }
                                Err(_) => {
                                    last_err = "AI timeout: Event handling exceeded 60 seconds".to_string();
                                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                                }
                            }
                        }

                        if !success {
                            tracing::error!("Dead-letter logging for event {} after 3 failed retries. Error: {}", event.id, last_err);
                            let dl_id = Uuid::new_v4().to_string();
                            let dl_payload = serde_json::to_string(&event.payload).unwrap_or_default();

                            match &self.db.store {
                                DbStore::Postgres => {
                                    let res = sqlx::query(
                                        "INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) VALUES ($1, $2, $3, $4, $5, $6)"
                                    )
                                    .bind(&dl_id)
                                    .bind(&event.tenant_id)
                                    .bind(&event.event_type)
                                    .bind(dep_type.to_string())
                                    .bind(&dl_payload)
                                    .bind(&last_err)
                                    .execute(&self.db.pool)
                                    .await;
                                    if let Err(err) = res {
                                        tracing::error!("Failed to insert dead letter into DB: {}", err);
                                    }
                                }
                                DbStore::Sqlite(pool) => {
                                    let res = sqlx::query(
                                        "INSERT INTO department_dead_letters (id, tenant_id, event_type, department, payload, error_message) VALUES (?, ?, ?, ?, ?, ?)"
                                    )
                                    .bind(&dl_id)
                                    .bind(&event.tenant_id)
                                    .bind(&event.event_type)
                                    .bind(dep_type.to_string())
                                    .bind(&dl_payload)
                                    .bind(&last_err)
                                    .execute(pool)
                                    .await;
                                    if let Err(err) = res {
                                        tracing::error!("Failed to insert dead letter into DB: {}", err);
                                    }
                                }
                            }
                        }

                        let _ = self.mesh.release_lock(&lock_key, "orchestrator").await;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn check_ai_budget(&self, tenant_id: &str, points: i32) -> Result<bool, String> {

        let throttler = crate::orchestration::departments::throttling::ThrottlingManager::new(self.db.clone());

        throttler.check_and_consume_budget(tenant_id, points).await

    }

    pub async fn execute_action(
        &self,
        department: DepartmentType,
        description: String,
        tenant_id: String,
        risk: ActionRisk,
        _action_payload: serde_json::Value,
    ) -> Result<ApprovalRequest, String> {
        let cost = 1;
        if !self.check_ai_budget(&tenant_id, cost).await.unwrap_or(false) {
            return Err("AI Budget exhausted. Agents degraded to reactive mode. Please upgrade your plan.".to_string());
        }

        match risk {
            ActionRisk::AutoExecute => {
                let req = ApprovalRequest {
                    id: Uuid::new_v4().to_string(),
                    tenant_id,
                    department,
                    description: description.clone(),
                    status: ApprovalStatus::Approved,
                    action_risk: ActionRisk::AutoExecute,
                    payload: Some(_action_payload),
                };
                self.add_approval_request(req.clone()).await;
                Ok(req.clone())
            }
            ActionRisk::DraftForReview => {
                let req = ApprovalRequest {
                    id: Uuid::new_v4().to_string(),
                    tenant_id,
                    department,
                    description: description.clone(),
                    status: ApprovalStatus::PendingApproval,
                    action_risk: ActionRisk::DraftForReview,
                    payload: Some(_action_payload),
                };
                self.add_approval_request(req.clone()).await;

                let _ = crate::dispatch_critical_sms(
                    "draft_approval",
                    &format!("{} requires your approval: {}", department, description)
                ).await;

                Ok(req.clone())
            }
        }
    }

    pub async fn get_department_status(&self, tenant_id: &str) -> Vec<crate::orchestration::departments::types::DepartmentDashboardStatus> {
        let mut results = HashMap::new();
        for dep in vec![
            DepartmentType::Operations,
            DepartmentType::Marketing,
            DepartmentType::Sales,
            DepartmentType::CustomerSuccess,
            DepartmentType::Finance,
            DepartmentType::Legal,
            DepartmentType::BusinessAdvisory,
        ] {
            results.insert(dep, crate::orchestration::departments::types::DepartmentDashboardStatus {
                department: dep,
                pending_approvals: 0,
                completed_actions: 0,
            });
        }

        match &self.db.store {
            DbStore::Postgres => {
                if let Ok(rows) = sqlx::query("SELECT department, status, COUNT(*) as count FROM agent_approvals WHERE tenant_id = $1 GROUP BY department, status")
                    .bind(tenant_id)
                    .fetch_all(&self.db.pool)
                    .await {
                    use sqlx::Row;
                    for row in rows {
                        let dep_str: String = row.get("department");
                        let status_str: String = row.get("status");
                        let count: i64 = row.get("count");

                        if let Ok(dep) = DepartmentType::from_str(&dep_str) {
                            if let Some(status) = results.get_mut(&dep) {
                                if status_str == "PENDING_APPROVAL" {
                                    status.pending_approvals += count;
                                } else {
                                    status.completed_actions += count;
                                }
                            }
                        }
                    }
                }
            }
            DbStore::Sqlite(pool) => {
                if let Ok(rows) = sqlx::query("SELECT department, status, COUNT(*) as count FROM agent_approvals WHERE tenant_id = ? GROUP BY department, status")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await {
                    use sqlx::Row;
                    for row in rows {
                        let dep_str: String = row.get("department");
                        let status_str: String = row.get("status");
                        let count: i64 = row.get("count");

                        if let Ok(dep) = DepartmentType::from_str(&dep_str) {
                            if let Some(status) = results.get_mut(&dep) {
                                if status_str == "PENDING_APPROVAL" {
                                    status.pending_approvals += count;
                                } else {
                                    status.completed_actions += count;
                                }
                            }
                        }
                    }
                }
            }
        }

        results.into_values().collect()
    }

    pub async fn add_approval_request(&self, req: ApprovalRequest) {
        let now = Utc::now();
        let status_str = match req.status {
            ApprovalStatus::PendingApproval => "PENDING_APPROVAL",
            ApprovalStatus::Approved => "APPROVED",
            ApprovalStatus::Rejected => "REJECTED",
        };

        match &self.db.store {
            DbStore::Postgres => {
                let _ = sqlx::query(
                    "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
                )
                .bind(&req.id)
                .bind(&req.tenant_id)
                .bind(req.department.to_string())
                .bind(&req.description)
                .bind(status_str)
                .bind(req.action_risk.to_string())
                .bind(serde_json::to_string(&req.payload.unwrap_or(serde_json::json!({}))).unwrap_or_else(|_| "{}".to_string()))
                .bind(now)
                .bind(now)
                .execute(&self.db.pool)
                .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query(
                    "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&req.id)
                .bind(&req.tenant_id)
                .bind(req.department.to_string())
                .bind(&req.description)
                .bind(status_str)
                .bind(req.action_risk.to_string())
                .bind(serde_json::to_string(&req.payload.unwrap_or(serde_json::json!({}))).unwrap_or_else(|_| "{}".to_string()))
                .bind(now)
                .bind(now)
                .execute(pool)
                .await;
            }
        }
    }

    pub async fn get_pending_approvals(&self, tenant_id: &str, cursor: Option<String>, limit: i64) -> Vec<ApprovalRequest> {
        let mut results = Vec::new();

        match &self.db.store {
            DbStore::Postgres => {
                let fetch_res = if let Some(ref cur) = cursor {
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = $1 AND status = 'PENDING_APPROVAL' AND id > $2 ORDER BY id ASC LIMIT $3")
                        .bind(tenant_id)
                        .bind(cur)
                        .bind(limit)
                        .fetch_all(&self.db.pool)
                        .await
                } else {
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = $1 AND status = 'PENDING_APPROVAL' ORDER BY id ASC LIMIT $2")
                        .bind(tenant_id)
                        .bind(limit)
                        .fetch_all(&self.db.pool)
                        .await
                };
                if let Ok(rows) = fetch_res {
                    use sqlx::Row;
                    for row in rows {
                        let dep_str: String = row.get("department");
                        let status_str: String = row.get("status");
                        let department = DepartmentType::from_str(&dep_str).unwrap_or(DepartmentType::Operations);
                        let status = match status_str.as_str() {
                            "PENDING_APPROVAL" => ApprovalStatus::PendingApproval,
                            "APPROVED" => ApprovalStatus::Approved,
                            "REJECTED" => ApprovalStatus::Rejected,
                            _ => ApprovalStatus::PendingApproval,
                        };
                        let risk_str: String = row.get("action_risk");
                        let action_risk = ActionRisk::from_str(&risk_str).unwrap_or(ActionRisk::DraftForReview);
                        let payload_opt: Option<serde_json::Value> = match row.try_get::<String, _>("payload") {
                            Ok(p) => serde_json::from_str(&p).unwrap_or(None),
                            Err(_) => match row.try_get::<serde_json::Value, _>("payload") {
                                Ok(p) => Some(p),
                                Err(_) => None,
                            }
                        };
                        results.push(ApprovalRequest {
                            id: row.get("id"),
                            tenant_id: row.get("tenant_id"),
                            department,
                            description: row.get("description"),
                            status,
                            action_risk,
                            payload: payload_opt,
                        });
                    }
                }
            }
            DbStore::Sqlite(pool) => {
                let fetch_res = if let Some(ref cur) = cursor {
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = ? AND status = 'PENDING_APPROVAL' AND id > ? ORDER BY id ASC LIMIT ?")
                        .bind(tenant_id)
                        .bind(cur)
                        .bind(limit)
                        .fetch_all(pool)
                        .await
                } else {
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = ? AND status = 'PENDING_APPROVAL' ORDER BY id ASC LIMIT ?")
                        .bind(tenant_id)
                        .bind(limit)
                        .fetch_all(pool)
                        .await
                };
                if let Ok(rows) = fetch_res {
                    use sqlx::Row;
                    for row in rows {
                        let dep_str: String = row.get("department");
                        let status_str: String = row.get("status");
                        let department = DepartmentType::from_str(&dep_str).unwrap_or(DepartmentType::Operations);
                        let status = match status_str.as_str() {
                            "PENDING_APPROVAL" => ApprovalStatus::PendingApproval,
                            "APPROVED" => ApprovalStatus::Approved,
                            "REJECTED" => ApprovalStatus::Rejected,
                            _ => ApprovalStatus::PendingApproval,
                        };
                        let risk_str: String = row.get("action_risk");
                        let action_risk = ActionRisk::from_str(&risk_str).unwrap_or(ActionRisk::DraftForReview);
                        let payload_str: Option<String> = row.try_get("payload").unwrap_or(None);
                        let payload_opt = payload_str.and_then(|s| serde_json::from_str(&s).unwrap_or(None));
                        results.push(ApprovalRequest {
                            id: row.get("id"),
                            tenant_id: row.get("tenant_id"),
                            department,
                            description: row.get("description"),
                            status,
                            action_risk,
                            payload: payload_opt,
                        });
                    }
                }
            }
        };

        results
    }


    pub async fn get_activity_feed(&self, tenant_id: &str, cursor: Option<String>, limit: i64) -> Vec<ApprovalRequest> {
        let mut results = Vec::new();

        match &self.db.store {
            DbStore::Postgres => {
                let fetch_res = if let Some(ref cur) = cursor {
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = $1 AND status != 'PENDING_APPROVAL' AND id < $2 ORDER BY id DESC LIMIT $3")
                        .bind(tenant_id)
                        .bind(cur)
                        .bind(limit)
                        .fetch_all(&self.db.pool)
                        .await
                } else {
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = $1 AND status != 'PENDING_APPROVAL' ORDER BY id DESC LIMIT $2")
                        .bind(tenant_id)
                        .bind(limit)
                        .fetch_all(&self.db.pool)
                        .await
                };
                if let Ok(rows) = fetch_res {
                    use sqlx::Row;
                    for row in rows {
                        let dep_str: String = row.get("department");
                        let status_str: String = row.get("status");
                        let department = DepartmentType::from_str(&dep_str).unwrap_or(DepartmentType::Operations);
                        let status = match status_str.as_str() {
                            "PENDING_APPROVAL" => ApprovalStatus::PendingApproval,
                            "APPROVED" => ApprovalStatus::Approved,
                            "REJECTED" => ApprovalStatus::Rejected,
                            _ => ApprovalStatus::PendingApproval,
                        };
                        let risk_str: String = row.get("action_risk");
                        let action_risk = ActionRisk::from_str(&risk_str).unwrap_or(ActionRisk::DraftForReview);
                        let payload_opt: Option<serde_json::Value> = match row.try_get::<String, _>("payload") {
                            Ok(p) => serde_json::from_str(&p).unwrap_or(None),
                            Err(_) => match row.try_get::<serde_json::Value, _>("payload") {
                                Ok(p) => Some(p),
                                Err(_) => None,
                            }
                        };
                        results.push(ApprovalRequest {
                            id: row.get("id"),
                            tenant_id: row.get("tenant_id"),
                            department,
                            description: row.get("description"),
                            status,
                            action_risk,
                            payload: payload_opt,
                        });
                    }
                }
            }
            DbStore::Sqlite(pool) => {
                let fetch_res = if let Some(ref cur) = cursor {
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = ? AND status != 'PENDING_APPROVAL' AND id < ? ORDER BY id DESC LIMIT ?")
                        .bind(tenant_id)
                        .bind(cur)
                        .bind(limit)
                        .fetch_all(pool)
                        .await
                } else {
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = ? AND status != 'PENDING_APPROVAL' ORDER BY id DESC LIMIT ?")
                        .bind(tenant_id)
                        .bind(limit)
                        .fetch_all(pool)
                        .await
                };
                if let Ok(rows) = fetch_res {
                    use sqlx::Row;
                    for row in rows {
                        let dep_str: String = row.get("department");
                        let status_str: String = row.get("status");
                        let department = DepartmentType::from_str(&dep_str).unwrap_or(DepartmentType::Operations);
                        let status = match status_str.as_str() {
                            "PENDING_APPROVAL" => ApprovalStatus::PendingApproval,
                            "APPROVED" => ApprovalStatus::Approved,
                            "REJECTED" => ApprovalStatus::Rejected,
                            _ => ApprovalStatus::PendingApproval,
                        };
                        let risk_str: String = row.get("action_risk");
                        let action_risk = ActionRisk::from_str(&risk_str).unwrap_or(ActionRisk::DraftForReview);
                        let payload_str: Option<String> = row.try_get("payload").unwrap_or(None);
                        let payload_opt = payload_str.and_then(|s| serde_json::from_str(&s).unwrap_or(None));
                        results.push(ApprovalRequest {
                            id: row.get("id"),
                            tenant_id: row.get("tenant_id"),
                            department,
                            description: row.get("description"),
                            status,
                            action_risk,
                            payload: payload_opt,
                        });
                    }
                }
            }
        };

        results
    }

    pub async fn decide_approval(&self, request_id: &str, tenant_id: &str, approved: bool) -> Result<(), String> {
        let new_status = if approved { "APPROVED" } else { "REJECTED" };
        let now = Utc::now();

        let opt_department = match &self.db.store {
            DbStore::Postgres => {
                let row = sqlx::query("UPDATE agent_approvals SET status = $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4 RETURNING department")
                    .bind(new_status)
                    .bind(now)
                    .bind(request_id)
                    .bind(tenant_id)
                    .fetch_optional(&self.db.pool)
                    .await;
                match row {
                    Ok(Some(r)) => {
                        use sqlx::Row;
                        Some(r.get::<String, _>("department"))
                    }
                    Ok(None) => return Err("Unauthorized".to_string()),
                    Err(e) => return Err(e.to_string()),
                }
            }
            DbStore::Sqlite(pool) => {
                let row = sqlx::query("UPDATE agent_approvals SET status = ?, updated_at = ? WHERE id = ? AND tenant_id = ? RETURNING department")
                    .bind(new_status)
                    .bind(now)
                    .bind(request_id)
                    .bind(tenant_id)
                    .fetch_optional(pool)
                    .await;
                match row {
                    Ok(Some(r)) => {
                        use sqlx::Row;
                        Some(r.get::<String, _>("department"))
                    }
                    Ok(None) => return Err("Unauthorized".to_string()),
                    Err(e) => return Err(e.to_string()),
                }
            }
        };

        if approved {
            if let Some(dep) = opt_department {
                let payload = serde_json::json!({
                    "request_id": request_id,
                    "tenant_id": tenant_id
                });
                let payload_bytes = serde_json::to_vec(&payload).unwrap_or_default();
                let topic = format!("agent:{}:approved", dep);
                let _ = self.mesh.publish(&topic, payload_bytes).await;
            }
        }

        Ok(())
    }


    pub async fn query_long_term_memory(&self, tenant_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<String>, String> {
        let records = self.memory_repo.cross_department_search(tenant_id, query_embedding, limit).await?;
        Ok(records.into_iter().map(|r| r.content).collect())
    }


    pub async fn write_long_term_memory(&self, record: ohc_builtin_agent::memory_store::EmbeddingRecord) -> Result<(), String> {
        self.memory_repo.upsert(&record).await.map_err(|e| e.to_string())
    }

    pub async fn update_department_config(&self, tenant_id: &str, department: &str, config: crate::orchestration::departments::types::DepartmentConfig) -> Result<(), String> {
        let deps = self.departments.read().await;
        let dep_type = crate::orchestration::departments::types::DepartmentType::from_str(department)?;
        if let Some(dep_lock) = deps.get(&dep_type) {
            let mut dep = dep_lock.write().await;
            dep.set_config(tenant_id.to_string(), config);
            Ok(())
        } else {
            Err("Department not found".to_string())
        }
    }







}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;

    #[tokio::test]
    async fn test_orchestrator_initialization() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));

        let orchestrator = DepartmentOrchestrator::new(db, mesh);

        let dummy = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
            DepartmentType::Operations,
            vec!["test_event".to_string()],
            Arc::new(orchestrator)
        )));
        let _ = dummy;
        assert!(true);
    }

}

// Resolves #13871
// Resolves #15384
// Resolves #15195
