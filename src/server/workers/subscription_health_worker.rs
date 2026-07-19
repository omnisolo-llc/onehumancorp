use sqlx::Row;
use std::sync::Arc;
use crate::db::DB;
use crate::orchestration::departments::types::DepartmentEvent;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use uuid::Uuid;

pub struct SubscriptionHealthWorker {
    pub db: Arc<DB>,
    pub orchestrator: Option<Arc<DepartmentOrchestrator>>,
}

impl SubscriptionHealthWorker {
    pub fn new(db: Arc<DB>) -> Self { Self { db, orchestrator: None } }
    pub fn with_orchestrator(mut self, orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        self.orchestrator = Some(orchestrator); self
    }
    pub fn start(&self) {
        let db = self.db.clone();
        let orchestrator = self.orchestrator.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30)); // Poll every 30s
            loop {
                interval.tick().await;
                let job = match &db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING' WHERE id = (SELECT id FROM ohc_job_queue WHERE status = 'PENDING' AND next_retry_at <= NOW() AND job_type = 'subscription_health' FOR UPDATE SKIP LOCKED LIMIT 1) RETURNING id, tenant_id, payload")
                            .fetch_optional(&db.pool).await.unwrap_or(None)
                            .map(|r| {
                                let job_id: String = r.get("id");
                                let tenant_id: String = r.get("tenant_id");
                                let payload_str: String = r.get("payload");
                                (job_id, tenant_id, payload_str)
                            })
                    },
                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                         sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING' WHERE id = (SELECT id FROM ohc_job_queue WHERE status = 'PENDING' AND next_retry_at <= CURRENT_TIMESTAMP AND job_type = 'subscription_health' LIMIT 1) RETURNING id, tenant_id, payload")
                            .fetch_optional(sqlite_pool).await.unwrap_or(None)
                            .map(|r| {
                                let job_id: String = r.get("id");
                                let tenant_id: String = r.get("tenant_id");
                                let payload_str: String = r.get("payload");
                                (job_id, tenant_id, payload_str)
                            })
                    }
                };

                if let Some((job_id, tenant_id, payload_str)) = job {

                    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&payload_str) {
                        if let (Some(subscriber_id), Some(customer_id)) = (payload.get("subscriber_id").and_then(|v| v.as_str()), payload.get("customer_id").and_then(|v| v.as_str())) {

                            let (status, last_engagement_at) = match &db.store {
                                crate::db::DbStore::Postgres => {
                                    sqlx::query("SELECT status, last_engagement_at FROM subscribers WHERE id = $1 AND tenant_id = $2")
                                        .bind(subscriber_id).bind(&tenant_id).fetch_optional(&db.pool).await.unwrap_or(None).map(|r| {
                                            let s: String = r.get("status");
                                            let dt: Option<chrono::DateTime<chrono::Utc>> = r.try_get("last_engagement_at").unwrap_or(None);
                                            (s, dt.map(|d| d.timestamp()))
                                        }).unwrap_or(("ACTIVE".to_string(), None))
                                },
                                crate::db::DbStore::Sqlite(sqlite_pool) => {
                                    sqlx::query("SELECT status, last_engagement_at FROM subscribers WHERE id = ? AND tenant_id = ?")
                                        .bind(subscriber_id).bind(&tenant_id).fetch_optional(sqlite_pool).await.unwrap_or(None).map(|r| {
                                            let s: String = r.get("status");
                                            let dt: Option<String> = r.try_get("last_engagement_at").unwrap_or(None);
                                            let ts = dt.and_then(|d| chrono::DateTime::parse_from_rfc3339(d.as_str()).ok()).map(|d| d.timestamp());
                                            (s, ts)
                                        }).unwrap_or(("ACTIVE".to_string(), None))
                                }
                            };

                            let mut health_score = 100;
                            if status == "PAST_DUE" { health_score -= 50; }
                            if let Some(ts) = last_engagement_at {
                                let now = chrono::Utc::now().timestamp();
                                if now - ts > 1814400 { health_score -= 20; }
                            } else { health_score -= 10; }

                             match &db.store {
                                crate::db::DbStore::Postgres => {
                                    let _ = sqlx::query("UPDATE subscribers SET health_score = $1 WHERE id = $2 AND tenant_id = $3").bind(health_score).bind(subscriber_id).bind(&tenant_id).execute(&db.pool).await;
                                },
                                crate::db::DbStore::Sqlite(sqlite_pool) => {
                                    let _ = sqlx::query("UPDATE subscribers SET health_score = ? WHERE id = ? AND tenant_id = ?").bind(health_score).bind(subscriber_id).bind(&tenant_id).execute(sqlite_pool).await;
                                }
                            }

                            if health_score < 50 {
                                let already_drafted = match &db.store {
                                    crate::db::DbStore::Postgres => {
                                        sqlx::query("SELECT id FROM agent_action_requests WHERE tenant_id = $1 AND payload->>'subscriber_id' = $2 AND created_at > NOW() - INTERVAL '30 days'")
                                            .bind(&tenant_id).bind(subscriber_id).fetch_optional(&db.pool).await.unwrap_or(None).is_some()
                                    },
                                    crate::db::DbStore::Sqlite(sqlite_pool) => {
                                        sqlx::query("SELECT id FROM agent_action_requests WHERE tenant_id = ? AND json_extract(payload, '$.subscriber_id') = ? AND created_at > datetime('now', '-30 days')")
                                            .bind(&tenant_id).bind(subscriber_id).fetch_optional(sqlite_pool).await.unwrap_or(None).is_some()
                                    }
                                };

                                if !already_drafted {
                                    if let Some(orch) = &orchestrator {
                                        let event = DepartmentEvent {
                                            id: Uuid::new_v4().to_string(),
                                            tenant_id: tenant_id.clone(),
                                            event_type: "tenant.subscription.at_risk".to_string(),
                                            payload: serde_json::json!({
                                                "subscriber_id": subscriber_id,
                                                "customer_id": customer_id,
                                                "health_score": health_score
                                            }),
                                        };
                                        let _ = orch.dispatch_event(event).await;
                                    }
                                }
                            }
                        }
                    }

                    match &db.store {
                         crate::db::DbStore::Postgres => { let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1").bind(job_id).execute(&db.pool).await; },
                         crate::db::DbStore::Sqlite(sqlite_pool) => { let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?").bind(job_id).execute(sqlite_pool).await; }
                    }
                }
            }
        });
    }
}
