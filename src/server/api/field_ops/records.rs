//! Canonical owner authority and durable, tenant-scoped field mutation receipts.
use axum::http::{HeaderMap, StatusCode};
use chrono::{DateTime, Utc};
use serde::{Serialize, de::DeserializeOwned};
use server_auth::commit_authority::{AuthorityError, AuthorizedPgOwner, CanonicalPgAuthority};
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, PgPool};
use std::sync::Arc;

pub type FieldError = (StatusCode, String);
pub fn unavailable(error: impl std::fmt::Display) -> FieldError {
    tracing::warn!(error=%error, "Field persistence could not be confirmed");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        "Field records are temporarily unavailable; the save is unconfirmed".into(),
    )
}
pub fn authority_error(error: AuthorityError) -> FieldError {
    match error {
        AuthorityError::Forbidden => (
            StatusCode::FORBIDDEN,
            "Current business owner authority is required".into(),
        ),
        error => unavailable(error),
    }
}
pub fn conflict() -> FieldError {
    (
        StatusCode::CONFLICT,
        "The field record changed; reload before editing".into(),
    )
}
pub fn missing() -> FieldError {
    (StatusCode::NOT_FOUND, "Field record not found".into())
}
pub fn invalid(message: &str) -> FieldError {
    (StatusCode::BAD_REQUEST, message.into())
}
#[derive(Clone)]
pub struct FieldAccess {
    pub pool: Option<PgPool>,
    pub store: Arc<server_auth::Store>,
}
impl FieldAccess {
    pub async fn authorize(
        &self,
        claims: &server_common::Claims,
        headers: &HeaderMap,
    ) -> Result<AuthorizedPgOwner, FieldError> {
        server_auth::commit_authority::verify_owner(&self.store, claims, headers)
            .await
            .map_err(authority_error)?;
        let pool = self
            .pool
            .as_ref()
            .ok_or_else(|| unavailable("No proven canonical field pool"))?;
        CanonicalPgAuthority::bind(self.store.clone(), pool)
            .map_err(authority_error)?
            .authorize(claims, headers)
            .await
            .map_err(authority_error)
    }
}
/// Prove that identity and all field relations name the same actual database objects.
/// A copied identity table or equal connection URL is never a write capability.
pub async fn canonical_pool(
    store: &server_auth::Store,
    configured: Option<&PgPool>,
) -> Option<PgPool> {
    let repository = store.portable_repo()?;
    match server_auth::commit_authority::canonical_pg_data_pool(
        repository.connection(),
        configured?,
        &[
            "appointments",
            "customers",
            "job_templates",
            "service_routes",
            "job_locations",
            "department_tasks",
            "field_mutation_receipts",
        ],
    )
    .await
    {
        Ok(pool) => Some(pool),
        Err(error) => {
            tracing::warn!(error=%error, "Canonical field storage binding unavailable");
            None
        }
    }
}
pub fn expected(value: Option<DateTime<Utc>>) -> Result<DateTime<Utc>, FieldError> {
    value.ok_or((
        StatusCode::PRECONDITION_REQUIRED,
        "An observed expected_updated_at is required; reload the record".into(),
    ))
}
pub fn is_terminal(status: &str) -> bool {
    ["completed", "cancelled", "canceled", "done"]
        .iter()
        .any(|value| status.eq_ignore_ascii_case(value))
}
pub fn check_transition(current: &str, requested: &str) -> Result<(), FieldError> {
    if is_terminal(current) && !current.eq_ignore_ascii_case(requested) {
        return Err(conflict());
    }
    Ok(())
}
pub fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 512
        && value.trim() == value
        && !value.chars().any(char::is_control)
}
pub fn coordinates(lat: Option<f64>, lng: Option<f64>) -> Result<(), FieldError> {
    if lat.is_some_and(|v| !v.is_finite() || !(-90.0..=90.0).contains(&v))
        || lng.is_some_and(|v| !v.is_finite() || !(-180.0..=180.0).contains(&v))
        || lat.is_some() != lng.is_some()
    {
        return Err(invalid("A valid latitude/longitude pair is required"));
    }
    Ok(())
}
pub async fn private_response(mut response: axum::response::Response) -> axum::response::Response {
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        axum::http::HeaderValue::from_static("private, no-store"),
    );
    response
}

pub struct Receipt {
    tenant: String,
    operation: &'static str,
    key: String,
    actor: String,
    digest: String,
}
impl Receipt {
    pub fn new(
        headers: &HeaderMap,
        owner: &AuthorizedPgOwner,
        operation: &'static str,
        payload: &impl Serialize,
    ) -> Result<Option<Self>, FieldError> {
        let mut values = headers.get_all("idempotency-key").iter();
        let Some(value) = values.next() else {
            return Ok(None);
        };
        let key = value
            .to_str()
            .map_err(|_| invalid("Invalid Idempotency-Key"))?;
        if values.next().is_some()
            || key.is_empty()
            || key.len() > 128
            || key.chars().any(|c| c.is_whitespace() || c.is_control())
        {
            return Err(invalid("Invalid Idempotency-Key"));
        }
        let canonical =
            serde_json::to_vec(&serde_json::json!({"actor":owner.actor_id(),"payload":payload}))
                .map_err(unavailable)?;
        Ok(Some(Self {
            tenant: owner.tenant_id().into(),
            operation,
            key: key.into(),
            actor: owner.actor_id().into(),
            digest: format!("{:x}", Sha256::digest(canonical)),
        }))
    }
    pub async fn replay<T: DeserializeOwned>(
        &self,
        connection: &mut PgConnection,
    ) -> Result<Option<T>, FieldError> {
        sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock(pg_catalog.hashtextextended(pg_catalog.jsonb_build_array('field-receipt-v1',$1::text,$2::text,$3::text)::text,0))")
            .bind(&self.tenant).bind(self.operation).bind(&self.key).execute(&mut *connection).await.map_err(unavailable)?;
        let saved: Option<(String,serde_json::Value)> = sqlx::query_as("SELECT request_sha256,response FROM field_mutation_receipts WHERE tenant_id=$1 AND operation=$2 AND idempotency_key=$3")
            .bind(&self.tenant).bind(self.operation).bind(&self.key).fetch_optional(connection).await.map_err(unavailable)?;
        match saved {
            Some((digest, _)) if digest != self.digest => Err((
                StatusCode::CONFLICT,
                "Idempotency-Key belongs to a different request".into(),
            )),
            Some((_, response)) => serde_json::from_value(response)
                .map(Some)
                .map_err(unavailable),
            None => Ok(None),
        }
    }
    pub async fn save(
        &self,
        connection: &mut PgConnection,
        response: &impl Serialize,
    ) -> Result<(), FieldError> {
        let response = serde_json::to_value(response).map_err(unavailable)?;
        sqlx::query("INSERT INTO field_mutation_receipts(tenant_id,operation,idempotency_key,actor_id,request_sha256,response)VALUES($1,$2,$3,$4,$5,$6)")
            .bind(&self.tenant).bind(self.operation).bind(&self.key).bind(&self.actor).bind(&self.digest).bind(response).execute(connection).await.map_err(unavailable)?;
        Ok(())
    }
}
