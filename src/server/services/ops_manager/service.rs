use ::server_ohc::ops_manager::operations_manager_service_server::OperationsManagerService;
use ::server_ohc::ops_manager::{
    CreateEntityRequest, DeleteEntityRequest, Entity, EmptyResponse, GetEntityRequest,
    ListEntitiesRequest, ListEntitiesResponse, UpdateEntityRequest,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct MyOperationsManagerService {
    pool: sqlx::PgPool,
}

impl MyOperationsManagerService {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl OperationsManagerService for MyOperationsManagerService {
    async fn create_entity(
        &self,
        request: Request<CreateEntityRequest>,
    ) -> Result<Response<Entity>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let req = request.into_inner();

        // Use a mock ID for now as we simulate the CRUD operations without a specific schema.
        let id = format!("{}-{}", req.r#type, chrono::Utc::now().timestamp_millis());

        Ok(Response::new(Entity {
            id,
            tenant_id,
            r#type: req.r#type,
            payload_json: req.payload_json,
        }))
    }

    async fn get_entity(
        &self,
        request: Request<GetEntityRequest>,
    ) -> Result<Response<Entity>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let req = request.into_inner();

        Ok(Response::new(Entity {
            id: req.id,
            tenant_id,
            r#type: "mock".to_string(),
            payload_json: "{}".to_string(),
        }))
    }

    async fn update_entity(
        &self,
        request: Request<UpdateEntityRequest>,
    ) -> Result<Response<Entity>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let req = request.into_inner();

        Ok(Response::new(Entity {
            id: req.id,
            tenant_id,
            r#type: req.r#type,
            payload_json: req.payload_json,
        }))
    }

    async fn delete_entity(
        &self,
        request: Request<DeleteEntityRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (_tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let _req = request.into_inner();

        Ok(Response::new(EmptyResponse {}))
    }

    async fn list_entities(
        &self,
        request: Request<ListEntitiesRequest>,
    ) -> Result<Response<ListEntitiesResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let req = request.into_inner();

        let mock_entity = Entity {
            id: format!("{}-1", req.r#type),
            tenant_id,
            r#type: req.r#type,
            payload_json: "{}".to_string(),
        };

        Ok(Response::new(ListEntitiesResponse {
            entities: vec![mock_entity],
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tonic::Request;
    use tonic::metadata::MetadataValue;

    async fn setup_test_service() -> MyOperationsManagerService {
        let database_url = "sqlite::memory:";
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(1))
            .connect(database_url).await.unwrap();

        // NOTE: We need a PgPool for the actual struct, but for simple tests without DB calls
        // any initialized connection (or mocked) works if we don't query it. Since we're using
        // mock logic in this implementation, we will connect to a dummy postgres string
        // but it won't be queried.
        let pg_pool = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();

        MyOperationsManagerService::new(pg_pool)
    }

    fn create_mock_request<T>(req: T) -> Request<T> {
        let mut request = Request::new(req);
        request.metadata_mut().insert(
            "x-spiffe-id",
            MetadataValue::try_from("spiffe://onehumancorp.com/tenant/tenant123/ops").unwrap(),
        );
        request
    }

    #[tokio::test]
    async fn test_create_entity() {
        let service = setup_test_service().await;
        let req = CreateEntityRequest {
            r#type: "inventory".to_string(),
            payload_json: "{\"stock\": 10}".to_string(),
        };

        let res = service.create_entity(create_mock_request(req)).await.unwrap().into_inner();
        assert_eq!(res.tenant_id, "tenant123");
        assert_eq!(res.r#type, "inventory");
        assert_eq!(res.payload_json, "{\"stock\": 10}");
        assert!(res.id.starts_with("inventory-"));
    }

    #[tokio::test]
    async fn test_get_entity() {
        let service = setup_test_service().await;
        let req = GetEntityRequest {
            id: "inventory-1".to_string(),
        };

        let res = service.get_entity(create_mock_request(req)).await.unwrap().into_inner();
        assert_eq!(res.id, "inventory-1");
        assert_eq!(res.tenant_id, "tenant123");
    }

    #[tokio::test]
    async fn test_update_entity() {
        let service = setup_test_service().await;
        let req = UpdateEntityRequest {
            id: "inventory-1".to_string(),
            r#type: "inventory".to_string(),
            payload_json: "{\"stock\": 5}".to_string(),
        };

        let res = service.update_entity(create_mock_request(req)).await.unwrap().into_inner();
        assert_eq!(res.id, "inventory-1");
        assert_eq!(res.tenant_id, "tenant123");
        assert_eq!(res.payload_json, "{\"stock\": 5}");
    }

    #[tokio::test]
    async fn test_delete_entity() {
        let service = setup_test_service().await;
        let req = DeleteEntityRequest {
            id: "inventory-1".to_string(),
        };

        let res = service.delete_entity(create_mock_request(req)).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_list_entities() {
        let service = setup_test_service().await;
        let req = ListEntitiesRequest {
            r#type: "order".to_string(),
        };

        let res = service.list_entities(create_mock_request(req)).await.unwrap().into_inner();
        assert_eq!(res.entities.len(), 1);
        assert_eq!(res.entities[0].tenant_id, "tenant123");
        assert_eq!(res.entities[0].r#type, "order");
    }
}
