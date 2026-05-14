use tonic::{Request, Response, Status};
use crate::proto::{
    GetHelpCenterRequest, GetHelpCenterResponse, HelpArticle,
    GetTooltipsRequest, GetTooltipsResponse, HelpTooltip,
    GetHelpVideosRequest, GetHelpVideosResponse, HelpVideo,
};

pub struct HelpServiceImpl {}

impl HelpServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl crate::proto::help_service_server::HelpService for HelpServiceImpl {
    async fn get_help_articles(
        &self,
        request: Request<GetHelpCenterRequest>,
    ) -> Result<Response<GetHelpCenterResponse>, Status> {
        let _req = request.into_inner();
        let articles = vec![
            HelpArticle {
                id: "1".to_string(),
                title: "Getting Started with OneHuman".to_string(),
                content: "Welcome to OneHuman! We're excited to help you set up your small business.".to_string(),
                category: "Getting Started".to_string(),
                summary: "Learn the basics of setting up your account and launching your store.".to_string(),
            },
            HelpArticle {
                id: "2".to_string(),
                title: "Accepting Payments".to_string(),
                content: "Connect your bank account to start accepting credit card payments from your customers.".to_string(),
                category: "Payments".to_string(),
                summary: "How to connect a bank account and accept payments.".to_string(),
            },
            HelpArticle {
                id: "3".to_string(),
                title: "Adding AI Support Agents".to_string(),
                content: "You can hire AI support agents to answer common customer questions automatically.".to_string(),
                category: "AI Agents".to_string(),
                summary: "Hire an AI agent to handle customer support.".to_string(),
            },
        ];

        Ok(Response::new(GetHelpCenterResponse { articles }))
    }

    async fn get_tooltips(
        &self,
        request: Request<GetTooltipsRequest>,
    ) -> Result<Response<GetTooltipsResponse>, Status> {
        let _req = request.into_inner();
        let tooltips = vec![
            HelpTooltip {
                id: "tt_1".to_string(),
                content: "Click here to add your store logo.".to_string(),
                target_element: "logo-upload".to_string(),
            },
            HelpTooltip {
                id: "tt_2".to_string(),
                content: "Set the price your customers will pay.".to_string(),
                target_element: "price-input".to_string(),
            },
        ];

        Ok(Response::new(GetTooltipsResponse { tooltips }))
    }

    async fn get_help_videos(
        &self,
        request: Request<GetHelpVideosRequest>,
    ) -> Result<Response<GetHelpVideosResponse>, Status> {
        let _req = request.into_inner();
        let videos = vec![
            HelpVideo {
                id: "v_1".to_string(),
                title: "How to add a product".to_string(),
                url: "https://example.com/videos/add-product.mp4".to_string(),
                thumbnail: "https://example.com/videos/add-product-thumb.jpg".to_string(),
            },
            HelpVideo {
                id: "v_2".to_string(),
                title: "Connecting your bank account".to_string(),
                url: "https://example.com/videos/connect-bank.mp4".to_string(),
                thumbnail: "https://example.com/videos/connect-bank-thumb.jpg".to_string(),
            },
        ];

        Ok(Response::new(GetHelpVideosResponse { videos }))
    }
}
