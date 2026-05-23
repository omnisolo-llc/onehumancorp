use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Extension,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::hub::Hub;

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialPostRequest {
    pub content: String,
    pub platforms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialPostResponse {
    pub posted: bool,
    pub post_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignRequest {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub target_segment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignResponse {
    pub campaign_id: String,
    pub emails_sent: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackVisitorRequest {
    pub page_url: String,
    pub referrer: Option<String>,
    pub visitor_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackVisitorResponse {
    pub tracked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub id: String,
    pub title: String,
    pub description: String,
    pub reached: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MilestonesResponse {
    pub milestones: Vec<Milestone>,
}

pub fn router<S>(pool: PgPool, hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/social/post", post(handle_social_post))
        .route("/campaign/send", post(handle_send_campaign))
        .route("/storefront/track", post(handle_track_visitor))
        .route("/storefront/embed", get(handle_storefront_embed))
        .route("/milestones/check", get(handle_check_milestones))
        .route("/team-invites", get(handle_get_team_invites).post(handle_create_team_invite))
        .route("/team-invites/metrics", get(handle_team_invites_metrics))
        .route("/referrals/click", post(handle_referral_click))
        .route("/referrals/convert", post(handle_referral_convert))
        .route("/team-invites/accept", post(handle_team_invite_accept))
        .route("/referrals/generate", post(handle_referral_generate))
        .layer(Extension(GrowthState { pool, hub }))
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralIdRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteIdRequest {
    pub id: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralGenerateResponse {
    pub referral_link: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TeamInvitesMetricsResponse {
    pub total_invites: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamInviteRequest {
    pub team_id: String,
    pub inviter_id: String,
    pub invitee_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTeamInvitesQuery {
    pub team_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamInvitesResponse {
    pub invites: Vec<crate::services::growth::invites::TeamInvite>,
}

#[derive(Clone)]
struct GrowthState {
    pool: PgPool,
    hub: Arc<Hub>,
}

async fn handle_social_post(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<SocialPostRequest>,
) -> impl IntoResponse {
    Json(SocialPostResponse {
        posted: true,
        post_id: uuid::Uuid::new_v4().to_string(),
    })
}

async fn handle_send_campaign(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<CampaignRequest>,
) -> impl IntoResponse {
    Json(CampaignResponse {
        campaign_id: uuid::Uuid::new_v4().to_string(),
        emails_sent: 150,
    })
}

async fn handle_track_visitor(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<TrackVisitorRequest>,
) -> impl IntoResponse {
    Json(TrackVisitorResponse { tracked: true })
}

async fn handle_storefront_embed() -> impl IntoResponse {
    let html = r##"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body { margin: 0; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; }
        .card { border: 1px solid #eaeaea; border-radius: 8px; padding: 16px; max-width: 300px; box-shadow: 0 4px 6px rgba(0,0,0,0.05); }
        .title { font-size: 1.2rem; font-weight: bold; margin: 0 0 8px 0; }
        .price { color: #555; font-size: 1rem; margin: 0 0 16px 0; }
        .btn { display: block; width: 100%; text-align: center; background: #007bff; color: white; padding: 10px; text-decoration: none; border-radius: 4px; font-weight: bold; }
        .footer { text-align: center; margin-top: 16px; font-size: 0.85rem; }
        .footer a { color: #333; text-decoration: none; font-weight: bold; }
    </style>
</head>
<body>
    <div class="card">
        <h2 class="title">Premium Product</h2>
        <p class="price">$49.99</p>
        <a href="#" class="btn">Buy Now</a>
        <div class="footer" style="padding: 12px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 8px; margin-top: 20px; text-align: center;">
            <a href="https://ohc.store/join?ref=embed" style="color: white; text-decoration: none; font-weight: 600; display: block; font-size: 0.95rem; margin-bottom: 4px;">⚡ Powered by OHC</a>
            <p style="color: rgba(255,255,255,0.9); font-size: 0.8rem; margin: 0;">Launch your own AI storefront today and get $50 credit.</p>
        </div>
    </div>
</body>
</html>
"##;
    axum::response::Html(html)
}

async fn handle_check_milestones(
    Extension(_state): Extension<GrowthState>,
) -> impl IntoResponse {
    let milestones = vec![
        Milestone {
            id: "1".to_string(),
            title: String::from("First Teammate"),
            description: String::from("Hire your first AI agent"),
            reached: true,
        },
        Milestone {
            id: "2".to_string(),
            title: String::from("Global Reach"),
            description: String::from("Connect to a partner organization"),
            reached: false,
        },
        Milestone {
            id: "3".to_string(),
            title: "🎉 10th Order!".to_string(),
            description: "You've successfully processed your 10th order on OHC.".to_string(),
            reached: true,
        },
    ];
    Json(MilestonesResponse { milestones })
}

async fn handle_get_team_invites(
    Extension(state): Extension<GrowthState>,
    axum::extract::Query(query): axum::extract::Query<GetTeamInvitesQuery>,
) -> Result<Json<TeamInvitesResponse>, StatusCode> {
    let repo = std::sync::Arc::new(crate::services::growth::invites::InviteRepository::new(state.pool.clone()));
    let tracker = crate::services::growth::invites::InviteTracker::new(repo);

    match tracker.get_team_invites(&query.team_id).await {
        Ok(invites) => Ok(Json(TeamInvitesResponse { invites })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_team_invites_metrics(
    Extension(state): Extension<GrowthState>,
    axum::extract::Query(query): axum::extract::Query<GetTeamInvitesQuery>,
) -> Result<Json<TeamInvitesMetricsResponse>, StatusCode> {
    let repo = std::sync::Arc::new(crate::services::growth::invites::InviteRepository::new(state.pool.clone()));
    let tracker = crate::services::growth::invites::InviteTracker::new(repo);

    match tracker.get_team_invites_count(&query.team_id).await {
        Ok(total_invites) => Ok(Json(TeamInvitesMetricsResponse { total_invites })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_referral_click(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<ReferralIdRequest>,
) -> Result<Json<()>, StatusCode> {
    match sqlx::query("UPDATE referrals SET clicks = clicks + 1 WHERE id = $1")
        .bind(&req.id)
        .execute(&state.pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return Err(StatusCode::NOT_FOUND);
            }
            state.hub.referral_tracker().record_click(&req.id);
            Ok(Json(()))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_referral_convert(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<ReferralIdRequest>,
) -> Result<Json<()>, StatusCode> {
    match sqlx::query("UPDATE referrals SET conversions = conversions + 1 WHERE id = $1")
        .bind(&req.id)
        .execute(&state.pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return Err(StatusCode::NOT_FOUND);
            }
            state.hub.referral_tracker().record_conversion(&req.id);
            Ok(Json(()))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


async fn handle_referral_generate(
    Extension(state): Extension<GrowthState>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<ReferralGenerateResponse>, StatusCode> {
    let ref_code = uuid::Uuid::new_v4().to_string();
    let ref_id = uuid::Uuid::new_v4().to_string();
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;

    match sqlx::query("INSERT INTO referrals (id, organization_id, user_id, referral_code, clicks, conversions, created_at_unix) VALUES ($1, $2, $3, $4, 0, 0, $5)")
        .bind(&ref_id)
        .bind(&auth_info.org_id)
        .bind(&auth_info.agent_id)
        .bind(&ref_code)
        .bind(now)
        .execute(&state.pool)
        .await
    {
        Ok(_) => Ok(Json(ReferralGenerateResponse {
            referral_link: format!("https://ohc.app/ref/{}", ref_code),
        })),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_team_invite_accept(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<InviteIdRequest>,
) -> Result<Json<()>, StatusCode> {
    let repo = std::sync::Arc::new(crate::services::growth::invites::InviteRepository::new(state.pool.clone()));
    let tracker = crate::services::growth::invites::InviteTracker::new(repo);

    match tracker.accept_invite(&req.id).await {
        Ok(_) => Ok(Json(())),
        Err(e) if e == "not found" => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_create_team_invite(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<CreateTeamInviteRequest>,
) -> Result<Json<()>, StatusCode> {
    let repo = std::sync::Arc::new(crate::services::growth::invites::InviteRepository::new(state.pool.clone()));
    let tracker = crate::services::growth::invites::InviteTracker::new(repo);

    match tracker.record_invite(&req.team_id, &req.inviter_id, &req.invitee_id).await {
        Ok(_) => Ok(Json(())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Extension;
    use axum::Json;
    use axum::extract::Query;
    use sqlx::PgPool;

    async fn setup_db() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(500))
            .max_connections(1)
            .connect_lazy(&database_url)
            .expect("Failed to connect to DB");
        pool
    }

    #[tokio::test]
    async fn test_create_and_get_team_invites() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            println!("Skipping DB test, DB not available");
            return;
        }

        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let state = GrowthState { pool: pool.clone(), hub };

        let req = CreateTeamInviteRequest {
            team_id: "team-test-direct".to_string(),
            inviter_id: "user-xyz".to_string(),
            invitee_id: "user-abc".to_string(),
        };

        // Call create handler directly
        let res = handle_create_team_invite(Extension(state.clone()), Json(req)).await;
        assert!(res.is_ok());

        // Call get handler directly
        let query = GetTeamInvitesQuery {
            team_id: "team-test-direct".to_string(),
        };
        let get_res = handle_get_team_invites(Extension(state.clone()), Query(query)).await;
        assert!(get_res.is_ok());

        let get_res_json = get_res.unwrap().0;
        assert!(!get_res_json.invites.is_empty());

        let mut found = false;
        let mut invite_id = String::new();
        for inv in &get_res_json.invites {
            if inv.team_id == "team-test-direct" && inv.invitee_id == "user-abc" {
                found = true;
                invite_id = inv.id.clone();
                break;
            }
        }
        assert!(found);

        let accept_req = InviteIdRequest {
            id: invite_id,
        };
        let accept_res = handle_team_invite_accept(Extension(state.clone()), Json(accept_req)).await;
        assert!(accept_res.is_ok());

        // Call metrics handler directly
        let metrics_query = GetTeamInvitesQuery {
            team_id: "team-test-direct".to_string(),
        };
        let metrics_res = handle_team_invites_metrics(Extension(state.clone()), Query(metrics_query)).await;
        assert!(metrics_res.is_ok());
        let metrics_res_json = metrics_res.unwrap().0;
        assert_eq!(metrics_res_json.total_invites, 1);
    }

    #[tokio::test]
    async fn test_referral_click_and_convert() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            println!("Skipping DB test, DB not available");
            return;
        }

        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let state = GrowthState { pool: pool.clone(), hub: hub.clone() };

        // Insert dummy referral
        let ref_id = "ref-code-123";
        sqlx::query("INSERT INTO referrals (id, organization_id, user_id, referral_code, clicks, conversions, created_at_unix) VALUES ($1, 'org1', 'user1', 'code1', 0, 0, 0) ON CONFLICT DO NOTHING")
            .bind(ref_id)
            .execute(&pool).await.unwrap();

        let click_req = ReferralIdRequest {
            id: "ref-code-123".to_string(),
        };
        let res = handle_referral_click(Extension(state.clone()), Json(click_req)).await;
        assert!(res.is_ok());

        let convert_req = ReferralIdRequest {
            id: "ref-code-123".to_string(),
        };
        let res = handle_referral_convert(Extension(state.clone()), Json(convert_req)).await;
        assert!(res.is_ok());

        // Test missing referral
        let click_req_not_found = ReferralIdRequest {
            id: "ref-code-123-not-found".to_string(),
        };
        let res_not_found = handle_referral_click(Extension(state.clone()), Json(click_req_not_found)).await;
        assert!(res_not_found.is_err());
        assert_eq!(res_not_found.unwrap_err(), StatusCode::NOT_FOUND);

        let convert_req_not_found = ReferralIdRequest {
            id: "ref-code-123-not-found".to_string(),
        };
        let res2_not_found = handle_referral_convert(Extension(state.clone()), Json(convert_req_not_found)).await;
        assert!(res2_not_found.is_err());
        assert_eq!(res2_not_found.unwrap_err(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_referral_clicks_and_conversions() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            println!("Skipping DB test, DB not available");
            return;
        }

        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let state = GrowthState { pool: pool.clone(), hub: hub.clone() };

        // Insert dummy referral
        let ref_id = "test-ref-123";
        sqlx::query("INSERT INTO referrals (id, organization_id, user_id, referral_code, clicks, conversions, created_at_unix) VALUES ($1, 'org1', 'user1', 'code1', 0, 0, 0) ON CONFLICT DO NOTHING")
            .bind(ref_id)
            .execute(&pool).await.unwrap();

        let req = ReferralIdRequest { id: ref_id.to_string() };

        // Test Click
        let res = handle_referral_click(Extension(state.clone()), Json(req)).await;
        assert!(res.is_ok());

        let clicks: i32 = sqlx::query_scalar("SELECT clicks FROM referrals WHERE id = $1")
            .bind(ref_id)
            .fetch_one(&pool).await.unwrap();
        assert_eq!(clicks, 1);

        let req2 = ReferralIdRequest { id: ref_id.to_string() };
        // Test Convert
        let res2 = handle_referral_convert(Extension(state.clone()), Json(req2)).await;
        assert!(res2.is_ok());

        let conversions: i32 = sqlx::query_scalar("SELECT conversions FROM referrals WHERE id = $1")
            .bind(ref_id)
            .fetch_one(&pool).await.unwrap();
        assert_eq!(conversions, 1);
    }


    #[tokio::test]
    async fn test_referral_generate() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            println!("Skipping DB test, DB not available");
            return;
        }

        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let state = GrowthState { pool: pool.clone(), hub: hub.clone() };

        let auth_info = ::server_auth::orchestration::AuthInfo {
            spiffe_id: "spiffe://ohc.app/test".to_string(),
            org_id: "test-org".to_string(),
            agent_id: "test-agent".to_string(),
        };

        let res = handle_referral_generate(Extension(state.clone()), axum::extract::Extension(auth_info.clone())).await.unwrap();
        let ref_link = res.0.referral_link;
        assert!(ref_link.starts_with("https://ohc.app/ref/"));

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM referrals WHERE organization_id = 'test-org' AND user_id = 'test-agent'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_team_invite_accept() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            println!("Skipping DB test, DB not available");
            return;
        }

        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let state = GrowthState { pool: pool.clone(), hub: hub.clone() };

        // Insert dummy invite
        let invite_id = "test-invite-123";
        sqlx::query("INSERT INTO team_invites (id, team_id, inviter_id, invitee_id, status, created_at, updated_at) VALUES ($1, 'team1', 'inviter1', 'invitee1', 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) ON CONFLICT DO NOTHING")
            .bind(invite_id)
            .execute(&pool).await.unwrap();

        let req = InviteIdRequest { id: invite_id.to_string() };

        let res = handle_team_invite_accept(Extension(state.clone()), Json(req)).await;
        assert!(res.is_ok());

        let status: String = sqlx::query_scalar("SELECT status FROM team_invites WHERE id = $1")
            .bind(invite_id)
            .fetch_one(&pool).await.unwrap();
        assert_eq!(status, "ACCEPTED");

        // Test missing invite
        let missing_req = InviteIdRequest { id: "missing-invite-404".to_string() };
        let res_missing = handle_team_invite_accept(Extension(state.clone()), Json(missing_req)).await;
        assert!(res_missing.is_err());
        assert_eq!(res_missing.unwrap_err(), StatusCode::NOT_FOUND);
    }
}
