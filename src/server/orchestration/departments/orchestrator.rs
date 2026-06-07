use std::sync::Arc;
use serde_json::Value;

use crate::db::DB;
use crate::orchestration::departments::types::{DepartmentType, DepartmentEvent, DepartmentConfig, ApprovalRequest, ActionRisk, ApprovalStatus};
use crate::orchestration::mesh::CentrifugeNode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentTriggerType {
    Schedule,
    EventDriven,
    AdHoc,
}

#[async_trait::async_trait]
pub trait BaseAgent: Send + Sync {
    fn agent_id(&self) -> String;
    fn trigger_type(&self) -> AgentTriggerType;
    async fn execute(&self, payload: Value) -> Result<(), String>;
}

#[async_trait::async_trait]
pub trait Department: BaseAgent {
    fn department_type(&self) -> DepartmentType;
    fn subscribed_events(&self) -> Vec<String>;
    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String>;
    fn get_config(&self, tenant_id: &str) -> Option<DepartmentConfig>;
    fn set_config(&mut self, tenant_id: String, config: DepartmentConfig);
    async fn query_memory(&self, query: &str) -> Result<Vec<String>, String>;
    async fn request_approval(&self, description: String, tenant_id: String, risk: ActionRisk) -> Result<ApprovalRequest, String>;
}

#[derive(Clone)]
pub struct DepartmentOrchestrator {
    db: Arc<DB>,
    handoff_mesh: Arc<CentrifugeNode>,
    departments: Arc<tokio::sync::RwLock<std::collections::HashMap<DepartmentType, Arc<tokio::sync::RwLock<dyn Department>>>>>,
    agents: Arc<tokio::sync::RwLock<std::collections::HashMap<String, Arc<tokio::sync::RwLock<dyn BaseAgent>>>>>,
}

