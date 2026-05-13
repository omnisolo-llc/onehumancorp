use tonic::{Request, Response, Status};
use crate::ohc::orchestration::email_marketing_service_server::EmailMarketingService;
use crate::ohc::orchestration::{
    GenerateEmailTemplateRequest, GenerateEmailTemplateResponse, SendEmailCampaignRequest,
    SendEmailCampaignResponse,
};

#[derive(Default)]
pub struct MyEmailMarketingService {}

impl MyEmailMarketingService {
    pub fn new() -> Self {
        MyEmailMarketingService {}
    }
}

#[tonic::async_trait]
impl EmailMarketingService for MyEmailMarketingService {
    async fn generate_email_template(
        &self,
        request: Request<GenerateEmailTemplateRequest>,
    ) -> Result<Response<GenerateEmailTemplateResponse>, Status> {
        let req = request.into_inner();
        let content = format!("Subject: Don't miss out on our {}!\n\nHi there,\nWe're excited to announce our latest {}. Shop now!", req.template_type, req.template_type);
        Ok(Response::new(GenerateEmailTemplateResponse { content }))
    }

    async fn send_email_campaign(
        &self,
        _request: Request<SendEmailCampaignRequest>,
    ) -> Result<Response<SendEmailCampaignResponse>, Status> {
        Ok(Response::new(SendEmailCampaignResponse { success: true }))
    }
}
