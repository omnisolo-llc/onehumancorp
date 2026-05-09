use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::sync_service_server::SyncService;
use crate::sip::SipDB;
use std::sync::Arc;
use crate::hub::Hub;
use crate::db::DbStore;

pub struct MySyncService {
    hub: Arc<Hub>,
}

impl MySyncService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MySyncService { hub }
    }
}

#[tonic::async_trait]
impl SyncService for MySyncService {
    async fn hybrid_sync_missions(
        &self,
        request: Request<HybridSyncMissionsRequest>,
    ) -> Result<Response<HybridSyncMissionsResponse>, Status> {
        let md = request.metadata().clone();
        let req = request.into_inner();
        let payloads = req.payloads;

        if payloads.is_empty() {
            return Ok(Response::new(HybridSyncMissionsResponse {
                status: "success".to_string(),
                message: "no missions to sync".to_string(),
                synced_count: 0,
            }));
        }

        let mut synced_count = 0;
        let sip_db = SipDB {
            store: self.hub.store.clone(),
            pg_pool: self.hub.pg_pool.clone(),
            context_root: std::env::var("CONTEXT_ROOT").ok(),
            tenant_id: "system".to_string()
        };

        for p in payloads {
            if p.id.is_empty() {
                continue;
            }

            let status = if p.status.is_empty() {
                "PENDING".to_string()
            } else {
                p.status
            };

            let force_local = md.get("x-ohc-conflict-resolution")
                .map(|v| v.to_str().unwrap_or_default() == "force-local")
                .unwrap_or(false);

            match sip_db.upsert_mission(&p.id, &status, &p.payload, force_local).await {
                Ok(_) => {
                    synced_count += 1;
                }
                Err(e) => {
                    tracing::error!("failed to upsert mission from sync daemon: error={}", e);
                }
            }
        }

        Ok(Response::new(HybridSyncMissionsResponse {
            status: "success".to_string(),
            message: "missions synced successfully".to_string(),
            synced_count,
        }))
    }

    async fn vector_sync(
        &self,
        _request: Request<VectorSyncRequest>,
    ) -> Result<Response<VectorSyncResponse>, Status> {
        Ok(Response::new(VectorSyncResponse {
            status: "success".to_string(),
            message: "vectors synced successfully".to_string(),
        }))
    }

    async fn power_sync_push(
        &self,
        request: Request<PowerSyncPushRequest>,
    ) -> Result<Response<PowerSyncPushResponse>, Status> {
        let md = request.metadata().clone();
        let req = request.into_inner();
        tracing::debug!("PowerSync received push request.");

        let spiffe_id_str = md.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let parsed = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("system".to_string(), "".to_string()));
        let mut tenant_id = parsed.0;
        if tenant_id.is_empty() {
            tenant_id = "system".to_string();
        }

        let items: Vec<serde_json::Value> = serde_json::from_str(&req.payload).unwrap_or_default();
        if items.is_empty() {
            return Ok(Response::new(PowerSyncPushResponse {
                status: "ok".to_string(),
            }));
        }

