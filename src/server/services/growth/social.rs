use tonic::{Request, Response, Status};
use crate::ohc::orchestration::{
    GenerateSocialPostRequest, GenerateSocialPostResponse,
    ScheduleSocialPostRequest, ScheduleSocialPostResponse,
};

pub async fn generate_social_post(
    req: Request<GenerateSocialPostRequest>,
) -> Result<Response<GenerateSocialPostResponse>, Status> {
    let req = req.into_inner();

    // Simulate AI generation process with dynamic variations based on product info
    let content = if req.product_info.is_empty() {
        "Check out our amazing products! Limited time offer inside! ✨ #Sales #Growth".to_string()
    } else {
        format!("Big news! 🚀 We just released {} - perfectly designed for you. Check the link in our bio! #NewRelease #{}", req.product_info, req.strategy.replace(" ", ""))
    };

    Ok(Response::new(GenerateSocialPostResponse {
        drafted_post: content,
    }))
}

pub async fn schedule_social_post(
    req: Request<ScheduleSocialPostRequest>,
) -> Result<Response<ScheduleSocialPostResponse>, Status> {
    let req = req.into_inner();

    if req.content.is_empty() {
        return Err(Status::invalid_argument("Cannot schedule empty post"));
    }

    // Fake database interaction logic representing the actual scheduling
    // e.g. SQL INSERT INTO social_posts (content, platform, schedule_time)
    Ok(Response::new(ScheduleSocialPostResponse { success: true }))
}
