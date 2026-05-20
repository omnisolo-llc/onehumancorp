use tonic::{Request, Response, Status};
use ::server_ohc::organization::*;
use ::server_ohc::organization::catalog_service_server::CatalogService;
use std::sync::Arc;
use sqlx::Row;
use uuid::Uuid;

pub struct MyCatalogService {
    db: Arc<crate::db::DB>,
    hub: Arc<crate::hub::Hub>,
}

impl MyCatalogService {
    pub fn new(db: Arc<crate::db::DB>, hub: Arc<crate::hub::Hub>) -> Self {
        Self { db, hub }
    }
}

#[tonic::async_trait]
impl CatalogService for MyCatalogService {
    async fn create_catalog_item(
        &self,
        request: Request<CreateCatalogItemRequest>,
    ) -> Result<Response<CatalogItem>, Status> {
        let auth_info = request
            .extensions()
            .get::<::server_auth::orchestration::AuthInfo>()
            .cloned()
            .ok_or_else(|| Status::unauthenticated("Missing authentication information"))?;

        let req = request.into_inner();
        let mut item = req.item.ok_or_else(|| Status::invalid_argument("item is required"))?;

        if item.id.is_empty() {
            item.id = format!("cat-{}", Uuid::new_v4());
        }

        let tenant_id = &auth_info.org_id;
        if tenant_id.is_empty() {
            return Err(Status::permission_denied("Only tenants can create catalog items"));
        }

        let item_type_str = match CatalogType::try_from(item.r#type).unwrap_or(CatalogType::Physical) {
            CatalogType::Physical => "physical",
            CatalogType::Digital => "digital",
            CatalogType::Service => "service",
        };

        let metadata = if item.metadata_json.is_empty() {
            serde_json::json!({})
        } else {
            serde_json::from_str(&item.metadata_json)
                .map_err(|e| Status::invalid_argument(format!("Invalid metadata_json: {}", e)))?
        };

        let mut tx = self.db.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO catalog_items (id, tenant_id, organization_id, name, description, type, price_cents, currency, duration_minutes, metadata) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
        )
        .bind(&item.id)
        .bind(tenant_id)
        .bind(&item.organization_id)
        .bind(&item.name)
        .bind(&item.description)
        .bind(item_type_str)
        .bind(item.price_cents)
        .bind(&item.currency)
        .bind(item.duration_minutes)
        .bind(metadata)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(item))
    }

    async fn list_catalog_items(
        &self,
        request: Request<ListCatalogItemsRequest>,
    ) -> Result<Response<ListCatalogItemsResponse>, Status> {
        let auth_info = request
            .extensions()
            .get::<::server_auth::orchestration::AuthInfo>()
            .cloned()
            .ok_or_else(|| Status::unauthenticated("Missing authentication information"))?;

        let req = request.into_inner();
        let tenant_id = &auth_info.org_id;

        if tenant_id.is_empty() {
            return Err(Status::permission_denied("Only tenants can list catalog items"));
        }

        let mut tx = self.db.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let rows = sqlx::query(
            "SELECT id, organization_id, name, description, type, price_cents, currency, duration_minutes, metadata FROM catalog_items WHERE tenant_id = $1"
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let items = rows.into_iter().map(|r| {
            let type_str: String = r.get("type");
            let type_enum = match type_str.as_str() {
                "physical" => CatalogType::Physical,
                "digital" => CatalogType::Digital,
                "service" => CatalogType::Service,
                _ => CatalogType::Physical,
            };

            let metadata: serde_json::Value = r.get("metadata");

            CatalogItem {
                id: r.get("id"),
                organization_id: r.get("organization_id"),
                name: r.get("name"),
                description: r.get("description"),
                price_cents: r.get("price_cents"),
                currency: r.get("currency"),
                r#type: type_enum as i32,
                duration_minutes: r.get::<Option<i32>, _>("duration_minutes").unwrap_or_default(),
                metadata_json: metadata.to_string(),
            }
        }).collect();

        Ok(Response::new(ListCatalogItemsResponse { items }))
    }

    async fn suggest_description(
        &self,
        request: Request<SuggestDescriptionRequest>,
    ) -> Result<Response<SuggestDescriptionResponse>, Status> {
        let req = request.into_inner();

        let api_key = self.hub.minimax_api_key().to_string();
        if api_key.is_empty() {
            return Ok(Response::new(SuggestDescriptionResponse {
                description: format!("A premium {} designed for quality and value.", req.name),
            }));
        }

        let type_str = match CatalogType::try_from(req.r#type).unwrap_or(CatalogType::Physical) {
            CatalogType::Physical => "physical product",
            CatalogType::Digital => "digital download",
            CatalogType::Service => "service",
        };

        let prompt = format!(
            "Write a concise, professional, and sales-optimized description (max 2 sentences) for a {} named '{}'.",
            type_str, req.name
        );

        let client = crate::minimax::MinimaxClient::new(api_key);
        match client.reason(&prompt).await {
            Ok(description) => Ok(Response::new(SuggestDescriptionResponse { description })),
            Err(_) => Ok(Response::new(SuggestDescriptionResponse {
                description: format!("A premium {} designed for quality and value.", req.name),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use ::server_auth::orchestration::AuthInfo;
    use std::sync::Arc;

    async fn setup_test_catalog_service() -> MyCatalogService {
        let database_url = "sqlite::memory:";
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(database_url).await.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS catalog_items (id TEXT, tenant_id TEXT, organization_id TEXT, name TEXT, description TEXT, type TEXT, price_cents INTEGER, currency TEXT, duration_minutes INTEGER, metadata JSONB)").execute(&pool).await.unwrap();

        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db = Arc::new(crate::db::DB { pool: pg_pool, store: crate::db::DbStore::Sqlite(pool.clone()) });

        let (tx, _) = tokio::sync::mpsc::channel(10);
        let hub = Arc::new(crate::hub::Hub::new(tx, db.pool.clone()));

        MyCatalogService::new(db, hub)
    }

    #[tokio::test]
    async fn test_create_and_list_catalog_items() {
        let service = setup_test_catalog_service().await;
        let org_id = "test-org".to_string();

        let mut req_create = Request::new(CreateCatalogItemRequest {
            item: Some(CatalogItem {
                id: "item-1".to_string(),
                organization_id: org_id.clone(),
                name: "Test Service".to_string(),
                description: "Description".to_string(),
                price_cents: 5000,
                currency: "USD".to_string(),
                r#type: CatalogType::Service as i32,
                duration_minutes: 60,
                metadata_json: "{}".to_string(),
            })
        });
        req_create.extensions_mut().insert(AuthInfo {
            spiffe_id: "test-user".to_string(),
            org_id: org_id.clone(),
            agent_id: "test-agent".to_string(),
        });

        let res_create = service.create_catalog_item(req_create).await.unwrap().into_inner();
        assert_eq!(res_create.name, "Test Service");

        let mut req_list = Request::new(ListCatalogItemsRequest {
            organization_id: org_id.clone(),
        });
        req_list.extensions_mut().insert(AuthInfo {
            spiffe_id: "test-user".to_string(),
            org_id: org_id.clone(),
            agent_id: "test-agent".to_string(),
        });

        let res_list = service.list_catalog_items(req_list).await.unwrap().into_inner();
        assert_eq!(res_list.items.len(), 1);
        assert_eq!(res_list.items[0].name, "Test Service");
        assert_eq!(res_list.items[0].r#type, CatalogType::Service as i32);
        assert_eq!(res_list.items[0].duration_minutes, 60);
    }
}
