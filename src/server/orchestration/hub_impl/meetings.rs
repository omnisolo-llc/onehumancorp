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
    pub async fn impl_open_meeting(
        &self,
        request: Request<OpenMeetingRequest>,
    ) -> Result<Response<MeetingRoom>, Status> {
        let req = request.into_inner();
        let meeting = self.hub.open_meeting(req.meeting_id, req.participants, req.agenda);
        Ok(Response::new(meeting))
    }
    pub async fn impl_get_meetings(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<GetMeetingsResponse>, Status> {
        let meetings = self.hub.get_meetings();
        Ok(Response::new(GetMeetingsResponse { meetings: meetings.to_vec() }))
    }
}