impl DepartmentOrchestrator {
    pub fn new(db: Arc<DB>, handoff_mesh: Arc<CentrifugeNode>) -> Self {
        Self {
            db,
            handoff_mesh,
            departments: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
            agents: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn register_agent(&self, agent: Arc<tokio::sync::RwLock<dyn BaseAgent>>) {
        let id = {
            let a = agent.read().await;
            a.agent_id()
        };
        self.agents.write().await.insert(id, agent);
    }

    pub async fn register_department(&self, department: Arc<tokio::sync::RwLock<dyn Department>>) {
        let dtype = {
            let d = department.read().await;
            d.department_type()
        };
        self.departments.write().await.insert(dtype, department);
    }

    pub async fn dispatch_event(&self, event: DepartmentEvent) -> Result<(), String> {
        tracing::debug!("Dispatching event: {} for tenant: {}", event.event_type, event.tenant_id);

        let deps = self.departments.read().await;
        for dept in deps.values() {
            let d = dept.read().await;
            if d.subscribed_events().contains(&event.event_type) {
                if let Err(e) = d.handle_event(&event).await {
                    tracing::error!("Error handling event {} by {:?}: {}", event.event_type, d.department_type(), e);
                }
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
        payload: Value,
    ) -> Result<ApprovalRequest, String> {
        let req_id = uuid::Uuid::new_v4().to_string();

        let mut status = ApprovalStatus::PendingApproval;
        let mut executed = false;
        if risk == ActionRisk::AutoExecute {
            status = ApprovalStatus::Approved;
            executed = true;
        }

        let req = ApprovalRequest {
            id: req_id.clone(),
            tenant_id: tenant_id.clone(),
            department,
            description,
            status: status.clone(),
            action_risk: risk,
            payload: Some(payload.clone()),
        };

        self.add_approval_request(req.clone()).await;

        if executed {
            tracing::info!("Action auto-executed: {}", req.description);
            let event = DepartmentEvent {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: req.tenant_id.clone(),
                event_type: format!("agent:{}:approved", req.department),
                payload: serde_json::json!({
                    "request_id": req.id,
                    "original_payload": req.payload
                }),
            };
            let _ = self.dispatch_event(event).await;
        } else {
            tracing::info!("Action pending review: {}", req.description);
            let event = DepartmentEvent {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: req.tenant_id.clone(),
                event_type: format!("agent:{}:pending", req.department),
                payload: serde_json::json!({
                    "request_id": req.id,
                    "original_payload": req.payload
                }),
            };
            let _ = self.dispatch_event(event).await;
        }

        Ok(req)
    }

    pub async fn add_approval_request(&self, req: ApprovalRequest) {
        let payload_str = serde_json::to_string(&req.payload).unwrap_or_else(|_| "{}".to_string());
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let status_str = format!("{:?}", req.status);
                let risk_str = format!("{:?}", req.action_risk);
                let dept_str = format!("{:?}", req.department);

                let _ = sqlx::query(
                    r#"
                    INSERT INTO agent_approvals (id, tenant_id, department, description, status, risk_level, payload)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#
                )
                .bind(&req.id)
                .bind(&req.tenant_id)
                .bind(&dept_str)
                .bind(&req.description)
                .bind(&status_str)
                .bind(&risk_str)
                .bind(&payload_str)
                .execute(&self.db.pool)
                .await;
            }
            crate::db::DbStore::Sqlite(pool) => {
                let status_str = format!("{:?}", req.status);
                let risk_str = format!("{:?}", req.action_risk);
                let dept_str = format!("{:?}", req.department);

                let _ = sqlx::query(
                    r#"
                    INSERT INTO agent_approvals (id, tenant_id, department, description, status, risk_level, payload)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&req.id)
                .bind(&req.tenant_id)
                .bind(&dept_str)
                .bind(&req.description)
                .bind(&status_str)
                .bind(&risk_str)
                .bind(&payload_str)
                .execute(pool)
                .await;
            }
        }
    }

    pub async fn decide_approval(&self, request_id: &str, tenant_id: &str, approved: bool) -> Result<(), String> {
        let status_str = if approved { "Approved" } else { "Rejected" };

        let mut dept_opt: Option<String> = None;
        let mut payload_opt: Option<String> = None;

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let r = sqlx::query(
                    "UPDATE agent_approvals SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3 RETURNING *"
                )
                .bind(status_str)
                .bind(request_id)
                .bind(tenant_id)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;

                if let Some(row) = r {
                    use sqlx::Row;
                    dept_opt = Some(row.get("department"));
                    payload_opt = Some(row.get("payload"));
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let _ = sqlx::query(
                    "UPDATE agent_approvals SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?"
                )
                .bind(status_str)
                .bind(request_id)
                .bind(tenant_id)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;

                let res = sqlx::query("SELECT * FROM agent_approvals WHERE id = ? AND tenant_id = ?")
                    .bind(request_id)
                    .bind(tenant_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(row) = res {
                    use sqlx::Row;
                    dept_opt = Some(row.get("department"));
                    payload_opt = Some(row.get("payload"));
                }
            }
        }

        if let (Some(dept_str), Some(payload_str)) = (dept_opt, payload_opt) {
            let payload: Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

            if approved {
                let event = DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: tenant_id.to_string(),
                    event_type: format!("agent:{}:approved", dept_str.to_lowercase()),
                    payload: serde_json::json!({
                        "request_id": request_id,
                        "original_payload": payload
                    }),
                };
                let _ = self.dispatch_event(event).await;
            }
            Ok(())
        } else {
            Err("Approval request not found".to_string())
        }
    }

    pub async fn update_inbox_message_status(&self, message_id: &str, tenant_id: &str, new_status: &str) -> Result<(), String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let r = sqlx::query("UPDATE inbox_messages SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3")
                    .bind(new_status)
                    .bind(message_id)
                    .bind(tenant_id)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if r.rows_affected() == 0 {
                    Err("Inbox message not found".to_string())
                } else {
                    Ok(())
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let r = sqlx::query("UPDATE inbox_messages SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                    .bind(new_status)
                    .bind(message_id)
                    .bind(tenant_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if r.rows_affected() == 0 {
                    Err("Inbox message not found".to_string())
                } else {
                    Ok(())
                }
            }
        }
    }

    pub async fn simulate_smart_pricing(&self, tenant_id: &str) -> Result<(), String> {
        tracing::info!("Simulating smart pricing analysis for {}", tenant_id);

        let sales_projection = "A sudden 20% drop in expected weekend pre-orders detected.";
        let suggestion = "Run a 15% flash sale on Chocolate Cakes to recover the $400 projected gap.";

        let payload = serde_json::json!({
            "feature_type": "smart_pricing",
            "context": {
                "smart_pricing": true,
                "sales_projection": sales_projection,
                "actionable_suggestion": suggestion,
                "discount_percentage": 15,
                "target_product": "Chocolate Cakes",
                "estimated_recovery": 400.0,
            }
        });

        self.execute_action(
            DepartmentType::Finance,
            format!("Smart Pricing Alert: {} {}", sales_projection, suggestion),
            tenant_id.to_string(),
            ActionRisk::DraftForReview,
            payload,
        ).await?;

        Ok(())
    }

    pub async fn update_inbox_message_draft(&self, message_id: &str, tenant_id: &str, draft_reply: &str) -> Result<(), String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let r = sqlx::query("UPDATE inbox_messages SET draft_reply = $1, status = 'draft', updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3")
                    .bind(draft_reply)
                    .bind(message_id)
                    .bind(tenant_id)
                    .execute(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if r.rows_affected() == 0 {
                    Err("Inbox message not found".to_string())
                } else {
                    Ok(())
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let r = sqlx::query("UPDATE inbox_messages SET draft_reply = ?, status = 'draft', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                    .bind(draft_reply)
                    .bind(message_id)
                    .bind(tenant_id)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if r.rows_affected() == 0 {
                    Err("Inbox message not found".to_string())
                } else {
                    Ok(())
                }
            }
        }
    }

    pub async fn check_ai_budget(&self, tenant_id: &str, points: i32) -> Result<bool, String> {
        let budget_limit: i32 = match std::env::var("OHC_AGENT_BUDGET_LIMIT") {
            Ok(v) => v.parse().unwrap_or(2000),
            Err(_) => 2000,
        };

        if budget_limit < 0 {
            return Ok(true);
        }

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let current_spend: i64 = sqlx::query_scalar(
                    "SELECT COALESCE(SUM(points_used), 0) FROM ai_budgets WHERE tenant_id = $1"
                )
                .bind(tenant_id)
                .fetch_one(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;

                if current_spend + points as i64 > budget_limit as i64 {
                    tracing::warn!("Tenant {} hit AI budget limit ({} >= {})", tenant_id, current_spend + points as i64, budget_limit);
                    return Ok(false);
                }

                sqlx::query(
                    "INSERT INTO ai_budgets (id, tenant_id, points_used, feature) VALUES ($1, $2, $3, $4)"
                )
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(tenant_id)
                .bind(points)
                .bind("agent_action")
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(true)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let current_spend: i64 = sqlx::query_scalar(
                    "SELECT COALESCE(SUM(points_used), 0) FROM ai_budgets WHERE tenant_id = ?"
                )
                .bind(tenant_id)
                .fetch_one(pool)
                .await
                .map_err(|e| e.to_string())?;

                if current_spend + points as i64 > budget_limit as i64 {
                    tracing::warn!("Tenant {} hit AI budget limit ({} >= {})", tenant_id, current_spend + points as i64, budget_limit);
                    return Ok(false);
                }

                sqlx::query(
                    "INSERT INTO ai_budgets (id, tenant_id, points_used, feature) VALUES (?, ?, ?, ?)"
                )
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(tenant_id)
                .bind(points)
                .bind("agent_action")
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(true)
            }
        }
    }


