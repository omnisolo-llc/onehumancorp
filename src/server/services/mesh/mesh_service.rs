use ::server_ohc::mesh::unified_mesh_service_server::UnifiedMeshService;
use ::server_ohc::mesh::{ReserveResourceRequest, ReserveResourceResponse, CommitReservationRequest, CommitReservationResponse};

use tonic::{Request, Response, Status};

pub struct MyMeshService {
    redis_client: Option<redis::Client>,
}

impl MyMeshService {
    pub fn new(redis_client: Option<redis::Client>) -> Self {
        Self { redis_client }
    }
}

#[tonic::async_trait]
impl UnifiedMeshService for MyMeshService {
    async fn reserve_resource(
        &self,
        request: Request<ReserveResourceRequest>,
    ) -> Result<Response<ReserveResourceResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();

        let lock_id = uuid::Uuid::new_v4().to_string();

        if let Some(client) = &self.redis_client {
            let mut con = client.get_async_connection().await.map_err(|e| Status::internal(e.to_string()))?;
            let key = format!("mesh:{}:{}:hold", tenant_id, req.resource_id);

            // Using redis SET NX EX to try to acquire the hold
            let success: bool = redis::cmd("SET")
                .arg(&key)
                .arg(&lock_id)
                .arg("NX")
                .arg("EX")
                .arg(req.ttl_seconds)
                .query_async(&mut con)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            if success {
                Ok(Response::new(ReserveResourceResponse {
                    success: true,
                    lock_id,
                    error_message: "".to_string(),
                }))
            } else {
                Ok(Response::new(ReserveResourceResponse {
                    success: false,
                    lock_id: "".to_string(),
                    error_message: "Resource already reserved".to_string(),
                }))
            }
        } else {
            Err(Status::internal("Redis client not available"))
        }
    }

    async fn commit_reservation(
        &self,
        request: Request<CommitReservationRequest>,
    ) -> Result<Response<CommitReservationResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();

        if let Some(client) = &self.redis_client {
            let mut con = client.get_async_connection().await.map_err(|e| Status::internal(e.to_string()))?;
            let key = format!("mesh:{}:{}:hold", tenant_id, req.resource_id);

            // Fetch current lock
            let current_lock_id: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut con).await.unwrap_or(None);

            if let Some(current_lock_id) = current_lock_id {
                if current_lock_id == req.lock_id {
                    let _deleted: bool = redis::cmd("DEL").arg(&key).query_async(&mut con).await.unwrap_or(false);
                    // At this point we would commit to postgres
                    // and mark the hold as confirmed

                    return Ok(Response::new(CommitReservationResponse {
                        success: true,
                        error_message: "".to_string(),
                    }));
                }
            }

            Ok(Response::new(CommitReservationResponse {
                success: false,
                error_message: "Invalid or expired lock ID".to_string(),
            }))

        } else {
            Err(Status::internal("Redis client not available"))
        }
    }
}
