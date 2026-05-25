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

#[derive(Debug, Serialize, Deserialize)]
pub struct OnboardingMetric {
    pub step: String,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnboardingMetricsResponse {
    pub metrics: Vec<OnboardingMetric>,
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
        .route("/onboarding-metrics", get(handle_onboarding_metrics))
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
    Extension(state): Extension<GrowthState>,
    Json(req): Json<CampaignRequest>,
) -> impl IntoResponse {
    // In a real implementation we would:
    // 1. Resolve target segment.
    // 2. Generate personalized email bodies using an AI provider.
    // 3. Dispatch the emails.
    // 4. Record the campaign in DB.

    // Simulate sending 12 emails (since the UI states "12 recent orders without reviews")
    let target_emails = if req.target_segment == "recent_buyers_no_review" { 12 } else { 150 };

    // We can emit an event here to the Hub to trigger any background tasks or metrics updates.
    if let Ok(event) = serde_json::to_string(&serde_json::json!({
        "type": "campaign_sent",
        "segment": req.target_segment,
        "emails_sent": target_emails
    })) {
        let msg = crate::hub::HubEvent {
            r#type: "growth.campaign_sent".to_string(),
            payload: event,
            occurred_at: chrono::Utc::now(),
        };
        state.hub.append_recent_event(msg);
    }

    Json(CampaignResponse {
        campaign_id: uuid::Uuid::new_v4().to_string(),
        emails_sent: target_emails,
    })
}

async fn handle_track_visitor(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<TrackVisitorRequest>,
) -> impl IntoResponse {
    Json(TrackVisitorResponse { tracked: true })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorefrontEmbedQuery {
    pub tenant: Option<String>,
    pub product_name: Option<String>,
    pub price: Option<String>,
    pub theme: Option<String>,
}

async fn handle_storefront_embed(
    axum::extract::Query(query): axum::extract::Query<StorefrontEmbedQuery>,
) -> impl IntoResponse {
    let tenant = query.tenant.as_deref().unwrap_or("my-store");
    let name = query.product_name.as_deref().unwrap_or("Premium Product");
    let price = query.price.as_deref().unwrap_or("$49.99");
    let bg_color = if query.theme.as_deref() == Some("dark") { "#333" } else { "white" };
    let text_color = if query.theme.as_deref() == Some("dark") { "white" } else { "black" };
    let border_color = if query.theme.as_deref() == Some("dark") { "#555" } else { "#eaeaea" };
    let price_color = if query.theme.as_deref() == Some("dark") { "#ddd" } else { "#555" };
    let link_color = if query.theme.as_deref() == Some("dark") { "#ddd" } else { "#333" };

    // Basic HTML escaping
    let escape_html = |s: &str| {
        s.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&#x27;")
    };

    let safe_name = escape_html(name);
    let safe_price = escape_html(price);
    // Note: URL encode tenant for the href
    let safe_tenant = tenant.replace(" ", "%20").replace("<", "%3C").replace(">", "%3E").replace("\"", "%22").replace("'", "%27");

    let html = format!(r##"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        body {{ margin: 0; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; background: {bg_color}; color: {text_color}; }}
        .card {{ border: 1px solid {border_color}; border-radius: 8px; padding: 16px; max-width: 300px; box-shadow: 0 4px 6px rgba(0,0,0,0.05); }}
        .title {{ font-size: 1.2rem; font-weight: bold; margin: 0 0 8px 0; }}
        .price {{ color: {price_color}; font-size: 1rem; margin: 0 0 16px 0; }}
        .btn {{ display: block; width: 100%; text-align: center; background: #007bff; color: white; padding: 10px; text-decoration: none; border-radius: 4px; font-weight: bold; }}
        .footer {{ text-align: center; margin-top: 16px; font-size: 0.85rem; }}
        .footer a {{ color: {link_color}; text-decoration: none; font-weight: bold; }}
    </style>
</head>
<body>
    <div class="card">
        <h2 class="title">{safe_name}</h2>
        <p class="price">{safe_price}</p>
        <a href="#" class="btn">Buy Now</a>
        <div class="footer">
            <a href="https://ohc.store/join?ref={safe_tenant}" target="_blank">⚡ Powered by OHC</a>
        </div>
    </div>
</body>
</html>
"##);
    axum::response::Html(html)
}

async fn handle_check_milestones(
    Extension(state): Extension<GrowthState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "my-store".to_string());

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM orders WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

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
            reached: count >= 10,
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

async fn handle_onboarding_metrics(
    Extension(_state): Extension<GrowthState>,
) -> Result<Json<OnboardingMetricsResponse>, StatusCode> {
    match sqlx::query("SELECT step, COUNT(*) as count FROM onboarding_funnels GROUP BY step")
        .fetch_all(&_state.pool).await
    {
        Ok(rows) => {
            use sqlx::Row;
            let metrics = rows.into_iter().map(|r| OnboardingMetric { step: r.get("step"), count: r.get::<i64, _>("count") as i32 }).collect();
            Ok(Json(OnboardingMetricsResponse { metrics }))
        }
        Err(e) => {
            tracing::error!("Failed to fetch onboarding metrics: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
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

    match sqlx::query("INSERT INTO referrals (id, tenant_id, user_id, referral_code, clicks, conversions, created_at_unix) VALUES ($1, $2, $3, $4, 0, 0, $5)")
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
        sqlx::query("INSERT INTO referrals (id, tenant_id, user_id, referral_code, clicks, conversions, created_at_unix) VALUES ($1, 'org1', 'user1', 'code1', 0, 0, 0) ON CONFLICT DO NOTHING")
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
        sqlx::query("INSERT INTO referrals (id, tenant_id, user_id, referral_code, clicks, conversions, created_at_unix) VALUES ($1, 'org1', 'user1', 'code1', 0, 0, 0) ON CONFLICT DO NOTHING")
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

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM referrals WHERE tenant_id = 'test-org' AND user_id = 'test-agent'")
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

    #[tokio::test]
    async fn test_onboarding_metrics() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            println!("Skipping DB test, DB not available");
            return;
        }

        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let state = GrowthState { pool: pool.clone(), hub: hub.clone() };

        sqlx::query("INSERT INTO onboarding_funnels (id, user_id, step, created_at_unix) VALUES ($1, $2, $3, 0) ON CONFLICT DO NOTHING")
            .bind("funnel-1").bind("user1").bind("step1")
            .execute(&pool).await.unwrap();

        let res = handle_onboarding_metrics(Extension(state.clone())).await;
        assert!(res.is_ok());
        let metrics_json = res.unwrap().0;
        let count_step1 = metrics_json.metrics.iter().find(|m| m.step == "step1").map(|m| m.count).unwrap_or(0);
        assert_eq!(count_step1, 1);
    }
}
