use tonic::{Request, Response, Status};
use tokio_stream::Stream;
use std::pin::Pin;
use crate::MyHubService;
use ::server_ohc::orchestration::hub_service_server::HubService;
use ::server_ohc::orchestration::*;

pub mod billing;
pub mod provisioning;
pub mod tasks;
pub mod mesh;
pub mod wizard;
pub mod meetings;
pub mod growth;
pub mod onboarding;
pub mod misc;

#[tonic::async_trait]
impl HubService for MyHubService {

    type PollTasksStream = Pin<Box<dyn Stream<Item = Result<::server_ohc::orchestration::SharedTask, Status>> + Send>>;
    type DiscoverAgentsStream = Pin<Box<dyn Stream<Item = Result<::server_ohc::orchestration::AgentCapabilities, Status>> + Send>>;
    type StreamMeshEventsStream = Pin<Box<dyn Stream<Item = Result<::server_ohc::orchestration::MeshEvent, Status>> + Send>>;
    type StreamTeammateMeshStream = Pin<Box<dyn Stream<Item = Result<::server_ohc::orchestration::TeammateMeshEvent, Status>> + Send>>;
    type StreamMessagesStream = Pin<Box<dyn Stream<Item = Result<::server_ohc::orchestration::Message, Status>> + Send>>;
    async fn get_my_plan(
        &self,
        request: tonic::Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::MyPlanResponse>, tonic::Status> {
        self.impl_get_my_plan(request).await
    }


    async fn get_cost_dashboard(
        &self,
        request: tonic::Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::CostDashboardResponse>, tonic::Status> {
        self.impl_get_cost_dashboard(request).await
    }


    async fn select_plan(
        &self,
        request: tonic::Request<::server_ohc::orchestration::SelectPlanRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::SelectPlanResponse>, tonic::Status> {
        self.impl_select_plan(request).await
    }


    async fn cancel_subscription(
        &self,
        request: tonic::Request<::server_ohc::orchestration::CancelSubscriptionRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::CancelSubscriptionResponse>, tonic::Status> {
        self.impl_cancel_subscription(request).await
    }


    async fn download_invoice(
        &self,
        _request: tonic::Request<::server_ohc::orchestration::DownloadInvoiceRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::DownloadInvoiceResponse>, tonic::Status> {
        self.impl_download_invoice(_request).await
    }


    async fn register_agent(
        &self,
        request: Request<RegisterAgentRequest>,
    ) -> Result<Response<RegisterAgentResponse>, Status> {
        self.impl_register_agent(request).await
    }


    async fn handle_config_wizard(
        &self,
        _request: tonic::Request<::server_ohc::orchestration::AgentConfig>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::WizardResponse>, tonic::Status> {
        self.impl_handle_config_wizard(_request).await
    }


    async fn handle_prompt_tuning(
        &self,
        _request: tonic::Request<::server_ohc::orchestration::PromptTuningConfig>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::WizardResponse>, tonic::Status> {
        self.impl_handle_prompt_tuning(_request).await
    }


    async fn open_meeting(
        &self,
        request: Request<OpenMeetingRequest>,
    ) -> Result<Response<MeetingRoom>, Status> {
        self.impl_open_meeting(request).await
    }


    async fn publish(
        &self,
        request: Request<PublishMessageRequest>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        self.impl_publish(request).await
    }


    async fn delegate_task(
        &self,
        request: Request<DelegateTaskRequest>,
    ) -> Result<Response<DelegateTaskResponse>, Status> {
        self.impl_delegate_task(request).await
    }


    async fn verify_environment(
        &self,
        request: tonic::Request<VerifyEnvironmentRequest>,
    ) -> Result<tonic::Response<VerifyEnvironmentResponse>, tonic::Status> {
        self.impl_verify_environment(request).await
    }


    async fn generate_config(
        &self,
        request: tonic::Request<GenerateConfigRequest>,
    ) -> Result<tonic::Response<GenerateConfigResponse>, tonic::Status> {
        self.impl_generate_config(request).await
    }


    async fn save_wizard_state(
        &self,
        request: tonic::Request<SaveWizardStateRequest>,
    ) -> Result<tonic::Response<SaveWizardStateResponse>, tonic::Status> {
        self.impl_save_wizard_state(request).await
    }


    async fn get_wizard_state(
        &self,
        request: tonic::Request<GetWizardStateRequest>,
    ) -> Result<tonic::Response<GetWizardStateResponse>, tonic::Status> {
        self.impl_get_wizard_state(request).await
    }


    async fn reset_wizard_state(
        &self,
        request: tonic::Request<ResetWizardStateRequest>,
    ) -> Result<tonic::Response<ResetWizardStateResponse>, tonic::Status> {
        self.impl_reset_wizard_state(request).await
    }


    async fn provision(
        &self,
        request: tonic::Request<ProvisionRequest>,
    ) -> Result<tonic::Response<ProvisionResponse>, tonic::Status> {
        self.impl_provision(request).await
    }


    async fn publish_site(
        &self,
        request: tonic::Request<PublishSiteRequest>,
    ) -> Result<tonic::Response<PublishSiteResponse>, tonic::Status> {
        self.impl_publish_site(request).await
    }


    async fn audit_setup(
        &self,
        request: tonic::Request<AuditSetupRequest>,
    ) -> Result<tonic::Response<AuditSetupResponse>, tonic::Status> {
        self.impl_audit_setup(request).await
    }


    async fn diagnostics(
        &self,
        _request: tonic::Request<DiagnosticsRequest>,
    ) -> Result<tonic::Response<DiagnosticsResponse>, tonic::Status> {
        self.impl_diagnostics(_request).await
    }


    async fn get_wizard_profile(
        &self,
        request: tonic::Request<GetWizardProfileRequest>,
    ) -> Result<tonic::Response<GetWizardProfileResponse>, tonic::Status> {
        self.impl_get_wizard_profile(request).await
    }


    async fn create_task(
        &self,
        request: Request<CreateTaskRequest>,
    ) -> Result<Response<SharedTask>, Status> {
        self.impl_create_task(request).await
    }


    async fn poll_tasks(
        &self,
        request: Request<PollTasksRequest>,
    ) -> Result<Response<Self::PollTasksStream>, Status> {
        self.impl_poll_tasks(request).await
    }


    async fn update_task_status(
        &self,
        request: Request<UpdateTaskStatusRequest>,
    ) -> Result<Response<UpdateTaskStatusResponse>, Status> {
        self.impl_update_task_status(request).await
    }


    async fn approve_task(
        &self,
        request: Request<ApproveTaskRequest>,
    ) -> Result<Response<ApproveTaskResponse>, Status> {
        self.impl_approve_task(request).await
    }


    async fn get_pending_approvals(
        &self,
        request: Request<GetPendingApprovalsRequest>,
    ) -> Result<Response<GetPendingApprovalsResponse>, Status> {
        self.impl_get_pending_approvals(request).await
    }


    async fn trigger_custom_order(
        &self,
        request: Request<TriggerCustomOrderRequest>,
    ) -> Result<Response<TriggerCustomOrderResponse>, Status> {
        self.impl_trigger_custom_order(request).await
    }


    async fn decompose_task(
        &self,
        request: Request<DecomposeTaskRequest>,
    ) -> Result<Response<DecomposeTaskResponse>, Status> {
        self.impl_decompose_task(request).await
    }


    async fn stream_messages(
        &self,
        request: Request<StreamMessagesRequest>,
    ) -> Result<Response<Self::StreamMessagesStream>, Status> {
        self.impl_stream_messages(request).await
    }


    async fn reason(
        &self,
        request: Request<ReasonRequest>,
    ) -> Result<Response<ReasonResponse>, Status> {
        self.impl_reason(request).await
    }


    async fn delegate_sub_task(
        &self,
        request: Request<SubTask>,
    ) -> Result<Response<DelegateTaskResponse>, Status> {
        self.impl_delegate_sub_task(request).await
    }


    async fn advertise_capabilities(
        &self,
        request: Request<AgentCapabilities>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        self.impl_advertise_capabilities(request).await
    }


    async fn discover_agents(
        &self,
        _request: Request<Query>,
    ) -> Result<Response<Self::DiscoverAgentsStream>, Status> {
        self.impl_discover_agents(_request).await
    }


    async fn publish_mesh_event(
        &self,
        request: Request<::server_ohc::orchestration::PublishMeshEventRequest>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        self.impl_publish_mesh_event(request).await
    }


    async fn stream_mesh_events(
        &self,
        request: Request<EventStreamRequest>,
    ) -> Result<Response<Self::StreamMeshEventsStream>, Status> {
        self.impl_stream_mesh_events(request).await
    }


    async fn publish_teammate_mesh_event(
        &self,
        request: Request<PublishTeammateMeshEventRequest>,
    ) -> Result<Response<PublishMessageResponse>, Status> {
        self.impl_publish_teammate_mesh_event(request).await
    }


    async fn stream_teammate_mesh(
        &self,
        request: Request<EventStreamRequest>,
    ) -> Result<Response<Self::StreamTeammateMeshStream>, Status> {
        self.impl_stream_teammate_mesh(request).await
    }


    async fn invite(
        &self,
        request: Request<InviteRequest>,
    ) -> Result<Response<InviteResponse>, Status> {
        self.impl_invite(request).await
    }


    async fn accept_invite(
        &self,
        request: Request<AcceptInviteRequest>,
    ) -> Result<Response<AcceptInviteResponse>, Status> {
        self.impl_accept_invite(request).await
    }


    async fn get_meetings(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<GetMeetingsResponse>, Status> {
        self.impl_get_meetings(_request).await
    }


    async fn start_onboarding(
        &self,
        request: Request<StartOnboardingRequest>,
    ) -> Result<Response<StartOnboardingResponse>, Status> {
        self.impl_start_onboarding(request).await
    }


}
