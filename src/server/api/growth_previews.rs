//! Authenticated growth preview routes; source-owned rendering and real pool reads.
use axum::{
    Extension, Router,
    http::{HeaderValue, StatusCode, header},
    response::{Html, Response},
    routing::get,
};
use serde::Deserialize;
use server_common::Claims;
use sqlx::PgPool;

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    Router::new()
        .route("/post-purchase/embed", get(handle_post_purchase_embed))
        .route(
            "/customer-referral/embed",
            get(handle_customer_referral_embed),
        )
        .layer(Extension(pool))
        .layer(axum::middleware::from_fn(no_store))
}

pub(crate) type PreviewError = (StatusCode, &'static str);

async fn no_store(request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, no-store"),
    );
    response
}

async fn verified_paid_preview(
    pool: &PgPool,
    claims: Option<Extension<Claims>>,
    requested_tenant: Option<&str>,
) -> Result<bool, PreviewError> {
    let unavailable = (
        StatusCode::SERVICE_UNAVAILABLE,
        "Current plan could not be verified. Please try again.",
    );
    let forbidden = (
        StatusCode::FORBIDDEN,
        "This preview is unavailable for the current account.",
    );
    let claims = claims
        .ok_or((StatusCode::UNAUTHORIZED, "Sign in to preview this draft."))?
        .0;
    let tenant = claims
        .organization_id
        .as_deref()
        .filter(|tenant| {
            !tenant.trim().is_empty()
                && tenant.trim() == *tenant
                && !tenant.eq_ignore_ascii_case("system")
        })
        .ok_or((StatusCode::UNAUTHORIZED, "A verified account is required."))?;
    if claims.sub.trim().is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "A verified account is required."));
    }
    if requested_tenant.is_some_and(|requested| requested != tenant) {
        return Err(forbidden);
    }
    let mut transaction = pool.begin().await.map_err(|_| unavailable)?;
    server_common::auth_utils::set_org_context(&mut *transaction, tenant)
        .await
        .map_err(|_| unavailable)?;
    // Recheck actual membership after transport authentication. The query uses
    // the same transaction-local tenant context as the protected account rows.
    let row: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT COALESCE(t.plan_tier, t.tier) FROM tenants t JOIN users u ON u.tenant_id::text=t.id::text WHERE t.id::text=$1 AND u.id::text=$2 AND u.active=TRUE",
    ).bind(tenant).bind(&claims.sub).fetch_optional(&mut *transaction).await.map_err(|_| unavailable)?;
    let plan = row.ok_or(forbidden)?.0.ok_or(unavailable)?;
    let paid = match plan.to_ascii_lowercase().as_str() {
        "free" | "starter" => false,
        "pro" | "business" => true,
        _ => return Err(unavailable),
    };
    transaction.commit().await.map_err(|_| unavailable)?;
    Ok(paid)
}

#[derive(Deserialize)]
pub struct PostPurchaseEmbedQuery {
    pub tenant: Option<String>,
    pub discount: Option<String>,
    pub theme: Option<String>,
    #[serde(rename = "hideBranding")]
    pub hide_branding: Option<String>,
}

#[derive(Deserialize)]
pub struct CustomerReferralEmbedQuery {
    pub tenant: Option<String>,
    pub give: Option<String>,
    pub get: Option<String>,
    pub theme: Option<String>,
    pub hide_branding: Option<String>,
}

