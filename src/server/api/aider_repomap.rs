use axum::{Json, Router, routing::get};
use ohc_builtin_agent::aider_repomap::RepoMap;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
pub struct RepoMapResponse {
    pub map: String,
    pub root_path: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/", get(get_repomap_handler))
}

async fn get_repomap_handler() -> Result<Json<RepoMapResponse>, Json<ErrorResponse>> {
    let current_dir = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let repo_map = RepoMap::new(&current_dir);
    match repo_map.generate_map() {
        Ok(map) => Ok(Json(RepoMapResponse {
            map,
            root_path: current_dir.to_string_lossy().to_string(),
        })),
        Err(e) => Err(Json(ErrorResponse {
            error: format!("Failed to generate RepoMap: {}", e),
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_get_repomap_handler() {
        let app = router();
        let server = TestServer::new(app).unwrap();

        let response = server.get("/").await;
        response.assert_status(StatusCode::OK);

        let json = response.json::<serde_json::Value>();
        assert!(json.get("map").is_some());
        assert!(json.get("root_path").is_some());
    }
}