use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::auto_dream_service_server::AutoDreamService;
use std::sync::Arc;
use crate::autodream::AutoDreamWorker;

pub struct MyAutoDreamService {
    worker: Arc<AutoDreamWorker>,
}

impl MyAutoDreamService {
    pub fn new(worker: Arc<AutoDreamWorker>) -> Self {
        MyAutoDreamService { worker }
    }
}

#[tonic::async_trait]
impl AutoDreamService for MyAutoDreamService {
    async fn sync_auto_dream(
        &self,
        _request: Request<AutoDreamSyncRequest>,
    ) -> Result<Response<AutoDreamSyncResponse>, Status> {
        match self.worker.consolidate_epoch().await {
            Ok(_) => Ok(Response::new(AutoDreamSyncResponse { status: "success".to_string() })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn query_auto_dream(
        &self,
        request: Request<AutoDreamQueryRequest>,
    ) -> Result<Response<AutoDreamQueryResult>, Status> {
        let req = request.into_inner();
        if req.query_text.is_empty() {
            return Err(Status::invalid_argument("query_text is required"));
        }

        let limit = if req.limit <= 0 { 5 } else { req.limit };

        let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
        let client = crate::minimax::MinimaxClient::new(api_key);
        let embedding = match client.generate_embedding(&req.query_text).await {
            Ok(emb) => serde_json::to_string(&emb).unwrap_or_else(|_| format!("[{}]", vec!["0.0"; 1536].join(", "))),
            Err(e) => {
                tracing::error!("AutoDream service: failed to generate embedding: {}", e);
                format!("[{}]", vec!["0.0"; 1536].join(", "))
            }
        };

        match self.worker.search_memories(&embedding, &tenant_id, limit).await {
            Ok(results) => Ok(Response::new(AutoDreamQueryResult { results })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
