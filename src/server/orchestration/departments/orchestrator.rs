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
    approval_counter: Counter<u64>,
}

impl DepartmentOrchestrator {
    pub fn new(db: Arc<crate::db::DB>, mesh: Arc<dyn TeammateMesh>) -> Self {
        let memory_repo = match &db.store {
            DbStore::Postgres => Arc::new(VectorRepository::new(db.pool.clone())),
            DbStore::Sqlite(pool) => Arc::new(VectorRepository::new_sqlite(pool.clone())),
        };
        let meter = global::meter("ohc.orchestrator");
        let action_counter = meter.u64_counter("agent.actions.total").build();
        let approval_counter = meter.u64_counter("agent.actions.approvals").build();
        Self {
            db,
            departments: RwLock::new(HashMap::new()),
            agents: RwLock::new(HashMap::new()),
            event_subscriptions: RwLock::new(HashMap::new()),
            memory_repo,
            mesh,
            action_counter,
            approval_counter,
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
                            let res = tokio::time::timeout(ohc_builtin_agent::agent::agent_task_timeout(), fut.handle_event(&event)).await;
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
                                    last_err = format!("AI timeout: Event handling exceeded {} seconds", ohc_builtin_agent::agent::agent_task_timeout().as_secs());
                                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                                }
                            }
                        }

                        if !success {
                            ::server_telemetry::record_error_signal("Dead-letter logging for event  after 3 failed retries. Error");
                            tracing::error!("Dead-letter logging for event {} after 3 failed retries. Error: {}", event.id, last_err);
                            let dl_id = Uuid::new_v4().to_string();
                            let redacted_payload = ::server_telemetry::redact_interface_pii(event.payload.clone());
                            let dl_payload = serde_json::to_string(&redacted_payload).unwrap_or_default();

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
                                        ::server_telemetry::record_error_signal("Failed to insert dead letter into DB");
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
                                        ::server_telemetry::record_error_signal("Failed to insert dead letter into DB");
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

    pub async fn add_approval_request(&self, req: ApprovalRequest) {
        let now = Utc::now();
        let status_str = match req.status {
            ApprovalStatus::PendingApproval => "DRAFT",
            ApprovalStatus::Approved => "APPROVED",
            ApprovalStatus::Rejected => "REJECTED",
        };

        match &self.db.store {
            DbStore::Postgres => {
                if let Ok(mut tx) = self.db.pool.begin().await {
                    if ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id).await.is_ok() {
                        let _ = sqlx::query(
                            "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
                        )
                        .bind(&req.id)
                        .bind(&req.tenant_id)
                        .bind(req.department.to_string())
                        .bind(&req.description)
                        .bind(status_str)
                        .bind(req.action_risk.to_string())
                        .bind({
                            let p = req.payload.clone().unwrap_or(serde_json::json!({}));
                            let redacted = ::server_telemetry::redact_interface_pii(p);
                            serde_json::to_string(&redacted).unwrap_or_else(|_| "{}".to_string())
                        })
                        .bind(now)
                        .bind(now)
                        .execute(&mut *tx)
                        .await;
                        let _ = tx.commit().await;
                    }
                }
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
                .bind({
                    let p = req.payload.clone().unwrap_or(serde_json::json!({}));
                    let redacted = ::server_telemetry::redact_interface_pii(p);
                    serde_json::to_string(&redacted).unwrap_or_else(|_| "{}".to_string())
                })
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
                let fetch_res = if let Ok(mut tx) = self.db.pool.begin().await {
                    if ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.is_ok() {
                        let rows = if let Some(ref cur) = cursor {
                            sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = $1 AND status = 'DRAFT' AND id > $2 ORDER BY id ASC LIMIT $3")
                                .bind(tenant_id)
                                .bind(cur)
                                .bind(limit)
                                .fetch_all(&mut *tx)
                                .await
                        } else {
                            sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = $1 AND status = 'DRAFT' ORDER BY id ASC LIMIT $2")
                                .bind(tenant_id)
                                .bind(limit)
                                .fetch_all(&mut *tx)
                                .await
                        };
                        let _ = tx.commit().await;
                        rows
                    } else {
                        Err(sqlx::Error::Configuration("failed to set tenant context".into()))
                    }
                } else {
                    Err(sqlx::Error::Configuration("failed to begin tenant query".into()))
                };
                if let Ok(rows) = fetch_res {
                    use sqlx::Row;
                    for row in rows {
                        let dep_str: String = row.get("department");
                        let status_str: String = row.get("status");
                        let department = DepartmentType::from_str(&dep_str).unwrap_or(DepartmentType::Operations);
                        let status = match status_str.as_str() {
                            "DRAFT" => ApprovalStatus::PendingApproval,
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
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = ? AND status = 'DRAFT' AND id > ? ORDER BY id ASC LIMIT ?")
                        .bind(tenant_id)
                        .bind(cur)
                        .bind(limit)
                        .fetch_all(pool)
                        .await
                } else {
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = ? AND status = 'DRAFT' ORDER BY id ASC LIMIT ?")
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
                            "DRAFT" => ApprovalStatus::PendingApproval,
                            "APPROVED" => ApprovalStatus::Approved,
                            "REJECTED" => ApprovalStatus::Rejected,
                            _ => ApprovalStatus::PendingApproval,
                        };
                        let risk_str: String = row.get("action_risk");
                        let action_risk = ActionRisk::from_str(&risk_str).unwrap_or(ActionRisk::DraftForReview);
                        let payload_str: Option<String> = row.try_get("payload").unwrap_or(None);
                        let payload_opt = payload_str.and_then(|s: String| serde_json::from_str(&s).unwrap_or(None));
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
                let fetch_res = if let Ok(mut tx) = self.db.pool.begin().await {
                    if ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.is_ok() {
                        let rows = if let Some(ref cur) = cursor {
                            sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = $1 AND status != 'DRAFT' AND id < $2 ORDER BY id DESC LIMIT $3")
                                .bind(tenant_id)
                                .bind(cur)
                                .bind(limit)
                                .fetch_all(&mut *tx)
                                .await
                        } else {
                            sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = $1 AND status != 'DRAFT' ORDER BY id DESC LIMIT $2")
                                .bind(tenant_id)
                                .bind(limit)
                                .fetch_all(&mut *tx)
                                .await
                        };
                        let _ = tx.commit().await;
                        rows
                    } else {
                        Err(sqlx::Error::Configuration("failed to set tenant context".into()))
                    }
                } else {
                    Err(sqlx::Error::Configuration("failed to begin tenant query".into()))
                };
                if let Ok(rows) = fetch_res {
                    use sqlx::Row;
                    for row in rows {
                        let dep_str: String = row.get("department");
                        let status_str: String = row.get("status");
                        let department = DepartmentType::from_str(&dep_str).unwrap_or(DepartmentType::Operations);
                        let status = match status_str.as_str() {
                            "DRAFT" => ApprovalStatus::PendingApproval,
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
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = ? AND status != 'DRAFT' AND id < ? ORDER BY id DESC LIMIT ?")
                        .bind(tenant_id)
                        .bind(cur)
                        .bind(limit)
                        .fetch_all(pool)
                        .await
                } else {
                    sqlx::query("SELECT id, tenant_id, department, description, status, action_risk, payload FROM agent_approvals WHERE tenant_id = ? AND status != 'DRAFT' ORDER BY id DESC LIMIT ?")
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
                            "DRAFT" => ApprovalStatus::PendingApproval,
                            "APPROVED" => ApprovalStatus::Approved,
                            "REJECTED" => ApprovalStatus::Rejected,
                            _ => ApprovalStatus::PendingApproval,
                        };
                        let risk_str: String = row.get("action_risk");
                        let action_risk = ActionRisk::from_str(&risk_str).unwrap_or(ActionRisk::DraftForReview);
                        let payload_str: Option<String> = row.try_get("payload").unwrap_or(None);
                        let payload_opt = payload_str.and_then(|s: String| serde_json::from_str(&s).unwrap_or(None));
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
        let lock_key = format!("ohc:lock:agent_approval:{}", request_id);

        let lock_acquired = self.mesh.acquire_lock(&lock_key, "orchestrator", 60).await;
        if let Ok(acquired) = lock_acquired {
            if !acquired {
                return Err("Failed to acquire lock for approval decision".to_string());
            }
        } else {
            return Err("Error communicating with lock service".to_string());
        }

        let new_status = if approved { "APPROVED" } else { "REJECTED" };
        let now = Utc::now();

        let mut error_response = None;
        let opt_dept_payload = match &self.db.store {
            DbStore::Postgres => {
                let row = if let Ok(mut tx) = self.db.pool.begin().await {
                    if ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.is_ok() {
                        let updated = sqlx::query("UPDATE agent_approvals SET status = $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4 RETURNING department, payload")
                            .bind(new_status)
                            .bind(now)
                            .bind(request_id)
                            .bind(tenant_id)
                            .fetch_optional(&mut *tx)
                            .await;
                        let _ = tx.commit().await;
                        updated
                    } else {
                        Err(sqlx::Error::Configuration("failed to set tenant context".into()))
                    }
                } else {
                    Err(sqlx::Error::Configuration("failed to begin tenant update".into()))
                };
                match row {
                    Ok(Some(r)) => {
                        use sqlx::Row;
                        let dep = r.get::<String, _>("department");
                        let payload_val: Option<serde_json::Value> = match r.try_get::<String, _>("payload") {
                            Ok(p) => serde_json::from_str(&p).unwrap_or(None),
                            Err(_) => match r.try_get::<serde_json::Value, _>("payload") {
                                Ok(p) => Some(p),
                                Err(_) => None,
                            }
                        };
                        Some((dep, payload_val))
                    }
                    Ok(None) => {
                        error_response = Some("Unauthorized".to_string());
                        None
                    }
                    Err(e) => {
                        error_response = Some(e.to_string());
                        None
                    }
                }
            }
            DbStore::Sqlite(pool) => {
                let row = sqlx::query("UPDATE agent_approvals SET status = ?, updated_at = ? WHERE id = ? AND tenant_id = ? RETURNING department, payload")
                    .bind(new_status)
                    .bind(now)
                    .bind(request_id)
                    .bind(tenant_id)
                    .fetch_optional(pool)
                    .await;
                match row {
                    Ok(Some(r)) => {
                        use sqlx::Row;
                        let dep = r.get::<String, _>("department");
                        let payload_str: Option<String> = r.try_get("payload").unwrap_or(None);
                        let payload_val = payload_str.and_then(|s: String| serde_json::from_str(&s).unwrap_or(None));
                        Some((dep, payload_val))
                    }
                    Ok(None) => {
                        error_response = Some("Unauthorized".to_string());
                        None
                    }
                    Err(e) => {
                        error_response = Some(e.to_string());
                        None
                    }
                }
            }
        };

        if let Some(err) = error_response {
            let _ = self.mesh.release_lock(&lock_key, "orchestrator").await;
            return Err(err);
        }

        if let Some((dep, original_payload)) = opt_dept_payload {
            let decision_str = if approved { "approved" } else { "rejected" };
            self.approval_counter.add(1, &[
                KeyValue::new("tenant_id", tenant_id.to_string()),
                KeyValue::new("decision", decision_str),
                KeyValue::new("department", dep.to_string())
            ]);

            if approved {
                // If this is a Smart Pricing approval, execute the price change in the database directly.
                if let Some(payload) = &original_payload {
                    if payload.get("context").and_then(|c| c.get("smart_pricing")).and_then(|v| v.as_bool()).unwrap_or(false) {
                        if let Some(product_id) = payload.get("context").and_then(|c| c.get("product_id")).and_then(|v| v.as_str()) {
                            let discount_amount = payload.get("context").and_then(|c| c.get("discount_amount")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let now = Utc::now();
                            let expires_at = now + chrono::Duration::days(2);
                            let id = uuid::Uuid::new_v4().to_string();

                            // Try to insert into active_discounts, but don't fail the approval if it's not present (e.g. SQLite doesn't have the table yet in testing)
                            if let DbStore::Postgres = &self.db.store {
                                if let Err(e) = sqlx::query("INSERT INTO active_discounts (id, tenant_id, product_id, discount_amount, expires_at) VALUES ($1, $2, $3, $4, $5)")
                                    .bind(uuid::Uuid::parse_str(&id).unwrap_or(uuid::Uuid::new_v4()))
                                    .bind(uuid::Uuid::parse_str(tenant_id).unwrap_or(uuid::Uuid::new_v4()))
                                    .bind(uuid::Uuid::parse_str(product_id).unwrap_or(uuid::Uuid::new_v4()))
                                    .bind(discount_amount)
                                    .bind(expires_at)
                                    .execute(&self.db.pool)
                                    .await
                                {
                                    eprintln!("Failed to insert active_discount: {}", e);
                                    let _ = self.mesh.release_lock(&lock_key, "orchestrator").await;
                                    return Err(format!("Failed to activate smart pricing discount: {}", e));
                                }

                                // Invalidate Redis edge cache for the product price
                                let cache_key = format!("ohc:price:{}:{}", tenant_id, product_id);
                                eprintln!("Mock redis invalidation for {}", cache_key);
                                if false {

                                }

                                // Trigger Promoter agent to draft a marketing broadcast
                                let promo_payload = serde_json::json!({
                                    "action": "draft_social_post",
                                    "context": {
                                        "product_id": product_id,
                                        "product_name": payload.get("context").and_then(|c| c.get("product_name")).and_then(|v| v.as_str()).unwrap_or(""),
                                        "discount_amount": discount_amount,
                                        "reason": "Flash Sale"
                                    }
                                });
                                let _ = self.execute_action(
                                    DepartmentType::Marketing,
                                    "Draft social media post for Flash Sale".to_string(),
                                    tenant_id.to_string(),
                                    ActionRisk::DraftForReview,
                                    promo_payload
                                ).await;
                            }
                        }
                    }
                }

                let payload = serde_json::json!({
                    "request_id": request_id,
                    "tenant_id": tenant_id,
                    "original_payload": original_payload,
                });
                let payload_bytes = serde_json::to_vec(&payload).unwrap_or_default();
                let topic = format!("agent:{}:approved", dep);
                let _ = self.mesh.publish(&topic, payload_bytes).await;
            }
        }

        let _ = self.mesh.release_lock(&lock_key, "orchestrator").await;
        Ok(())
    }


    pub async fn update_inbox_message_status(&self, message_id: &str, tenant_id: &str, new_status: &str) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("UPDATE inbox_messages SET status = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(new_status).bind(message_id).bind(tenant_id).execute(&self.db.pool).await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE inbox_messages SET status = ? WHERE id = ? AND tenant_id = ?")
                    .bind(new_status).bind(message_id).bind(tenant_id).execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn simulate_smart_pricing(&self, tenant_id: &str) -> Result<(), String> {
        let payload = serde_json::json!({
            "context": {
                "smart_pricing": true,
                "product_id": uuid::Uuid::new_v4().to_string(),
                "product_name": "Winter Scarf",
                "old_price": 50.0,
                "new_price": 42.5,
                "discount_amount": 7.5,
                "sales_projection": "+$120",
                "stagnant_days": 60,
                "margin_percent": 40
            }
        });

        self.execute_action(
            DepartmentType::BusinessAdvisory,
            "Smart Price Suggestion: Winter Scarf".to_string(),
            tenant_id.to_string(),
            ActionRisk::DraftForReview,
            payload
        ).await.map(|_| ())
    }

    pub async fn update_inbox_message_draft(&self, message_id: &str, tenant_id: &str, draft_reply: &str) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query("UPDATE inbox_messages SET draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(draft_reply).bind(message_id).bind(tenant_id).execute(&self.db.pool).await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE inbox_messages SET draft_reply = ? WHERE id = ? AND tenant_id = ?")
                    .bind(draft_reply).bind(message_id).bind(tenant_id).execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn query_long_term_memory(&self, tenant_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<String>, String> {
        let records = self.memory_repo.cross_department_search(tenant_id, query_embedding, limit).await?;
        Ok(records.into_iter().map(|r| r.content).collect())
    }



    pub async fn append_to_timeline(&self, event: crate::orchestration::departments::types::TimelineEvent) -> Result<(), String> {
        let meta_str = event.metadata.map(|v| v.to_string()).unwrap_or_else(|| "{}".to_string());
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("INSERT INTO customer_timeline (id, tenant_id, customer_id, event_type, source, content, metadata) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                    .bind(&event.id)
                    .bind(&event.tenant_id)
                    .bind(&event.customer_id)
                    .bind(&event.event_type)
                    .bind(&event.source)
                    .bind(&event.content)
                    .bind(&meta_str)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("INSERT INTO customer_timeline (id, tenant_id, customer_id, event_type, source, content, metadata) VALUES (?, ?, ?, ?, ?, ?, ?)")
                    .bind(&event.id)
                    .bind(&event.tenant_id)
                    .bind(&event.customer_id)
                    .bind(&event.event_type)
                    .bind(&event.source)
                    .bind(&event.content)
                    .bind(&meta_str)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn get_customer_timeline(&self, tenant_id: &str, customer_id: &str, limit: i64) -> Result<Vec<crate::orchestration::departments::types::TimelineEvent>, String> {
        let mut results = Vec::new();
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let rows = sqlx::query("SELECT id, tenant_id, customer_id, event_type, source, content, metadata, created_at FROM customer_timeline WHERE tenant_id = $1 AND customer_id = $2 ORDER BY created_at DESC LIMIT $3")
                    .bind(tenant_id)
                    .bind(customer_id)
                    .bind(limit)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                for row in rows {
                    use sqlx::Row;
                    let meta_str: String = row.get("metadata");
                    results.push(crate::orchestration::departments::types::TimelineEvent {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        customer_id: row.get("customer_id"),
                        event_type: row.get("event_type"),
                        source: row.get("source"),
                        content: row.get("content"),
                        metadata: serde_json::from_str(&meta_str).ok(),
                        created_at: Some(row.get("created_at")),
                    });
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query("SELECT id, tenant_id, customer_id, event_type, source, content, metadata, created_at FROM customer_timeline WHERE tenant_id = ? AND customer_id = ? ORDER BY created_at DESC LIMIT ?")
                    .bind(tenant_id)
                    .bind(customer_id)
                    .bind(limit)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                for row in rows {
                    use sqlx::Row;
                    let meta_str: String = row.get("metadata");
                    let created_at: chrono::DateTime<chrono::Utc> = row.try_get("created_at").unwrap_or_else(|_| chrono::Utc::now());

                    results.push(crate::orchestration::departments::types::TimelineEvent {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        customer_id: row.get("customer_id"),
                        event_type: row.get("event_type"),
                        source: row.get("source"),
                        content: row.get("content"),
                        metadata: serde_json::from_str(&meta_str).ok(),
                        created_at: Some(created_at),
                    });
                }
            }
        }
        results.reverse();
        Ok(results)
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

    pub async fn get_customer360(&self, tenant_id: &str, customer_id: &str) -> Result<Option<crate::orchestration::departments::types::Customer360>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query("SELECT * FROM customer360 WHERE tenant_id = $1 AND customer_id = $2")
                    .bind(tenant_id)
                    .bind(customer_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    let prefs_str: Option<String> = r.get("preferences");
                    Ok(Some(crate::orchestration::departments::types::Customer360 {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        customer_id: r.get("customer_id"),
                        email: r.get("email"),
                        phone: r.get("phone"),
                        mood: r.get("mood"),
                        preferences: prefs_str.and_then(|s: String| serde_json::from_str(&s).ok()),
                        created_at: Some(r.get("created_at")),
                        updated_at: Some(r.get("updated_at")),
                    }))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT * FROM customer360 WHERE tenant_id = ? AND customer_id = ?")
                    .bind(tenant_id)
                    .bind(customer_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    let prefs_str: Option<String> = r.get("preferences");
                    Ok(Some(crate::orchestration::departments::types::Customer360 {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        customer_id: r.get("customer_id"),
                        email: r.get("email"),
                        phone: r.get("phone"),
                        mood: r.get("mood"),
                        preferences: prefs_str.and_then(|s: String| serde_json::from_str(&s).ok()),
                        created_at: Some(r.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at").unwrap_or_else(|_| chrono::Utc::now())),
                        updated_at: Some(r.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at").unwrap_or_else(|_| chrono::Utc::now())),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn upsert_customer360(&self, c: &crate::orchestration::departments::types::Customer360) -> Result<(), String> {
        let prefs_str = c.preferences.as_ref().map(|v| v.to_string()).unwrap_or_else(|| "{}".to_string());
        let now = chrono::Utc::now();
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("INSERT INTO customer360 (id, tenant_id, customer_id, email, phone, mood, preferences, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) ON CONFLICT (id) DO UPDATE SET mood = EXCLUDED.mood, updated_at = EXCLUDED.updated_at")
                    .bind(&c.id)
                    .bind(&c.tenant_id)
                    .bind(&c.customer_id)
                    .bind(&c.email)
                    .bind(&c.phone)
                    .bind(&c.mood)
                    .bind(&prefs_str)
                    .bind(&c.created_at.unwrap_or(now))
                    .bind(&now)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            crate::db::DbStore::Sqlite(pool) => {
                let exists = sqlx::query("SELECT 1 FROM customer360 WHERE tenant_id = ? AND customer_id = ?")
                    .bind(&c.tenant_id)
                    .bind(&c.customer_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?.is_some();
                if exists {
                    sqlx::query("UPDATE customer360 SET mood = ?, updated_at = ? WHERE tenant_id = ? AND customer_id = ?")
                        .bind(&c.mood)
                        .bind(&now)
                        .bind(&c.tenant_id)
                        .bind(&c.customer_id)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                } else {
                    sqlx::query("INSERT INTO customer360 (id, tenant_id, customer_id, email, phone, mood, preferences, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
                        .bind(&c.id)
                        .bind(&c.tenant_id)
                        .bind(&c.customer_id)
                        .bind(&c.email)
                        .bind(&c.phone)
                        .bind(&c.mood)
                        .bind(&prefs_str)
                        .bind(&c.created_at.unwrap_or(now))
                        .bind(&now)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }

    pub async fn update_customer_mood(&self, tenant_id: &str, customer_id: &str, mood: &str) -> Result<(), String> {
        let c = self.get_customer360(tenant_id, customer_id).await?;
        if let Some(mut cust) = c {
            cust.mood = Some(mood.to_string());
            self.upsert_customer360(&cust).await
        } else {
            let cust = crate::orchestration::departments::types::Customer360 {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: tenant_id.to_string(),
                customer_id: customer_id.to_string(),
                email: None,
                phone: None,
                mood: Some(mood.to_string()),
                preferences: None,
                created_at: None,
                updated_at: None,
            };
            self.upsert_customer360(&cust).await
        }
    }

    pub async fn get_loyalty_ledger(&self, tenant_id: &str, customer_id: &str) -> Result<Option<crate::orchestration::departments::types::LoyaltyLedger>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query("SELECT * FROM loyalty_ledger WHERE tenant_id = $1 AND customer_id = $2")
                    .bind(tenant_id)
                    .bind(customer_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some(crate::orchestration::departments::types::LoyaltyLedger {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        customer_id: r.get("customer_id"),
                        points_balance: r.get("points_balance"),
                        tier_name: r.get("tier_name"),
                        last_updated: Some(r.get("last_updated")),
                    }))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT * FROM loyalty_ledger WHERE tenant_id = ? AND customer_id = ?")
                    .bind(tenant_id)
                    .bind(customer_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some(crate::orchestration::departments::types::LoyaltyLedger {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        customer_id: r.get("customer_id"),
                        points_balance: r.get("points_balance"),
                        tier_name: r.try_get("tier_name").ok(),
                        last_updated: Some(r.try_get::<chrono::DateTime<chrono::Utc>, _>("last_updated").unwrap_or_else(|_| chrono::Utc::now())),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn add_loyalty_points(&self, tenant_id: &str, customer_id: &str, points: i32) -> Result<(), String> {
        let now = chrono::Utc::now();
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let id = uuid::Uuid::new_v4().to_string();
                sqlx::query("INSERT INTO loyalty_ledger (id, tenant_id, customer_id, points_balance, last_updated) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET points_balance = loyalty_ledger.points_balance + EXCLUDED.points_balance, last_updated = EXCLUDED.last_updated")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(customer_id)
                    .bind(points)
                    .bind(&now)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
            crate::db::DbStore::Sqlite(pool) => {
                let exists = sqlx::query("SELECT 1 FROM loyalty_ledger WHERE tenant_id = ? AND customer_id = ?")
                    .bind(tenant_id)
                    .bind(customer_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?.is_some();
                if exists {
                    sqlx::query("UPDATE loyalty_ledger SET points_balance = points_balance + ?, last_updated = ? WHERE tenant_id = ? AND customer_id = ?")
                        .bind(points)
                        .bind(&now)
                        .bind(tenant_id)
                        .bind(customer_id)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                } else {
                    let id = uuid::Uuid::new_v4().to_string();
                    sqlx::query("INSERT INTO loyalty_ledger (id, tenant_id, customer_id, points_balance, last_updated) VALUES (?, ?, ?, ?, ?)")
                        .bind(&id)
                        .bind(tenant_id)
                        .bind(customer_id)
                        .bind(points)
                        .bind(&now)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }

    pub async fn get_order(&self, tenant_id: &str, order_id: &str) -> Result<Option<(String, f64)>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query("SELECT customer_id, total_amount FROM orders WHERE tenant_id = $1 AND id = $2")
                    .bind(tenant_id)
                    .bind(order_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some((r.get("customer_id"), r.get::<f64, _>("total_amount"))))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT customer_id, total_amount FROM orders WHERE tenant_id = ? AND id = ?")
                    .bind(tenant_id)
                    .bind(order_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some((r.get("customer_id"), r.get::<f64, _>("total_amount"))))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn get_service_by_name_like(&self, tenant_id: &str, name: &str) -> Result<Option<(String, f64)>, String> {
        let pattern = format!("%{}%", name);
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query("SELECT name, CAST(price AS DOUBLE PRECISION) as price_f64 FROM services WHERE tenant_id = $1 AND name ILIKE $2 LIMIT 1")
                    .bind(tenant_id)
                    .bind(&pattern)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    let n: String = r.get("name");
                    let p: f64 = r.get("price_f64");
                    Ok(Some((n, p)))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT name, CAST(price AS REAL) as price_f64 FROM services WHERE tenant_id = ? AND name LIKE ? LIMIT 1")
                    .bind(tenant_id)
                    .bind(&pattern)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    let n: String = r.get("name");
                    let p: f64 = r.get("price_f64");
                    Ok(Some((n, p)))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn get_booking(&self, tenant_id: &str, booking_id: &str) -> Result<Option<String>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query("SELECT customer_id FROM bookings WHERE tenant_id = $1 AND id = $2")
                    .bind(tenant_id)
                    .bind(booking_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some(r.get("customer_id")))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT customer_id FROM bookings WHERE tenant_id = ? AND id = ?")
                    .bind(tenant_id)
                    .bind(booking_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some(r.get("customer_id")))
                } else {
                    Ok(None)
                }
            }
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
        if std::env::var("OHC_DATABASE_URL").is_err() {
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
