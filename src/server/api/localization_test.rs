use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_fx_rates_endpoint() {
    let app = crate::api::localization::router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/localization/fx-rates/tenant123")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_fx_margins_endpoint() {
    let app = crate::api::localization::router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/localization/fx-margins/tenant123")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_i18n_strings_endpoint() {
    let app = crate::api::localization::router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/localization/i18n/tenant123/es")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
