use axum::{
    Json, Router,
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
struct TenantAuthority(String);

impl TenantAuthority {
    fn from_claims(claims: &::server_common::Claims) -> Result<Self, axum::http::StatusCode> {
        match claims.organization_id.as_deref() {
            Some(tenant_id) if !tenant_id.is_empty() => Ok(Self(tenant_id.to_string())),
            _ => Err(axum::http::StatusCode::UNAUTHORIZED),
        }
    }

    fn tenant_id(&self) -> &str {
        &self.0
    }
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new().route("/", post(create_inquiry))
}

#[derive(Deserialize)]
pub struct CreateInquiryRequest {
    pub raw_message: String,
    pub source: String,
}

#[derive(Serialize)]
pub struct CreateInquiryResponse {
    pub id: String,
}

async fn create_inquiry(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateInquiryRequest>,
) -> impl IntoResponse {
    let authority = match TenantAuthority::from_claims(&claims) {
        Ok(authority) => authority,
        Err(status) => return status.into_response(),
    };

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if let Err(e) =
        ::server_common::auth_utils::set_org_context(&mut *tx, authority.tenant_id()).await
    {
        tracing::error!("Failed to set org context: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let inquiry_id = Uuid::new_v4();
    if let Err(e) = sqlx::query(
        "INSERT INTO inquiries (id, tenant_id, source, raw_message, status) VALUES ($1, $2, $3, $4, 'NEW')"
    )
    .bind(inquiry_id)
    .bind(authority.tenant_id())
    .bind(&payload.source)
    .bind(&payload.raw_message)
    .execute(&mut *tx)
    .await
    {
        tracing::error!("Failed to insert inquiry: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let job_id = Uuid::new_v4().to_string();
    let job_payload = serde_json::json!({
        "inquiry_id": inquiry_id.to_string(),
        "raw_message": payload.raw_message,
    });

    if let Err(e) = sqlx::query(
        "INSERT INTO ohc_job_queue (id, parent_task_id, job_type, payload, status, next_retry_at, tenant_id)
         VALUES ($1, '', 'inquiry_intake_agent', $2, 'PENDING', CURRENT_TIMESTAMP, $3)"
    )
    .bind(job_id)
    .bind(&job_payload)
    .bind(authority.tenant_id())
    .execute(&mut *tx)
    .await
    {
        tracing::error!("Failed to enqueue inquiry intake job: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::CREATED, Json(CreateInquiryResponse { id: inquiry_id.to_string() })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_common::Claims;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_create_inquiry() {
        // Just making sure it compiles for now, integration tests handle full flows usually
    }
}
