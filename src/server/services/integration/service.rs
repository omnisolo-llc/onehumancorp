use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::integration_service_server::IntegrationService;
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
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        let instances = if !req.category.is_empty() {
            self.registry.instances_by_category(&tenant_id, &req.category)
        } else {
            self.registry.instances(&tenant_id)
        };
        Ok(Response::new(GetIntegrationsResponse { instances }))
    }

    async fn connect_integration(
        &self,
        request: Request<ConnectIntegrationRequest>,
    ) -> Result<Response<IntegrationInstance>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        match self.registry.connect(&tenant_id, &req.integration_id, &req.base_url, req.clone()) {
            Ok(inst) => Ok(Response::new(inst)),
            Err(e) => Err(Status::invalid_argument(e)),
        }
    }

    async fn disconnect_integration(
        &self,
        request: Request<DisconnectIntegrationRequest>,
    ) -> Result<Response<IntegrationInstance>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        match self.registry.disconnect(&tenant_id, &req.integration_id) {
            Ok(inst) => Ok(Response::new(inst)),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn get_pull_requests(
        &self,
        request: Request<GetPullRequestsRequest>,
    ) -> Result<Response<GetPullRequestsResponse>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        let pull_requests = self.registry.pull_requests(&tenant_id, &req.integration_id);
        Ok(Response::new(GetPullRequestsResponse { pull_requests }))
    }

    async fn create_pull_request(
        &self,
        request: Request<CreatePrRequest>,
    ) -> Result<Response<PullRequest>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        match self.registry.create_pull_request(&tenant_id, &req.integration_id, &req.repository, &req.title, &req.body, &req.source_branch, &req.target_branch, &req.created_by) {
            Ok(pr) => Ok(Response::new(pr)),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn merge_pull_request(
        &self,
        request: Request<PrActionRequest>,
    ) -> Result<Response<PullRequest>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        match self.registry.merge_pull_request(&tenant_id, &req.pr_id) {
            Ok(pr) => Ok(Response::new(pr)),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn close_pull_request(
        &self,
        request: Request<PrActionRequest>,
    ) -> Result<Response<PullRequest>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        match self.registry.close_pull_request(&tenant_id, &req.pr_id) {
            Ok(pr) => Ok(Response::new(pr)),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn get_issues(
        &self,
        request: Request<GetIssuesRequest>,
    ) -> Result<Response<GetIssuesResponse>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        let issues = self.registry.issues(&tenant_id, &req.integration_id);
        Ok(Response::new(GetIssuesResponse { issues }))
    }

    async fn create_issue(
        &self,
        request: Request<CreateIssueRequest>,
    ) -> Result<Response<Issue>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        match self.registry.create_issue(&tenant_id, &req.integration_id, &req.project, &req.title, &req.description, &req.created_by, &req.priority, req.labels) {
            Ok(issue) => Ok(Response::new(issue)),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn update_issue_status(
        &self,
        request: Request<IssueStatusRequest>,
    ) -> Result<Response<Issue>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        match self.registry.update_issue_status(&tenant_id, &req.issue_id, &req.status) {
            Ok(issue) => Ok(Response::new(issue)),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn assign_issue(
        &self,
        request: Request<IssueAssignRequest>,
    ) -> Result<Response<Issue>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|v| v.to_str().unwrap_or("default")).unwrap_or("default").to_string();
        let req = request.into_inner();
        match self.registry.assign_issue(&tenant_id, &req.issue_id, &req.assignee) {
            Ok(issue) => Ok(Response::new(issue)),
            Err(e) => Err(Status::not_found(e)),
        }
    }
}
