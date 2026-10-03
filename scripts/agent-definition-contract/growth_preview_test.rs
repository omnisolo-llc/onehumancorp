//! Actual authenticated preview routes and forced-RLS PostgreSQL rows.
use super::*;
use axum::http::HeaderMap;

const ROUTES: [(&str, &str); 2] = [
    ("post-purchase", "hideBranding"),
    ("customer-referral", "hide_branding"),
];
async fn fixture() -> (PgFixture, Fixture, Router) {
    let pg = PgFixture::new().await;
    sqlx::raw_sql(include_str!(
        "../../src/server/migrations/110_trial_extension_claim.sql"
    ))
    .execute(&pg.admin)
    .await
    .unwrap();
    sqlx::raw_sql("ALTER TABLE tenants ENABLE ROW LEVEL SECURITY; ALTER TABLE tenants FORCE ROW LEVEL SECURITY; CREATE POLICY preview_tenant_scope ON tenants USING(id=current_setting('app.current_tenant',true)) WITH CHECK(id=current_setting('app.current_tenant',true));")
        .execute(&pg.admin).await.unwrap();
    let placeholder = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    let f = Fixture::with_store(pg.store.clone(), placeholder).await;
    let app = Router::new()
        .nest(
            "/api/v1/growth",
            crate::growth_previews::router(pg.pool.clone()),
        )
        .layer(axum::middleware::from_fn_with_state(
            f.auth.clone(),
            server_auth::strict_bearer_auth_middleware,
        ));
    (pg, f, app)
}
async fn get(app: &Router, token: Option<&str>, route: &str) -> (StatusCode, HeaderMap, String) {
    let mut request = Request::builder()
        .uri(route)
        .header("x-tenant-id", "definition-b")
        .header("x-user-id", "owner-c")
        .header(
            "x-spiffe-id",
            "spiffe://omnisolo.io/org/definition-b/agent/owner-c",
        );
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(request.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let (parts, body) = response.into_parts();
    let body = to_bytes(body, 65_536).await.unwrap();
    (
        parts.status,
        parts.headers,
        String::from_utf8(body.to_vec()).unwrap(),
    )
}
fn path(route: &str, hide_key: &str, tenant: &str) -> String {
    format!(
        "/api/v1/growth/{route}/embed?tenant={tenant}&discount=20pct&give=15&get=20&{hide_key}=true"
    )
}

#[tokio::test]
async fn paid_branding_uses_current_pro_and_business_rows_under_forced_tenant_rls() {
    let (pg, f, app) = fixture().await;
    for (tier, plan, branded) in [
        ("free", Some("free"), true),
        ("starter", Some("starter"), true),
        ("pro", Some("pro"), false),
        ("business", Some("business"), false),
        ("pro", None, false),
    ] {
        sqlx::query("UPDATE tenants SET tier=$1,plan_tier=$2 WHERE id='definition-a'")
            .bind(tier)
            .bind(plan)
            .execute(&pg.admin)
            .await
            .unwrap();
        for (route, hide_key) in ROUTES {
            let (status, _, html) =
                get(&app, Some(&f.owner), &path(route, hide_key, "definition-a")).await;
            assert_eq!(status, StatusCode::OK);
            assert_eq!(
                html.contains("⚡ OmniSolo"),
                branded,
                "{route} must use actual {tier} entitlement with RLS active"
            );
        }
        let persisted:(String,Option<String>,bool)=sqlx::query_as("SELECT tier,plan_tier,has_claimed_trial_extension FROM tenants WHERE id='definition-a'").fetch_one(&pg.admin).await.unwrap();
        assert_eq!(persisted, (tier.into(), plan.map(str::to_owned), false));
    }
    drop(f);
    pg.close().await;
}

#[tokio::test]
async fn authenticated_previews_never_adopt_another_tenants_paid_plan() {
    let (pg, f, app) = fixture().await;
    sqlx::query("UPDATE tenants SET tier='pro',plan_tier='pro' WHERE id='definition-b'")
        .execute(&pg.admin)
        .await
        .unwrap();
    for (route, hide_key) in ROUTES {
        assert_eq!(
            get(&app, Some(&f.owner), &path(route, hide_key, "definition-b"))
                .await
                .0,
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            get(&app, None, &path(route, hide_key, "definition-b"))
                .await
                .0,
            StatusCode::UNAUTHORIZED
        );
    }
    drop(f);
    pg.close().await;
}

#[tokio::test]
async fn preview_is_an_explicit_private_draft_without_invented_discount_or_referral_fulfillment() {
    let (pg, f, app) = fixture().await;
    for (route, hide_key) in ROUTES {
        let (status, headers, html) =
            get(&app, Some(&f.owner), &path(route, hide_key, "definition-a")).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(
            headers
                .get("cache-control")
                .and_then(|value| value.to_str().ok()),
            Some("private, no-store")
        );
        assert!(html.contains("offer-draft"));
        assert!(html.contains("not configured"));
        assert!(!html.contains("/referrals/click"));
        assert!(!html.contains("Share and Get"));
        assert!(!html.contains("Give your friends"));
    }
    drop(f);
    pg.close().await;
}

#[tokio::test]
async fn preview_rechecks_current_membership_after_transport_authentication() {
    let (pg, f, _) = fixture().await;
    let admin = pg.admin.clone();
    let app=Router::new().nest("/api/v1/growth",crate::growth_previews::router(pg.pool.clone()))
        .layer(axum::middleware::from_fn(move |request: axum::extract::Request,next: axum::middleware::Next| {
            let admin=admin.clone(); async move {
                sqlx::query("UPDATE users SET active=false WHERE username='owner-a' AND tenant_id='definition-a'").execute(&admin).await.unwrap();
                next.run(request).await
            }
        }))
        .layer(axum::middleware::from_fn_with_state(f.auth.clone(),server_auth::strict_bearer_auth_middleware));
    for (route, hide_key) in ROUTES {
        sqlx::query(
            "UPDATE users SET active=true WHERE username='owner-a' AND tenant_id='definition-a'",
        )
        .execute(&pg.admin)
        .await
        .unwrap();
        assert_eq!(
            get(&app, Some(&f.owner), &path(route, hide_key, "definition-a"))
                .await
                .0,
            StatusCode::FORBIDDEN
        );
    }
    drop(f);
    pg.close().await;
}

#[tokio::test]
async fn unknown_or_unreadable_plan_never_becomes_a_successful_free_or_pro_preview() {
    let (pg, f, app) = fixture().await;
    sqlx::query("UPDATE tenants SET tier='unknown',plan_tier='unknown' WHERE id='definition-a'")
        .execute(&pg.admin)
        .await
        .unwrap();
    for (route, hide_key) in ROUTES {
        assert_eq!(
            get(&app, Some(&f.owner), &path(route, hide_key, "definition-a"))
                .await
                .0,
            StatusCode::SERVICE_UNAVAILABLE
        );
    }
    sqlx::query("ALTER TABLE tenants RENAME TO unavailable_tenants")
        .execute(&pg.admin)
        .await
        .unwrap();
    for (route, hide_key) in ROUTES {
        assert_eq!(
            get(&app, Some(&f.owner), &path(route, hide_key, "definition-a"))
                .await
                .0,
            StatusCode::SERVICE_UNAVAILABLE
        );
    }
    drop(f);
    pg.close().await;
}

#[tokio::test]
async fn preview_plan_policy_and_explicit_hide_request_preserve_paid_base_plans() {
    let (pg, f, app) = fixture().await;
    for plan in [
        "starter",
        "enterprise",
        "premium",
        "pro-plus",
        " pro ",
        "",
        "PrO",
        "BuSiNeSs",
    ] {
        sqlx::query("UPDATE tenants SET tier=$1,plan_tier=$1 WHERE id='definition-a'")
            .bind(plan)
            .execute(&pg.admin)
            .await
            .unwrap();
        for (route, hide_key) in ROUTES {
            let (status, _, html) =
                get(&app, Some(&f.owner), &path(route, hide_key, "definition-a")).await;
            match plan {
                "starter" => {
                    assert_eq!(status, StatusCode::OK);
                    assert!(html.contains("⚡ OmniSolo"));
                }
                "PrO" | "BuSiNeSs" => {
                    assert_eq!(status, StatusCode::OK);
                    assert!(!html.contains("⚡ OmniSolo"));
                }
                _ => assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE),
            }
        }
        let stored: (String, Option<String>) =
            sqlx::query_as("SELECT tier,plan_tier FROM tenants WHERE id='definition-a'")
                .fetch_one(&pg.admin)
                .await
                .unwrap();
        assert_eq!(stored, (plan.into(), Some(plan.into())));
    }
    for (route, hide_key) in ROUTES {
        for hide in [None, Some("false"), Some("TRUE"), Some("1")] {
            let mut target = format!("/api/v1/growth/{route}/embed?tenant=definition-a");
            if let Some(hide) = hide {
                target.push_str(&format!("&{hide_key}={hide}"));
            }
            let (status, _, html) = get(&app, Some(&f.owner), &target).await;
            assert_eq!(status, StatusCode::OK);
            assert!(
                html.contains("⚡ OmniSolo"),
                "even paid plans must explicitly request removal"
            );
        }
    }
    drop(f);
    pg.close().await;
}

