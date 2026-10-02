//! Reviewed public content and inactive installations; no runtime activation.
use crate::persistence::agent_definitions::{
    DefinitionStore, Error, InstallRequest, ListQuery, Owner, PublishRequest,
};
use axum::{
    Json, Router,
    extract::{
        Extension, Path, Query, State,
        rejection::{JsonRejection, QueryRejection},
    },
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use server_common::Claims;
use std::sync::Arc;
use uuid::Uuid;

pub fn router<S: Clone + Send + Sync + 'static>(
    store: DefinitionStore,
    auth: Arc<server_auth::Store>,
) -> Router<S> {
    Router::new()
        .route("/api/v1/agents/definitions", get(list).post(publish))
        .route("/api/v1/agents/definitions/{id}/install", post(install))
        .route(
            "/api/v1/agents/definitions/operations/{request_id}",
            get(operation),
        )
        .layer(axum::extract::DefaultBodyLimit::max(262_144))
        .layer(axum::middleware::from_fn(expected_owner))
        .layer(axum::middleware::from_fn_with_state(
            auth,
            server_auth::strict_bearer_auth_middleware,
        ))
        .layer(axum::middleware::from_fn(no_store))
        .with_state(store)
}
async fn no_store(req: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let mut response = next.run(req).await;
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, no-store"),
    );
    response
}
fn failure(status: StatusCode, reason: &str) -> Response {
    (
        status,
        Json(serde_json::json!({"success":false,"reason":reason})),
    )
        .into_response()
}
fn owner(claims: &Claims, mutation: bool) -> Result<Owner, (StatusCode, &'static str)> {
    let tenant = server_common::auth_utils::signed_tenant_id(claims)
        .ok_or((StatusCode::UNAUTHORIZED, "verified_owner_required"))?;
    if claims.organization_id.as_deref() != Some(tenant.as_str()) || claims.sub.trim().is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "verified_owner_required"));
    }
    if mutation
        && !claims
            .roles
            .iter()
            .any(|role| role.eq_ignore_ascii_case("ADMIN") || role.eq_ignore_ascii_case("OWNER"))
    {
        return Err((StatusCode::FORBIDDEN, "owner_or_admin_required"));
    }
    Ok(Owner {
        tenant,
        user: claims.sub.clone(),
    })
}
async fn expected_owner(req: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let Some(claims) = req.extensions().get::<Claims>() else {
        return failure(StatusCode::UNAUTHORIZED, "verified_owner_required");
    };
    let verified = match owner(claims, false) {
        Ok(owner) => owner,
        Err((status, reason)) => return failure(status, reason),
    };
    let expected_user = req.headers().get_all("x-ohc-expected-user");
    let expected_tenant = req.headers().get_all("x-ohc-expected-tenant");
    if expected_user.iter().next().is_some() || expected_tenant.iter().next().is_some() {
        let exact = |values: axum::http::header::GetAll<'_, HeaderValue>, actual: &str| {
            let mut values = values.iter();
            values.next().and_then(|v| v.to_str().ok()) == Some(actual) && values.next().is_none()
        };
        if !exact(expected_user, &verified.user) || !exact(expected_tenant, &verified.tenant) {
            return failure(StatusCode::CONFLICT, "session_identity_changed");
        }
    }
    next.run(req).await
}
fn result<T: serde::Serialize>(value: Result<T, Error>) -> Response {
    match value {
        Ok(value) => Json(value).into_response(),
        Err(Error::Forbidden) => failure(StatusCode::FORBIDDEN, "owner_or_admin_required"),
        Err(Error::Invalid) => failure(StatusCode::BAD_REQUEST, "invalid_request"),
        Err(Error::Conflict) => failure(StatusCode::CONFLICT, "request_or_definition_conflict"),
        Err(Error::NotFound) => failure(StatusCode::NOT_FOUND, "operation_not_found"),
        Err(Error::Unavailable) => failure(
            StatusCode::SERVICE_UNAVAILABLE,
            "durable_storage_unavailable",
        ),
        Err(Error::Database(error)) => {
            tracing::warn!(error=%error,"agent definition database operation failed; receipt recovery required");
            failure(
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_operation_failed",
            )
        }
        Err(Error::Corrupt) => failure(
            StatusCode::INTERNAL_SERVER_ERROR,
            "stored_receipt_unverified",
        ),
    }
}
async fn publish(
    State(store): State<DefinitionStore>,
    Extension(claims): Extension<Claims>,
    request: Result<Json<PublishRequest>, JsonRejection>,
) -> Response {
    let owner = match owner(&claims, true) {
        Ok(owner) => owner,
        Err((status, reason)) => return failure(status, reason),
    };
    let Ok(Json(request)) = request else {
        return failure(StatusCode::BAD_REQUEST, "invalid_request");
    };
    result(store.publish(&owner, &request).await)
}
async fn install(
    State(store): State<DefinitionStore>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    request: Result<Json<InstallRequest>, JsonRejection>,
) -> Response {
    let owner = match owner(&claims, true) {
        Ok(owner) => owner,
        Err((status, reason)) => return failure(status, reason),
    };
    let (Ok(id), Ok(Json(request))) = (Uuid::parse_str(&id), request) else {
        return failure(StatusCode::BAD_REQUEST, "invalid_request");
    };
    result(store.install(&owner, id, &request).await)
}
async fn operation(
    State(store): State<DefinitionStore>,
    Extension(claims): Extension<Claims>,
    Path(request_id): Path<String>,
) -> Response {
    let owner = match owner(&claims, false) {
        Ok(owner) => owner,
        Err((status, reason)) => return failure(status, reason),
    };
    let Ok(request_id) = Uuid::parse_str(&request_id) else {
        return failure(StatusCode::BAD_REQUEST, "invalid_request");
    };
    result(store.operation(&owner, request_id).await)
}
async fn list(
    State(store): State<DefinitionStore>,
    Extension(claims): Extension<Claims>,
    query: Result<Query<ListQuery>, QueryRejection>,
) -> Response {
    let owner = match owner(&claims, false) {
        Ok(owner) => owner,
        Err((status, reason)) => return failure(status, reason),
    };
    let Ok(Query(query)) = query else {
        return failure(StatusCode::BAD_REQUEST, "invalid_request");
    };
    result(store.list(&owner, &query).await)
}
