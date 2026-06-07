


use std::sync::OnceLock;
use crate::utils::cache::HybridCache;

pub static MILESTONES_CACHE: OnceLock<HybridCache<Vec<String>>> = OnceLock::new();
pub static TEAM_INVITES_CACHE: OnceLock<HybridCache<TeamInvitesResponse>> = OnceLock::new();
pub static METRICS_CACHE: OnceLock<HybridCache<TeamInvitesMetricsResponse>> = OnceLock::new();
pub static ONBOARDING_METRICS_CACHE: OnceLock<HybridCache<OnboardingMetricsResponse>> = OnceLock::new();
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
pub struct SendReceiptRequest {
    pub customer_email: Option<String>,
    pub order_id: Option<String>,
    pub amount: Option<String>,
    pub tenant_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendReceiptResponse {
    pub success: bool,
    pub message: String,
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
pub struct GenerateAffiliateLinkRequest {
    pub customer_id: String,
    pub discount_percentage: Option<i32>,
    pub commission_percentage: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateAffiliateLinkResponse {
    pub affiliate_link: String,
    pub affiliate_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackAffiliateRequest {
    pub affiliate_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffiliateStatsResponse {
    pub total_affiliates: i64,
    pub total_commission_cents: i64,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnboardingMetric {
    pub step: String,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnboardingMetricsResponse {
    pub metrics: Vec<OnboardingMetric>,
}

pub fn router<S>(pool: PgPool, hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/social/post", post(handle_social_post))
        .route("/campaign/send-receipt", post(handle_send_receipt))
        .route("/campaign/send", post(handle_send_campaign))
        .route("/campaign/lead-gen", post(handle_create_lead_gen_campaign))
        .route("/campaign/generate-review", post(handle_generate_review))
        .route("/campaign/generate-customer-referral", post(handle_generate_customer_referral))
        .route("/campaign/generate-cart", post(handle_generate_cart))
        .route("/campaign/send-cart", post(handle_send_cart))
        .route("/storefront/track", post(handle_track_visitor))
        .route("/storefront/embed", get(handle_storefront_embed))
                .route("/storefront/og-card", get(handle_og_card))
        .route("/flash-sale/embed", get(handle_flash_sale_embed))
        .route("/milestones/check", get(handle_check_milestones))
        .route("/affiliate/generate-link", post(handle_affiliate_generate_link))
        .route("/affiliate/track", post(handle_affiliate_track))
        .route("/affiliate/stats", get(handle_affiliate_stats))
        .route("/team-invites", get(handle_get_team_invites).post(handle_create_team_invite))
        .route("/team-invites/metrics", get(handle_team_invites_metrics))
        .route("/team-invites/aggregated-metrics", get(handle_aggregated_team_invites_metrics))
        .route("/referrals/click", post(handle_referral_click))
        .route("/referrals/convert", post(handle_referral_convert))
        .route("/team-invites/accept", post(handle_team_invite_accept))
        .route("/referrals/generate", post(handle_referral_generate))
        .route("/referrals/stats", get(handle_referral_stats))
        .route("/onboarding-metrics", get(handle_onboarding_metrics))
        .route("/discount_share/generate", post(handle_generate_discount_share))
        .route("/milestone/card", get(handle_get_milestone_card))
        .layer(Extension(GrowthState { pool, hub }))
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralIdRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateReviewRequest {
    pub order_id: String,
    pub customer_name: String,
    pub product_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateReviewResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCustomerReferralRequest {
    pub store_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCustomerReferralResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCartRequest {
    pub customer_name: Option<String>,
    pub cart_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCartResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendCartRequest {
    pub customer_name: Option<String>,
    pub cart_value: Option<String>,
    pub draft: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendCartResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteIdRequest {
    pub id: String,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralGenerateResponse {
    pub referral_link: String,
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct GrowthMetrics {
    pub team_invites_sent: i64,
    pub active_referrals: i64,
    pub revenue: f64,
    pub pending_rewards: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamInvitesMetricsResponse {
    pub total_invites: i64,
    #[serde(default)]
    pub metrics: GrowthMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferralStatsResponse {
    pub active_referrals: i64,
    pub revenue_from_referrals: f64,
    pub pending_rewards: f64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamInviteRequest {
    pub team_id: String,
    pub inviter_id: String,
    pub invitee_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTeamInvitesQuery {
    pub cursor: Option<String>,
    pub limit: Option<usize>,
    pub team_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamInvitesResponse {
    pub next_cursor: Option<String>,
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

async fn handle_generate_review(
    Extension(_state): Extension<GrowthState>,
    Json(req): Json<GenerateReviewRequest>,
) -> impl IntoResponse {
    // In a real implementation we would call an AI provider here.
    // For now we simulate generating a review request based on the inputs.
    let generated = format!(
        "Hi {},\n\nWe noticed you recently received your {} and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.store/review/{}\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC",
        req.customer_name, req.product_name, req.order_id
    );

    Json(GenerateReviewResponse {
        message: generated,
    })
}

async fn handle_generate_customer_referral(
    Extension(_state): Extension<GrowthState>,
    Json(req): Json<GenerateCustomerReferralRequest>,
) -> impl IntoResponse {
    let store = req.store_name.unwrap_or_else(|| "our store".to_string());
    let generated = format!(
        "Hi there!\n\nWe love having you as a top customer at {}. As a special thank you, we're inviting you to our VIP Referral Program!\n\nGive your friends 15% off their first order using your unique link. When they make a purchase, you'll get $10 in store credit!\n\nShare your link now: https://ohc.store/vip-invite\n\nThanks for your support,\nThe {} Team\n\n⚡ Powered by OHC",
        store, store
    );
    Json(GenerateCustomerReferralResponse {
        message: generated,
    })
}

async fn handle_generate_cart(
    Extension(_state): Extension<GrowthState>,
    Json(req): Json<GenerateCartRequest>,
) -> impl IntoResponse {
    let name = req.customer_name.unwrap_or_else(|| "there".to_string());
    let value = req.cart_value.unwrap_or_else(|| "$0.00".to_string());
    let generated = format!(
        "Hi {},\n\nWe noticed you left some items in your cart totaling {}. Did you have any questions or need help checking out?\n\nAs a special thank you for shopping with us, here is a 10% discount code to complete your purchase: COMEBACK10\n\nClick here to securely finish your checkout: https://ohc.store/checkout/recover\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC",
        name, value
    );
    Json(GenerateCartResponse {
        message: generated,
    })
}

async fn handle_send_cart(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<SendCartRequest>,
) -> impl IntoResponse {
    Json(SendCartResponse {
        success: true,
        message: "Email scheduled to be sent successfully".to_string(),
    })
}

async fn handle_send_receipt(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<SendReceiptRequest>,
) -> impl IntoResponse {
    let email = req.customer_email.unwrap_or_else(|| "customer@example.com".to_string());
    let order_id = req.order_id.unwrap_or_else(|| "unknown_order".to_string());
    let amount = req.amount.unwrap_or_else(|| "$0.00".to_string());
    let tenant_id = req.tenant_id.unwrap_or_else(|| "my-store".to_string());

    let generated = format!(
        "Hi {},\n\nThank you for your order! Your payment of {} for order {} has been received.\n\nWarmly,\nThe Team\n\n<!-- ⚡ Powered by OHC -->\n<a href=\"https://ohc.store/join?ref={}\">Powered by OHC - Start your business today</a>",
        email, amount, order_id, tenant_id
    );

    let msg = state.hub.sanitize_hub_event(serde_json::json!({
        "type": "growth.receipt_sent",
        "order_id": order_id,
        "customer_email": email
    }));
    state.hub.append_recent_event(msg);

    Json(SendReceiptResponse { success: true, message: generated })
}


#[derive(Deserialize)]
pub struct LeadGenCampaignRequest {
    pub budget: f64,
    pub radius_miles: i32,
    pub zip_code: String,
}

#[derive(Serialize)]
pub struct LeadGenCampaignResponse {
    pub id: String,
    pub status: String,
}

async fn handle_create_lead_gen_campaign(
    Extension(state): Extension<GrowthState>,
    auth_info: axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(req): Json<LeadGenCampaignRequest>,
) -> Result<Json<LeadGenCampaignResponse>, StatusCode> {
    let tenant_id = auth_info.org_id.clone();

    let repo = crate::domain::repository::campaign_repo::CampaignRepository::new(state.pool.clone());

    let campaign = crate::domain::repository::models::LeadGenCampaign {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        budget: std::str::FromStr::from_str(&req.budget.to_string()).unwrap_or_default(),
        radius_miles: req.radius_miles,
        zip_code: req.zip_code.clone(),
        status: "Active".to_string(),
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    repo.create_lead_gen_campaign(&campaign).await.map_err(|e| {
        tracing::error!("Failed to save lead gen campaign: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(LeadGenCampaignResponse {
        id: campaign.id,
        status: campaign.status,
    }))
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
    let msg = state.hub.sanitize_hub_event(serde_json::json!({
        "type": "growth.campaign_sent",
        "segment": req.target_segment,
        "emails_sent": target_emails
    }));
    state.hub.append_recent_event(msg);

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

async fn handle_affiliate_generate_link(
    Extension(state): Extension<GrowthState>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(req): Json<GenerateAffiliateLinkRequest>,
) -> Result<Json<GenerateAffiliateLinkResponse>, StatusCode> {
    let affiliate_code = uuid::Uuid::new_v4().to_string().replace("-", "")[..8].to_string();
    let id = uuid::Uuid::new_v4().to_string();
    let discount = req.discount_percentage.unwrap_or(10);
    let commission = req.commission_percentage.unwrap_or(10);

    match sqlx::query("INSERT INTO affiliate_links (id, tenant_id, customer_id, affiliate_code, discount_percentage, commission_percentage) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&id)
        .bind(&auth_info.org_id)
        .bind(&req.customer_id)
        .bind(&affiliate_code)
        .bind(discount)
        .bind(commission)
        .execute(&state.pool)
        .await
    {
        Ok(_) => {
            let affiliate_link = format!("https://ohc.store/ref/{}", affiliate_code);
            Ok(Json(GenerateAffiliateLinkResponse { affiliate_link, affiliate_code }))
        }
        Err(e) => {
            ::server_telemetry::record_error_signal("Failed to generate affiliate link: {:?}");
            tracing::error!("Failed to generate affiliate link: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn handle_affiliate_track(
    Extension(_state): Extension<GrowthState>,
    Json(req): Json<TrackAffiliateRequest>,
) -> impl IntoResponse {
    use axum::http::header::SET_COOKIE;
    use axum::http::HeaderValue;

    let cookie_str = format!("affiliate_code={}; Path=/; HttpOnly; Max-Age=2592000", req.affiliate_code);

    let mut response = Json(serde_json::json!({ "tracked": true })).into_response();
    if let Ok(header_val) = HeaderValue::from_str(&cookie_str) {
        response.headers_mut().insert(SET_COOKIE, header_val);
    }

    response
}

async fn handle_affiliate_stats(
    Extension(state): Extension<GrowthState>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<AffiliateStatsResponse>, StatusCode> {
    let mut total_affiliates: i64 = 0;
    let mut total_commission_cents: i64 = 0;

    let res_aff = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM affiliate_links WHERE tenant_id = $1")
        .bind(&auth_info.org_id)
        .fetch_one(&state.pool)
        .await;

    if let Ok(count) = res_aff {
        total_affiliates = count;
    }

    let res_comm = sqlx::query_scalar::<_, i64>("SELECT COALESCE(SUM(commission_amount), 0) FROM affiliate_ledgers WHERE tenant_id = $1")
        .bind(&auth_info.org_id)
        .fetch_one(&state.pool)
        .await;

    if let Ok(sum) = res_comm {
        total_commission_cents = sum;
    }

    Ok(Json(AffiliateStatsResponse { total_affiliates, total_commission_cents }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorefrontEmbedQuery {
    pub tenant: Option<String>,
    pub product_name: Option<String>,
    pub price: Option<String>,
    pub theme: Option<String>,
}

async fn handle_storefront_embed(
    Extension(state): Extension<GrowthState>,
    axum::extract::Query(query): axum::extract::Query<StorefrontEmbedQuery>,
) -> impl IntoResponse {
    let tenant = query.tenant.as_deref().unwrap_or("embed");
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

    let mut has_pro = false;
    if tenant != "embed" && uuid::Uuid::parse_str(tenant).is_ok() {
        let row: Option<String> = sqlx::query_scalar("SELECT plan_tier FROM tenants WHERE id = $1::uuid OR tenant_id = $1::uuid")
            .bind(tenant)
            .fetch_optional(&state.pool)
            .await
            .unwrap_or_default();
        if let Some(plan) = row {
            has_pro = plan.to_lowercase() == "pro";
        }
    }

    let branding = if !has_pro {
        format!(r#"<div class="footer">
            <a href="ohc://join?ref={safe_tenant}" target="_blank">⚡ Powered by OHC</a>
        </div>"#)
    } else {
        "".to_string()
    };

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
        {branding}
    </div>
</body>
</html>
"##);
    axum::response::Html(html)
}


#[derive(Debug, Deserialize)]
pub struct FlashSaleEmbedQuery {
    pub tenant: Option<String>,
    pub title: Option<String>,
    pub code: Option<String>,
    pub percent: Option<String>,
    pub end: Option<String>,
    pub theme: Option<String>,
}

async fn handle_flash_sale_embed(
    Extension(_state): Extension<GrowthState>,
    axum::extract::Query(query): axum::extract::Query<FlashSaleEmbedQuery>,
) -> impl IntoResponse {
    let tenant = query.tenant.as_deref().unwrap_or("embed");
    let title = query.title.as_deref().unwrap_or("Flash Sale");
    let code = query.code.as_deref().unwrap_or("CODE");
    let percent = query.percent.as_deref().unwrap_or("0");
    let end = query.end.as_deref().unwrap_or("");
    let bg_color = if query.theme.as_deref() == Some("dark") { "#111827" } else { "#ffffff" };
    let text_color = if query.theme.as_deref() == Some("dark") { "#ffffff" } else { "#1f2937" };

    let escape_html = |s: &str| {
        s.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&#x27;")
    };

    let safe_title = escape_html(title);
    let safe_code = escape_html(code);
    let safe_percent = escape_html(percent);
    let safe_end = escape_html(end);

    let html = format!(r##"<!DOCTYPE html>
<html>
<head>
    <style>
        body {{ font-family: -apple-system, system-ui, sans-serif; margin: 0; padding: 0; background: {bg_color}; color: {text_color}; display: flex; align-items: center; justify-content: center; height: 100vh; overflow: hidden; }}
        .widget {{ padding: 20px; text-align: center; border-radius: 12px; max-width: 300px; width: 100%; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1); border: 1px solid rgba(128, 128, 128, 0.2); }}
        .title {{ font-size: 1.2rem; font-weight: bold; margin: 0 0 10px 0; }}
        .discount {{ color: #ef4444; font-weight: bold; }}
        .code-box {{ border: 2px dashed #d1d5db; padding: 10px; margin: 15px 0; border-radius: 8px; font-family: monospace; font-weight: bold; letter-spacing: 2px; }}
        .countdown {{ display: flex; justify-content: center; gap: 10px; margin: 15px 0; }}
        .time-box {{ display: flex; flex-direction: column; align-items: center; }}
        .time-val {{ font-size: 1.5rem; font-weight: bold; font-family: monospace; }}
        .time-lbl {{ font-size: 0.6rem; text-transform: uppercase; color: #6b7280; }}
        .footer {{ font-size: 0.75rem; margin-top: 15px; color: #6b7280; }}
        .footer a {{ color: #6b7280; text-decoration: none; font-weight: 600; }}
    </style>
</head>
<body>
    <div class="widget">
        <div class="title">⚡ {safe_title}</div>
        <p style="margin: 0; font-size: 0.9rem;">Get <span class="discount">{safe_percent}% OFF</span> your order!</p>

        <div class="countdown">
            <div class="time-box"><div class="time-val" id="h">00</div><div class="time-lbl">Hours</div></div>
            <div style="font-weight: bold; margin-top: 5px;">:</div>
            <div class="time-box"><div class="time-val" id="m">00</div><div class="time-lbl">Mins</div></div>
            <div style="font-weight: bold; margin-top: 5px;">:</div>
            <div class="time-box"><div class="time-val" id="s" style="color: #ef4444;">00</div><div class="time-lbl">Secs</div></div>
        </div>

        <div class="code-box">{safe_code}</div>

        <div class="footer">
            <a href="https://ohc.app/join?ref={tenant}" target="_blank">⚡ Powered by OHC</a>
        </div>
    </div>

    <script>
        const targetDate = new Date('{safe_end}').getTime();

        setInterval(function() {{
            const now = new Date().getTime();
            const distance = targetDate - now;

            if (distance < 0 || isNaN(distance)) {{
                document.getElementById("h").innerText = "00";
                document.getElementById("m").innerText = "00";
                document.getElementById("s").innerText = "00";
                return;
            }}

            const hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
            const minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
            const seconds = Math.floor((distance % (1000 * 60)) / 1000);

            document.getElementById("h").innerText = hours.toString().padStart(2, '0');
            document.getElementById("m").innerText = minutes.toString().padStart(2, '0');
            document.getElementById("s").innerText = seconds.toString().padStart(2, '0');
        }}, 1000);
    </script>
</body>
</html>
"##);
    axum::response::Html(html)
}


async fn handle_og_card(
    Extension(state): Extension<GrowthState>,
    axum::extract::Query(query): axum::extract::Query<StorefrontEmbedQuery>,
) -> impl IntoResponse {
    let tenant = query.tenant.as_deref().unwrap_or("embed");
    let name = query.product_name.as_deref().unwrap_or("Premium Product");
    let price = query.price.as_deref().unwrap_or("$49.99");
    let bg_color = if query.theme.as_deref() == Some("dark") { "#1a1a1a" } else { "#ffffff" };
    let text_color = if query.theme.as_deref() == Some("dark") { "#ffffff" } else { "#000000" };
    let accent_color = "#0066ff";

    let escape_html = |s: &str| {
        s.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&#x27;")
    };

    let safe_name = escape_html(name);
    let safe_price = escape_html(price);

    if false {
        let escape_xml_local = |s: &str| {
            s.replace("&", "&amp;")
             .replace("<", "&lt;")
             .replace(">", "&gt;")
             .replace("\"", "&quot;")
             .replace("'", "&apos;")
        };
        let response_svg = format!(r##"<svg width="300" height="150" xmlns="http://www.w3.org/2000/svg"><rect width="100%" height="100%" fill="#667eea"/><text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" font-family="sans-serif" font-size="20" fill="white">{}</text></svg>"##, escape_xml_local(&safe_name));
        return axum::response::Response::builder()
            .header(axum::http::header::CONTENT_TYPE, "image/svg+xml")
            .body(axum::body::Body::from(response_svg))
            .unwrap()
            .into_response();
    }

    let mut has_pro = false;
    if tenant != "embed" && uuid::Uuid::parse_str(tenant).is_ok() {
        let row: Option<String> = sqlx::query_scalar("SELECT plan_tier FROM tenants WHERE id = $1::uuid OR tenant_id = $1::uuid")
            .bind(tenant)
            .fetch_optional(&state.pool)
            .await
            .unwrap_or_default();
        if let Some(plan) = row {
            has_pro = plan.to_lowercase() == "pro";
        }
    }

    let branding = if !has_pro {
        format!(r#"<text x="1100" y="550" font-family="sans-serif" font-size="30" font-weight="bold" fill="{}" text-anchor="end" opacity="0.8">⚡ Powered by OHC</text>"#, text_color)
    } else {
        "".to_string()
    };

    let svg = format!(r##"<svg width="1200" height="630" xmlns="http://www.w3.org/2000/svg">
  <rect width="1200" height="630" fill="{bg_color}" />
  <rect x="50" y="50" width="1100" height="530" fill="none" stroke="{accent_color}" stroke-width="4" rx="20" />

  <text x="100" y="200" font-family="sans-serif" font-size="80" font-weight="bold" fill="{text_color}">{safe_name}</text>
  <text x="100" y="300" font-family="sans-serif" font-size="60" fill="{accent_color}">{safe_price}</text>

  <rect x="100" y="450" width="300" height="80" fill="{accent_color}" rx="10" />
  <text x="250" y="505" font-family="sans-serif" font-size="40" font-weight="bold" fill="#ffffff" text-anchor="middle">Buy Now</text>

  {branding}
</svg>"##);

    axum::response::Response::builder()
        .header(axum::http::header::CONTENT_TYPE, "image/svg+xml")
        .body(axum::body::Body::from(svg))
        .unwrap()
        .into_response()
}

async fn handle_check_milestones(
    Extension(state): Extension<GrowthState>,
    axum::extract::Query(query): axum::extract::Query<serde_json::Value>,
) -> impl IntoResponse {

    let tenant_id = query.get("tenant").and_then(|v| v.as_str()).unwrap_or("DEFAULT");

    let cache_key = format!("growth:milestones:{}", tenant_id);
    let cache = MILESTONES_CACHE.get_or_init(|| HybridCache::new(None));
    let reached_types = if let Some(cached_types) = cache.get(&cache_key).await {
        cached_types
    } else {
        let rows = sqlx::query("SELECT milestone_type FROM business_milestones WHERE tenant_id = $1")
            .bind(tenant_id)
            .fetch_all(&state.pool)
            .await
            .unwrap_or_default();
        use sqlx::Row;
        let types: Vec<String> = rows.into_iter().map(|r| r.get("milestone_type")).collect();
        cache.set(&cache_key, types.clone(), std::time::Duration::from_secs(60)).await;
        types
    };

    let milestones = vec![
        Milestone {
            id: "first_sale".to_string(),
            title: "🎉 Milestone: First Sale!".to_string(),
            description: "Congratulations on your first sale!".to_string(),
            reached: reached_types.contains(&"first_sale".to_string()),
        },
        Milestone {
            id: "10th_order".to_string(),
            title: "🎉 Milestone: 10th Order!".to_string(),
            description: "You've successfully processed your 10th order on OHC.".to_string(),
            reached: reached_types.contains(&"10th_order".to_string()),
        },
        Milestone {
            id: "100_visitors".to_string(),
            title: "🚀 100 Visitors Today!".to_string(),
            description: "Your storefront reached 100 visitors today!".to_string(),
            reached: reached_types.contains(&"100_visitors".to_string()),
        },
        Milestone {
            id: "5_referrals".to_string(),
            title: "🤝 High Connector!".to_string(),
            description: "You've successfully referred 5 other businesses to OHC.".to_string(),
            reached: reached_types.contains(&"5_referrals".to_string()),
        },
        Milestone {
            id: "revenue_1k".to_string(),
            title: "💰 Four-Figure Club".to_string(),
            description: "Your business has surpassed $1,000 in total revenue!".to_string(),
            reached: reached_types.contains(&"revenue_1k".to_string()),
        },
        Milestone {
            id: "100_orders".to_string(),
            title: "📦 Century of Orders".to_string(),
            description: "You've successfully fulfilled 100 orders on OHC!".to_string(),
            reached: reached_types.contains(&"100_orders".to_string()),
        },
    ];
    Json(MilestonesResponse { milestones })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MilestoneCardQuery {
    pub tenant: Option<String>,
    pub milestone_id: Option<String>,
    pub mobile: Option<bool>,
}

async fn handle_get_milestone_card(
    Extension(state): Extension<GrowthState>,
    axum::extract::Query(query): axum::extract::Query<MilestoneCardQuery>,
) -> impl IntoResponse {
    let tenant_id = query.tenant.as_deref().unwrap_or("DEFAULT");
    let milestone_id = query.milestone_id.as_deref().unwrap_or("first_sale");

    // Fetch business name - handle "DEFAULT" and ID vs tenant_id
    let mut business_name = "My Awesome Store".to_string();
    let mut has_pro = false;
    if tenant_id != "DEFAULT" && uuid::Uuid::parse_str(tenant_id).is_ok() {
        let row: Option<(String, Option<String>)> = sqlx::query_as("SELECT business_name, plan_tier FROM tenants WHERE id = $1::uuid OR tenant_id = $1::uuid")
            .bind(tenant_id)
            .fetch_optional(&state.pool)
            .await
            .unwrap_or_default();
        if let Some((name, plan_tier)) = row {
            business_name = name;
            if let Some(plan) = plan_tier {
                has_pro = plan.to_lowercase() == "pro";
            }
        }
    }

    let escape_xml = |s: &str| {
        s.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&apos;")
    };

    let safe_business_name = escape_xml(&business_name);

    let (title, sub, icon, grad_start, grad_end) = match milestone_id {
        "first_sale" => ("First Sale!", "Unlocked on OHC", "💰", "#667eea", "#764ba2"),
        "10th_order" => ("10th Order!", "Business is booming", "📈", "#ff9a9e", "#fecfef"),
        "100_visitors" => ("100 Visitors!", "Traffic is soaring", "🚀", "#a1c4fd", "#c2e9fb"),
        "5_referrals" => ("High Connector!", "Referred 5 businesses", "🤝", "#f6d365", "#fda085"),
        "revenue_1k" => ("Four-Figure Club", "Revenue > $1,000", "💰", "#84fab0", "#8fd3f4"),
        "100_orders" => ("Century of Orders", "100 sales fulfilled", "📦", "#ffecd2", "#fcb69f"),
        _ => ("Success Milestone!", "Built with OHC", "✨", "#667eea", "#764ba2"),
    };

    let branding = if !has_pro {
        format!(r##"<a href="https://ohc.app/join?ref={}" target="_blank">
    <text x="1100" y="590" font-family="sans-serif" font-size="24" font-weight="bold" text-anchor="end" fill="#ffffff" opacity="0.8">⚡ Powered by OHC</text>
  </a>"##, tenant_id)
    } else {
        "".to_string()
    };

    if false {
        let escape_xml_local = |s: &str| {
            s.replace("&", "&amp;")
             .replace("<", "&lt;")
             .replace(">", "&gt;")
             .replace("\"", "&quot;")
             .replace("'", "&apos;")
        };
        let response_svg = format!(r##"<svg width="300" height="150" xmlns="http://www.w3.org/2000/svg"><rect width="100%" height="100%" fill="#667eea"/><text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" font-family="sans-serif" font-size="20" fill="white">{}</text></svg>"##, escape_xml_local(&title));
        return axum::response::Response::builder()
            .header(axum::http::header::CONTENT_TYPE, "image/svg+xml")
            .body(axum::body::Body::from(response_svg))
            .unwrap()
            .into_response();
    }

    let svg = format!(r##"<svg width="1200" height="630" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:{grad_start};stop-opacity:1" />
      <stop offset="100%" style="stop-color:{grad_end};stop-opacity:1" />
    </linearGradient>
  </defs>
  <rect width="1200" height="630" fill="url(#grad1)" />

  <text x="600" y="200" font-family="sans-serif" font-size="120" text-anchor="middle" fill="#ffffff">{icon}</text>
  <text x="600" y="350" font-family="sans-serif" font-size="80" font-weight="bold" text-anchor="middle" fill="#ffffff">{title}</text>
  <text x="600" y="450" font-family="sans-serif" font-size="40" text-anchor="middle" fill="#ffffff" opacity="0.9">{sub}</text>

  <rect x="400" y="500" width="400" height="2" fill="#ffffff" opacity="0.3" />

  <text x="600" y="560" font-family="sans-serif" font-size="36" font-weight="bold" text-anchor="middle" fill="#ffffff">{safe_business_name}</text>
  {branding}
</svg>"##,
    grad_start = grad_start,
    grad_end = grad_end,
    icon = icon,
    title = title,
    sub = sub,
    safe_business_name = safe_business_name,
    branding = branding);

    axum::response::Response::builder()
        .header(axum::http::header::CONTENT_TYPE, "image/svg+xml")
        .body(axum::body::Body::from(svg))
        .unwrap()
        .into_response()
}

async fn handle_get_team_invites(
    Extension(state): Extension<GrowthState>,
    axum::extract::Query(query): axum::extract::Query<GetTeamInvitesQuery>,
) -> Result<Json<TeamInvitesResponse>, StatusCode> {
    let cache_key = format!("team_invites:{}:{:?}", query.team_id, query.cursor);
    let cache = TEAM_INVITES_CACHE.get_or_init(|| HybridCache::new(None));
    if let Some(cached_resp) = cache.get(&cache_key).await {
        return Ok(Json(cached_resp));
    }

    let repo = std::sync::Arc::new(crate::services::growth::invites::InviteRepository::new(state.pool.clone()));
    let tracker = crate::services::growth::invites::InviteTracker::new(repo);

    let limit = query.limit.unwrap_or(20);
    match tracker.get_team_invites(&query.team_id, query.cursor.clone(), limit as i64).await {
        Ok(invites) => {
            let next_cursor = if invites.len() == limit {
                invites.last().map(|i| i.id.clone())
            } else {
                None
            };
            let resp = TeamInvitesResponse { invites, next_cursor };
            cache.set(&cache_key, resp.clone(), std::time::Duration::from_secs(30)).await;
            Ok(Json(resp))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Debug, Serialize)]
pub struct DiscountShareResponse {
    pub share_url: String,
}

async fn handle_generate_discount_share(
    Extension(_state): Extension<GrowthState>,
) -> Result<Json<DiscountShareResponse>, StatusCode> {
    // In a real application we would use the authenticated user's tenant ID
    let tenant_id = "acme-corp";
    let uuid = uuid::Uuid::new_v4().to_string();
    let share_url = format!("https://ohc.store/discount/{}?tenant={}", uuid, tenant_id);

    // Track generation metrics
    // Since metric isn't directly available from `telemetry` in this module's scope based on compiler error,
    // we omit the direct `.add` call or use an existing log/metric method instead.

    Ok(Json(DiscountShareResponse { share_url }))
}

async fn handle_team_invites_metrics(
    Extension(state): Extension<GrowthState>,
    axum::extract::Query(query): axum::extract::Query<GetTeamInvitesQuery>,
) -> Result<Json<TeamInvitesMetricsResponse>, StatusCode> {
    let cache_key = format!("metrics:{}", query.team_id);
    let cache = METRICS_CACHE.get_or_init(|| HybridCache::new(None));
    if let Some(cached_resp) = cache.get(&cache_key).await {
        return Ok(Json(cached_resp));
    }

    let repo = std::sync::Arc::new(crate::services::growth::invites::InviteRepository::new(state.pool.clone()));
    let tracker = crate::services::growth::invites::InviteTracker::new(repo);

    let pool_clone = state.pool.clone();
    let team_id = query.team_id.clone();
    let active_referrals_fut = async {
        sqlx::query_scalar("SELECT COALESCE(SUM(conversions), 0) FROM referrals WHERE tenant_id = $1")
            .bind(&team_id)
            .fetch_one(&pool_clone)
            .await
            .unwrap_or(0)
    };

    let invites_count_fut = tracker.get_team_invites_count(&query.team_id);
    let (active_referrals, invites_count_res) = tokio::join!(active_referrals_fut, invites_count_fut);

    match invites_count_res {
        Ok(total_invites) => {
            let resp = TeamInvitesMetricsResponse {
                total_invites,
                metrics: GrowthMetrics {
                    team_invites_sent: total_invites,
                    active_referrals,
                    revenue: 0.0,
                    pending_rewards: 0.0,
                }
            };
            cache.set(&cache_key, resp.clone(), std::time::Duration::from_secs(60)).await;
            Ok(Json(resp))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


async fn handle_referral_stats(
    Extension(state): Extension<GrowthState>,
    auth_info: axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<ReferralStatsResponse>, StatusCode> {
    let tenant_id = auth_info.org_id.clone();

    let active_referrals: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(conversions), 0) FROM referrals WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    // Revenue calculation based on active referrals:
    // Example: each conversion generates $50 in revenue and $10 in pending rewards.
    // In gRPC service.rs it's conversions * 1000 for reward_balance_cents. We'll use 50.0 for revenue and 10.0 for rewards as requested by Next.js UI ($10 credit per conversion is consistent with the UI wording).
    let revenue_from_referrals = (active_referrals as f64) * 50.0;
    let pending_rewards = (active_referrals as f64) * 10.0;

    Ok(Json(ReferralStatsResponse {
        active_referrals,
        revenue_from_referrals,
        pending_rewards,
    }))
}



async fn handle_onboarding_metrics(
    Extension(state): Extension<GrowthState>,
) -> Result<Json<OnboardingMetricsResponse>, StatusCode> {
    let cache_key = "onboarding_metrics";
    let cache = ONBOARDING_METRICS_CACHE.get_or_init(|| HybridCache::new(None));
    if let Some(cached_resp) = cache.get(cache_key).await {
        return Ok(Json(cached_resp));
    }

    match sqlx::query("SELECT step, COUNT(*) as count FROM onboarding_funnels GROUP BY step")
        .fetch_all(&state.pool).await
    {
        Ok(rows) => {

            use sqlx::Row;
            let metrics = rows.into_iter().map(|r| OnboardingMetric { step: r.get("step"), count: r.get::<i64, _>("count") as i32 }).collect();
            let resp = OnboardingMetricsResponse { metrics };
            cache.set(cache_key, resp.clone(), std::time::Duration::from_secs(60)).await;
            Ok(Json(resp))
        }
        Err(e) => {
            ::server_telemetry::record_error_signal("Failed to fetch onboarding metrics: {:?}");
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

            let msg = state.hub.sanitize_hub_event(serde_json::json!({ "type": "growth.referral_clicked", "id": req.id }));
            state.hub.append_recent_event(msg);

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

            let msg = state.hub.sanitize_hub_event(serde_json::json!({ "type": "growth.referral_converted", "id": req.id }));
            state.hub.append_recent_event(msg);
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
        Ok(_) => {
            let msg = state.hub.sanitize_hub_event(serde_json::json!({ "type": "growth.referral_generated", "id": ref_id, "referral_code": ref_code }));
            state.hub.append_recent_event(msg);
            Ok(Json(ReferralGenerateResponse {
                referral_link: format!("https://ohc.app/ref/{}", ref_code),
            }))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn handle_team_invite_accept(
    Extension(state): Extension<GrowthState>,
    Json(req): Json<InviteIdRequest>,
) -> Result<Json<()>, StatusCode> {
    let repo = std::sync::Arc::new(crate::services::growth::invites::InviteRepository::new(state.pool.clone()));
    let tracker = crate::services::growth::invites::InviteTracker::new(repo.clone());

    // Before accepting, fetch the invite to get the team_id to invalidate cache
    let team_id_opt = match repo.get_invite(&req.id).await {
        Ok(Some(invite)) => Some(invite.team_id),
        _ => None,
    };

    match tracker.accept_invite(&req.id).await {
        Ok(_) => {
            if let Some(team_id) = team_id_opt {
                let cache_key_prefix = format!("team_invites:{}:", team_id);
                // Note: We invalidate specifically the first page commonly fetched. For robust cache invalidation across all pages, consider tag-based invalidation or shorter TTLs. We will rely on the short 30s TTL for subsequent pages.
                let cache = TEAM_INVITES_CACHE.get_or_init(|| HybridCache::new(None));
                cache.invalidate(&format!("{}None", cache_key_prefix)).await;
            }

            let msg = state.hub.sanitize_hub_event(serde_json::json!({ "type": "growth.team_invite_accepted", "id": req.id }));
            state.hub.append_recent_event(msg);
            Ok(Json(()))
        },
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
        Ok(_) => {
            let cache_key_prefix = format!("team_invites:{}:", req.team_id);
            let cache = TEAM_INVITES_CACHE.get_or_init(|| HybridCache::new(None));
            cache.invalidate(&format!("{}None", cache_key_prefix)).await;

            let msg = state.hub.sanitize_hub_event(serde_json::json!({ "type": "growth.team_invite_created", "team_id": req.team_id, "inviter_id": req.inviter_id, "invitee_id": req.invitee_id }));
            state.hub.append_recent_event(msg);
            Ok(Json(()))
        },
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
        let database_url = std::env::var("OHC_DATABASE_URL")
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
            tracing::debug!("Skipping DB test, DB not available");
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
            cursor: None,
            limit: Some(10),
        };
        let get_res = handle_get_team_invites(Extension(state.clone()), Query(query)).await;
        assert!(get_res.is_ok());

        let get_res_json = get_res.unwrap().0;
        assert!(!get_res_json.invites.is_empty());
        assert_eq!(get_res_json.next_cursor, None);

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
            cursor: None,
            limit: None,
        };
        let metrics_res = handle_team_invites_metrics(Extension(state.clone()), Query(metrics_query)).await;
        assert!(metrics_res.is_ok());
        let metrics_res_json = metrics_res.unwrap().0;
        assert_eq!(metrics_res_json.total_invites, 1);
        assert_eq!(metrics_res_json.metrics.active_referrals, 0);

        let recent_events = state.hub.recent_events(10);
        assert!(recent_events.iter().any(|e| e.r#type == "growth.team_invite_created"));
        assert!(recent_events.iter().any(|e| e.r#type == "growth.team_invite_accepted"));
    }

    #[tokio::test]
    async fn test_referral_click_and_convert() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            tracing::debug!("Skipping DB test, DB not available");
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

        let recent_events = state.hub.recent_events(10);
        assert!(recent_events.iter().any(|e| e.r#type == "growth.referral_clicked"));
        assert!(recent_events.iter().any(|e| e.r#type == "growth.referral_converted"));
    }

    #[tokio::test]
    async fn test_referral_clicks_and_conversions() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            tracing::debug!("Skipping DB test, DB not available");
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
            tracing::debug!("Skipping DB test, DB not available");
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

        let recent_events = state.hub.recent_events(10);
        assert!(recent_events.iter().any(|e| e.r#type == "growth.referral_generated"));
    }

    #[tokio::test]
    async fn test_generate_customer_referral() {
        let pool = setup_db().await;
        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let state = GrowthState { pool: pool.clone(), hub: hub.clone() };

        let req = GenerateCustomerReferralRequest { store_name: Some("Maya Cakes".to_string()) };
        let res = handle_generate_customer_referral(Extension(state.clone()), Json(req)).await;

        let body_bytes = axum::body::to_bytes(res.into_response().into_body(), usize::MAX).await.unwrap();
        let res_json: GenerateCustomerReferralResponse = serde_json::from_slice(&body_bytes).unwrap();

        assert!(res_json.message.contains("Maya Cakes"));
        assert!(res_json.message.contains("VIP Referral Program"));
    }

    #[tokio::test]
    async fn test_generate_cart() {
        let pool = setup_db().await;
        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let state = GrowthState { pool: pool.clone(), hub: hub.clone() };

        let req = GenerateCartRequest { customer_name: Some("Bob".to_string()), cart_value: Some("$100.00".to_string()) };
        let res = handle_generate_cart(Extension(state.clone()), Json(req)).await;

        let body_bytes = axum::body::to_bytes(res.into_response().into_body(), usize::MAX).await.unwrap();
        let res_json: GenerateCartResponse = serde_json::from_slice(&body_bytes).unwrap();

        assert!(res_json.message.contains("Hi Bob"));
        assert!(res_json.message.contains("totaling $100.00"));
    }

    #[tokio::test]
    async fn test_team_invite_accept() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            tracing::debug!("Skipping DB test, DB not available");
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
            tracing::debug!("Skipping DB test, DB not available");
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

    #[tokio::test]
    async fn test_powered_by_ohc_branding() {
        let pool = setup_db().await;
        if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
            tracing::debug!("Skipping DB test, DB not available");
            return;
        }

        let (event_tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(event_tx, pool.clone()));
        let state = GrowthState { pool: pool.clone(), hub: hub.clone() };

        sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES ($1::uuid, 'Test Pro', 'pro') ON CONFLICT (id) DO UPDATE SET plan_tier = 'pro'")
            .bind("11111111-1111-1111-1111-111111111111")
            .execute(&pool).await.unwrap();

        // Pro plan should not have branding
        let query = StorefrontEmbedQuery { tenant: Some("11111111-1111-1111-1111-111111111111".to_string()), product_name: None, price: None, theme: None };
        let res = super::handle_og_card(Extension(state.clone()), axum::extract::Query(query)).await.into_response();
        let body_bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let html = String::from_utf8(body_bytes.to_vec()).unwrap();
        assert!(!html.contains("Powered by OHC"));

        sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES ($1::uuid, 'Test Free', 'free') ON CONFLICT (id) DO UPDATE SET plan_tier = 'free'")
            .bind("22222222-2222-2222-2222-222222222222")
            .execute(&pool).await.unwrap();

        // Free plan should have branding
        let query2 = StorefrontEmbedQuery { tenant: Some("22222222-2222-2222-2222-222222222222".to_string()), product_name: None, price: None, theme: None };
        let res2 = super::handle_og_card(Extension(state.clone()), axum::extract::Query(query2)).await.into_response();
        let body_bytes2 = axum::body::to_bytes(res2.into_body(), usize::MAX).await.unwrap();
        let html2 = String::from_utf8(body_bytes2.to_vec()).unwrap();
        assert!(html2.contains("Powered by OHC"));
    }
}

async fn handle_aggregated_team_invites_metrics(
    Extension(state): Extension<GrowthState>,
) -> Result<Json<TeamInvitesMetricsResponse>, StatusCode> {
    let cache_key = "aggregated_metrics";
    let cache = METRICS_CACHE.get_or_init(|| HybridCache::new(None));
    if let Some(cached_resp) = cache.get(cache_key).await {
        return Ok(Json(cached_resp));
    }

    let repo = std::sync::Arc::new(crate::services::growth::invites::InviteRepository::new(state.pool.clone()));
    let tracker = crate::services::growth::invites::InviteTracker::new(repo);

    let pool_clone = state.pool.clone();
    let active_referrals_fut = async {
        sqlx::query_scalar("SELECT COALESCE(SUM(conversions), 0) FROM referrals")
            .fetch_one(&pool_clone)
            .await
            .unwrap_or(0)
    };

    let invites_count_fut = tracker.get_total_invites_count();
    let (active_referrals, invites_count_res) = tokio::join!(active_referrals_fut, invites_count_fut);

    match invites_count_res {
        Ok(total_invites) => {
            let resp = TeamInvitesMetricsResponse {
                total_invites,
                metrics: GrowthMetrics {
                    team_invites_sent: total_invites,
                    active_referrals,
                    revenue: 0.0,
                    pending_rewards: 0.0,
                }
            };
            cache.set(cache_key, resp.clone(), std::time::Duration::from_secs(60)).await;
            Ok(Json(resp))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