    pub async fn query_long_term_memory(&self, tenant_id: &str, query_embedding: &[f32], limit: i64) -> Result<Vec<String>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let embedding_str = format!("[{}]", query_embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                let rows = sqlx::query(
                    r#"
                    SELECT content, 1 - (embedding <=> $1::vector) as similarity
                    FROM agent_long_term_memory
                    WHERE tenant_id = $2
                    ORDER BY embedding <=> $1::vector ASC
                    LIMIT $3
                    "#
                )
                .bind(embedding_str)
                .bind(tenant_id)
                .bind(limit)
                .fetch_all(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut results = Vec::new();
                for r in rows {
                    use sqlx::Row;
                    let content: String = r.get("content");
                    results.push(content);
                }
                Ok(results)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    r#"
                    SELECT content
                    FROM agent_long_term_memory
                    WHERE tenant_id = ?
                    ORDER BY created_at DESC
                    LIMIT ?
                    "#
                )
                .bind(tenant_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut results = Vec::new();
                for r in rows {
                    use sqlx::Row;
                    let content: String = r.get("content");
                    results.push(content);
                }
                Ok(results)
            }
        }
    }

    pub async fn append_to_timeline(&self, event: crate::orchestration::departments::types::TimelineEvent) -> Result<(), String> {
        let meta_str = event.metadata.map(|v| v.to_string()).unwrap_or_else(|| "{}".to_string());
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO customer_timeline (id, tenant_id, customer_id, event_type, source, content, metadata, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
                )
                .bind(event.id)
                .bind(event.tenant_id)
                .bind(event.customer_id)
                .bind(event.event_type)
                .bind(event.source)
                .bind(event.content)
                .bind(meta_str)
                .bind(event.created_at.unwrap_or_else(chrono::Utc::now))
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(())
            }
            crate::db::DbStore::Sqlite(pool) => {
                sqlx::query(
                    "INSERT INTO customer_timeline (id, tenant_id, customer_id, event_type, source, content, metadata, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(event.id)
                .bind(event.tenant_id)
                .bind(event.customer_id)
                .bind(event.event_type)
                .bind(event.source)
                .bind(event.content)
                .bind(meta_str)
                .bind(event.created_at.unwrap_or_else(chrono::Utc::now))
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn write_long_term_memory(&self, record: ohc_builtin_agent::memory_store::EmbeddingRecord) -> Result<(), String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let embedding_str = format!("[{}]", record.embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                let meta_str = record.metadata.map(|v| v.to_string()).unwrap_or_else(|| "{}".to_string());
                sqlx::query(
                    r#"
                    INSERT INTO agent_long_term_memory (id, tenant_id, agent_id, content, embedding, source_type, reliability_score, metadata, created_at)
                    VALUES ($1, $2, $3, $4, $5::vector, $6, $7, $8, $9)
                    "#
                )
                .bind(record.id)
                .bind(record.tenant_id)
                .bind(record.agent_id)
                .bind(record.content)
                .bind(embedding_str)
                .bind(record.source_type)
                .bind(record.reliability_score)
                .bind(meta_str)
                .bind(record.created_at)
                .execute(&self.db.pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(())
            }
            crate::db::DbStore::Sqlite(pool) => {
                let meta_str = record.metadata.map(|v| v.to_string()).unwrap_or_else(|| "{}".to_string());
                sqlx::query(
                    r#"
                    INSERT INTO agent_long_term_memory (id, tenant_id, agent_id, content, source_type, reliability_score, metadata, created_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(record.id)
                .bind(record.tenant_id)
                .bind(record.agent_id)
                .bind(record.content)
                .bind(record.source_type)
                .bind(record.reliability_score)
                .bind(meta_str)
                .bind(record.created_at)
                .execute(pool)
                .await
                .map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn get_pending_approvals(&self, tenant_id: &str, cursor: Option<String>, limit: i64) -> Vec<ApprovalRequest> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let rows = sqlx::query(
                    "SELECT * FROM agent_approvals WHERE tenant_id = $1 AND status = 'PendingApproval' ORDER BY created_at DESC LIMIT $2"
                )
                .bind(tenant_id)
                .bind(limit)
                .fetch_all(&self.db.pool)
                .await
                .unwrap_or_default();

                rows.into_iter().map(|r| {
                    use sqlx::Row;
                    let dept_str: String = r.get("department");
                    let risk_str: String = r.get("risk_level");
                    let status_str: String = r.get("status");
                    let payload_str: String = r.get("payload");
                    let payload: Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

                    ApprovalRequest {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        department: dept_str.parse().unwrap_or(DepartmentType::Operations),
                        description: r.get("description"),
                        status: if status_str == "PendingApproval" { ApprovalStatus::PendingApproval } else if status_str == "Approved" { ApprovalStatus::Approved } else { ApprovalStatus::Rejected },
                        action_risk: risk_str.parse().unwrap_or(ActionRisk::DraftForReview),
                        payload: Some(payload),
                    }
                }).collect()
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    "SELECT * FROM agent_approvals WHERE tenant_id = ? AND status = 'PendingApproval' ORDER BY created_at DESC LIMIT ?"
                )
                .bind(tenant_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .unwrap_or_default();

                rows.into_iter().map(|r| {
                    use sqlx::Row;
                    let dept_str: String = r.get("department");
                    let risk_str: String = r.get("risk_level");
                    let status_str: String = r.get("status");
                    let payload_str: String = r.get("payload");
                    let payload: Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

                    ApprovalRequest {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        department: dept_str.parse().unwrap_or(DepartmentType::Operations),
                        description: r.get("description"),
                        status: if status_str == "PendingApproval" { ApprovalStatus::PendingApproval } else if status_str == "Approved" { ApprovalStatus::Approved } else { ApprovalStatus::Rejected },
                        action_risk: risk_str.parse().unwrap_or(ActionRisk::DraftForReview),
                        payload: Some(payload),
                    }
                }).collect()
            }
        }
    }

    pub async fn get_activity_feed(&self, tenant_id: &str, cursor: Option<String>, limit: i64) -> Vec<ApprovalRequest> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let rows = sqlx::query(
                    "SELECT * FROM agent_approvals WHERE tenant_id = $1 AND status != 'PendingApproval' ORDER BY created_at DESC LIMIT $2"
                )
                .bind(tenant_id)
                .bind(limit)
                .fetch_all(&self.db.pool)
                .await
                .unwrap_or_default();

                rows.into_iter().map(|r| {
                    use sqlx::Row;
                    let dept_str: String = r.get("department");
                    let risk_str: String = r.get("risk_level");
                    let status_str: String = r.get("status");
                    let payload_str: String = r.get("payload");
                    let payload: Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

                    ApprovalRequest {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        department: dept_str.parse().unwrap_or(DepartmentType::Operations),
                        description: r.get("description"),
                        status: if status_str == "Approved" { ApprovalStatus::Approved } else { ApprovalStatus::Rejected },
                        action_risk: risk_str.parse().unwrap_or(ActionRisk::DraftForReview),
                        payload: Some(payload),
                    }
                }).collect()
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    "SELECT * FROM agent_approvals WHERE tenant_id = ? AND status != 'PendingApproval' ORDER BY created_at DESC LIMIT ?"
                )
                .bind(tenant_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .unwrap_or_default();

                rows.into_iter().map(|r| {
                    use sqlx::Row;
                    let dept_str: String = r.get("department");
                    let risk_str: String = r.get("risk_level");
                    let status_str: String = r.get("status");
                    let payload_str: String = r.get("payload");
                    let payload: Value = serde_json::from_str(&payload_str).unwrap_or_else(|_| serde_json::json!({}));

                    ApprovalRequest {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        department: dept_str.parse().unwrap_or(DepartmentType::Operations),
                        description: r.get("description"),
                        status: if status_str == "Approved" { ApprovalStatus::Approved } else { ApprovalStatus::Rejected },
                        action_risk: risk_str.parse().unwrap_or(ActionRisk::DraftForReview),
                        payload: Some(payload),
                    }
                }).collect()
            }
        }
    }

    pub async fn update_department_config(&self, tenant_id: &str, department: &str, config: crate::orchestration::departments::types::DepartmentConfig) -> Result<(), String> {
        let dtype = department.parse::<DepartmentType>().map_err(|e| e.to_string())?;

        let deps = self.departments.write().await;
        if let Some(dept_arc) = deps.get(&dtype) {
            let mut dept = dept_arc.write().await;
            dept.set_config(tenant_id.to_string(), config);
            Ok(())
        } else {
            Err("Department not found".to_string())
        }
    }

    pub async fn list_customer360(&self, tenant_id: &str) -> Result<Vec<crate::orchestration::departments::types::Customer360>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let rows = sqlx::query("SELECT * FROM customer360 WHERE tenant_id = $1 ORDER BY updated_at DESC")
                    .bind(tenant_id)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let mut customers = Vec::new();
                for r in rows {
                    use sqlx::Row;
                    let preferences_str: Option<String> = r.get("preferences");
                    let preferences = preferences_str.and_then(|s| serde_json::from_str(&s).ok());
                    customers.push(crate::orchestration::departments::types::Customer360 {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        customer_id: r.get("customer_id"),
                        email: r.get("email"),
                        phone: r.get("phone"),
                        mood: r.get("mood"),
                        preferences,
                        created_at: r.get("created_at"),
                        updated_at: r.get("updated_at"),
                    });
                }
                Ok(customers)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query("SELECT * FROM customer360 WHERE tenant_id = ? ORDER BY updated_at DESC")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let mut customers = Vec::new();
                for r in rows {
                    use sqlx::Row;
                    let preferences_str: Option<String> = r.get("preferences");
                    let preferences = preferences_str.and_then(|s| serde_json::from_str(&s).ok());
                    customers.push(crate::orchestration::departments::types::Customer360 {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        customer_id: r.get("customer_id"),
                        email: r.get("email"),
                        phone: r.get("phone"),
                        mood: r.get("mood"),
                        preferences,
                        created_at: r.get("created_at"),
                        updated_at: r.get("updated_at"),
                    });
                }
                Ok(customers)
            }
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
                        created_at: r.get("created_at"),
                        updated_at: r.get("updated_at"),
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
                    .map_err(|e| e.to_string())?;

                if exists.is_some() {
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
                        last_updated: r.get("last_updated"),
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
                        tier_name: r.get("tier_name"),
                        last_updated: r.get("last_updated"),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn add_loyalty_points(&self, tenant_id: &str, customer_id: &str, points: i32) -> Result<(), String> {
        let l = self.get_loyalty_ledger(tenant_id, customer_id).await?;
        let now = chrono::Utc::now();
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                if l.is_some() {
                    sqlx::query("UPDATE loyalty_ledger SET points_balance = points_balance + $1, last_updated = $2 WHERE tenant_id = $3 AND customer_id = $4")
                        .bind(points)
                        .bind(now)
                        .bind(tenant_id)
                        .bind(customer_id)
                        .execute(&self.db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                } else {
                    sqlx::query("INSERT INTO loyalty_ledger (id, tenant_id, customer_id, points_balance, last_updated) VALUES ($1, $2, $3, $4, $5)")
                        .bind(uuid::Uuid::new_v4().to_string())
                        .bind(tenant_id)
                        .bind(customer_id)
                        .bind(points)
                        .bind(now)
                        .execute(&self.db.pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                if l.is_some() {
                    sqlx::query("UPDATE loyalty_ledger SET points_balance = points_balance + ?, last_updated = ? WHERE tenant_id = ? AND customer_id = ?")
                        .bind(points)
                        .bind(now)
                        .bind(tenant_id)
                        .bind(customer_id)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                } else {
                    sqlx::query("INSERT INTO loyalty_ledger (id, tenant_id, customer_id, points_balance, last_updated) VALUES (?, ?, ?, ?, ?)")
                        .bind(uuid::Uuid::new_v4().to_string())
                        .bind(tenant_id)
                        .bind(customer_id)
                        .bind(points)
                        .bind(now)
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }

    pub async fn get_customer_timeline(&self, tenant_id: &str, customer_id: &str, limit: i64) -> Result<Vec<crate::orchestration::departments::types::TimelineEvent>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let rows = sqlx::query("SELECT * FROM customer_timeline WHERE tenant_id = $1 AND customer_id = $2 ORDER BY created_at DESC LIMIT $3")
                    .bind(tenant_id)
                    .bind(customer_id)
                    .bind(limit)
                    .fetch_all(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                let mut events = Vec::new();
                for r in rows {
                    use sqlx::Row;
                    let meta_str: String = r.get("metadata");
                    events.push(crate::orchestration::departments::types::TimelineEvent {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        customer_id: r.get("customer_id"),
                        event_type: r.get("event_type"),
                        source: r.get("source"),
                        content: r.get("content"),
                        metadata: serde_json::from_str(&meta_str).ok(),
                        created_at: Some(r.get("created_at")),
                    });
                }
                Ok(events)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query("SELECT * FROM customer_timeline WHERE tenant_id = ? AND customer_id = ? ORDER BY created_at DESC LIMIT ?")
                    .bind(tenant_id)
                    .bind(customer_id)
                    .bind(limit)
                    .fetch_all(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                let mut events = Vec::new();
                for r in rows {
                    use sqlx::Row;
                    let meta_str: String = r.get("metadata");
                    events.push(crate::orchestration::departments::types::TimelineEvent {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        customer_id: r.get("customer_id"),
                        event_type: r.get("event_type"),
                        source: r.get("source"),
                        content: r.get("content"),
                        metadata: serde_json::from_str(&meta_str).ok(),
                        created_at: r.get("created_at"),
                    });
                }
                Ok(events)
            }
        }
    }

    pub async fn get_order(&self, tenant_id: &str, order_id: &str) -> Result<Option<(String, f64)>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query("SELECT id, total_amount FROM orders WHERE tenant_id = $1 AND id = $2")
                    .bind(tenant_id)
                    .bind(order_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    let amt: f64 = r.get("total_amount");
                    Ok(Some((r.get("id"), amt)))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT id, total_amount FROM orders WHERE tenant_id = ? AND id = ?")
                    .bind(tenant_id)
                    .bind(order_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    let amt: f64 = r.get("total_amount");
                    Ok(Some((r.get("id"), amt)))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn get_service_by_name_like(&self, tenant_id: &str, name: &str) -> Result<Option<(String, f64)>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let pattern = format!("%{}%", name);
                let row = sqlx::query("SELECT id, price FROM services WHERE tenant_id = $1 AND title ILIKE $2 LIMIT 1")
                    .bind(tenant_id)
                    .bind(pattern)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    let price: f64 = r.get("price");
                    Ok(Some((r.get("id"), price)))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let pattern = format!("%{}%", name);
                let row = sqlx::query("SELECT id, price FROM services WHERE tenant_id = ? AND title LIKE ? LIMIT 1")
                    .bind(tenant_id)
                    .bind(pattern)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    let price: f64 = r.get("price");
                    Ok(Some((r.get("id"), price)))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn get_booking(&self, tenant_id: &str, booking_id: &str) -> Result<Option<String>, String> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let row = sqlx::query("SELECT id FROM bookings WHERE tenant_id = $1 AND id = $2")
                    .bind(tenant_id)
                    .bind(booking_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some(r.get("id")))
                } else {
                    Ok(None)
                }
            }
            crate::db::DbStore::Sqlite(pool) => {
                let row = sqlx::query("SELECT id FROM bookings WHERE tenant_id = ? AND id = ?")
                    .bind(tenant_id)
                    .bind(booking_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                if let Some(r) = row {
                    use sqlx::Row;
                    Ok(Some(r.get("id")))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
