use axum::{
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
};
use bytes::Bytes;
use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::MeshTransport;
use prost::Message;

fn check_spiffe_auth(headers: &HeaderMap) -> Result<String, axum::response::Response> {
    let spiffe_id = headers.get("x-spiffe-id")
        .and_then(|val| val.to_str().ok())
        .unwrap_or("");

    if spiffe_id.is_empty() {
        let error_res = serde_json::json!({ "error": "unauthorized" });
        return Err((axum::http::StatusCode::UNAUTHORIZED, axum::response::Json(error_res)).into_response());
    }
    Ok(spiffe_id.to_string())
}

pub async fn handle_mesh_v2_broadcast(
    headers: HeaderMap,
    State(transport): State<Arc<dyn MeshTransport>>,
    body: Bytes,
) -> impl IntoResponse {
    if let Err(err_response) = check_spiffe_auth(&headers) {
        return err_response;
    }

    let req = match ::server_ohc::hub::PublishTeammateMeshEventRequest::decode(body) {
        Ok(msg) => msg,
        Err(e) => {
            let error_res = serde_json::json!({ "error": format!("failed to decode protobuf: {}", e) });
            return (axum::http::StatusCode::BAD_REQUEST, axum::response::Json(error_res)).into_response();
        }
    };

    if req.channel.is_empty() {
        let error_res = serde_json::json!({ "error": "channel is required" });
        return (axum::http::StatusCode::BAD_REQUEST, axum::response::Json(error_res)).into_response();
    }

    if let Some(event) = req.event {
        match transport.publish(&req.channel, event).await {
            Ok(_) => axum::response::Json(serde_json::json!({ "success": true })).into_response(),
            Err(e) => {
                let error_res = serde_json::json!({ "error": e.to_string() });
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, axum::response::Json(error_res)).into_response()
            }
        }
    } else {
        let error_res = serde_json::json!({ "error": "event is required" });
        (axum::http::StatusCode::BAD_REQUEST, axum::response::Json(error_res)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;

    #[tokio::test]
    async fn test_handle_mesh_v2_broadcast() {
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());

        let app = Router::new()
            .route("/api/mesh/v2/broadcast", axum::routing::post(handle_mesh_v2_broadcast))
            .with_state(transport);

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .unwrap();
        });

        let client = reqwest::Client::new();
        let url = format!("http://{}/api/mesh/v2/broadcast", addr);

        let event = ::server_ohc::orchestration::TeammateMeshEvent {
            agent_id: "test_agent".to_string(),
            action: "test_action".to_string(),
            status: "ok".to_string(),
            payload: b"test payload".to_vec(),
            msg_id: "msg1".to_string(),
        };

        let req_body = ::server_ohc::hub::PublishTeammateMeshEventRequest {
            channel: "test_channel".to_string(),
            event: Some(event),
        };

        let mut buf = Vec::new();
        req_body.encode(&mut buf).unwrap();

        // Missing spiffe ID
        let res = client.post(&url).body(buf.clone()).send().await.unwrap();
        assert_eq!(res.status(), 401);

        // With spiffe ID
        let res = client.post(&url).header("x-spiffe-id", "spiffe://example.org/test").body(buf.clone()).send().await.unwrap();
        assert_eq!(res.status(), 200);
        let text = res.text().await.unwrap();
        assert!(text.contains("\"success\":true"));

        // Invalid body
        let res = client.post(&url).header("x-spiffe-id", "spiffe://example.org/test").body("invalid").send().await.unwrap();
        assert_eq!(res.status(), 400);
    }
}
