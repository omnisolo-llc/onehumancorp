use axum::{
    extract::{Extension, Query, Form},
    response::{IntoResponse, Html, Response},
    Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use crate::hub::Hub;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct WorkIntakeSubmitQuery {
    pub tenant: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct WorkIntakeForm {
    pub name: String,
    pub email: String,
    pub details: String,
}

pub async fn handle_work_intake_submit(
    Extension(hub): Extension<Arc<Hub>>,
    Query(query): Query<WorkIntakeSubmitQuery>,
    Form(form): Form<WorkIntakeForm>,
) -> impl IntoResponse {
    let tenant = query.tenant.unwrap_or_else(|| "my-business".to_string());

    // Attempt to notify the backend via a hub event. In reality, it should call the webhook endpoint or just publish to hub.
    // Given the Next.js API made an HTTP POST to /api/agents/webhook, we will replicate the hub publication directly.
    let msg = hub.sanitize_hub_event(serde_json::json!({
        "type": "work_intake",
        "tenant_id": tenant,
        "source": "work_intake",
        "message": form.details,
        "customer_name": form.name,
        "customer_email": form.email,
    }));
    hub.append_recent_event(msg);

    let encoded_tenant = tenant.replace(" ", "%20").replace("<", "%3C").replace(">", "%3E").replace("\"", "%22").replace("'", "%27");

    let safe_name = form.name.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;").replace("'", "&#x27;");

    let html = format!(r#"

    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Request Submitted</title>
      <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&display=swap" rel="stylesheet">
      <style>
        body {{ font-family: 'Inter', sans-serif; margin: 0; padding: 16px; background: transparent; }}
        .font-outfit {{ font-family: 'Outfit', sans-serif; }}
        .card {{
            background-color: #ffffff;
            border: 1px solid #e5e7eb;
            border-radius: 16px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            overflow: hidden;
            display: flex;
            flex-direction: column;
            max-width: 24rem;
            margin: 0 auto;
        }}
        .content {{ padding: 40px 20px; text-align: center; }}
        .icon {{
            font-size: 4rem;
            margin-bottom: 16px;
        }}
        .title {{
            color: #111827;
            font-size: 1.5rem;
            font-weight: 700;
            margin-bottom: 8px;
        }}
        .desc {{
            color: #4b5563;
            font-size: 1rem;
            margin-bottom: 24px;
            line-height: 1.5;
        }}
        .footer {{
            padding-top: 16px;
            margin-top: 16px;
            border-top: 1px solid #f3f4f6;
            color: #6b7280;
            font-size: 0.75rem;
            text-align: center;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
        }}
        .footer a {{
            font-weight: 700;
            color: #3b82f6;
            text-decoration: none;
            transition: color 0.15s ease;
        }}
        .footer a:hover {{ color: #2563eb; text-decoration: underline; }}
      </style>
    </head>
    <body>
      <div class="card">
        <div class="content">
            <div class="icon">✅</div>
            <h2 class="title font-outfit">Request Received!</h2>
            <p class="desc">Thanks, {safe_name}! We've received your request and will be in touch shortly.</p>
        </div>
        <div style="padding: 0 20px 20px;">
             <!-- Viral Growth Loop Footer -->
             <div class="footer">
                <span>⚡ Powered by</span>
                <a href="/api/v1/growth/referrals/click?target=/onboarding&ref={encoded_tenant}" target="_blank" rel="noopener noreferrer">OHC</a>
             </div>
        </div>
      </div>
    </body>
    </html>
    "#);

    axum::response::Html(html)
}

pub fn router<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/submit", post(handle_work_intake_submit))
        .layer(Extension(hub))
}
