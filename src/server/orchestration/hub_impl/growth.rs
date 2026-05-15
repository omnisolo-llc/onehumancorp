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
    pub async fn impl_invite(
        &self,
        request: Request<InviteRequest>,
    ) -> Result<Response<InviteResponse>, Status> {
        let req = request.into_inner();

        if req.team_id.is_empty() || req.inviter_id.is_empty() || req.invitee_id.is_empty() {
            return Err(Status::invalid_argument("Missing required fields"));
        }

        self.invite_tracker.record_invite(&req.team_id, &req.inviter_id, &req.invitee_id).await
            .map_err(|e| Status::internal(format!("Failed to record invite: {}", e)))?;

        self.viral_loop_tracker.record_invite_sent(&req.inviter_id);

        Ok(Response::new(InviteResponse { success: true }))
    }
    pub async fn impl_accept_invite(
        &self,
        request: Request<AcceptInviteRequest>,
    ) -> Result<Response<AcceptInviteResponse>, Status> {
        let req = request.into_inner();

        if req.invitee_id.is_empty() {
            return Err(Status::invalid_argument("Missing invitee_id"));
        }

        self.viral_loop_tracker.record_invite_accepted(&req.invitee_id);

        Ok(Response::new(AcceptInviteResponse { success: true }))
    }
}
