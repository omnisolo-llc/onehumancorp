use crate::orchestration::departments::memory::layer::CrossDepartmentMemoryLayer;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;
use std::str::FromStr;

use crate::orchestration::departments::types::{DepartmentType, DepartmentConfig, DepartmentEvent, ApprovalRequest, ApprovalStatus};
use crate::db::DbStore;

#[async_trait::async_trait]
pub trait Department: Send + Sync {
    fn department_type(&self) -> DepartmentType;
    fn subscribed_events(&self) -> Vec<String>;
    async fn handle_event(&self, event: &DepartmentEvent) -> Result<(), String>;
    async fn query_memory(&self, query: &str) -> Result<Vec<String>, String>;
    async fn request_approval(&self, description: String, tenant_id: String) -> Result<ApprovalRequest, String>;
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
        Ok(())
    }

    async fn query_memory(&self, _query: &str) -> Result<Vec<String>, String> {
        // Dummy implementation
        Ok(vec![])
    }

    async fn request_approval(&self, description: String, tenant_id: String) -> Result<ApprovalRequest, String> {
        let req = ApprovalRequest {
            id: Uuid::new_v4().to_string(),
            tenant_id,
            department: self.dep_type,
            description,
            status: ApprovalStatus::Pending,
            action_risk: "HIGH".to_string(),
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
    pub memory_layer: Option<Arc<CrossDepartmentMemoryLayer>>,
    departments: RwLock<HashMap<DepartmentType, Arc<tokio::sync::RwLock<dyn Department>>>>,
    event_subscriptions: RwLock<HashMap<String, Vec<DepartmentType>>>,
}

impl DepartmentOrchestrator {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self {
            db,
            departments: RwLock::new(HashMap::new()),
            event_subscriptions: RwLock::new(HashMap::new()),
            memory_layer: None,
        }
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
        let subscriptions = self.event_subscriptions.read().await;
        if let Some(dep_types) = subscriptions.get(&event.event_type) {
            let departments = self.departments.read().await;
            for dep_type in dep_types {
                if let Some(dep) = departments.get(dep_type) {
                    dep.read().await.handle_event(&event).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn add_approval_request(&self, req: ApprovalRequest) {
        let now = Utc::now();
        let status_str = match req.status {
            ApprovalStatus::Pending => "PENDING",
            ApprovalStatus::Approved => "APPROVED",
            ApprovalStatus::Rejected => "REJECTED",
        };

        match &self.db.store {
            DbStore::Postgres => {
                let _ = sqlx::query(
                    "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
                )
                .bind(&req.id)
                .bind(&req.tenant_id)
                .bind(req.department.to_string())
                .bind(&req.description)
                .bind(status_str)
                .bind(&req.action_risk)
                .bind(now)
                .bind(now)
                .execute(&self.db.pool)
                .await;
            }
            DbStore::Sqlite(pool) => {
                let _ = sqlx::query(
                    "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&req.id)
                .bind(&req.tenant_id)
                .bind(req.department.to_string())
                .bind(&req.description)
                .bind(status_str)
                .bind(&req.action_risk)
                .bind(now)
                .bind(now)
                .execute(pool)
                .await;
            }
        }
    }

    pub async fn get_pending_approvals(&self, tenant_id: &str) -> Vec<ApprovalRequest> {
        let mut results = Vec::new();

        match &self.db.store {
            DbStore::Postgres => {
                let fetch_res = sqlx::query("SELECT id, tenant_id, department, description, status, action_risk FROM agent_approvals WHERE tenant_id = $1 AND status = 'PENDING'")
                    .bind(tenant_id)
                    .fetch_all(&self.db.pool)
                    .await;
                if let Ok(rows) = fetch_res {
                    use sqlx::Row;
                    for row in rows {
                        let dep_str: String = row.get("department");
                        let status_str: String = row.get("status");
                        let department = DepartmentType::from_str(&dep_str).unwrap_or(DepartmentType::Operations);
                        let status = match status_str.as_str() {
                            "PENDING" => ApprovalStatus::Pending,
                            "APPROVED" => ApprovalStatus::Approved,
                            "REJECTED" => ApprovalStatus::Rejected,
                            _ => ApprovalStatus::Pending,
                        };
                        results.push(ApprovalRequest {
                            id: row.get("id"),
                            tenant_id: row.get("tenant_id"),
                            department,
                            description: row.get("description"),
                            status,
                            action_risk: row.get("action_risk"),
                        });
                    }
                }
            }
            DbStore::Sqlite(pool) => {
                let fetch_res = sqlx::query("SELECT id, tenant_id, department, description, status, action_risk FROM agent_approvals WHERE tenant_id = ? AND status = 'PENDING'")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await;
                if let Ok(rows) = fetch_res {
                    use sqlx::Row;
                    for row in rows {
                        let dep_str: String = row.get("department");
                        let status_str: String = row.get("status");
                        let department = DepartmentType::from_str(&dep_str).unwrap_or(DepartmentType::Operations);
                        let status = match status_str.as_str() {
                            "PENDING" => ApprovalStatus::Pending,
                            "APPROVED" => ApprovalStatus::Approved,
                            "REJECTED" => ApprovalStatus::Rejected,
                            _ => ApprovalStatus::Pending,
                        };
                        results.push(ApprovalRequest {
                            id: row.get("id"),
                            tenant_id: row.get("tenant_id"),
                            department,
                            description: row.get("description"),
                            status,
                            action_risk: row.get("action_risk"),
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

        match &self.db.store {
            DbStore::Postgres => {
                let update_res = sqlx::query("UPDATE agent_approvals SET status = $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4")
                    .bind(new_status)
                    .bind(now)
                    .bind(request_id)
                    .bind(tenant_id)
                    .execute(&self.db.pool)
                    .await;
                match update_res {
                    Ok(result) => {
                        if result.rows_affected() > 0 { Ok(()) } else { Err("Unauthorized".to_string()) }
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            DbStore::Sqlite(pool) => {
                let update_res = sqlx::query("UPDATE agent_approvals SET status = ?, updated_at = ? WHERE id = ? AND tenant_id = ?")
                    .bind(new_status)
                    .bind(now)
                    .bind(request_id)
                    .bind(tenant_id)
                    .execute(pool)
                    .await;
                match update_res {
                    Ok(result) => {
                        if result.rows_affected() > 0 { Ok(()) } else { Err("Unauthorized".to_string()) }
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
        }
    }

}
