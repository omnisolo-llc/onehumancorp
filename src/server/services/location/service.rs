use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::location_service_server::LocationService;
use std::sync::Arc;
use crate::hub::Hub;
use sqlx::Row;
use uuid::Uuid;

pub struct MyLocationService {
    hub: Arc<Hub>,
}

impl MyLocationService {
    pub fn new(hub: Arc<Hub>) -> Self {
        Self { hub }
    }
}

#[tonic::async_trait]
impl LocationService for MyLocationService {
    #[tracing::instrument(skip(self, request))]
    async fn get_locations(
        &self,
        request: Request<GetLocationsRequest>,
    ) -> Result<Response<GetLocationsResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };

        let rows = sqlx::query("SELECT id, tenant_id, name, extract(epoch from created_at) as created_at_unix, extract(epoch from updated_at) as updated_at_unix FROM locations WHERE tenant_id = $1")
            .bind(&org_id)
            .fetch_all(&self.hub.pool)
            .await
            .map_err(|e| Status::internal(format!("db error: {}", e)))?;

        let locations = rows.into_iter().map(|row| LocationProto {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            name: row.get("name"),
            created_at_unix: row.get::<Option<f64>, _>("created_at_unix").unwrap_or(0.0) as i64,
            updated_at_unix: row.get::<Option<f64>, _>("updated_at_unix").unwrap_or(0.0) as i64,
        }).collect();

        Ok(Response::new(GetLocationsResponse { locations }))
    }

    #[tracing::instrument(skip(self, request))]
    async fn get_role_assignments(
        &self,
        request: Request<GetRoleAssignmentsRequest>,
    ) -> Result<Response<GetRoleAssignmentsResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };

        let rows = sqlx::query("SELECT id, tenant_id, user_id, location_id, role, extract(epoch from created_at) as created_at_unix, extract(epoch from updated_at) as updated_at_unix FROM role_assignments WHERE tenant_id = $1")
            .bind(&org_id)
            .fetch_all(&self.hub.pool)
            .await
            .map_err(|e| Status::internal(format!("db error: {}", e)))?;

        let assignments = rows.into_iter().map(|row| RoleAssignmentProto {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            user_id: row.get("user_id"),
            location_id: row.get("location_id"),
            role: row.get("role"),
            created_at_unix: row.get::<Option<f64>, _>("created_at_unix").unwrap_or(0.0) as i64,
            updated_at_unix: row.get::<Option<f64>, _>("updated_at_unix").unwrap_or(0.0) as i64,
        }).collect();

        Ok(Response::new(GetRoleAssignmentsResponse { assignments }))
    }

    #[tracing::instrument(skip(self, request))]
    async fn create_escalation(
        &self,
        request: Request<CreateEscalationRequest>,
    ) -> Result<Response<CreateEscalationResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, user_id) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };

        let req = request.into_inner();
        let id = Uuid::new_v4().to_string();

        let row = sqlx::query(
            "INSERT INTO escalations (id, tenant_id, location_id, task_id, summary, status, created_by)
             VALUES ($1, $2, $3, $4, $5, 'PENDING', $6)
             RETURNING id, tenant_id, location_id, task_id, summary, status, created_by, extract(epoch from created_at) as created_at_unix, extract(epoch from updated_at) as updated_at_unix"
        )
        .bind(&id)
        .bind(&org_id)
        .bind(&req.location_id)
        .bind(if req.task_id.is_empty() { None } else { Some(req.task_id) })
        .bind(&req.summary)
        .bind(&user_id)
        .fetch_one(&self.hub.pool)
        .await
        .map_err(|e| Status::internal(format!("db error: {}", e)))?;

        let escalation = EscalationProto {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            location_id: row.get("location_id"),
            task_id: row.get::<Option<String>, _>("task_id").unwrap_or_default(),
            summary: row.get("summary"),
            status: row.get("status"),
            created_by: row.get("created_by"),
            created_at_unix: row.get::<Option<f64>, _>("created_at_unix").unwrap_or(0.0) as i64,
            updated_at_unix: row.get::<Option<f64>, _>("updated_at_unix").unwrap_or(0.0) as i64,
        };

        Ok(Response::new(CreateEscalationResponse { escalation: Some(escalation) }))
    }

    #[tracing::instrument(skip(self, request))]
    async fn get_escalations(
        &self,
        request: Request<GetEscalationsRequest>,
    ) -> Result<Response<GetEscalationsResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };

        let req = request.into_inner();
        let query = if req.location_id.is_empty() {
            "SELECT id, tenant_id, location_id, task_id, summary, status, created_by, extract(epoch from created_at) as created_at_unix, extract(epoch from updated_at) as updated_at_unix FROM escalations WHERE tenant_id = $1"
        } else {
            "SELECT id, tenant_id, location_id, task_id, summary, status, created_by, extract(epoch from created_at) as created_at_unix, extract(epoch from updated_at) as updated_at_unix FROM escalations WHERE tenant_id = $1 AND location_id = $2"
        };

        let mut q = sqlx::query(query).bind(&org_id);
        if !req.location_id.is_empty() {
            q = q.bind(&req.location_id);
        }

        let rows = q.fetch_all(&self.hub.pool)
            .await
            .map_err(|e| Status::internal(format!("db error: {}", e)))?;

        let escalations = rows.into_iter().map(|row| EscalationProto {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            location_id: row.get("location_id"),
            task_id: row.get::<Option<String>, _>("task_id").unwrap_or_default(),
            summary: row.get("summary"),
            status: row.get("status"),
            created_by: row.get("created_by"),
            created_at_unix: row.get::<Option<f64>, _>("created_at_unix").unwrap_or(0.0) as i64,
            updated_at_unix: row.get::<Option<f64>, _>("updated_at_unix").unwrap_or(0.0) as i64,
        }).collect();

        Ok(Response::new(GetEscalationsResponse { escalations }))
    }

    #[tracing::instrument(skip(self, request))]
    async fn update_escalation_status(
        &self,
        request: Request<UpdateEscalationStatusRequest>,
    ) -> Result<Response<UpdateEscalationStatusResponse>, Status> {
        let spiffe_id_str = ::server_auth::extract_spiffe_id_from_metadata(request.metadata())
            .map_err(|e| Status::unauthenticated(e))?;
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(&spiffe_id_str)?;
        let org_id = if tenant_id.is_empty() { "system".to_string() } else { tenant_id };

        let req = request.into_inner();
        let row = sqlx::query(
            "UPDATE escalations SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3
             RETURNING id, tenant_id, location_id, task_id, summary, status, created_by, extract(epoch from created_at) as created_at_unix, extract(epoch from updated_at) as updated_at_unix"
        )
        .bind(&req.status)
        .bind(&req.escalation_id)
        .bind(&org_id)
        .fetch_one(&self.hub.pool)
        .await
        .map_err(|e| Status::internal(format!("db error: {}", e)))?;

        let escalation = EscalationProto {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            location_id: row.get("location_id"),
            task_id: row.get::<Option<String>, _>("task_id").unwrap_or_default(),
            summary: row.get("summary"),
            status: row.get("status"),
            created_by: row.get("created_by"),
            created_at_unix: row.get::<Option<f64>, _>("created_at_unix").unwrap_or(0.0) as i64,
            updated_at_unix: row.get::<Option<f64>, _>("updated_at_unix").unwrap_or(0.0) as i64,
        };

        Ok(Response::new(UpdateEscalationStatusResponse { escalation: Some(escalation) }))
    }
}