#[tokio::test]
async fn preview_binds_raw_owner_tenant_without_adopting_encoded_lookalikes() {
    let (pg, f, app) = fixture().await;
    let raw = "O'Brien & café";
    let lookalike = "O&#x27;Brien &amp; café";
    let mut tokens = Vec::new();
    for (index, tenant) in [raw, lookalike].into_iter().enumerate() {
        sqlx::query("INSERT INTO tenants(id,name,tier,plan_tier) VALUES($1,'Owned preview fixture','free','free')")
            .bind(tenant).execute(&pg.admin).await.unwrap();
        let user = f
            .auth
            .create_user(
                format!("preview-owner-{index}"),
                format!("preview-owner-{index}@example.test"),
                "public-local-preview-fixture-password".into(),
                vec!["OWNER".into()],
                tenant.into(),
            )
            .await
            .unwrap();
        tokens.push(f.auth.issue_token(&user).unwrap());
    }
    for paid in [raw, lookalike] {
        sqlx::query("UPDATE tenants SET tier=CASE WHEN id=$1 THEN 'pro' ELSE 'free' END, plan_tier=CASE WHEN id=$1 THEN 'pro' ELSE 'free' END WHERE id IN($1,$2)")
            .bind(paid).bind(if paid==raw {lookalike} else {raw}).execute(&pg.admin).await.unwrap();
        for (index, tenant) in [raw, lookalike].into_iter().enumerate() {
            for (route, hide_key) in ROUTES {
                let (status, _, html) = get(
                    &app,
                    Some(&tokens[index]),
                    &path(route, hide_key, &urlencoding::encode(tenant)),
                )
                .await;
                assert_eq!(status, StatusCode::OK);
                assert_eq!(html.contains("⚡ OmniSolo"), tenant != paid);
                let foreign = if tenant == raw { lookalike } else { raw };
                assert_eq!(
                    get(
                        &app,
                        Some(&tokens[index]),
                        &path(route, hide_key, &urlencoding::encode(foreign))
                    )
                    .await
                    .0,
                    StatusCode::FORBIDDEN
                );
            }
        }
    }
    drop(f);
    pg.close().await;
}

