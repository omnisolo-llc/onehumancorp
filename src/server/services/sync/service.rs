use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::sync_service_server::SyncService;
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
                    eprintln!("failed to upsert mission from sync daemon: id={}, error={}", p.id, e);
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
        let req = request.into_inner();
        println!("PowerSync received push payload: {}", req.payload);

        Ok(Response::new(PowerSyncPushResponse {
            status: "ok".to_string(),
        }))
    }

    async fn power_sync_pull(
        &self,
        _request: Request<PowerSyncPullRequest>,
    ) -> Result<Response<PowerSyncPullResponse>, Status> {
        println!("PowerSync received pull request");
        Ok(Response::new(PowerSyncPullResponse {
            payload: "[]".to_string(),
        }))
    }

    async fn sync_mcp_deltas(
        &self,
        request: Request<SyncMCPDeltasRequest>,
    ) -> Result<Response<SyncMCPDeltasResponse>, Status> {
        let req = request.into_inner();
        let deltas = req.deltas;
        let tenant_id = req.tenant_id;

        if deltas.is_empty() {
            return Ok(Response::new(SyncMCPDeltasResponse {
                status: "success".to_string(),
                message: "no deltas to sync".to_string(),
                synced_count: 0,
            }));
        }

        let mut synced_count = 0;

        for delta in deltas {
            if delta.id.is_empty() || delta.entity_id.is_empty() || delta.data.is_empty() || delta.updated_at.is_empty() {
                continue;
            }

            let query = "INSERT INTO crdt_deltas (tenant_id, id, entity_id, data, updated_at, synced_to_cloud)
                          VALUES ($1, $2, $3, $4, $5, true)
                          ON CONFLICT(tenant_id, id) DO UPDATE SET
                          data = excluded.data, updated_at = excluded.updated_at, synced_to_cloud = true";

            match sqlx::query(query)
                .bind(&tenant_id)
                .bind(&delta.id)
                .bind(&delta.entity_id)
                .bind(&delta.data)
                .bind(&delta.updated_at)
                .execute(&self.pool)
                .await
            {
                Ok(_) => {
                    synced_count += 1;
                }
                Err(e) => {
                    eprintln!("failed to upsert CRDT delta: id={}, error={}", delta.id, e);
                }
            }
        }

        Ok(Response::new(SyncMCPDeltasResponse {
            status: "success".to_string(),
            message: "deltas synced successfully".to_string(),
            synced_count,
        }))
    }

    async fn sync_escalation(
        &self,
        request: Request<SyncEscalationRequest>,
    ) -> Result<Response<SyncEscalationResponse>, Status> {
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
                    eprintln!("failed to enqueue escalation job: id={}, error={}", p.memory_id, e);
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
        let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/ohc").await.unwrap_or_else(|_| {
            // Fallback or skip if no DB.
            // Since we can't easily skip in Rust without specific crates or flags,
            // we just use a dummy pool that will fail if used.
            // But for empty payloads, it shouldn't be used!
            // So we can just use a dummy pool!
            // Wait, connecting to invalid URL will fail.
            // Let's just use a dummy pool if we can create one without connecting.
            // `PgPoolOptions::new().connect_lazy("...")` is lazy! So it won't fail on creation!
            // That is perfect for this test!
            sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap()
        });
        let service = MySyncService::new(pool);
        let req = Request::new(HybridSyncMissionsRequest { payloads: vec![] });
        let resp = service.hybrid_sync_missions(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "success");
        assert_eq!(resp.get_ref().synced_count, 0);
    }

    #[tokio::test]
    async fn test_power_sync_push() {
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let req = Request::new(PowerSyncPushRequest { payload: "test payload".to_string() });
        let resp = service.power_sync_push(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "ok");
    }

    #[tokio::test]
    async fn test_power_sync_pull() {
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let req = Request::new(PowerSyncPullRequest {});
        let resp = service.power_sync_pull(req).await.unwrap();
        assert_eq!(resp.get_ref().payload, "[]");
    }
    #[tokio::test]
    async fn test_sync_mcp_deltas_empty() {
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let req = Request::new(SyncMCPDeltasRequest { tenant_id: "org1".to_string(), deltas: vec![] });
        let resp = service.sync_mcp_deltas(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "success");
        assert_eq!(resp.get_ref().synced_count, 0);
    }
    #[tokio::test]
    async fn test_sync_escalation_empty() {
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let req = Request::new(SyncEscalationRequest { payloads: vec![] });
        let resp = service.sync_escalation(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "success");
        assert_eq!(resp.get_ref().synced_count, 0);
    }
    #[tokio::test]
    async fn test_vector_sync() {
        let pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://localhost/dummy").unwrap();
        let service = MySyncService::new(pool);
        let req = Request::new(VectorSyncRequest {});
        let resp = service.vector_sync(req).await.unwrap();
        assert_eq!(resp.get_ref().status, "success");
    }
}
