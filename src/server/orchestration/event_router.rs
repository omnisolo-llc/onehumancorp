use std::sync::Arc;
use crate::orchestration::queue::{OHCAsyncJob, OHCAsyncJobQueue};
use crate::orchestration::departments::types::DepartmentEvent;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::queue::async_worker_pool::AsyncJobHandler;
use uuid::Uuid;

pub struct EventRouter {
    orchestrator: Arc<DepartmentOrchestrator>,
}

impl EventRouter {
    pub fn new(orchestrator: Arc<DepartmentOrchestrator>) -> Self {
        Self { orchestrator }
    }
}

impl AsyncJobHandler for EventRouter {
    fn handle(&self, job: OHCAsyncJob) -> tokio::task::JoinHandle<Result<(), String>> {
        let orchestrator = self.orchestrator.clone();
        tokio::spawn(async move {
            let payload: serde_json::Value = serde_json::from_str(&job.payload).map_err(|e| e.to_string())?;
            let event = DepartmentEvent {
                id: job.id,
                tenant_id: job.tenant_id,
                event_type: job.event_type,
                payload,
            };

            orchestrator.dispatch_event(event).await
        })
    }
}
