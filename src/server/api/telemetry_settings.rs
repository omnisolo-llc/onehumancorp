//! Instance-wide telemetry controls. A hosted tenant administrator is not a
//! platform operator and cannot change another tenant's collector policy.
use crate::settings::Store;
use axum::{
    Json, Router,
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use server_common::Claims;
use std::sync::Arc;

#[derive(Clone, Copy)]
struct Policy {
    standalone: bool,
    multitenant: bool,
    configured_enabled: bool,
}
#[derive(Clone)]
struct TelemetrySettings {
    store: Arc<Store>,
    policy: Policy,
}
#[derive(Serialize)]
struct TelemetryStatus {
    /// Retained for existing clients; this is the saved preference, not the
    /// effective state of an operator-configured collector.
    product_telemetry_enabled: bool,
    preference_enabled: bool,
    effective_enabled: bool,
    operator_enforced: bool,
    can_change: bool,
    change_block_reason: Option<&'static str>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TelemetryUpdate {
    product_telemetry_enabled: bool,
}
#[derive(Serialize)]
struct TelemetryUpdateResponse {
    success: bool,
    #[serde(flatten)]
    state: TelemetryStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'static str>,
}

pub fn router<S: Clone + Send + Sync + 'static>(store: Arc<Store>) -> Router<S> {
    let config = ::server_config::get();
    router_with_policy(
        store,
        Policy {
            standalone: config.standalone,
            multitenant: config.multitenant,
            configured_enabled: config.telemetry_enabled,
        },
    )
}
fn router_with_policy<S: Clone + Send + Sync + 'static>(
    store: Arc<Store>,
    policy: Policy,
) -> Router<S> {
    Router::new()
        .route("/api/v1/settings/telemetry", get(read).post(update))
        .with_state(TelemetrySettings { store, policy })
}
impl TelemetrySettings {
    fn blocked(&self, user: &Claims) -> Option<(StatusCode, &'static str)> {
        if !self.policy.standalone || self.policy.multitenant {
            // No platform-wide operator identity is established by tenant JWTs.
            return Some((StatusCode::FORBIDDEN, "hosted_global_control_unavailable"));
        }
        if !user
            .roles
            .iter()
            .any(|role| role.eq_ignore_ascii_case("ADMIN"))
        {
            return Some((StatusCode::FORBIDDEN, "admin_required"));
        }
        if self.policy.configured_enabled {
            return Some((StatusCode::CONFLICT, "operator_enforced"));
        }
        if !self.store.has_persistent_storage() {
            return Some((
                StatusCode::SERVICE_UNAVAILABLE,
                "persistent_storage_unavailable",
            ));
        }
        None
    }
    fn snapshot(&self, user: &Claims) -> TelemetryStatus {
        let (preference, runtime) = self.store.telemetry_snapshot();
        let blocked = self.blocked(user);
        TelemetryStatus {
            product_telemetry_enabled: preference,
            preference_enabled: preference,
            effective_enabled: self.policy.configured_enabled || runtime,
            operator_enforced: self.policy.configured_enabled,
            can_change: blocked.is_none(),
            change_block_reason: blocked.map(|(_, reason)| reason),
        }
    }
}
fn user_identity(user: Option<Extension<Claims>>) -> Option<Claims> {
    user.map(|Extension(user)| user)
        .filter(|user| !user.sub.trim().is_empty())
}
fn unauthorized_response() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({"success":false,"error":"unauthenticated"})),
    )
        .into_response()
}
async fn read(State(state): State<TelemetrySettings>, user: Option<Extension<Claims>>) -> Response {
    let Some(user) = user_identity(user) else {
        return unauthorized_response();
    };
    Json(state.snapshot(&user)).into_response()
}
async fn update(
    State(state): State<TelemetrySettings>,
    user: Option<Extension<Claims>>,
    Json(request): Json<TelemetryUpdate>,
) -> Response {
    let Some(user) = user_identity(user) else {
        return unauthorized_response();
    };
    if let Some((status, reason)) = state.blocked(&user) {
        return (
            status,
            Json(TelemetryUpdateResponse {
                success: false,
                state: state.snapshot(&user),
                error: Some(reason),
            }),
        )
            .into_response();
    }
    if let Err(error) = state
        .store
        .set_product_telemetry(request.product_telemetry_enabled)
    {
        tracing::warn!(%error,"Telemetry preference was not saved");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(TelemetryUpdateResponse {
                success: false,
                state: state.snapshot(&user),
                error: Some("settings_persistence_failed"),
            }),
        )
            .into_response();
    }
    Json(TelemetryUpdateResponse {
        success: true,
        state: state.snapshot(&user),
        error: None,
    })
    .into_response()
}

#[cfg(test)]
#[path = "telemetry_settings_test.rs"]
mod tests;
