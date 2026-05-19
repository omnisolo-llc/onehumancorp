#[cfg(test)]
mod tests {
    use axum::{routing::get, Router};
    use axum::http::{header, StatusCode, Request};
    use axum::body::Body;
    use tower::ServiceExt;
    use tower_http::compression::CompressionLayer;
    use flate2::read::GzDecoder;
    use std::io::Read;

    #[tokio::test]
    async fn test_compression_layer_works() {
        let app = Router::new()
            .route("/", get(|| async { "hello world".repeat(100) }))
            .layer(CompressionLayer::new());

        let req = Request::builder()
            .header(header::ACCEPT_ENCODING, "gzip")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.headers().get(header::CONTENT_ENCODING).unwrap(), "gzip");

        let body = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let mut decoder = GzDecoder::new(&body[..]);
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed).unwrap();
        assert_eq!(decompressed, "hello world".repeat(100));
    }
}
