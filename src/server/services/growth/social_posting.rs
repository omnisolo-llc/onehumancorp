use tonic::{Request, Response, Status};
use crate::ohc::orchestration::social_posting_service_server::SocialPostingService;
use crate::ohc::orchestration::{
    ConnectSocialPlatformRequest, ConnectSocialPlatformResponse, GenerateSocialPostRequest,
    GenerateSocialPostResponse, ScheduleSocialPostRequest, ScheduleSocialPostResponse,
};

#[derive(Default)]
pub struct MySocialPostingService {}

impl MySocialPostingService {
    pub fn new() -> Self {
        MySocialPostingService {}
    }
}

#[tonic::async_trait]
impl SocialPostingService for MySocialPostingService {
    async fn generate_social_post(
        &self,
        request: Request<GenerateSocialPostRequest>,
    ) -> Result<Response<GenerateSocialPostResponse>, Status> {
        let req = request.into_inner();
        let content = format!("Generated AI post for: {}. Check out our awesome new product! 🚀", req.prompt);
        Ok(Response::new(GenerateSocialPostResponse { content }))
    }

    async fn schedule_social_post(
        &self,
        _request: Request<ScheduleSocialPostRequest>,
    ) -> Result<Response<ScheduleSocialPostResponse>, Status> {
        Ok(Response::new(ScheduleSocialPostResponse { success: true }))
    }

    async fn connect_social_platform(
        &self,
        _request: Request<ConnectSocialPlatformRequest>,
    ) -> Result<Response<ConnectSocialPlatformResponse>, Status> {
        Ok(Response::new(ConnectSocialPlatformResponse { success: true }))
    }
}
