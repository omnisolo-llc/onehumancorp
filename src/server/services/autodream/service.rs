use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::auto_dream_service_server::AutoDreamService;
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

        match self.worker.search_memories("[0.0]", limit).await {
            Ok(results) => Ok(Response::new(AutoDreamQueryResult { results })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
