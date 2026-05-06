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
    departments: RwLock<HashMap<DepartmentType, Arc<tokio::sync::RwLock<dyn Department>>>>,
    event_subscriptions: RwLock<HashMap<String, Vec<DepartmentType>>>,
}

impl DepartmentOrchestrator {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self {
            db,
            departments: RwLock::new(HashMap::new()),
            event_subscriptions: RwLock::new(HashMap::new()),
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

pub async fn setup_dummy_orchestrator(db: Arc<crate::db::DB>) -> Arc<DepartmentOrchestrator> {
    let orchestrator = Arc::new(DepartmentOrchestrator::new(db));

    let operations = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
        DepartmentType::Operations,
        vec!["OrderPlaced".to_string(), "InventoryLow".to_string()],
        orchestrator.clone()
    )));
    orchestrator.register_department(operations).await;

    let marketing = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
        DepartmentType::Marketing,
        vec!["NewProductAdded".to_string()],
        orchestrator.clone()
    )));
    orchestrator.register_department(marketing).await;

    let sales = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
        DepartmentType::Sales,
        vec!["LeadGenerated".to_string()],
        orchestrator.clone()
    )));
    orchestrator.register_department(sales).await;

    let customer_success = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
        DepartmentType::CustomerSuccess,
        vec!["CustomerMessageReceived".to_string(), "OrderDelivered".to_string()],
        orchestrator.clone()
    )));
    orchestrator.register_department(customer_success).await;

    let finance = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
        DepartmentType::Finance,
        vec!["PaymentReceived".to_string(), "OrderPlaced".to_string()],
        orchestrator.clone()
    )));
    orchestrator.register_department(finance).await;

    let legal = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
        DepartmentType::Legal,
        vec!["NewContractRequested".to_string()],
        orchestrator.clone()
    )));
    orchestrator.register_department(legal).await;

    let advisory = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
        DepartmentType::BusinessAdvisory,
        vec!["WeeklyMetricsReady".to_string()],
        orchestrator.clone()
    )));
    orchestrator.register_department(advisory).await;

    orchestrator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestrator_routing() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let dummy_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; conn.execute("RESET ROLE").await?; Ok(true) }) })
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();
        let db = Arc::new(crate::db::DB { pool: dummy_pool, store: DbStore::Sqlite(sqlite_pool) });

        if let DbStore::Sqlite(pool) = &db.store {
            let _ = sqlx::query(
                "CREATE TABLE IF NOT EXISTS agent_approvals (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT NOT NULL,
                    department TEXT NOT NULL,
                    description TEXT NOT NULL,
                    status TEXT NOT NULL,
                    action_risk TEXT NOT NULL DEFAULT 'HIGH',
                    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                );"
            )
            .execute(pool)
            .await;
        }

        let orchestrator = Arc::new(DepartmentOrchestrator::new(db));

        let operations = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
            DepartmentType::Operations,
            vec!["OrderPlaced".to_string()],
            orchestrator.clone()
        )));
        orchestrator.register_department(operations.clone()).await;

        let finance = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
            DepartmentType::Finance,
            vec!["OrderPlaced".to_string()],
            orchestrator.clone()
        )));
        orchestrator.register_department(finance.clone()).await;

        let marketing = Arc::new(tokio::sync::RwLock::new(DummyDepartment::new(
            DepartmentType::Marketing,
            vec!["NewProductAdded".to_string()],
            orchestrator.clone()
        )));
        orchestrator.register_department(marketing.clone()).await;


        let event = DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: "tenant1".to_string(),
            event_type: "OrderPlaced".to_string(),
            payload: serde_json::json!({"order_id": "123"}),
        };

        let result = orchestrator.dispatch_event(event).await;
        assert!(result.is_ok());

        // Verify routing works
        let ops_lock = operations.read().await;
        assert_eq!(ops_lock.received_events.lock().unwrap().len(), 1);

        let fin_lock = finance.read().await;
        assert_eq!(fin_lock.received_events.lock().unwrap().len(), 1);

        let mkt_lock = marketing.read().await;
        assert_eq!(mkt_lock.received_events.lock().unwrap().len(), 0);

    }

    #[tokio::test]
    async fn test_approval_workflow_tenant_isolation() {
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let dummy_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; conn.execute("RESET ROLE").await?; Ok(true) }) })
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();
        let db = Arc::new(crate::db::DB { pool: dummy_pool, store: DbStore::Sqlite(sqlite_pool) });

        if let DbStore::Sqlite(pool) = &db.store {
            let _ = sqlx::query(
                "CREATE TABLE IF NOT EXISTS agent_approvals (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT NOT NULL,
                    department TEXT NOT NULL,
                    description TEXT NOT NULL,
                    status TEXT NOT NULL,
                    action_risk TEXT NOT NULL DEFAULT 'HIGH',
                    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
                );"
            )
            .execute(pool)
            .await;
        }

        let orchestrator = setup_dummy_orchestrator(db).await;

        let req = ApprovalRequest {
            id: "req1".to_string(),
            tenant_id: "tenant1".to_string(),
            department: DepartmentType::CustomerSuccess,
            description: "Reply to angry customer".to_string(),
            status: ApprovalStatus::Pending,
            action_risk: "HIGH".to_string(),
        };

        orchestrator.add_approval_request(req).await;

        // Tenant 2 shouldn't see Tenant 1's approvals
        let pending_t2 = orchestrator.get_pending_approvals("tenant2").await;
        assert_eq!(pending_t2.len(), 0);

        // Tenant 1 should see it
        let pending_t1 = orchestrator.get_pending_approvals("tenant1").await;
        assert_eq!(pending_t1.len(), 1);

        // Tenant 2 cannot approve Tenant 1's request
        let result = orchestrator.decide_approval("req1", "tenant2", true).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unauthorized");

        // Tenant 1 can approve
        let result = orchestrator.decide_approval("req1", "tenant1", true).await;
        assert!(result.is_ok());

        let pending_after = orchestrator.get_pending_approvals("tenant1").await;
        assert_eq!(pending_after.len(), 0);
    }
}
