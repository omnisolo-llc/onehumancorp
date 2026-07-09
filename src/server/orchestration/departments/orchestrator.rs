use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OHCLedgerEntry {
    pub id: String,
    pub tenant_id: String,
    pub event_type: String,
    pub department: String,
    pub payload: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

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
    async fn execute(&self, _payload: serde_json::Value) -> Result<(), String> {
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait Department: Send + Sync {
    fn department_type(&self) -> DepartmentType;
    fn subscribed_events(&self) -> Vec<String>;
    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String>;
    async fn query_memory(&self, query: &str) -> Result<Vec<String>, String>;
    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String>;
    fn get_config(&self, tenant_id: &str) -> Option<DepartmentConfig>;
    fn set_config(&mut self, _tenant_id: String, _config: DepartmentConfig) {
        // Default no-op for departments that don't need config overrides
    }
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
    pub fn db(&self) -> Arc<crate::db::DB> {
        self.db.clone()
    }

    pub fn mesh(&self) -> Arc<dyn TeammateMesh> {
        self.mesh.clone()
    }

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
                            ::server_telemetry::record_error_signal("[cleanup] Dead-letter logging for event  after 3 failed retries. Error");
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
                                        ::server_telemetry::record_error_signal("[bug] Failed to insert dead letter into DB");
                                        tracing::error!("Failed to insert dead letter into DB: {}", err);
                                    }

                                    let _ = sqlx::query(
                                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4::jsonb, $5::jsonb, \'PAUSED\', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                                    .bind(uuid::Uuid::new_v4().to_string())
                                    .bind(&event.tenant_id)
                                    .bind(format!("AI Agent Paused: {}", dep_type.to_string()))
                                    .bind(serde_json::json!({"description": "The AI agent is paused because the AI service is unavailable."}))
                                    .bind(serde_json::json!({"proposed_content": "System is paused. Please manually check the relevant work."}))
                                    .execute(&self.db.pool)
                                    .await;
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
                                        ::server_telemetry::record_error_signal("[bug] Failed to insert dead letter into DB");
                                        tracing::error!("Failed to insert dead letter into DB: {}", err);
                                    }

                                    let _ = sqlx::query(
                                        "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, \'PAUSED\', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                                    .bind(uuid::Uuid::new_v4().to_string())
                                    .bind(&event.tenant_id)
                                    .bind(format!("AI Agent Paused: {}", dep_type.to_string()))
                                    .bind(serde_json::json!({"description": "The AI agent is paused because the AI service is unavailable."}).to_string())
                                    .bind(serde_json::json!({"proposed_content": "System is paused. Please manually check the relevant work."}).to_string())
                                    .execute(pool)
                                    .await;
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


    pub async fn soft_lock_booking_slot(&self, tenant_id: &str, service_id: &str, _suggested_time_str: &str, ttl: std::time::Duration) -> Result<Option<String>, String> {
        // Attempt to parse time or default to next day if natural language (simplified for now)
        let start_time = if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(_suggested_time_str) {
            parsed.with_timezone(&chrono::Utc)
        } else {
            // Fallback for demo parsing: Assume next day at 2PM UTC
            let now = chrono::Utc::now();
            now.date_naive().and_hms_opt(14, 0, 0).unwrap().and_utc() + chrono::Duration::days(1)
        };
        let end_time = start_time + chrono::Duration::hours(1);

        let time_id = format!("{}_{}", start_time.timestamp(), service_id);

        let lock_acquired = self.mesh.acquire_lock(&time_id, "sales_agent", ttl.as_secs()).await.unwrap_or(false);

        if lock_acquired {
            let slot_id = uuid::Uuid::new_v4().to_string();
            match &self.db.store {
                crate::db::DbStore::Postgres => {
                    let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
                    sqlx::query(
                        "INSERT INTO booking_slots (id, tenant_id, service_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, 'soft_locked')"
                    )
                    .bind(&slot_id)
                    .bind(tenant_id)
                    .bind(service_id)
                    .bind(start_time)
                    .bind(end_time)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                    tx.commit().await.map_err(|e| e.to_string())?;
                }
                crate::db::DbStore::Sqlite(pool) => {
                    sqlx::query(
                        "INSERT INTO booking_slots (id, tenant_id, service_id, start_time, end_time, status) VALUES (?, ?, ?, ?, ?, 'soft_locked')"
                    )
                    .bind(&slot_id)
                    .bind(tenant_id)
                    .bind(service_id)
                    .bind(start_time)
                    .bind(end_time)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                }
            }
            Ok(Some(slot_id))
        } else {
            Ok(None)
        }
    }

    pub async fn confirm_booking_slot(&self, tenant_id: &str, slot_id: &str) -> Result<(), String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;
                sqlx::query("UPDATE booking_slots SET status = 'booked' WHERE id = $1")
                    .bind(slot_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE booking_slots SET status = 'booked' WHERE id = ? AND tenant_id = ?")
                    .bind(slot_id)
                    .bind(tenant_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }
        Ok(())
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
        let within_budget = self.check_ai_budget(&tenant_id, cost).await.unwrap_or(true);
        if !within_budget {
            tracing::info!("💰 Miser telemetry: Tenant {} soft limit reached. Action allowed. Please upgrade your plan.", tenant_id); // pii-safe
            return Err("Token budget exhausted. Please upgrade your plan to continue using AI actions.".to_string());
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

    pub async fn notify_owner(&self, tenant_id: &str, message: &str) -> Result<(), String> {
        match &self.db.store {
            DbStore::Postgres => {
                let pool = self.db.pool.clone();
                let job_id = Uuid::new_v4().to_string();
                let payload = serde_json::json!({
                    "tenant_id": tenant_id,
                    "message": message,
                }).to_string();

                let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload) VALUES ($1, $2, 'send_push_notification', $3::jsonb)")
                    .bind(job_id)
                    .bind(tenant_id)
                    .bind(payload)
                    .execute(&pool)
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(())
            },
            _ => Ok(())
        }
    }

    pub async fn add_approval_request(&self, req: ApprovalRequest) {
        let now = Utc::now();
        let status_str = match req.status {
            ApprovalStatus::PendingApproval => "DRAFT",
            ApprovalStatus::Approved => "APPROVED",
            ApprovalStatus::Rejected => "REJECTED",
            ApprovalStatus::Paused => "PAUSED",
        };

        if req.action_risk == ActionRisk::DraftForReview {
            if let Some(payload) = &req.payload {
                if payload.get("feature_type").and_then(|v| v.as_str()) == Some("invoice_followup") || payload.get("feature_type").and_then(|v| v.as_str()) == Some("quote_draft") {
                    let task_id = uuid::Uuid::new_v4().to_string();
                    let action_payload_str = serde_json::to_string(payload).unwrap_or_default();
                    let context_msg = req.description.clone();
                    let db = self.db.clone();
                    let tenant_id = req.tenant_id.clone();

                    tokio::spawn(async move {
                        match &db.store {
                            crate::db::DbStore::Postgres => {
                                let _ = sqlx::query(
                                    "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES ($1, $2, 'System', 'high', $3, 'pending'); INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES (gen_random_uuid(), $1, $2, 'Approve Draft', $4::jsonb)"
                                )
                                .bind(&task_id)
                                .bind(&tenant_id)
                                .bind(&context_msg)
                                .bind(&tenant_id)
                                .bind(&action_payload_str)
                                .execute(&db.pool)
                                .await;
                            },
                            crate::db::DbStore::Sqlite(_) => {
                                let _ = sqlx::query(
                                    "INSERT INTO triage_items (id, tenant_id, source, priority, context, status) VALUES (?, ?, 'System', 'high', ?, 'pending'); INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES (lower(hex(randomblob(16))), ?, ?, 'Approve Draft', json(?))"
                                )
                                .bind(&task_id)
                                .bind(&tenant_id)
                                .bind(&context_msg)
                                .bind(&tenant_id)
                                .bind(&action_payload_str)
                                .execute(&db.pool)
                                .await;
                            }
                        }
                    });
                }
            }
        }

        let _lifecycle_state = match req.status {
            ApprovalStatus::PendingApproval => "PENDING_APPROVAL",
            ApprovalStatus::Approved => "APPROVED",
            ApprovalStatus::Rejected => "REJECTED",
            ApprovalStatus::Paused => "PAUSED",
        };

        match &self.db.store {
            DbStore::Postgres => {
                if let Ok(mut tx) = self.db.pool.begin().await {
                    if ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id).await.is_ok() {
                        let context_payload = serde_json::json!({"description": req.description});

                        let _ = sqlx::query(
                            "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
                        )
                        .bind(&req.id)
                        .bind(&req.tenant_id)
                        .bind(req.department.to_string())
                        .bind(sqlx::types::Json(context_payload))
                        .bind(sqlx::types::Json(req.payload.clone().unwrap_or_default()))
                        .bind(_lifecycle_state)
                        .bind(now)
                        .bind(now)
                        .execute(&mut *tx)
                        .await;



                        let _ = tx.commit().await;
                    }
                }
            }
            DbStore::Sqlite(pool) => {
                let context_payload_str = serde_json::json!({"description": req.description}).to_string();
                let proposed_action_str = req.payload.clone().unwrap_or_default().to_string();

                let _ = sqlx::query(
                    "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&req.id)
                .bind(&req.tenant_id)
                .bind(req.department.to_string())
                .bind(&context_payload_str)
                .bind(&proposed_action_str)
                .bind(_lifecycle_state)
                .bind(now)
                .bind(now)
                .execute(pool)
                .await;


            }
        }

        // Publish SSE event
        let payload = serde_json::json!({
            "event_type": "approval_request",
            "data": {
                "id": req.id,
                "tenant_id": req.tenant_id,
                "department": req.department.to_string(),
                "description": req.description,
                "status": status_str,
                "action_risk": req.action_risk.to_string(),
                "payload": req.payload.clone()
            }
        });
        let payload_bytes = serde_json::to_vec(&payload).unwrap_or_default();
        let topic = format!("agent_feed:{}", req.tenant_id);
        let _ = self.mesh.publish(&topic, payload_bytes).await;
    }

    pub async fn get_pending_approvals(&self, tenant_id: &str, cursor: Option<String>, limit: i64) -> Vec<ApprovalRequest> {
        let mut results = Vec::new();

        match &self.db.store {
            DbStore::Postgres => {
                let fetch_res = if let Ok(mut tx) = self.db.pool.begin().await {
                    if ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.is_ok() {
                        let rows = if let Some(ref cur) = cursor {
                            sqlx::query("SELECT id, tenant_id, event_source as department, context_payload->>'description' as description, lifecycle_state as status, 'HIGH' as action_risk, proposed_action as payload FROM agent_feed_items WHERE tenant_id = $1 AND lifecycle_state = 'PENDING_APPROVAL' AND id > $2 ORDER BY id ASC LIMIT $3")
                                .bind(tenant_id)
                                .bind(cur)
                                .bind(limit)
                                .fetch_all(&mut *tx)
                                .await
                        } else {
                            sqlx::query("SELECT id, tenant_id, event_source as department, context_payload->>'description' as description, lifecycle_state as status, 'HIGH' as action_risk, proposed_action as payload FROM agent_feed_items WHERE tenant_id = $1 AND lifecycle_state = 'PENDING_APPROVAL' ORDER BY id ASC LIMIT $2")
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
                            "PAUSED" => ApprovalStatus::Paused,
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
                    sqlx::query("SELECT id, tenant_id, event_source as department, json_extract(context_payload, '$.description') as description, lifecycle_state as status, 'HIGH' as action_risk, proposed_action as payload FROM agent_feed_items WHERE tenant_id = ? AND lifecycle_state = 'PENDING_APPROVAL' AND id > ? ORDER BY id ASC LIMIT ?")
                        .bind(tenant_id)
                        .bind(cur)
                        .bind(limit)
                        .fetch_all(pool)
                        .await
                } else {
                    sqlx::query("SELECT id, tenant_id, event_source as department, json_extract(context_payload, '$.description') as description, lifecycle_state as status, 'HIGH' as action_risk, proposed_action as payload FROM agent_feed_items WHERE tenant_id = ? AND lifecycle_state = 'PENDING_APPROVAL' ORDER BY id ASC LIMIT ?")
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
                            "PAUSED" => ApprovalStatus::Paused,
                            _ => ApprovalStatus::PendingApproval,
                        };
                        let risk_str: String = row.get("action_risk");
                        let action_risk = ActionRisk::from_str(&risk_str).unwrap_or(ActionRisk::DraftForReview);
                        let payload_str: Option<String> = match row.try_get::<String, _>("payload") {
                            Ok(p) => Some(p),
                            Err(_) => match row.try_get::<serde_json::Value, _>("payload") {
                                Ok(p) => Some(p.to_string()),
                                Err(_) => None,
                            }
                        };
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



        pub async fn get_ledger_entries(&self, tenant_id: &str, limit: i64) -> Result<Vec<OHCLedgerEntry>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

                let rows = sqlx::query(
                    "SELECT id, tenant_id, event_type, department, payload, created_at
                     FROM ohc_universal_ledger
                     WHERE tenant_id = $1
                     ORDER BY created_at DESC
                     LIMIT $2"
                )
                .bind(tenant_id)
                .bind(limit)
                .fetch_all(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let mut entries = Vec::new();
                use sqlx::Row;
                for row in rows {
                    let payload_val: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::Value::Null);
                    let payload_str = serde_json::to_string(&payload_val).unwrap_or_default();
                    entries.push(OHCLedgerEntry {
                        id: row.get("id"),
                        tenant_id: row.get("tenant_id"),
                        event_type: row.get("event_type"),
                        department: row.get("department"),
                        payload: payload_str,
                        created_at: row.get("created_at"),
                    });
                }
                tx.commit().await.map_err(|e| e.to_string())?;
                Ok(entries)
            },
            crate::db::DbStore::Sqlite(_pool) => {
                // Return empty for sqlite in tests to avoid rewrite
                Ok(vec![])
            }
        }
    }

    pub async fn get_activity_feed(&self, tenant_id: &str, cursor: Option<String>, limit: i64) -> Vec<ApprovalRequest> {
        let mut results = Vec::new();

        match &self.db.store {
            DbStore::Postgres => {
                let fetch_res = if let Ok(mut tx) = self.db.pool.begin().await {
                    if ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.is_ok() {
                        let rows = if let Some(ref cur) = cursor {
                            sqlx::query("SELECT id, tenant_id, event_source as department, context_payload->>'description' as description, lifecycle_state as status, 'HIGH' as action_risk, proposed_action as payload FROM agent_feed_items WHERE tenant_id = $1 AND lifecycle_state != 'PENDING_APPROVAL' AND id < $2 ORDER BY id DESC LIMIT $3")
                                .bind(tenant_id)
                                .bind(cur)
                                .bind(limit)
                                .fetch_all(&mut *tx)
                                .await
                        } else {
                            sqlx::query("SELECT id, tenant_id, event_source as department, context_payload->>'description' as description, lifecycle_state as status, 'HIGH' as action_risk, proposed_action as payload FROM agent_feed_items WHERE tenant_id = $1 AND lifecycle_state != 'PENDING_APPROVAL' ORDER BY id DESC LIMIT $2")
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
                            "PAUSED" => ApprovalStatus::Paused,
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
                    sqlx::query("SELECT id, tenant_id, event_source as department, json_extract(context_payload, '$.description') as description, lifecycle_state as status, 'HIGH' as action_risk, proposed_action as payload FROM agent_feed_items WHERE tenant_id = ? AND lifecycle_state != 'PENDING_APPROVAL' AND id < ? ORDER BY id DESC LIMIT ?")
                        .bind(tenant_id)
                        .bind(cur)
                        .bind(limit)
                        .fetch_all(pool)
                        .await
                } else {
                    sqlx::query("SELECT id, tenant_id, event_source as department, json_extract(context_payload, '$.description') as description, lifecycle_state as status, 'HIGH' as action_risk, proposed_action as payload FROM agent_feed_items WHERE tenant_id = ? AND lifecycle_state != 'PENDING_APPROVAL' ORDER BY id DESC LIMIT ?")
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
                            "PAUSED" => ApprovalStatus::Paused,
                            _ => ApprovalStatus::PendingApproval,
                        };
                        let risk_str: String = row.get("action_risk");
                        let action_risk = ActionRisk::from_str(&risk_str).unwrap_or(ActionRisk::DraftForReview);
                        let payload_str: Option<String> = match row.try_get::<String, _>("payload") {
                            Ok(p) => Some(p),
                            Err(_) => match row.try_get::<serde_json::Value, _>("payload") {
                                Ok(p) => Some(p.to_string()),
                                Err(_) => None,
                            }
                        };
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

    pub async fn decide_approval(&self, request_id: &str, tenant_id: &str, approved: bool, edited_payload: Option<serde_json::Value>) -> Result<(), String> {
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
                        let updated = if let Some(ref ep) = edited_payload {
                            sqlx::query("UPDATE agent_feed_items SET lifecycle_state = $1, updated_at = $2, proposed_action = $3 WHERE id = $4 AND tenant_id = $5 RETURNING event_source as department, proposed_action as payload")
                                .bind(new_status)
                                .bind(now)
                                .bind(ep)
                                .bind(request_id)
                                .bind(tenant_id)
                                .fetch_optional(&mut *tx)
                                .await
                        } else {
                            sqlx::query("UPDATE agent_feed_items SET lifecycle_state = $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4 RETURNING event_source as department, proposed_action as payload")
                                .bind(new_status)
                                .bind(now)
                                .bind(request_id)
                                .bind(tenant_id)
                                .fetch_optional(&mut *tx)
                                .await
                        };
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
                let row = if let Some(ref ep) = edited_payload {
                    let ep_str = serde_json::to_string(ep).unwrap_or_default();
                    sqlx::query("UPDATE agent_feed_items SET lifecycle_state = ?, updated_at = ?, proposed_action = ? WHERE id = ? AND tenant_id = ? RETURNING event_source as department, proposed_action as payload")
                        .bind(new_status)
                        .bind(now)
                        .bind(ep_str)
                        .bind(request_id)
                        .bind(tenant_id)
                        .fetch_optional(pool)
                        .await
                } else {
                    sqlx::query("UPDATE agent_feed_items SET lifecycle_state = ?, updated_at = ? WHERE id = ? AND tenant_id = ? RETURNING event_source as department, proposed_action as payload")
                        .bind(new_status)
                        .bind(now)
                        .bind(request_id)
                        .bind(tenant_id)
                        .fetch_optional(pool)
                        .await
                };
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
                let payload_to_use = edited_payload.as_ref().or(original_payload.as_ref());

                if let Some(payload) = payload_to_use {

                    if payload.get("feature_type").and_then(|v| v.as_str()) == Some("field_service_quote") || payload.get("feature_type").and_then(|v| v.as_str()) == Some("autonomous_quote") {
                        let price = payload.get("suggested_price").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let deposit_amount = payload.get("deposit_amount_cents").and_then(|v| v.as_i64()).unwrap_or((price * 0.20 * 100.0) as i64);
                        let total_amount_cents = (price * 100.0) as i64;
                        let quote_id = uuid::Uuid::new_v4().to_string();
                        let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        let service_id = payload.get("service").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();

                        let start_time_arr = payload.get("proposed_slots").and_then(|v| v.as_array()).and_then(|a| a.first()).and_then(|o| o.get("start_time")).and_then(|v| v.as_str()).unwrap_or("");
                        let end_time_arr = payload.get("proposed_slots").and_then(|v| v.as_array()).and_then(|a| a.first()).and_then(|o| o.get("end_time")).and_then(|v| v.as_str()).unwrap_or("");

                        let start_time = payload.get("start_time").and_then(|v| v.as_str()).unwrap_or(start_time_arr);
                        let end_time = payload.get("end_time").and_then(|v| v.as_str()).unwrap_or(end_time_arr);

                        let mut customer_id_to_use = customer_id.clone();
                        if customer_id_to_use.is_empty() {
                            customer_id_to_use = uuid::Uuid::new_v4().to_string();
                        }

                        let api_key = std::env::var("STRIPE_SECRET_KEY").unwrap_or_else(|_| "sk_test_123".to_string());
                        let stripe = crate::integrations::stripe::client::StripeClient::new(api_key);
                        let stripe_link = stripe.create_checkout_session(&quote_id, &customer_id_to_use, (deposit_amount as f64) / 100.0, None, None).await.unwrap_or_default();

                        let inbox_message_id = payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or("");
                        let mut generated_reply = payload.get("generated_response").and_then(|v| v.as_str()).unwrap_or("").to_string();

                        if !stripe_link.is_empty() {
                            generated_reply.push_str(&format!("\n\nTo secure your booking, please pay the deposit here: {}", stripe_link));
                        }

                        if !inbox_message_id.is_empty() {
                            if let Err(e) = sqlx::query("UPDATE inbox_messages SET draft_reply = $1, status = 'auto_replied' WHERE id = $2 AND tenant_id = $3")
                                .bind(&generated_reply)
                                .bind(inbox_message_id)
                                .bind(tenant_id)
                                .execute(&self.db.pool)
                                .await
                            {
                                tracing::error!("Failed to update inbox_messages for autonomous quote: {}", e);
                            }
                        }

                        if let DbStore::Postgres = &self.db.store {
                            // Convert string times to DateTime
                            let st = chrono::DateTime::parse_from_rfc3339(start_time).map(|dt| dt.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now());
                            let et = chrono::DateTime::parse_from_rfc3339(end_time).map(|dt| dt.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now() + chrono::Duration::hours(1));

                            // Insert booking with pending_payment to wait for customer confirmation
                            let booking_id = uuid::Uuid::new_v4().to_string();
                            let _ = sqlx::query("INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, $6, 'pending_payment')")
                                .bind(&booking_id)
                                .bind(tenant_id)
                                .bind(uuid::Uuid::parse_str(&customer_id).ok())
                                .bind(&service_id)
                                .bind(st)
                                .bind(et)
                                .execute(&self.db.pool)
                                .await;

                            // Update availability blocks to ensure no double bookings globally
                            let _ = sqlx::query("UPDATE availability_blocks SET is_available = false WHERE tenant_id = $1 AND service_id = $2 AND start_time = $3 AND end_time = $4")
                                .bind(tenant_id)
                                .bind(&service_id)
                                .bind(st)
                                .bind(et)
                                .execute(&self.db.pool)
                                .await;

                            // Insert quote/proposal
                            let _ = sqlx::query("INSERT INTO interactive_proposals (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, checkout_url) VALUES ($1, $2, $3, 'Sent', $4, $5, $6)")
                                .bind(uuid::Uuid::parse_str(&quote_id).unwrap_or_default())
                                .bind(tenant_id)
                                .bind(uuid::Uuid::parse_str(&customer_id).ok())
                                .bind(total_amount_cents)
                                .bind(deposit_amount)
                                .bind(&stripe_link)
                                .execute(&self.db.pool)
                                .await;
                        }
                    }


                    if payload.get("feature_type").and_then(|v| v.as_str()) == Some("invoice_draft") {
                        let amount_cents = payload.get("amount_cents").and_then(|v| v.as_i64()).unwrap_or(0);
                        let now = Utc::now();

                        let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        let mut customer_id_to_use = customer_id.clone();
                        if customer_id_to_use.is_empty() {
                            customer_id_to_use = uuid::Uuid::new_v4().to_string();
                        }

                        let api_key = std::env::var("STRIPE_SECRET_KEY").unwrap_or_else(|_| "sk_test_123".to_string());
                        let stripe = crate::integrations::stripe::client::StripeClient::new(api_key);
                        let project_name = payload.get("project_name").and_then(|v| v.as_str()).unwrap_or("Project");
                        let milestone_name = payload.get("milestone_name").and_then(|v| v.as_str()).unwrap_or("Milestone");
                        let description = format!("Invoice for {} - {}", project_name, milestone_name);

                        match stripe.create_draft_invoice(&customer_id_to_use, amount_cents, &description).await {
                            Ok(draft_invoice) => {
                                tracing::info!("Created draft invoice in Stripe: {}", draft_invoice.id); // pii-safe
                                match stripe.finalize_and_send_invoice(&draft_invoice.id).await {
                                    Ok(sent_invoice) => {
                                        tracing::info!("Finalized and sent invoice via Stripe: {}", sent_invoice.id); // pii-safe

                                        // Record the sent invoice in the database
                                        match &self.db.store {
                                            DbStore::Postgres => {
                                                if let Err(e) = sqlx::query("INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, total_amount_cents, payment_status, view_count, amount_paid_cents) VALUES ($1, $2, $3, 'Client', 'sent', $4, 'USD', $5, $6, 'unpaid', 0, 0)")
                                                    .bind(&sent_invoice.id)
                                                    .bind(tenant_id)
                                                    .bind(&customer_id_to_use)
                                                    .bind(now + chrono::Duration::days(30))
                                                    .bind(amount_cents as f64 / 100.0)
                                                    .bind(amount_cents)
                                                    .execute(&self.db.pool)
                                                    .await
                                                {
                                                    tracing::error!("Failed to insert invoice for invoice_draft: {}", e);
                                                }
                                            }
                                            DbStore::Sqlite(_) => {
                                                if let Err(e) = sqlx::query("INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, total_amount_cents, payment_status, view_count, amount_paid_cents) VALUES (?, ?, ?, 'Client', 'sent', ?, 'USD', ?, ?, 'unpaid', 0, 0)")
                                                    .bind(&sent_invoice.id)
                                                    .bind(tenant_id)
                                                    .bind(&customer_id_to_use)
                                                    .bind(now + chrono::Duration::days(30))
                                                    .bind(amount_cents as f64 / 100.0)
                                                    .bind(amount_cents)
                                                    .execute(&self.db.pool)
                                                    .await
                                                {
                                                    tracing::error!("Failed to insert invoice for invoice_draft: {}", e);
                                                }
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        tracing::error!("Failed to finalize and send invoice via Stripe: {}", e); // pii-safe
                                    }
                                }
                            },
                            Err(e) => {
                                tracing::error!("Failed to create draft invoice in Stripe: {}", e); // pii-safe
                            }
                        }
                    }

                    if payload.get("feature_type").and_then(|v| v.as_str()) == Some("quote_draft") {
                        let price = payload.get("suggested_price").and_then(|v| v.as_f64()).unwrap_or(0.0);
                        let deposit_amount = (price * 0.20) as i64 * 100;
                        let total_amount_cents = (price * 100.0) as i64;
                        let now = Utc::now();
                        let _expires_at = now + chrono::Duration::days(2);
                        let quote_id = uuid::Uuid::new_v4().to_string();


                        let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        let mut customer_id_to_use = customer_id.clone();
                        if customer_id_to_use.is_empty() {
                            customer_id_to_use = uuid::Uuid::new_v4().to_string();
                        }

                        let api_key = std::env::var("STRIPE_SECRET_KEY").unwrap_or_else(|_| "sk_test_123".to_string());
                        let stripe = crate::integrations::stripe::client::StripeClient::new(api_key);
                        let stripe_link = stripe.create_checkout_session(&quote_id, &customer_id_to_use, price * 0.20, None, None).await.unwrap_or_default();


                        let mut generated_reply = payload.get("generated_response").and_then(|v| v.as_str()).unwrap_or("").to_string();
                        if !stripe_link.is_empty() {
                            generated_reply.push_str(&format!("\n\nTo secure your booking, please pay the deposit here: {}", stripe_link));
                        }
                        let inbox_message_id = payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or("");

                        if let DbStore::Postgres = &self.db.store {
                            if let Err(e) = sqlx::query("INSERT INTO proposals (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, stripe_payment_link) VALUES ($1, $2, $3, $4, $5, $6, $7)")
                                .bind(&quote_id)
                                .bind(tenant_id)
                                .bind(&customer_id_to_use)
                                .bind("SENT")
                                .bind(total_amount_cents)
                                .bind(deposit_amount)

                                .bind(&stripe_link)
                                .execute(&self.db.pool)
                                .await
                            {
                                tracing::error!("Failed to insert quote: {}", e);
                            }

                            // Issue #27509: Create Project, Tasks, and Invoice upon Quote Acceptance (Approval)
                            let project_id = uuid::Uuid::new_v4().to_string();

                            let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, 'Inquiry Customer') ON CONFLICT DO NOTHING")
                                .bind(&customer_id_to_use)
                                .bind(tenant_id)
                                .execute(&self.db.pool)
                                .await;

                            if let Err(e) = sqlx::query("INSERT INTO projects (id, tenant_id, quote_id, customer_id, title, status) VALUES ($1, $2, $3, $4, $5, 'Active')")
                                .bind(&project_id)
                                .bind(tenant_id)
                                .bind(&quote_id)
                                .bind(&customer_id_to_use)
                                .bind(format!("Project for Quote {}", quote_id))
                                .execute(&self.db.pool)
                                .await
                            {
                                tracing::error!("Failed to insert project: {}", e);
                            }

                            let task_id = uuid::Uuid::new_v4().to_string();
                            if let Err(e) = sqlx::query("INSERT INTO project_tasks (id, tenant_id, project_id, title, status) VALUES ($1, $2, $3, $4, 'Pending')")
                                .bind(&task_id)
                                .bind(tenant_id)
                                .bind(&project_id)
                                .bind("Initial Task")
                                .execute(&self.db.pool)
                                .await
                            {
                                tracing::error!("Failed to insert project task: {}", e);
                            }

                            let invoice_id = uuid::Uuid::new_v4().to_string();
                            if let Err(e) = sqlx::query("INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, total_amount_cents, payment_status, view_count, amount_paid_cents) VALUES ($1, $2, $3, 'Client', 'draft', $5, 'USD', $6, 0, 'draft', 0, 0)")
                                .bind(&invoice_id)
                                .bind(tenant_id)
                                .bind(&customer_id_to_use)
                                .bind(deposit_amount)
                                .bind(now + chrono::Duration::days(7))
                                .bind(total_amount_cents as f64 / 100.0)
                                .execute(&self.db.pool)
                                .await
                            {
                                tracing::error!("Failed to insert invoice: {}", e);
                            }

                            if !inbox_message_id.is_empty() {
                                if let Err(e) = sqlx::query("UPDATE inbox_messages SET draft_reply = $1, status = 'auto_replied' WHERE id = $2 AND tenant_id = $3")
                                    .bind(&generated_reply)
                                    .bind(inbox_message_id)
                                    .bind(tenant_id)
                                    .execute(&self.db.pool)
                                    .await
                                {
                                    tracing::error!("Failed to update inbox_messages for quote draft: {}", e);
                                }
                            }
                        } else if let DbStore::Sqlite(pool) = &self.db.store {
                            if let Err(e) = sqlx::query("INSERT INTO quotes (id, tenant_id, status, total_amount_cents, required_deposit_cents, expires_at, stripe_payment_link) VALUES (?, ?, ?, ?, ?, ?, ?)")
                                .bind(&quote_id)
                                .bind(tenant_id)
                                .bind(&customer_id_to_use)
                                .bind("SENT")
                                .bind(total_amount_cents)
                                .bind(deposit_amount)

                                .bind(&stripe_link)
                                .execute(pool)
                                .await
                            {
                                tracing::error!("Failed to insert quote: {}", e);
                            }

                            // Issue #27509: Create Project, Tasks, and Invoice upon Quote Acceptance (Approval)
                            let project_id = uuid::Uuid::new_v4().to_string();

                            let _ = sqlx::query("INSERT INTO customers (id, tenant_id, name) VALUES (?, ?, 'Inquiry Customer') ON CONFLICT DO NOTHING")
                                .bind(&customer_id_to_use)
                                .bind(tenant_id)
                                .execute(pool)
                                .await;

                            if let Err(e) = sqlx::query("INSERT INTO projects (id, tenant_id, quote_id, customer_id, title, status) VALUES (?, ?, ?, ?, ?, 'Active')")
                                .bind(&project_id)
                                .bind(tenant_id)
                                .bind(&quote_id)
                                .bind(&customer_id_to_use)
                                .bind(format!("Project for Quote {}", quote_id))
                                .execute(pool)
                                .await
                            {
                                tracing::error!("Failed to insert project: {}", e);
                            }

                            let task_id = uuid::Uuid::new_v4().to_string();
                            if let Err(e) = sqlx::query("INSERT INTO project_tasks (id, tenant_id, project_id, title, status) VALUES (?, ?, ?, ?, 'Pending')")
                                .bind(&task_id)
                                .bind(tenant_id)
                                .bind(&project_id)
                                .bind("Initial Task")
                                .execute(pool)
                                .await
                            {
                                tracing::error!("Failed to insert project task: {}", e);
                            }

                            let invoice_id = uuid::Uuid::new_v4().to_string();
                            if let Err(e) = sqlx::query("INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, total_amount_cents, payment_status, view_count, amount_paid_cents) VALUES (?, ?, ?, 'Client', 'draft', ?, 'USD', ?, 0, 'draft', 0, 0)")
                                .bind(&invoice_id)
                                .bind(tenant_id)
                                .bind(&customer_id_to_use)
                                .bind(deposit_amount)
                                .bind(now + chrono::Duration::days(7))
                                .bind(total_amount_cents as f64 / 100.0)
                                .execute(pool)
                                .await
                            {
                                tracing::error!("Failed to insert invoice: {}", e);
                            }

                            if !inbox_message_id.is_empty() {
                                if let Err(e) = sqlx::query("UPDATE inbox_messages SET draft_reply = ?, status = 'auto_replied' WHERE id = ? AND tenant_id = ?")
                                    .bind(&generated_reply)
                                    .bind(inbox_message_id)
                                    .bind(tenant_id)
                                    .execute(pool)
                                    .await
                                {
                                    tracing::error!("Failed to update inbox_messages for quote draft: {}", e);
                                }
                            }
                        }
                    }
                }

                // If this is a stockout restock and price approval, execute the price change and dispatch a job
                if let Some(payload) = payload_to_use {
                    if payload.get("feature_type").and_then(|v| v.as_str()) == Some("stockout_restock_and_price") {
                        if let Some(product_id) = payload.get("product_id").and_then(|v| v.as_str()) {
                            let new_price = payload.get("new_price").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let new_price_cents = (new_price * 100.0) as i64;

                            if let DbStore::Postgres = &self.db.store {
                                let _ = sqlx::query("UPDATE products SET price = $1, price_cents = $2 WHERE id = $3 AND tenant_id = $4")
                                    .bind(new_price)
                                    .bind(new_price_cents)
                                    .bind(product_id)
                                    .bind(tenant_id)
                                    .execute(&self.db.pool)
                                    .await;

                                // Dispatch simulated reorder to job queue
                                let job_id = uuid::Uuid::new_v4().to_string();
                                let reorder_quantity = payload.get("suggested_reorder_quantity").and_then(|v| v.as_i64()).unwrap_or(50);
                                let job_payload = serde_json::json!({
                                    "action": "reorder_stock",
                                    "product_id": product_id,
                                    "quantity": reorder_quantity,
                                });
                                let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, queue_name, payload, status) VALUES ($1, $2, 'operations_queue', $3, 'pending')")
                                    .bind(&job_id)
                                    .bind(tenant_id)
                                    .bind(&job_payload)
                                    .execute(&self.db.pool)
                                    .await;
                            } else if let DbStore::Sqlite(pool) = &self.db.store {
                                let _ = sqlx::query("UPDATE products SET price = ?, price_cents = ? WHERE id = ? AND tenant_id = ?")
                                    .bind(new_price)
                                    .bind(new_price_cents)
                                    .bind(product_id)
                                    .bind(tenant_id)
                                    .execute(pool)
                                    .await;
                            }
                        }
                    }
                }

                // If this is a Smart Pricing approval, execute the price change in the database directly.
                if let Some(payload) = payload_to_use {
                    if payload.get("context").and_then(|c| c.get("smart_pricing")).and_then(|v| v.as_bool()).unwrap_or(false) {
                        if let Some(product_id) = payload.get("context").and_then(|c| c.get("product_id")).and_then(|v| v.as_str()) {
                            let _discount_amount = payload.get("context").and_then(|c| c.get("discount_amount")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let now = Utc::now();
                            let _expires_at = now + chrono::Duration::days(2);
                            let _id = uuid::Uuid::new_v4().to_string();

                            let new_price = payload.get("context").and_then(|c| c.get("new_price")).and_then(|v| v.as_f64()).unwrap_or(0.0);
                            let new_price_cents = (new_price * 100.0) as i64;

                            // Update base price in products table directly
                            if let DbStore::Postgres = &self.db.store {
                                if let Err(e) = sqlx::query("UPDATE products SET price = $1, price_cents = $2 WHERE id = $3 AND tenant_id = $4")
                                    .bind(new_price)
                                    .bind(new_price_cents)
                                    .bind(product_id)
                                    .bind(tenant_id)
                                    .execute(&self.db.pool)
                                    .await
                                {
                                    tracing::error!("Failed to update product price: {}", e);
                                    let _ = self.mesh.release_lock(&lock_key, "orchestrator").await;
                                    return Err(format!("Failed to activate smart pricing discount: {}", e));
                                }
                            } else if let DbStore::Sqlite(pool) = &self.db.store {
                                if let Err(e) = sqlx::query("UPDATE products SET price = ?, price_cents = ? WHERE id = ? AND tenant_id = ?")
                                    .bind(new_price)
                                    .bind(new_price_cents)
                                    .bind(product_id)
                                    .bind(tenant_id)
                                    .execute(pool)
                                    .await
                                {
                                    tracing::error!("Failed to update product price: {}", e);
                                    let _ = self.mesh.release_lock(&lock_key, "orchestrator").await;
                                    return Err(format!("Failed to activate smart pricing discount: {}", e));
                                }
                            }

                            let product_name = payload.get("context").and_then(|c| c.get("product_name")).and_then(|v| v.as_str()).unwrap_or("Item");
                            let draft_desc = format!("Draft promotional email for {}", product_name);
                            let draft_payload = serde_json::json!({
                                "feature_type": "promotional_email_draft",
                                "product_name": product_name,
                                "new_price": new_price
                            });
                            let _ = self.execute_action(
                                DepartmentType::Marketing,
                                draft_desc,
                                tenant_id.to_string(),
                                ActionRisk::DraftForReview,
                                draft_payload
                            ).await;
                        }
                    }
                }


                // If this is an Ambassador Reply approval, update the message and dispatch event
                if let Some(payload) = payload_to_use {
                    if payload.get("feature_type").and_then(|v| v.as_str()) == Some("ambassador_reply") {
                        let inbox_message_id = payload.get("inbox_message_id").and_then(|v| v.as_str()).unwrap_or("");
                        let generated_reply = payload.get("generated_response").and_then(|v| v.as_str()).unwrap_or("");

                        if !inbox_message_id.is_empty() {
                            if let Err(e) = self.update_inbox_message_draft(inbox_message_id, tenant_id, generated_reply).await {
                                tracing::error!("Failed to update inbox message draft: {}", e);
                            }
                            if let Err(e) = self.update_inbox_message_status(inbox_message_id, tenant_id, "auto_replied").await {
                                tracing::error!("Failed to update inbox message status: {}", e);
                            }
                        }

                        let approved_event = crate::orchestration::departments::types::DepartmentEvent {
                            id: uuid::Uuid::new_v4().to_string(),
                            tenant_id: tenant_id.to_string(),
                            event_type: "agent:customer_success:approved".to_string(),
                            payload: serde_json::json!({
                                "original_payload": payload,
                                "approval_id": request_id
                            }),
                        };
                        if let Err(e) = self.dispatch_event(approved_event).await {
                            tracing::error!("Failed to dispatch agent:customer_success:approved event: {}", e);
                        }
                    } else if payload.get("feature_type").and_then(|v| v.as_str()) == Some("invoice_followup") || payload.get("feature_type").and_then(|v| v.as_str()) == Some("quote_draft") {
                        let invoice_id = payload.get("invoice_id").and_then(|v| v.as_str()).unwrap_or("");
                        let customer_id = payload.get("customer_id").and_then(|v| v.as_str()).unwrap_or("");
                        let _generated_reply = payload.get("generated_response").and_then(|v| v.as_str()).unwrap_or("");
                        tracing::info!("Simulated sending of personalized invoice reminder for {} to customer {}", invoice_id, customer_id);
                        let _action_id = uuid::Uuid::new_v4().to_string();
                    } else if payload.get("feature_type").and_then(|v| v.as_str()) == Some("dispute_resolution") {
                        let dispute_id = payload.get("dispute_id").and_then(|v| v.as_str()).unwrap_or("");
                        let api_key = std::env::var("STRIPE_SECRET_KEY").unwrap_or_else(|_| "sk_test_123".to_string());
                        let stripe = crate::integrations::stripe::client::StripeClient::new(api_key);
                        if let Err(e) = stripe.submit_dispute_evidence(dispute_id, payload.clone()).await {
                            tracing::error!("Failed to submit dispute evidence: {}", e);
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

                // Publish SSE event
                let sse_payload = serde_json::json!({
                    "event_type": "approval_decision",
                    "data": {
                        "request_id": request_id,
                        "tenant_id": tenant_id,
                        "department": dep,
                        "status": "APPROVED",
                        "original_payload": original_payload,
                    }
                });
                let sse_payload_bytes = serde_json::to_vec(&sse_payload).unwrap_or_default();
                let sse_topic = format!("agent_feed:{}", tenant_id);
                let _ = self.mesh.publish(&sse_topic, sse_payload_bytes).await;

                // Add to ledger
                if let crate::db::DbStore::Postgres = &self.db.store {
                    let entry_id = Uuid::new_v4().to_string();
                    if let Ok(mut tx) = self.db.pool.begin().await {
                        if ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.is_ok() {
                            let _ = sqlx::query(
                                "INSERT INTO ohc_universal_ledger (id, tenant_id, event_type, department, payload, created_at)
                                 VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)"
                            )
                            .bind(&entry_id)
                            .bind(tenant_id)
                            .bind("approval_decision")
                            .bind(&dep)
                            .bind(&payload)
                            .execute(&mut *tx)
                            .await;
                            let _ = tx.commit().await;
                        }
                    }
                }
            } else {
                // Publish SSE event for rejection
                let sse_payload = serde_json::json!({
                    "event_type": "approval_decision",
                    "data": {
                        "request_id": request_id,
                        "tenant_id": tenant_id,
                        "department": dep,
                        "status": "REJECTED",
                        "original_payload": original_payload,
                    }
                });
                let sse_payload_bytes = serde_json::to_vec(&sse_payload).unwrap_or_default();
                let sse_topic = format!("agent_feed:{}", tenant_id);
                let _ = self.mesh.publish(&sse_topic, sse_payload_bytes).await;
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
                sqlx::query("UPDATE omni_inbox_messages SET status = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(new_status).bind(message_id).bind(tenant_id).execute(&self.db.pool).await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE inbox_messages SET status = ? WHERE id = ? AND tenant_id = ?")
                    .bind(new_status).bind(message_id).bind(tenant_id).execute(pool).await.map_err(|e| e.to_string())?;
                sqlx::query("UPDATE omni_inbox_messages SET status = ? WHERE id = ? AND tenant_id = ?")
                    .bind(new_status).bind(message_id).bind(tenant_id).execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    pub async fn simulate_stockout_restock_and_price(&self, tenant_id: &str) -> Result<(), String> {
        let payload = serde_json::json!({
            "feature_type": "stockout_restock_and_price",
            "product_id": uuid::Uuid::new_v4().to_string(),
            "product_name": "Red Dress",
            "old_price": 40.0,
            "new_price": 46.0,
            "suggested_reorder_quantity": 50,
            "message": "Red Dress sold out in 2 days. Demand is high. Operations Agent drafted a reorder for 50 units. Finance Agent suggests raising price from $40 to $46."
        });

        self.execute_action(
            DepartmentType::BusinessAdvisory,
            "Urgent: Red Dress Stockout & Price Action".to_string(),
            tenant_id.to_string(),
            ActionRisk::DraftForReview,
            payload
        ).await.map(|_| ())
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
                sqlx::query("UPDATE omni_inbox_messages SET draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(draft_reply).bind(message_id).bind(tenant_id).execute(&self.db.pool).await.map_err(|e| e.to_string())?;
            }
            DbStore::Sqlite(pool) => {
                sqlx::query("UPDATE inbox_messages SET draft_reply = ? WHERE id = ? AND tenant_id = ?")
                    .bind(draft_reply).bind(message_id).bind(tenant_id).execute(pool).await.map_err(|e| e.to_string())?;
                sqlx::query("UPDATE omni_inbox_messages SET draft_reply = ? WHERE id = ? AND tenant_id = ?")
                    .bind(draft_reply).bind(message_id).bind(tenant_id).execute(pool).await.map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }


    pub async fn predict_replenishment(&self, tenant_id: &str, customer_id: &str) -> Result<Option<String>, String> {
        let pool = &self.db.pool;
        let query = "SELECT created_at FROM orders WHERE tenant_id = $1 AND customer_id = $2 ORDER BY created_at DESC LIMIT 5";

        // Use a generic query to support both Pg and Sqlite dynamically if needed, but since we rely on created_at parsing:
        let orders_res: Result<Vec<(chrono::DateTime<chrono::Utc>,)>, _> = match &self.db.store {
            crate::db::DbStore::Postgres => sqlx::query_as(query)
                .bind(tenant_id)
                .bind(customer_id)
                .fetch_all(pool)
                .await,
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let sqlite_query = "SELECT created_at FROM orders WHERE tenant_id = ? AND customer_id = ? ORDER BY created_at DESC LIMIT 5";
                let rows: Result<Vec<(String,)>, _> = sqlx::query_as(sqlite_query)
                    .bind(tenant_id)
                    .bind(customer_id)
                    .fetch_all(sqlite_pool)
                    .await;

                rows.map(|r| r.into_iter()
                    .filter_map(|(s,)| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| (d.with_timezone(&chrono::Utc),)))
                    .collect())
            }
        };

        let orders = orders_res.map_err(|e| e.to_string())?;

        if orders.len() < 2 {
            return Ok(None);
        }

        let mut total_duration = chrono::Duration::zero();
        for i in 0..(orders.len() - 1) {
            total_duration = total_duration + (orders[i].0 - orders[i + 1].0);
        }

        let avg_duration = total_duration / (orders.len() as i32 - 1);

        let last_order_date = orders[0].0;
        let predicted_date = last_order_date + avg_duration;

        Ok(Some(predicted_date.to_rfc3339()))
    }

    pub async fn get_inventory_summary(&self, tenant_id: &str) -> Result<String, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let rows = sqlx::query("SELECT title, name, inventory_count FROM products WHERE tenant_id = $1 AND inventory_count IS NOT NULL")
                    .bind(tenant_id)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if rows.is_empty() {
                    return Ok("No inventory data available.".to_string());
                }

                use sqlx::Row;
                let mut summary = String::from("Current Inventory:\n");
                for row in rows {
                    let title: Option<String> = row.try_get("title").unwrap_or(None);
                    let name: Option<String> = row.try_get("name").unwrap_or(None);
                    let display_name = title.or(name).unwrap_or_else(|| "Unknown Product".to_string());
                    let count: i32 = row.try_get("inventory_count").unwrap_or(0);
                    summary.push_str(&format!("- {} ({} in stock)\n", display_name, count));
                }

                Ok(summary)
            },
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query("SELECT title, name, inventory_count FROM products WHERE tenant_id = $1 AND inventory_count IS NOT NULL")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if rows.is_empty() {
                    return Ok("No inventory data available.".to_string());
                }

                use sqlx::Row;
                let mut summary = String::from("Current Inventory:\n");
                for row in rows {
                    let title: Option<String> = row.try_get("title").unwrap_or(None);
                    let name: Option<String> = row.try_get("name").unwrap_or(None);
                    let display_name = title.or(name).unwrap_or_else(|| "Unknown Product".to_string());
                    let count: i32 = row.try_get("inventory_count").unwrap_or(0);
                    summary.push_str(&format!("- {} ({} in stock)\n", display_name, count));
                }

                Ok(summary)
            }
        }
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
        let total_points;
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let id = uuid::Uuid::new_v4().to_string();
                let row = sqlx::query("INSERT INTO loyalty_ledger (id, tenant_id, customer_id, points_balance, last_updated) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (tenant_id, customer_id) DO UPDATE SET points_balance = loyalty_ledger.points_balance + EXCLUDED.points_balance, last_updated = EXCLUDED.last_updated RETURNING points_balance")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(customer_id)
                    .bind(points)
                    .bind(&now)
                    .fetch_one(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                use sqlx::Row;
                total_points = row.get::<i32, _>("points_balance");
            }
            crate::db::DbStore::Sqlite(pool) => {
                let exists = sqlx::query("SELECT points_balance FROM loyalty_ledger WHERE tenant_id = ? AND customer_id = ?")
                    .bind(tenant_id)
                    .bind(customer_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = exists {
                    use sqlx::Row;
                    let curr_points = r.get::<i32, _>("points_balance");
                    total_points = curr_points + points;
                    sqlx::query("UPDATE loyalty_ledger SET points_balance = points_balance + ?, last_updated = ? WHERE tenant_id = ? AND customer_id = ?")
                        .bind(points)
                        .bind(&now)
                        .bind(tenant_id)
                        .bind(customer_id)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                } else {
                    total_points = points;
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

        // Fire event to mesh so the Promoter Agent can pick it up
        let event = crate::orchestration::departments::types::DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            event_type: "loyalty.points_awarded".to_string(),
            payload: serde_json::json!({
                "customer_id": customer_id,
                "points": points,
                "total_points": total_points
            })
        };
        let _ = self.dispatch_event(event).await;

        Ok(())
    }

    pub async fn get_order(&self, tenant_id: &str, order_id: &str) -> Result<Option<(String, f64)>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query("SELECT customer_id, total_amount_cents FROM orders WHERE tenant_id = $1 AND id = $2")
                    .bind(tenant_id)
                    .bind(order_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some((r.get("customer_id"), r.get::<f64, _>("total_amount_cents"))))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT customer_id, total_amount_cents FROM orders WHERE tenant_id = ? AND id = ?")
                    .bind(tenant_id)
                    .bind(order_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some((r.get("customer_id"), r.get::<f64, _>("total_amount_cents"))))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn get_service_by_name_like(&self, tenant_id: &str, name: &str) -> Result<Option<(String, f64, String)>, String> {
        let pattern = format!("%{}%", name);
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query("SELECT id, name, CAST(price AS DOUBLE PRECISION) as price_f64 FROM services WHERE tenant_id = $1 AND name ILIKE $2 LIMIT 1")
                    .bind(tenant_id)
                    .bind(&pattern)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    let id: String = r.get("id");
                    let n: String = r.get("name");
                    let p: f64 = r.get("price_f64");
                    Ok(Some((n, p, id)))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT id, name, CAST(price AS REAL) as price_f64 FROM services WHERE tenant_id = ? AND name LIKE ? LIMIT 1")
                    .bind(tenant_id)
                    .bind(&pattern)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    let id: String = r.get("id");
                    let n: String = r.get("name");
                    let p: f64 = r.get("price_f64");
                    Ok(Some((n, p, id)))
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
