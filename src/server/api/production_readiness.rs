//! Explicit failures for legacy endpoints that have no verified implementation.
//!
//! These responses do not certify capability readiness. A replacement needs
//! authenticated tenant scope, actual persistence/provider receipts, and tests
//! of the mounted route before this gate can be removed.
use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

pub fn unavailable(capability: &'static str) -> Response {
    (
        StatusCode::NOT_IMPLEMENTED,
        [(header::CACHE_CONTROL, "no-store")],
        Json(serde_json::json!({
            "success": false,
            "code": "capability_unavailable",
            "capability": capability,
            "message": "No verified result is available. This capability is not implemented; no action was completed."
        })),
    )
        .into_response()
}
