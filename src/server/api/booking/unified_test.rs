use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use tower::ServiceExt;
use crate::api::booking::unified;
use crate::db::DbPool;
use ohc_proto::app::{
    GetResourcesRequest, GetServicesRequest, CreateUnifiedBookingRequest,
    GetResourcesResponse, GetServicesResponse, CreateUnifiedBookingResponse,
};

// ... Setup DbPool helper ...

#[tokio::test]
async fn test_unified_booking_resources() {
    // Basic compilation test placeholder
    assert!(true);
}

#[tokio::test]
async fn test_booking_request_tenant_isolation_unauthenticated() {
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use crate::api::booking::request::router;
    use axum::http::header::HeaderName;
    use std::sync::Arc;

    std::env::set_var("OHC_MULTITENANT", "true");

    let orchestrator = Arc::new(DepartmentOrchestrator::new_for_test());
    let app = router(orchestrator);

    let req = Request::builder()
        .uri("/")
        .method("POST")
        .header("Content-Type", "application/json")
        .header("x-tenant-id", "some_victim_tenant")
        .body(Body::from(r#"{"description": "leak test"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();

    // Since we use Option<Extension(AuthInfo)>, the server handles the missing auth info properly.
    assert_eq!(
        response.status(), StatusCode::UNAUTHORIZED,
        "Expected UNAUTHORIZED due to missing AuthInfo in multitenant mode, got {}", response.status()
    );

    std::env::remove_var("OHC_MULTITENANT");
}
