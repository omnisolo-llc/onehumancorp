use axum::body::{Body, to_bytes};
use axum::http::{Method, Request, StatusCode};
use tower::ServiceExt;

async fn assert_unavailable(method: Method, path: &str, capability: &str) {
    for _ in 0..2 {
        let response = crate::actual_routes()
            .oneshot(
                Request::builder()
                    .method(method.clone())
                    .uri(path)
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"id":"client-supplied","status":"completed"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED, "{path}");
        assert_eq!(response.headers().get("cache-control").unwrap(), "no-store");
        let bytes = to_bytes(response.into_body(), 16384).await.unwrap();
        let payload: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(payload["success"], false, "{path}");
        assert_eq!(payload["code"], "capability_unavailable", "{path}");
        assert_eq!(payload["capability"], capability, "{path}");
        assert!(
            payload["message"].as_str().unwrap().contains("No"),
            "{path}"
        );
        for invented in ["id", "organization", "totalCostUSD", "receipt", "status"] {
            assert!(payload.get(invented).is_none(), "{path}: {invented}");
        }
    }
}

macro_rules! unavailable_case {
    ($name:ident, $method:ident, $path:literal, $capability:literal) => {
        #[tokio::test]
        async fn $name() {
            assert_unavailable(Method::$method, $path, $capability).await;
        }
    };
}

unavailable_case!(
    dashboard_never_invents_organization,
    GET,
    "/api/v1/dashboard",
    "organization_dashboard"
);
unavailable_case!(
    costs_never_invent_zero_spend,
    GET,
    "/api/v1/costs",
    "cost_summary"
);
unavailable_case!(
    request_never_invents_approval,
    POST,
    "/api/v1/approvals/request",
    "approval_request"
);
unavailable_case!(
    decision_never_invents_approval,
    PUT,
    "/api/v1/approvals/decide",
    "approval_decision"
);
unavailable_case!(
    handoff_never_invents_delivery,
    POST,
    "/api/v1/handoffs",
    "handoff_creation"
);
unavailable_case!(
    skill_never_invents_import,
    POST,
    "/api/v1/skills/import",
    "skill_import"
);
unavailable_case!(
    snapshot_never_invents_storage,
    POST,
    "/api/v1/snapshots/create",
    "snapshot_creation"
);
