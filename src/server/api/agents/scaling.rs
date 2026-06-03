use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;

#[derive(Deserialize, Debug)]
pub struct ScaleRequest {
    pub role: String,
    pub count: i32,
}

#[derive(Serialize, Debug)]
pub struct ScaleResponse {
    pub status: String,
    pub role: String,
    pub count: i32,
    pub message: String,
}

pub fn router<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/scale", post(scale_handler))
        .with_state(hub)
}

use axum::extract::FromRequest;

async fn scale_handler(
    State(hub): State<Arc<Hub>>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match req.extensions().get::<::server_common::Claims>() {
        Some(claims) => claims.organization_id.clone().unwrap_or_else(|| "system".to_string()),
        None => "system".to_string(),
    };

    let (parts, body) = req.into_parts();
    let req2 = axum::extract::Request::from_parts(parts, body);

    let payload: ScaleRequest = match axum::extract::Json::<ScaleRequest>::from_request(req2, &()).await {
        Ok(Json(payload)) => payload,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(ScaleResponse { status: "error".to_string(), role: "".to_string(), count: 0, message: "Invalid payload".to_string() })).into_response(),
    };

    // Simulate intent processing and K8s reconciliation (design doc)
    let message = format!("Scaling intent registered for role {} to {} replicas.", payload.role, payload.count);

    let response = ScaleResponse {
        status: "success".to_string(),
        role: payload.role.clone(),
        count: payload.count,
        message,
    };

    (StatusCode::OK, Json(response)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_request_struct() {
        let req = ScaleRequest {
            role: "sales_rep".to_string(),
            count: 5,
        };
        assert_eq!(req.role, "sales_rep");
        assert_eq!(req.count, 5);
    }
}
