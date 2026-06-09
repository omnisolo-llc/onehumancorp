use agent_feed::agent_feed_service_server::AgentFeedService;
use agent_feed::{
    AgentFeedItem, FeedItemState, GetFeedRequest, GetFeedResponse, UpdateFeedItemStateRequest,
    UpdateFeedItemStateResponse,
};
use chrono::Utc;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct MyAgentFeedService {
    db: Arc<crate::db::DB>,
}

impl MyAgentFeedService {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl AgentFeedService for MyAgentFeedService {
    async fn get_feed(
        &self,
        request: Request<GetFeedRequest>,
    ) -> Result<Response<GetFeedResponse>, Status> {
        let req = request.into_inner();
        let limit = if req.limit > 0 { req.limit } else { 20 };

        let pool = self.db.pool.clone();

        // Ensure tenant isolation
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Note: For simplicity, cursor pagination is mocked here as an offset or we just fetch limit items.
        // In a real implementation, we would use the cursor to fetch items created before the cursor.

        let rows = if req.cursor.is_empty() {
            sqlx::query(
                "SELECT id, tenant_id, trigger_event, context_payload, proposed_action, state, created_at, updated_at FROM agent_feed_items WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT $2"
            )
            .bind(&req.tenant_id)
            .bind(limit as i64)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
        } else {
            let cursor_time = chrono::DateTime::parse_from_rfc3339(&req.cursor).map_err(|e| Status::invalid_argument(e.to_string()))?.with_timezone(&Utc);
            sqlx::query(
                "SELECT id, tenant_id, trigger_event, context_payload, proposed_action, state, created_at, updated_at FROM agent_feed_items WHERE tenant_id = $1 AND created_at < $3 ORDER BY created_at DESC LIMIT $2"
            )
            .bind(&req.tenant_id)
            .bind(limit as i64)
            .bind(cursor_time)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
        };


            "SELECT id, tenant_id, trigger_event, context_payload, proposed_action, state, created_at, updated_at FROM agent_feed_items WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT $2"
        )
        .bind(&req.tenant_id)
        .bind(limit as i64)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let mut items = Vec::new();
        for row in rows {
            let state_str: String = row.get("state");
            let state = match state_str.as_str() {
                "PENDING" => FeedItemState::Pending,
                "EXECUTED" => FeedItemState::Executed,
                "DISMISSED" => FeedItemState::Dismissed,
                _ => FeedItemState::Unspecified,
            };

            let created_at: chrono::DateTime<Utc> = row.get("created_at");
            let updated_at: chrono::DateTime<Utc> = row.get("updated_at");

            let context_payload: serde_json::Value = row.get("context_payload");
            let proposed_action: serde_json::Value = row.get("proposed_action");

            items.push(AgentFeedItem {
                id: row.get::<uuid::Uuid, _>("id").to_string(),
                tenant_id: row.get("tenant_id"),
                trigger_event: row.get("trigger_event"),
                context_payload_json: context_payload.to_string(),
                proposed_action_json: proposed_action.to_string(),
                state: state.into(),
                created_at_unix: created_at.timestamp(),
                updated_at_unix: updated_at.timestamp(),
            });
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetFeedResponse {
            items,
            next_cursor: "".to_string(),
        }))
    }

    async fn update_feed_item_state(
        &self,
        request: Request<UpdateFeedItemStateRequest>,
    ) -> Result<Response<UpdateFeedItemStateResponse>, Status> {
        let req = request.into_inner();
        let pool = self.db.pool.clone();

        let state_str = match FeedItemState::try_from(req.new_state) {
            Ok(FeedItemState::Pending) => "PENDING",
            Ok(FeedItemState::Executed) => "EXECUTED",
            Ok(FeedItemState::Dismissed) => "DISMISSED",
            _ => "UNSPECIFIED",
        };

        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let id = Uuid::parse_str(&req.id).map_err(|e| Status::invalid_argument(e.to_string()))?;

        let row = sqlx::query(
            "UPDATE agent_feed_items SET state = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3 RETURNING id, tenant_id, trigger_event, context_payload, proposed_action, state, created_at, updated_at"
        )
        .bind(state_str)
        .bind(id)
        .bind(&req.tenant_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let updated_state_str: String = row.get("state");
        let updated_state = match updated_state_str.as_str() {
            "PENDING" => FeedItemState::Pending,
            "EXECUTED" => FeedItemState::Executed,
            "DISMISSED" => FeedItemState::Dismissed,
            _ => FeedItemState::Unspecified,
        };

        let created_at: chrono::DateTime<Utc> = row.get("created_at");
        let updated_at: chrono::DateTime<Utc> = row.get("updated_at");
        let context_payload: serde_json::Value = row.get("context_payload");
        let proposed_action: serde_json::Value = row.get("proposed_action");

        let item = AgentFeedItem {
            id: row.get::<uuid::Uuid, _>("id").to_string(),
            tenant_id: row.get("tenant_id"),
            trigger_event: row.get("trigger_event"),
            context_payload_json: context_payload.to_string(),
            proposed_action_json: proposed_action.to_string(),
            state: updated_state.into(),
            created_at_unix: created_at.timestamp(),
            updated_at_unix: updated_at.timestamp(),
        };

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(UpdateFeedItemStateResponse {
            item: Some(item),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;

    async fn setup_test_db() -> Arc<crate::db::DB> {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await
            .expect("Failed to connect to database");

        // We will run migrations in the test environment if needed
        // For unit tests, we'll assume the schema is present or we use an empty test DB

        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        Arc::new(crate::db::DB { pool, store: crate::db::DbStore::Sqlite(sqlite_pool) })
    }

    #[tokio::test]
    async fn test_agent_feed_service_crud() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            std::env::set_var("OHC_DATABASE_URL", "postgres://postgres:postgres@localhost:5432/ohc_test");
        }

        }

        let db = setup_test_db().await;
        let service = MyAgentFeedService::new(db.clone());
        let tenant_id = "test-tenant-123";

        // Pre-req: insert an item directly
        let mut tx = db.pool.begin().await.unwrap();
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .unwrap();

        // Clear existing for this tenant
        sqlx::query("DELETE FROM agent_feed_items WHERE tenant_id = $1")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .unwrap();

        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO agent_feed_items (id, tenant_id, trigger_event, context_payload, proposed_action, state) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(id)
        .bind(tenant_id)
        .bind("test_event")
        .bind(serde_json::json!({"foo": "bar"}))
        .bind(serde_json::json!({"action": "reply"}))
        .bind("PENDING")
        .execute(&mut *tx)
        .await
        .unwrap();
        tx.commit().await.unwrap();

        // 1. GetFeed
        let req = GetFeedRequest {
            limit: 10,
            cursor: "".to_string(),
            tenant_id: tenant_id.to_string(),
        };
            tenant_id: tenant_id.to_string(),
            limit: 10,
            cursor: "".to_string(),
        };
        let res = service.get_feed(Request::new(req)).await.unwrap().into_inner();
        assert_eq!(res.items.len(), 1);
        assert_eq!(res.items[0].trigger_event, "test_event");
        assert_eq!(res.items[0].state, FeedItemState::Pending as i32);

        // 2. UpdateFeedItemState
        let update_req = UpdateFeedItemStateRequest {
            tenant_id: tenant_id.to_string(),
            id: id.to_string(),
            new_state: FeedItemState::Executed as i32,
        };
        let update_res = service.update_feed_item_state(Request::new(update_req)).await.unwrap().into_inner();
        let updated_item = update_res.item.unwrap();
        assert_eq!(updated_item.state, FeedItemState::Executed as i32);

        // 3. Verify GetFeed returns updated state
        let req2 = GetFeedRequest {
            tenant_id: tenant_id.to_string(),
            limit: 10,
            cursor: "".to_string(),
        };
        let res2 = service.get_feed(Request::new(req2)).await.unwrap().into_inner();
        assert_eq!(res2.items[0].state, FeedItemState::Executed as i32);

        // 4. Verify tenant isolation (another tenant shouldn't see it)
        let req3 = GetFeedRequest {
            tenant_id: "other-tenant".to_string(),
            limit: 10,
            cursor: "".to_string(),
        };
        let res3 = service.get_feed(Request::new(req3)).await.unwrap().into_inner();
        assert_eq!(res3.items.len(), 0);
    }
}