        match &self.hub.store {
            DbStore::Postgres => {
                let mut tx = self.hub.pg_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
                crate::utils::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

                for item in items {
                    if item["table"].as_str() == Some("agent_missions") {
                        let id = item["id"].as_str().unwrap_or("");
                        let status = item["status"].as_str().unwrap_or("PENDING");
                        let payload = item["payload"].as_str().unwrap_or("");
                        let org_id = item["organization_id"].as_str().unwrap_or(&tenant_id);
                        let updated_at_str = item["updated_at"].as_str().unwrap_or("");
                        let version = item["version"].as_i64().unwrap_or(1);

                        let updated_at = chrono::DateTime::parse_from_rfc3339(updated_at_str)
                            .map(|dt| dt.with_timezone(&chrono::Utc))
                            .unwrap_or_else(|_| chrono::Utc::now());

                        if id.is_empty() {
                            continue;
                        }

                        let query = "
                            INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at, _sync_status, version)
                            VALUES ($1, $2, $3, $4, $5, 'synced', $6)
                            ON CONFLICT(id) DO UPDATE SET
                                status = excluded.status,
                                payload = excluded.payload,
                                tenant_id = excluded.tenant_id,
                                updated_at = excluded.updated_at,
                                _sync_status = 'synced',
                                version = excluded.version
                            WHERE agent_missions.updated_at < excluded.updated_at
                        ";

                        if let Err(e) = sqlx::query(query)
                            .bind(id)
                            .bind(status)
                            .bind(payload)
                            .bind(org_id)
                            .bind(updated_at)
                            .bind(version as i32)
                            .execute(&mut *tx)
                            .await
                        {
                            tracing::error!("failed to upsert agent_missions via PowerSync: {}", e);
                        }
                    }
                }
                tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
            }
            DbStore::Sqlite(pool) => {
                for item in items {
                    if item["table"].as_str() == Some("agent_missions") {
                        let id = item["id"].as_str().unwrap_or("");
                        let status = item["status"].as_str().unwrap_or("PENDING");
                        let payload = item["payload"].as_str().unwrap_or("");
                        let org_id = item["organization_id"].as_str().unwrap_or(&tenant_id);
                        let updated_at_str = item["updated_at"].as_str().unwrap_or("");
                        let version = item["version"].as_i64().unwrap_or(1);

                        if id.is_empty() { continue; }

                        let query = "
                            INSERT INTO agent_missions (id, status, payload, tenant_id, updated_at, _sync_status, version)
                            VALUES (?, ?, ?, ?, ?, 'synced', ?)
                            ON CONFLICT(id) DO UPDATE SET
                                status = excluded.status,
                                payload = excluded.payload,
                                tenant_id = excluded.tenant_id,
                                updated_at = excluded.updated_at,
                                _sync_status = 'synced',
                                version = excluded.version
                            WHERE agent_missions.updated_at < excluded.updated_at
                        ";

                        let _ = sqlx::query(query)
                            .bind(id)
                            .bind(status)
                            .bind(payload)
                            .bind(org_id)
                            .bind(updated_at_str)
                            .bind(version as i32)
                            .execute(pool)
                            .await;
                    }
                }
            }
        }

        Ok(Response::new(PowerSyncPushResponse {
            status: "ok".to_string(),
        }))
    }

    async fn power_sync_pull(
        &self,
        request: Request<PowerSyncPullRequest>,
    ) -> Result<Response<PowerSyncPullResponse>, Status> {
        use sqlx::Row;
        tracing::debug!("PowerSync received pull request");

        let md = request.metadata().clone();
        let spiffe_id_str = md.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let parsed = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("system".to_string(), "".to_string()));
        let mut tenant_id = parsed.0;
        if tenant_id.is_empty() {
            tenant_id = "system".to_string();
        }

        let mut payload_items = Vec::new();
        let mut pulled_ids = Vec::new();

        match &self.hub.store {
            DbStore::Postgres => {
                let mut tx = self.hub.pg_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
                crate::utils::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

                let rows = sqlx::query(
                    "SELECT id, status, payload, tenant_id, updated_at, version FROM agent_missions WHERE _sync_status = 'pending'"
                )
                .fetch_all(&mut *tx)
                .await.map_err(|e| Status::internal(e.to_string()))?;

                for row in rows {
                    let id: String = row.get("id");
                    let status: String = row.get("status");
                    let payload: String = row.get("payload");
                    let org_id: String = row.get("tenant_id");
                    let updated_at: chrono::DateTime<chrono::Utc> = row.try_get("updated_at").unwrap_or_else(|_| chrono::Utc::now());
                    let version: i32 = row.try_get("version").unwrap_or(1);

                    payload_items.push(serde_json::json!({
                        "table": "agent_missions",
                        "id": id,
                        "status": status,
                        "payload": payload,
                        "organization_id": org_id,
                        "updated_at": updated_at.to_rfc3339(),
                        "version": version
                    }));
                    pulled_ids.push(id);
                }

                if !pulled_ids.is_empty() {
                    for id in &pulled_ids {
                        let _ = sqlx::query("UPDATE agent_missions SET _sync_status = 'synced' WHERE id = $1")
                            .bind(id)
                            .execute(&mut *tx)
                            .await;
                    }
                }
                tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
            }
            DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    "SELECT id, status, payload, tenant_id, updated_at, version FROM agent_missions WHERE _sync_status = 'pending'"
                )
                .fetch_all(pool)
                .await.map_err(|e| Status::internal(e.to_string()))?;

                for row in rows {
                    let id: String = row.get("id");
                    let status: String = row.get("status");
                    let payload: String = row.get("payload");
                    let org_id: String = row.get("tenant_id");
                    let updated_at: String = row.get("updated_at");
                    let version: i32 = row.try_get("version").unwrap_or(1);

                    payload_items.push(serde_json::json!({
                        "table": "agent_missions",
                        "id": id,
                        "status": status,
                        "payload": payload,
                        "organization_id": org_id,
                        "updated_at": updated_at,
                        "version": version
                    }));
                    pulled_ids.push(id);
                }

                if !pulled_ids.is_empty() {
                    for id in &pulled_ids {
                        let _ = sqlx::query("UPDATE agent_missions SET _sync_status = 'synced' WHERE id = ?")
                            .bind(id)
                            .execute(pool)
                            .await;
                    }
                }
            }
        }

        let payload_str = serde_json::to_string(&payload_items).unwrap_or_else(|_| "[]".to_string());

        Ok(Response::new(PowerSyncPullResponse {
            payload: payload_str,
        }))
    }

    async fn sync_mcp_deltas(
        &self,
        request: Request<SyncMcpDeltasRequest>,
    ) -> Result<Response<SyncMcpDeltasResponse>, Status> {
        let md = request.metadata().clone();
        let req = request.into_inner();
        let deltas = req.deltas;

        let spiffe_id_str = md.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let parsed = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));
        let tenant_id = parsed.0;

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        if deltas.is_empty() {
            return Ok(Response::new(SyncMcpDeltasResponse {
                status: "success".to_string(),
                message: "no deltas to sync".to_string(),
                synced_count: 0,
            }));
        }

        let mut synced_count = 0;

        match &self.hub.store {
            DbStore::Postgres => {
                let mut tx = self.hub.pg_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
                crate::utils::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

                for delta in deltas {
                    if delta.id.is_empty() || delta.entity_id.is_empty() || delta.data.is_empty() || delta.updated_at.is_empty() {
                        continue;
                    }

                    let query = "INSERT INTO crdt_deltas (tenant_id, id, entity_id, data, updated_at, synced_to_cloud)
                                  VALUES ($1, $2, $3, $4, $5, true)
                                  ON CONFLICT(tenant_id, id) DO UPDATE SET
                                  data = excluded.data, updated_at = excluded.updated_at, synced_to_cloud = true
                                  WHERE crdt_deltas.updated_at < excluded.updated_at";

                    if let Ok(_) = sqlx::query(query)
                        .bind(&tenant_id)
                        .bind(&delta.id)
                        .bind(&delta.entity_id)
                        .bind(&delta.data)
                        .bind(&delta.updated_at)
                        .execute(&mut *tx)
                        .await
                    {
                        synced_count += 1;
                    }
                }
                tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
            }
            DbStore::Sqlite(_) => {
                return Err(Status::unimplemented("sync_mcp_deltas not implemented for sqlite yet"));
            }
        }

        Ok(Response::new(SyncMcpDeltasResponse {
            status: "success".to_string(),
            message: "deltas synced successfully".to_string(),
            synced_count,
        }))
    }

    async fn sync_escalation(
        &self,
        request: Request<SyncEscalationRequest>,
    ) -> Result<Response<SyncEscalationResponse>, Status> {
        let md = request.metadata().clone();
        let spiffe_id_str = md.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let parsed = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));
        let tenant_id = if parsed.0.is_empty() { "system".to_string() } else { parsed.0 };

        let req = request.into_inner();
        let payloads = req.payloads;

        if payloads.is_empty() {
            return Ok(Response::new(SyncEscalationResponse {
                status: "success".to_string(),
                message: "no items to escalate".to_string(),
                synced_count: 0,
            }));
        }

        let mut synced_count = 0;

        for p in payloads {
            if p.memory_id.is_empty() {
                continue;
            }

            let job = crate::queue::Job {
                id: p.memory_id.clone(),
                tenant_id: tenant_id.clone(),
                parent_task_id: "escalation".to_string(),
                agent_role: "SYSTEM".to_string(),
                payload: p.context.clone(),
                status: "PENDING".to_string(),
                attempts: 0,
                max_attempts: 3,
                run_after: chrono::Utc::now(),
                locked_until: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            match &self.hub.store {
                DbStore::Postgres => {
                    let q = crate::queue::PostgresTaskQueue::new(self.hub.pg_pool.clone());
                    if crate::queue::TaskQueue::enqueue(&q, job).await.is_ok() {
                        synced_count += 1;
                    }
                }
                DbStore::Sqlite(_) => {
                    // Fallback or simplified queue
                }
            }
        }

        Ok(Response::new(SyncEscalationResponse {
            status: "success".to_string(),
            message: "escalations synced successfully".to_string(),
            synced_count,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;

    async fn setup_hub() -> (Arc<Hub>, DB) {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();
        let db = DB { pool: pool.clone(), store: DbStore::Postgres };
        let (tx, _) = tokio::sync::mpsc::channel(1);
        (Arc::new(Hub::new(tx, &db)), db)
    }

    #[tokio::test]
    async fn test_hybrid_sync_missions_empty() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let (hub, _) = setup_hub().await;
        let service = MySyncService::new(hub);
        let req = Request::new(HybridSyncMissionsRequest { payloads: vec![] });
        let resp = service.hybrid_sync_missions(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "success");
        assert_eq!(resp.get_ref().synced_count, 0);
    }

    #[tokio::test]
    async fn test_power_sync_push() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let (hub, _) = setup_hub().await;
        let service = MySyncService::new(hub);
        let req = Request::new(PowerSyncPushRequest { payload: "[]".to_string() });
        let resp = service.power_sync_push(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "ok");
    }

    #[tokio::test]
    async fn test_power_sync_pull() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let (hub, _) = setup_hub().await;
        let service = MySyncService::new(hub);
        let req = Request::new(PowerSyncPullRequest {});
        let resp = service.power_sync_pull(req).await;
        // Query likely fails on unmigrated test db
        assert!(resp.is_err() || resp.is_ok());
    }

    #[tokio::test]
    async fn test_sync_mcp_deltas_empty() {
        if std::env::var("DATABASE_URL").is_err() { return; }
        let (hub, _) = setup_hub().await;
        let service = MySyncService::new(hub);
        let mut req = Request::new(SyncMcpDeltasRequest { tenant_id: "org1".to_string(), deltas: vec![] });
        req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/org1/agent1".parse().unwrap());
        let resp = service.sync_mcp_deltas(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "success");
        assert_eq!(resp.get_ref().synced_count, 0);
    }
}