#[tokio::test]
async fn authenticated_preview_fields_are_literal_data_without_referral_or_script_actions() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let (pg, f, app) = fixture().await;
    for supplied in [
        "x');globalThis.injected=true;//",
        "O'Brien",
        "\"double\" & query=?#fragment",
        "é雪😀",
        "</script><script>globalThis.injected=true</script>",
        "&#x27; onclick=globalThis.injected=true",
        "tenant-a",
        "",
    ] {
        for (route, hide_key) in ROUTES {
            let value = urlencoding::encode(supplied);
            let target = format!(
                "/api/v1/growth/{route}/embed?tenant=definition-a&discount={value}&give={value}&get={value}&{hide_key}=false"
            );
            let (status, _, html) = get(&app, Some(&f.owner), &target).await;
            assert_eq!(status, StatusCode::OK);
            let mut child = Command::new("python3")
                .arg(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/check_preview_html.py"
                ))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            child
                .stdin
                .take()
                .unwrap()
                .write_all(
                    serde_json::to_string(&json!({"html":html,"supplied":supplied}))
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();
            let output = child.wait_with_output().unwrap();
            assert!(
                output.status.success(),
                "unsafe preview {route} for {supplied:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
    for (route, _) in ROUTES {
        let (status, _, html) = get(
            &app,
            Some(&f.owner),
            &format!("/api/v1/growth/{route}/embed"),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert!(html.contains("Not specified"));
        assert!(!html.contains("10%"));
    }
    drop(f);
    pg.close().await;
}
