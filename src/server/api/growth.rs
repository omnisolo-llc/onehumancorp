use axum::{
    routing::post,
    Router, Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct WidgetRequest {
    pub tenant_id: String,
    pub business_name: String,
}

#[derive(Serialize)]
pub struct WidgetResponse {
    pub embed_code: String,
}

pub async fn generate_referral_widget(Json(payload): Json<WidgetRequest>) -> Json<WidgetResponse> {
    let embed_code = format!(
        "<div class=\"ohc-widget\" data-tenant=\"{}\"><a href=\"https://onehumancorp.com/join?ref={}\" target=\"_blank\">Powered by OHC - Work Assistant for {}</a></div><script src=\"https://onehumancorp.com/widget.js\"></script>",
        payload.tenant_id, payload.tenant_id, payload.business_name
    );

    Json(WidgetResponse { embed_code })
}

pub fn growth_routes() -> Router {
    Router::new()
        .route("/growth/referrals/generate", post(generate_referral_widget))
}
