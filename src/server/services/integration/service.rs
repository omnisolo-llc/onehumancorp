use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::integration_service_server::IntegrationService;
use crate::integrations::registry::IntegrationsRegistry;
use std::sync::Arc;

pub struct MyIntegrationService {
    registry: Arc<IntegrationsRegistry>,
}

impl MyIntegrationService {
    pub fn new(registry: Arc<IntegrationsRegistry>) -> Self {
        MyIntegrationService { registry }
    }
}

#[tonic::async_trait]
impl IntegrationService for MyIntegrationService {
    async fn get_integrations(
        &self,
        request: Request<GetIntegrationsRequest>,
    ) -> Result<Response<GetIntegrationsResponse>, Status> {
        let req = request.into_inner();
        let instances = if !req.category.is_empty() {
            self.registry.instances_by_category(&req.category)
        } else {
            self.registry.instances()
        };
        Ok(Response::new(GetIntegrationsResponse { instances }))
    }

    async fn connect_integration(
        &self,
        request: Request<ConnectIntegrationRequest>,
    ) -> Result<Response<IntegrationInstance>, Status> {
        let req = request.into_inner();
        match self.registry.connect(&req.integration_id, &req.base_url, req.clone()) {
            Ok(inst) => Ok(Response::new(inst)),
            Err(e) => Err(Status::invalid_argument(e)),
        }
    }

    async fn disconnect_integration(
        &self,
        request: Request<DisconnectIntegrationRequest>,
    ) -> Result<Response<IntegrationInstance>, Status> {
        let req = request.into_inner();
        match self.registry.disconnect(&req.integration_id) {
            Ok(inst) => Ok(Response::new(inst)),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn get_pull_requests(
        &self,
        request: Request<GetPullRequestsRequest>,
    ) -> Result<Response<GetPullRequestsResponse>, Status> {
        let req = request.into_inner();
        let pull_requests = self.registry.pull_requests(&req.integration_id);
        Ok(Response::new(GetPullRequestsResponse { pull_requests }))
    }

    async fn create_pull_request(
        &self,
        request: Request<CreatePrRequest>,
    ) -> Result<Response<PullRequest>, Status> {
        let req = request.into_inner();
        match self.registry.create_pull_request(&req.integration_id, &req.repository, &req.title, &req.body, &req.source_branch, &req.target_branch, &req.created_by) {
            Ok(pr) => Ok(Response::new(pr)),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn merge_pull_request(
        &self,
        request: Request<PrActionRequest>,
    ) -> Result<Response<PullRequest>, Status> {
        let req = request.into_inner();
        match self.registry.merge_pull_request(&req.pr_id) {
            Ok(pr) => Ok(Response::new(pr)),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn close_pull_request(
        &self,
        request: Request<PrActionRequest>,
    ) -> Result<Response<PullRequest>, Status> {
        let req = request.into_inner();
        match self.registry.close_pull_request(&req.pr_id) {
            Ok(pr) => Ok(Response::new(pr)),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn get_issues(
        &self,
        request: Request<GetIssuesRequest>,
    ) -> Result<Response<GetIssuesResponse>, Status> {
        let req = request.into_inner();
        let issues = self.registry.issues(&req.integration_id);
        Ok(Response::new(GetIssuesResponse { issues }))
    }

    async fn create_issue(
        &self,
        request: Request<CreateIssueRequest>,
    ) -> Result<Response<Issue>, Status> {
        let req = request.into_inner();
        match self.registry.create_issue(&req.integration_id, &req.project, &req.title, &req.description, &req.created_by, &req.priority, req.labels) {
            Ok(issue) => Ok(Response::new(issue)),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn update_issue_status(
        &self,
        request: Request<IssueStatusRequest>,
    ) -> Result<Response<Issue>, Status> {
        let req = request.into_inner();
        match self.registry.update_issue_status(&req.issue_id, &req.status) {
            Ok(issue) => Ok(Response::new(issue)),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn assign_issue(
        &self,
        request: Request<IssueAssignRequest>,
    ) -> Result<Response<Issue>, Status> {
        let req = request.into_inner();
        match self.registry.assign_issue(&req.issue_id, &req.assignee) {
            Ok(issue) => Ok(Response::new(issue)),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn get_free_busy(
        &self,
        request: Request<GetFreeBusyRequest>,
    ) -> Result<Response<GetFreeBusyResponse>, Status> {
        let req = request.into_inner();
        match self.registry.get_free_busy(&req.integration_id, &req.time_min, &req.time_max).await {
            Ok(free_busy_data) => Ok(Response::new(GetFreeBusyResponse { free_busy_data })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn create_event(
        &self,
        request: Request<CreateEventRequest>,
    ) -> Result<Response<CreateEventResponse>, Status> {
        let req = request.into_inner();
        match self.registry.create_event(&req.integration_id, &req.summary, &req.start_time, &req.end_time).await {
            Ok(event_id) => Ok(Response::new(CreateEventResponse { event_id })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn get_booking_link(
        &self,
        request: Request<GetBookingLinkRequest>,
    ) -> Result<Response<GetBookingLinkResponse>, Status> {
        let req = request.into_inner();
        match self.registry.get_booking_link(&req.integration_id, &req.event_type).await {
            Ok(link) => Ok(Response::new(GetBookingLinkResponse { link })),
            Err(e) => Err(Status::internal(e)),
        }
    }
}
