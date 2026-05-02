use axum::{
    routing::get,
    Router,
    response::Html,
};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::hub::Hub;
use std::path::PathBuf;

pub async fn run(addr: SocketAddr, _hub: Arc<Hub>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Find WASM assets path
    let wasm_path = if let Ok(runfiles) = std::env::var("TEST_SRCDIR") {
        let workspace = std::env::var("TEST_WORKSPACE").unwrap_or_else(|_| "ohc".to_string());
        PathBuf::from(runfiles).join(workspace).join("src/app/app_wasm")
    } else {
        PathBuf::from("bazel-bin/src/app/app_wasm")
    };

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/index.html", get(index_handler))
        .fallback_service(ServeDir::new(wasm_path));

    println!("HTTP server listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../app/index.html"))
}
