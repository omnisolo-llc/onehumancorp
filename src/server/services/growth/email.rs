use tonic::{Request, Response, Status};
use crate::ohc::orchestration::{
    GenerateEmailTemplateRequest, GenerateEmailTemplateResponse,
    SendEmailCampaignRequest, SendEmailCampaignResponse,
};

pub async fn generate_email_template(
    req: Request<GenerateEmailTemplateRequest>,
) -> Result<Response<GenerateEmailTemplateResponse>, Status> {
    let req = req.into_inner();

    // Simulated AI text generation using dynamic templates
    let template = match req.theme.to_lowercase().as_str() {
        "flash sale" => "Hey there!

We are running a massive 24-hour flash sale. Don't miss out on these exclusive discounts!

Shop Now",
        "new arrivals" => "Welcome to the new season!

Check out the latest additions to our store. Fresh looks and premium quality await you.",
        "thank you" => "We appreciate your business!

Thank you for being a valued customer. Here is a special 10% discount code for your next purchase: THANKYOU10.",
        _ => "Hello from our store!

We have exciting updates and new products waiting for you. Come and take a look."
    }.to_string();

    Ok(Response::new(GenerateEmailTemplateResponse {
        template_body: template,
    }))
}

pub async fn send_email_campaign(
    req: Request<SendEmailCampaignRequest>,
) -> Result<Response<SendEmailCampaignResponse>, Status> {
    let req = req.into_inner();

    if req.template_body.is_empty() {
        return Err(Status::invalid_argument("Email template cannot be empty"));
    }

    // In a real implementation this would trigger SendGrid or Amazon SES
    let sent = if req.audience.contains("All Customers") {
        250 // Mock db count
    } else {
        42
    };

    Ok(Response::new(SendEmailCampaignResponse {
        success: true,
        emails_sent: sent,
    }))
}