pub(crate) async fn handle_post_purchase_embed(
    Extension(pool): Extension<PgPool>,
    claims: Option<Extension<Claims>>,
    axum::extract::Query(query): axum::extract::Query<PostPurchaseEmbedQuery>,
) -> Result<Html<String>, PreviewError> {
    let escape_html = |s: &str| {
        s.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#x27;")
    };

    let paid = verified_paid_preview(&pool, claims, query.tenant.as_deref()).await?;
    let discount = escape_html(query.discount.as_deref().unwrap_or("Not specified"));

    let discount_display = if discount.ends_with("pct") {
        format!("{}%", discount.trim_end_matches("pct"))
    } else if discount.ends_with("flat") {
        format!("{} (flat amount)", discount.trim_end_matches("flat"))
    } else {
        discount.clone()
    };

    let bg_color = if query.theme.as_deref() == Some("dark") {
        "#111827"
    } else {
        "#ffffff"
    };
    let text_color = if query.theme.as_deref() == Some("dark") {
        "#ffffff"
    } else {
        "#1f2937"
    };
    let border_color = if query.theme.as_deref() == Some("dark") {
        "#374151"
    } else {
        "#e5e7eb"
    };

    let branding = if paid && query.hide_branding.as_deref() == Some("true") {
        "".to_string()
    } else {
        r#"<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="https://omnisolo.co" target="_blank" rel="noopener noreferrer" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ OmniSolo</a></div>"#.to_owned()
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            background: {bg_color};
            color: {text_color};
            margin: 0;
            padding: 16px;
            text-align: center;
            display: flex;
            flex-direction: column;
            justify-content: center;
            height: 100vh;
            box-sizing: border-box;
        }}
        .widget-icon {{ font-size: 32px; margin-bottom: 12px; }}
        h3 {{ margin: 0 0 8px 0; font-size: 20px; font-weight: 700; }}
        p {{ margin: 0 0 16px 0; font-size: 14px; opacity: 0.8; line-height: 1.5; }}
        .input-group {{ display: flex; gap: 8px; justify-content: center; }}
        input {{
            padding: 12px;
            border: 1px solid {border_color};
            border-radius: 8px;
            background: rgba(128,128,128,0.1);
            color: {text_color};
            outline: none;
            width: 60%;
            max-width: 300px;
        }}
        button {{
            background: #0066FF;
            color: white;
            border: none;
            padding: 12px 20px;
            border-radius: 8px;
            font-weight: 600;
            cursor: pointer;
        }}
    </style>
</head>
<body>
    <div class="widget-icon">🎁</div>
    <h3 data-testid="offer-draft">Post-purchase offer draft</h3>
    <p>Draft discount: {discount_display}.</p>
    <p>Authenticated preview. Public embedding and discount fulfillment are not configured.</p>
    <div class="input-group">
        <input type="text" readonly disabled value="" placeholder="Referral program not configured" id="ref-link" />
        <button disabled>Referral program unavailable</button>
    </div>
    {branding}
</body>
</html>"#,
        bg_color = bg_color,
        text_color = text_color,
        border_color = border_color,
        discount_display = discount_display,
        branding = branding
    );

    Ok(Html(html))
}

pub(crate) async fn handle_customer_referral_embed(
    Extension(pool): Extension<PgPool>,
    claims: Option<Extension<Claims>>,
    axum::extract::Query(query): axum::extract::Query<CustomerReferralEmbedQuery>,
) -> Result<Html<String>, PreviewError> {
    let escape_html = |s: &str| {
        s.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("\'", "&#x27;")
    };

    let paid = verified_paid_preview(&pool, claims, query.tenant.as_deref()).await?;
    let give = escape_html(query.give.as_deref().unwrap_or("Not specified"));
    let get = escape_html(query.get.as_deref().unwrap_or("Not specified"));
    let bg_color = if query.theme.as_deref() == Some("dark") {
        "#111827"
    } else {
        "#ffffff"
    };
    let text_color = if query.theme.as_deref() == Some("dark") {
        "#ffffff"
    } else {
        "#1f2937"
    };
    let border_color = if query.theme.as_deref() == Some("dark") {
        "#374151"
    } else {
        "#e5e7eb"
    };
    let branding = if paid && query.hide_branding.as_deref() == Some("true") {
        "".to_string()
    } else {
        r#"<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="https://omnisolo.co" target="_blank" rel="noopener noreferrer" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ OmniSolo</a></div>"#.to_owned()
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            background-color: {bg_color};
            color: {text_color};
            margin: 0;
            padding: 20px;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            box-sizing: border-box;
        }}
        .card {{
            border: 1px solid {border_color};
            border-radius: 16px;
            padding: 24px;
            text-align: center;
            max-width: 400px;
            width: 100%;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
        }}
        .icon {{
            font-size: 48px;
            margin-bottom: 16px;
        }}
        h2 {{
            margin: 0 0 8px 0;
            font-size: 24px;
        }}
        p {{
            margin: 0 0 24px 0;
            color: #6b7280;
            font-size: 14px;
            line-height: 1.5;
        }}
        .button {{
            background-color: #10b981;
            color: white;
            border: none;
            border-radius: 8px;
            padding: 12px 24px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            width: 100%;
            transition: background-color 0.2s;
        }}
        .button:hover {{
            background-color: #059669;
        }}
    </style>
</head>
<body>
    <div class="card">
        <div class="icon">🎁</div>
        <h2 data-testid="offer-draft">Customer referral offer draft</h2>
        <p>Draft friend offer: {give}. Draft referrer offer: {get}.</p>
        <p>Authenticated preview. Public embedding and reward fulfillment are not configured.</p>
        <button class="button" id="referral-share" disabled>Referral program unavailable</button>
        {branding}
    </div>
</body>
</html>"#
    );

    Ok(Html(html))
}
