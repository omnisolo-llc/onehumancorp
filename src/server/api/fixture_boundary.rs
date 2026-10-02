//! Retired fixture names must not be reinterpreted as approval record IDs.
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn reject_retired_fixture_paths(request: Request, next: Next) -> Response {
    if request
        .uri()
        .path()
        .rsplit('/')
        .next()
        .is_some_and(|segment| segment.starts_with("simulate-"))
    {
        return StatusCode::NOT_FOUND.into_response();
    }
    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Body, http::Request as HttpRequest, routing::post};
    use tower::ServiceExt;

    #[tokio::test]
    async fn retired_names_never_reach_dynamic_approval_handlers() {
        let app = Router::new()
            .route("/{id}", post(|| async { StatusCode::NO_CONTENT }))
            .route_layer(axum::middleware::from_fn(reject_retired_fixture_paths));
        for name in [
            "simulate-quote-draft",
            "simulate-invoice-draft",
            "simulate-any-future-fixture",
        ] {
            for body in ["{}", "not json", "{\"approved\":true}"] {
                let response = app
                    .clone()
                    .oneshot(
                        HttpRequest::builder()
                            .method("POST")
                            .uri(format!("/{name}"))
                            .body(Body::from(body))
                            .unwrap(),
                    )
                    .await
                    .unwrap();
                assert_eq!(response.status(), StatusCode::NOT_FOUND);
            }
        }
        let response = app
            .oneshot(
                HttpRequest::builder()
                    .method("POST")
                    .uri("/recorded-approval-id")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
