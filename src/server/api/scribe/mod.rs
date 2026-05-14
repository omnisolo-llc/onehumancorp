use axum::{routing::{get, post}, Json, Router};
use std::sync::Arc;
use crate::services::scribe::service::MyScribeService;

pub fn router(scribe: Arc<MyScribeService>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/articles", get(get_articles))
        .route("/search", get(search_articles))
        .route("/tooltips", get(get_tooltips))
        .route("/walkthrough/:id", get(get_walkthrough))
        .route("/chat", post(ask_ai))
        .with_state(scribe);

    Router::new().merge(r)
}

async fn get_articles(
    axum::extract::State(scribe): axum::extract::State<Arc<MyScribeService>>,
) -> Json<serde_json::Value> {
    use crate::proto::orchestration::scribe_service_server::ScribeService;
    let resp = scribe.get_help_articles(tonic::Request::new(crate::proto::orchestration::EmptyRequest {})).await.unwrap();
    Json(serde_json::to_value(resp.into_inner()).unwrap())
}

async fn search_articles(
    axum::extract::State(scribe): axum::extract::State<Arc<MyScribeService>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Json<serde_json::Value> {
    use crate::proto::orchestration::scribe_service_server::ScribeService;
    let query = params.get("q").cloned().unwrap_or_default();
    let resp = scribe.search_help(tonic::Request::new(crate::proto::orchestration::SearchHelpRequest { query })).await.unwrap();
    Json(serde_json::to_value(resp.into_inner()).unwrap())
}

async fn get_tooltips(
    axum::extract::State(scribe): axum::extract::State<Arc<MyScribeService>>,
) -> Json<serde_json::Value> {
    use crate::proto::orchestration::scribe_service_server::ScribeService;
    let resp = scribe.get_tooltips(tonic::Request::new(crate::proto::orchestration::EmptyRequest {})).await.unwrap();
    Json(serde_json::to_value(resp.into_inner()).unwrap())
}

async fn get_walkthrough(
    axum::extract::State(scribe): axum::extract::State<Arc<MyScribeService>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    use crate::proto::orchestration::scribe_service_server::ScribeService;
    let resp = scribe.get_walkthrough(tonic::Request::new(crate::proto::orchestration::WalkthroughRequest { walkthrough_id: id })).await.unwrap();
    Json(serde_json::to_value(resp.into_inner()).unwrap())
}

async fn ask_ai(
    axum::extract::State(scribe): axum::extract::State<Arc<MyScribeService>>,
    Json(payload): Json<crate::proto::orchestration::HelpChatRequest>,
) -> Json<serde_json::Value> {
    use crate::proto::orchestration::scribe_service_server::ScribeService;
    let resp = scribe.ask_help_ai(tonic::Request::new(payload)).await.unwrap();
    Json(serde_json::to_value(resp.into_inner()).unwrap())
}
