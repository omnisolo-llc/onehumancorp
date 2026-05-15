use tonic::{Request, Response, Status};
use tokio_stream::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;
use crate::MyHubService;
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::hub_service_server::HubService;
use crate::hub::Hub;
use chrono::Utc;

impl MyHubService {
    pub async fn impl_start_onboarding(
        &self,
        request: Request<StartOnboardingRequest>,
    ) -> Result<Response<StartOnboardingResponse>, Status> {
        let req = request.into_inner();
        match self.onboarding_agent.start_onboarding(req).await {
            Ok(resp) => Ok(Response::new(resp)),
            Err(e) => Err(Status::internal(e)),
        }
    }
}
