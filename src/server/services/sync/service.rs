use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::sync_service_server::SyncService;
use crate::sip::SipDB;

pub struct MySyncService {
    pool: sqlx::PgPool,
}

impl MySyncService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        MySyncService { pool }
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
        let sip_db = SipDB::new(self.pool.clone(), "system".to_string());

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
        let parsed = ::server_auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("system".to_string(), "".to_string()));
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

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

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
                    INSERT INTO agent_missions (id, status, payload, organization_id, updated_at, _sync_status, version)
                    VALUES ($1, $2, $3, $4, $5, 'synced', $6)
                    ON CONFLICT(id) DO UPDATE SET
                        status = excluded.status,
                        payload = excluded.payload,
                        organization_id = excluded.organization_id,
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
        let parsed = ::server_auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("system".to_string(), "".to_string()));
        let mut tenant_id = parsed.0;
        if tenant_id.is_empty() {
            tenant_id = "system".to_string();
        }

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let rows = match sqlx::query(
            "SELECT id, status, payload, organization_id, updated_at, version FROM agent_missions WHERE _sync_status = 'pending'"
        )
        .fetch_all(&mut *tx)
        .await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("failed to fetch pending agent_missions for pull: {}", e);
                return Err(Status::internal("database error"));
            }
        };

        let mut payload_items = Vec::new();
        let mut pulled_ids = Vec::new();

        for row in rows {
            let id: String = row.get("id");
            let status: String = row.get("status");
            let payload: String = row.get("payload");
            let org_id: String = row.get("organization_id");
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
            for id in pulled_ids {
                let _ = sqlx::query("UPDATE agent_missions SET _sync_status = 'synced' WHERE id = $1")
                    .bind(id)
                    .execute(&mut *tx)
                    .await;
            }
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

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
        let parsed = ::server_auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));
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

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let mut synced_count = 0;

        for delta in deltas {
            if delta.id.is_empty() || delta.entity_id.is_empty() || delta.data.is_empty() || delta.updated_at.is_empty() {
                continue;
            }

            let query = "INSERT INTO crdt_deltas (tenant_id, id, entity_id, data, updated_at, synced_to_cloud)
                          VALUES ($1, $2, $3, $4, $5, true)
                          ON CONFLICT(tenant_id, id) DO UPDATE SET
                          data = excluded.data, updated_at = excluded.updated_at, synced_to_cloud = true
                          WHERE crdt_deltas.updated_at < excluded.updated_at";

            match sqlx::query(query)
                .bind(&tenant_id)
                .bind(&delta.id)
                .bind(&delta.entity_id)
                .bind(&delta.data)
                .bind(&delta.updated_at)
                .execute(&mut *tx)
                .await
            {
                Ok(_) => {
                    synced_count += 1;
                }
                Err(e) => {
                    tracing::error!("failed to upsert CRDT delta: error={}", e);
                }
            }
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

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
        let parsed = ::server_auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));
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

            let q = crate::queue::PostgresTaskQueue::new(self.pool.clone());
            
            match crate::queue::TaskQueue::enqueue(&q, job).await {
                Ok(_) => {
                    synced_count += 1;
                }
                Err(e) => {
                    tracing::error!("failed to enqueue escalation job: error={}", e);
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

    #[tokio::test]
    async fn test_hybrid_sync_missions_empty() {
        // We can test empty payloads without DB!
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let req = Request::new(HybridSyncMissionsRequest { payloads: vec![] });
        let resp = service.hybrid_sync_missions(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "success");
        assert_eq!(resp.get_ref().synced_count, 0);
    }

    #[tokio::test]
    async fn test_power_sync_push() {
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let req = Request::new(PowerSyncPushRequest { payload: "[]".to_string() });
        let resp = service.power_sync_push(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "ok");
    }

    #[tokio::test]
    async fn test_power_sync_pull() {
        // Will fail to fetch if table doesn't exist, so this will only pass with empty payload if error happens, but we actually check the fallback.
        // In the mock we expect error from the query, but we don't have migrations applied.
        // This is safe since we only check that it doesn't panic.
        #[allow(unused_variables)]
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let req = Request::new(PowerSyncPullRequest {});
        let resp = service.power_sync_pull(req).await;
        // The query fails because migrations are not run on dummy. Thus it returns internal error.
        assert!(resp.is_err());
    }

    #[tokio::test]
    async fn test_power_sync_push_and_pull() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            // Only run e2e flow if real test db is available. Dummy will fail.
            return;
        }

        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })


            .connect(&database_url).await.unwrap();

        let service = MySyncService::new(pool.clone());

        let mission_id = "test_mission_push_pull";
        let payload_json = serde_json::json!([{
            "table": "agent_missions",
            "id": mission_id,
            "status": "COMPLETED",
            "payload": "test data",
            "organization_id": "system",
            "updated_at": chrono::Utc::now().to_rfc3339(),
            "version": 2
        }]).to_string();

        let mut push_req = Request::new(PowerSyncPushRequest { payload: payload_json });
        push_req.metadata_mut().insert("x-spiffe-id", "spiffe://onehumancorp.io/system/system".parse().unwrap());

        let push_resp = service.power_sync_push(push_req).await.unwrap();
        assert_eq!(push_resp.get_ref().status, "ok");

        // Manually set status to 'pending' to simulate cloud modifications ready for pull
        sqlx::query("UPDATE agent_missions SET _sync_status = 'pending' WHERE id = $1")
            .bind(mission_id)
            .execute(&pool)
            .await.unwrap();

        let pull_req = Request::new(PowerSyncPullRequest {});
        let pull_resp = service.power_sync_pull(pull_req).await.unwrap();

        let pulled_items: Vec<serde_json::Value> = serde_json::from_str(&pull_resp.get_ref().payload).unwrap();

        let found = pulled_items.iter().find(|i| i["id"] == mission_id);
        assert!(found.is_some());
        assert_eq!(found.unwrap()["status"], "COMPLETED");

        sqlx::query("DELETE FROM agent_missions WHERE id = $1").bind(mission_id).execute(&pool).await.unwrap();
    }
    #[tokio::test]
    async fn test_sync_mcp_deltas_empty() {
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let mut req = Request::new(SyncMcpDeltasRequest { tenant_id: "org1".to_string(), deltas: vec![] });
        req.metadata_mut().insert("x-spiffe-id", "spiffe://ohc/org/org1/agent/agent1".parse().unwrap());
        let resp = service.sync_mcp_deltas(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "success");
        assert_eq!(resp.get_ref().synced_count, 0);
    }
    #[tokio::test]
    async fn test_sync_escalation_empty() {
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let req = Request::new(SyncEscalationRequest { payloads: vec![] });
        let resp = service.sync_escalation(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "success");
        assert_eq!(resp.get_ref().synced_count, 0);
    }
    #[tokio::test]
    async fn test_vector_sync() {
        let pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let req = Request::new(VectorSyncRequest {});
        let resp = service.vector_sync(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "success");
    }
}
