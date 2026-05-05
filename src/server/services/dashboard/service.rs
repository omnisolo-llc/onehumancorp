use tonic::{Request, Response, Status};
use crate::ohc::app::*;
use crate::ohc::app::dashboard_service_server::DashboardService;
use std::sync::Arc;

pub struct MyDashboardService {
    db: Arc<crate::db::DB>,
}

impl MyDashboardService {
    pub fn new(db: Arc<crate::db::DB>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl DashboardService for MyDashboardService {
    async fn get_dashboard(
        &self,
        _request: Request<GetDashboardRequest>,
    ) -> Result<Response<DashboardSnapshot>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn post_message(
        &self,
        _request: Request<PostMessageRequest>,
    ) -> Result<Response<PostMessageResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn seed_dashboard(
        &self,
        _request: Request<SeedDashboardRequest>,
    ) -> Result<Response<SeedDashboardResponse>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    async fn get_onboarding_state(
        &self,
        request: Request<GetOnboardingStateRequest>,
    ) -> Result<Response<GetOnboardingStateResponse>, Status> {
        let auth_info = request.extensions().get::<crate::auth::orchestration::AuthInfo>()
            .cloned()
            .ok_or_else(|| Status::unauthenticated("Missing authentication information"))?;

        let req = request.into_inner();
        let org_id = req.organization_id;

        if auth_info.org_id != "system" && auth_info.org_id != org_id {
            return Err(Status::permission_denied("You do not have permission to view this organization's state."));
        }

        use sqlx::Row;
        let res = sqlx::query("SELECT user_id, current_step, state_json FROM onboarding_state WHERE organization_id = $1 LIMIT 1")
            .bind(&org_id)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(row) = res {
            let state_json: serde_json::Value = row.try_get("state_json").unwrap_or_else(|_| serde_json::json!({}));
            Ok(Response::new(GetOnboardingStateResponse {
                state: Some(OnboardingState {
                    organization_id: org_id,
                    user_id: row.try_get("user_id").unwrap_or_default(),
                    current_step: row.try_get("current_step").unwrap_or_default(),
                    state_json: state_json.to_string(),
                }),
            }))
        } else {
            Err(Status::not_found("Onboarding state not found"))
        }
    }

    async fn get_video_tutorials(
        &self,
        _request: Request<GetVideoTutorialsRequest>,
    ) -> Result<Response<GetVideoTutorialsResponse>, Status> {
        let videos = vec![
            VideoMetadata {
                title: "How to add your first product".to_string(),
                description: "A quick 60-second guide to listing items in your store.".to_string(),
                duration_sec: 60,
                url: "https://ohc-video.example.com/tutorials/add_product.mp4".to_string(),
                thumbnail_url: "https://ohc-video.example.com/thumbnails/add_product.jpg".to_string(),
            },
            VideoMetadata {
                title: "Setting up AI Helpers".to_string(),
                description: "Learn how to let AI handle your customer emails and social media.".to_string(),
                duration_sec: 120,
                url: "https://ohc-video.example.com/tutorials/ai_helpers.mp4".to_string(),
                thumbnail_url: "https://ohc-video.example.com/thumbnails/ai_helpers.jpg".to_string(),
            },
        ];

        Ok(Response::new(GetVideoTutorialsResponse { videos }))
    }

    async fn update_onboarding_state(
        &self,
        request: Request<UpdateOnboardingStateRequest>,
    ) -> Result<Response<UpdateOnboardingStateResponse>, Status> {
        let auth_info = request.extensions().get::<crate::auth::orchestration::AuthInfo>()
            .cloned()
            .ok_or_else(|| Status::unauthenticated("Missing authentication information"))?;

        let req = request.into_inner();
        let state = req.state.ok_or_else(|| Status::invalid_argument("state is required"))?;

        if auth_info.org_id != "system" && auth_info.org_id != state.organization_id {
            return Err(Status::permission_denied("You do not have permission to update this organization's state."));
        }

        let state_json_val: serde_json::Value = serde_json::from_str(&state.state_json).map_err(|e| Status::invalid_argument(e.to_string()))?;

        sqlx::query(
            "UPDATE onboarding_state SET current_step = $1, state_json = $2, updated_at = CURRENT_TIMESTAMP WHERE organization_id = $3"
        )
        .bind(state.current_step)
        .bind(state_json_val)
        .bind(&state.organization_id)
        .execute(&self.db.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(UpdateOnboardingStateResponse { success: true }))
    }
}
