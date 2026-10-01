use super::*;
use axum::{body::to_bytes, extract::Query, http::StatusCode};
use sqlx::{postgres::PgPoolOptions, sqlite::SqlitePoolOptions};

const PRO: &str = "00000000-0000-0000-0000-000000000001";
const FREE: &str = "00000000-0000-0000-0000-000000000002";
const OTHER: &str = "00000000-0000-0000-0000-000000000003";
const UNKNOWN: &str = "00000000-0000-0000-0000-000000000004";

async fn setup() -> PgPool {
    let url = std::env::var("OHC_GROWTH_TEST_DATABASE_URL")
        .expect("real isolated PostgreSQL is required; missing database is not a pass");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(matches!(options.get_host(), "127.0.0.1" | "localhost"));
    assert!(options.get_database().unwrap_or("").starts_with("ohc_"));
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    // Connection-local table shadows any durable table in the borrowed test DB.
    sqlx::raw_sql(&PG_TENANTS_DDL.replace("CREATE TABLE IF NOT EXISTS", "CREATE TEMPORARY TABLE"))
        .execute(&pool)
        .await
        .unwrap();
    for (id, plan) in [(PRO, "pro"), (FREE, "free"), (OTHER, "enterprise")] {
        sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES ($1, 'Regression fixture', $2)")
            .bind(id).bind(plan).execute(&pool).await.unwrap();
    }
    pool
}

#[derive(Clone, Copy, Debug)]
enum Embed {
    PostPurchase,
    CustomerReferral,
    ViralGoal,
    BirthdayClub,
}
const EMBEDS: [Embed; 4] = [
    Embed::PostPurchase,
    Embed::CustomerReferral,
    Embed::ViralGoal,
    Embed::BirthdayClub,
];

async fn render(pool: &PgPool, embed: Embed, tenant: Option<&str>, hide: Option<&str>) -> String {
    let state = Extension(GrowthState { pool: pool.clone() });
    let tenant = tenant.map(str::to_owned);
    let hide_branding = hide.map(str::to_owned);
    let response = match embed {
        Embed::PostPurchase => handle_post_purchase_embed(
            state,
            Query(PostPurchaseEmbedQuery {
                tenant,
                discount: None,
                theme: None,
                hide_branding,
            }),
        )
        .await
        .into_response(),
        Embed::CustomerReferral => handle_customer_referral_embed(
            state,
            Query(CustomerReferralEmbedQuery {
                tenant,
                give: None,
                get: None,
                theme: None,
                hide_branding,
            }),
        )
        .await
        .into_response(),
        Embed::ViralGoal => handle_viral_goal_tracker(
            state,
            Query(ViralGoalTrackerQuery {
                tenant,
                target: None,
                reward: None,
                theme: None,
                hide_branding,
            }),
        )
        .await
        .into_response(),
        Embed::BirthdayClub => handle_birthday_club_embed(
            state,
            Query(BirthdayClubEmbedQuery {
                tenant,
                discount: None,
                theme: None,
                hide_branding,
            }),
        )
        .await
        .into_response(),
    };
    assert_eq!(response.status(), StatusCode::OK);
    String::from_utf8(
        to_bytes(response.into_body(), 1_000_000)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap()
}

macro_rules! pro_case {
    ($name:ident, $kind:expr) => {
        #[tokio::test]
        async fn $name() {
            let pool = setup().await;
            let html = render(&pool, $kind, Some(PRO), Some("true")).await;
            assert!(
                !html.contains("OmniSolo"),
                "verified Pro tenant must receive unbranded HTML"
            );
            let plans: Vec<(String, String)> =
                sqlx::query_as("SELECT id, plan_tier FROM tenants ORDER BY id")
                    .fetch_all(&pool)
                    .await
                    .unwrap();
            assert_eq!(
                plans,
                vec![
                    (PRO.into(), "pro".into()),
                    (FREE.into(), "free".into()),
                    (OTHER.into(), "enterprise".into())
                ]
            );
        }
    };
}
pro_case!(post_purchase_pro_uses_canonical_id, Embed::PostPurchase);
pro_case!(
    customer_referral_pro_uses_canonical_id,
    Embed::CustomerReferral
);
pro_case!(viral_goal_pro_uses_canonical_id, Embed::ViralGoal);
pro_case!(birthday_club_pro_uses_canonical_id, Embed::BirthdayClub);

#[tokio::test]
async fn branding_stays_for_free_unknown_malformed_and_missing_tenants() {
    let pool = setup().await;
    for embed in EMBEDS {
        for tenant in [
            Some(FREE),
            Some(UNKNOWN),
            Some("missing&tenant"),
            Some("' OR '1'='1"),
            None,
        ] {
            assert!(
                render(&pool, embed, tenant, Some("true"))
                    .await
                    .contains("OmniSolo"),
                "{embed:?}: {tenant:?}"
            );
        }
    }
}

#[tokio::test]
async fn exact_pro_policy_does_not_expand_to_other_plans_or_upgrade_them() {
    let pool = setup().await;
    for plan in [
        "starter",
        "enterprise",
        "premium",
        "pro-plus",
        " pro ",
        "",
        "PrO",
    ] {
        sqlx::query("UPDATE tenants SET plan_tier = $1 WHERE id = $2")
            .bind(plan)
            .bind(OTHER)
            .execute(&pool)
            .await
            .unwrap();
        for embed in EMBEDS {
            assert_eq!(
                !render(&pool, embed, Some(OTHER), Some("true"))
                    .await
                    .contains("OmniSolo"),
                plan == "PrO",
                "{embed:?}: {plan}"
            );
        }
        let saved: String = sqlx::query_scalar("SELECT plan_tier FROM tenants WHERE id = $1")
            .bind(OTHER)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(saved, plan);
    }
}

#[tokio::test]
async fn a_foreign_pro_row_never_unbrands_the_selected_free_tenant() {
    let pool = setup().await;
    for embed in EMBEDS {
        assert!(
            !render(&pool, embed, Some(PRO), Some("true"))
                .await
                .contains("OmniSolo")
        );
        assert!(
            render(&pool, embed, Some(FREE), Some("true"))
                .await
                .contains("OmniSolo")
        );
    }
    sqlx::query("UPDATE tenants SET plan_tier = CASE WHEN id = $1 THEN 'free' WHEN id = $2 THEN 'pro' ELSE plan_tier END").bind(PRO).bind(FREE).execute(&pool).await.unwrap();
    for embed in EMBEDS {
        assert!(
            render(&pool, embed, Some(PRO), Some("true"))
                .await
                .contains("OmniSolo")
        );
        assert!(
            !render(&pool, embed, Some(FREE), Some("true"))
                .await
                .contains("OmniSolo")
        );
    }
}

#[tokio::test]
async fn even_pro_requires_explicit_true_hide_request() {
    let pool = setup().await;
    for embed in EMBEDS {
        for hide in [None, Some("false"), Some("TRUE"), Some("1")] {
            assert!(
                render(&pool, embed, Some(PRO), hide)
                    .await
                    .contains("OmniSolo")
            );
        }
    }
}

#[tokio::test]
async fn lookup_failure_retains_branding() {
    let pool = setup().await;
    // A closed real pool returns a genuine error rather than a synthetic result.
    pool.close().await;
    for embed in EMBEDS {
        assert!(
            render(&pool, embed, Some(PRO), Some("true"))
                .await
                .contains("OmniSolo")
        );
    }
}

#[tokio::test]
async fn exact_production_lookups_work_on_canonical_sqlite_without_cross_tenant_fallback() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql(SQLITE_TENANTS_DDL)
        .execute(&pool)
        .await
        .unwrap();
    for (id, plan) in [(PRO, "pro"), (FREE, "free"), (OTHER, "enterprise")] {
        sqlx::query("INSERT INTO tenants (id, plan_tier) VALUES ($1, $2)")
            .bind(id)
            .bind(plan)
            .execute(&pool)
            .await
            .unwrap();
    }
    for query in LOOKUPS {
        for (id, expected) in [
            (PRO, Some("pro")),
            (FREE, Some("free")),
            (OTHER, Some("enterprise")),
            (UNKNOWN, None),
            ("' OR '1'='1", None),
        ] {
            let actual: Option<String> = sqlx::query_scalar(query)
                .bind(id)
                .fetch_optional(&pool)
                .await
                .unwrap();
            assert_eq!(actual.as_deref(), expected);
        }
    }
    let unchanged: Vec<(String, String)> =
        sqlx::query_as("SELECT id, plan_tier FROM tenants ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(
        unchanged,
        vec![
            (PRO.into(), "pro".into()),
            (FREE.into(), "free".into()),
            (OTHER.into(), "enterprise".into())
        ]
    );
}

fn assert_progress_unavailable(html: &str) {
    assert!(html.contains("Referral progress unavailable"));
    assert!(html.contains("not linked to a referral record"));
    assert!(!html.contains("referrals completed"));
    assert!(!html.contains("class=\"progress-bar\""));
    assert!(!html.contains("aria-valuenow"));
    assert!(!html.contains("Reward unlocked"));
    assert!(!html.contains("Unlock:"));
    assert!(!html.contains("Invite friends to unlock your reward"));
    assert!(!html.contains("Share to reach goal"));
    assert!(html.contains("Configured reward:"));
    assert!(html.contains("Open referral link"));
}

#[tokio::test]
async fn unbound_goal_does_not_borrow_any_users_or_tenants_referral_progress() {
    let pool = setup().await;
    sqlx::raw_sql(
        &PG_REFERRALS_DDL.replace("CREATE TABLE IF NOT EXISTS", "CREATE TEMPORARY TABLE"),
    )
    .execute(&pool)
    .await
    .unwrap();
    for (id, tenant, user, clicks, conversions) in [
        ("own-user", PRO, "alice", 20, 3),
        ("same-tenant-user", PRO, "bob", 40, 5),
        ("other-tenant-user", FREE, "carol", 100, 99),
    ] {
        sqlx::query("INSERT INTO referrals (id, tenant_id, user_id, referral_code, clicks, conversions, created_at_unix) VALUES ($1, $2, $3, $1, $4, $5, 0)")
            .bind(id).bind(tenant).bind(user).bind(clicks).bind(conversions)
            .execute(&pool).await.unwrap();
    }
    let query = "SELECT id, tenant_id, user_id, clicks, conversions FROM referrals ORDER BY id";
    let before: Vec<(String, String, String, i32, i32)> =
        sqlx::query_as(query).fetch_all(&pool).await.unwrap();
    assert_eq!(before.len(), 3);
    let own_total: i64 =
        sqlx::query_scalar("SELECT SUM(conversions) FROM referrals WHERE tenant_id = $1")
            .bind(PRO)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(own_total, 8);
    for tenant in [PRO, FREE] {
        let html = render(&pool, Embed::ViralGoal, Some(tenant), Some("true")).await;
        assert_progress_unavailable(&html);
        for identity in [
            "alice",
            "bob",
            "carol",
            "own-user",
            "same-tenant-user",
            "other-tenant-user",
        ] {
            assert!(!html.contains(identity));
        }
    }
    let after: Vec<(String, String, String, i32, i32)> =
        sqlx::query_as(query).fetch_all(&pool).await.unwrap();
    assert_eq!(
        after, before,
        "rendering must not create tracking or grant conversions"
    );
    let plans: Vec<(String, String)> =
        sqlx::query_as("SELECT id, plan_tier FROM tenants ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(
        plans,
        vec![
            (PRO.into(), "pro".into()),
            (FREE.into(), "free".into()),
            (OTHER.into(), "enterprise".into())
        ]
    );
}

#[tokio::test]
async fn unbound_goal_preserves_configuration_without_fabricated_percentages() {
    let pool = setup().await;
    for target in ["25", "0", "-1", "not-a-number"] {
        let response = handle_viral_goal_tracker(
            Extension(GrowthState { pool: pool.clone() }),
            Query(ViralGoalTrackerQuery {
                tenant: Some(PRO.into()),
                target: Some(target.into()),
                reward: Some("Early Access VIP".into()),
                theme: Some("dark".into()),
                hide_branding: Some("true".into()),
            }),
        )
        .await
        .into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let html = String::from_utf8(
            to_bytes(response.into_body(), 1_000_000)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        assert_progress_unavailable(&html);
        assert!(html.contains("Early Access VIP"));
        assert!(html.contains(&format!("{target} target")));
        assert!(!html.contains("NaN"));
        assert!(!html.contains("inf%"));
        assert!(
            !html.contains("OmniSolo"),
            "existing verified-Pro branding contract stays intact"
        );
    }
}

#[tokio::test]
async fn unavailable_tracking_never_becomes_zero_or_success_on_missing_tenant_or_database_error() {
    let pool = setup().await;
    for tenant in [Some(UNKNOWN), None] {
        let html = render(&pool, Embed::ViralGoal, tenant, Some("true")).await;
        assert_progress_unavailable(&html);
        assert!(!html.contains("0 referrals"));
        assert!(html.contains("OmniSolo"));
    }
    pool.close().await;
    let html = render(&pool, Embed::ViralGoal, Some(PRO), Some("true")).await;
    assert_progress_unavailable(&html);
    assert!(!html.contains("0 referrals"));
    assert!(html.contains("OmniSolo"));
}

async fn assert_referral_context_safe(kind: usize) {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let pool = setup().await;
    for tenant in [
        "x');globalThis.injected=true;//",
        "O'Brien",
        "\"double\" & query=?#fragment",
        "é雪😀",
        "</script><script>globalThis.injected=true</script>",
        "&#x27; onclick=globalThis.injected=true",
        "tenant-a",
        "",
    ] {
        let html = match kind {
            0 => render(&pool, Embed::CustomerReferral, Some(tenant), Some("true")).await,
            1 => render(&pool, Embed::ViralGoal, Some(tenant), Some("true")).await,
            2 => {
                let response = handle_viral_widget_embed(
                    Extension(GrowthState { pool: pool.clone() }),
                    Query(ViralWidgetEmbedQuery {
                        tenant: Some(tenant.into()),
                        theme: None,
                        title: None,
                        branding: None,
                    }),
                )
                .await
                .into_response();
                assert_eq!(response.status(), StatusCode::OK);
                String::from_utf8(
                    to_bytes(response.into_body(), 1_000_000)
                        .await
                        .unwrap()
                        .to_vec(),
                )
                .unwrap()
            }
            _ => unreachable!(),
        };
        let mut child = Command::new("python3")
            .arg(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/check_referral_html.py"
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
                serde_json::to_string(&serde_json::json!({"html":html,"tenant":tenant}))
                    .unwrap()
                    .as_bytes(),
            )
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "unsafe rendered tenant {tenant:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
#[tokio::test]
async fn customer_referral_tenant_is_data_not_javascript() {
    assert_referral_context_safe(0).await;
}
#[tokio::test]
async fn viral_goal_tenant_is_data_not_javascript() {
    assert_referral_context_safe(1).await;
}
#[tokio::test]
async fn viral_widget_tenant_is_data_not_javascript() {
    assert_referral_context_safe(2).await;
}

#[tokio::test]
async fn branding_binds_raw_tenant_identity_not_an_html_encoded_lookalike() {
    let pool = setup().await;
    let raw = "O'Brien & café";
    let lookalike = "O&#x27;Brien &amp; café";
    for (id, plan) in [(raw, "free"), (lookalike, "pro")] {
        sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES ($1, 'Regression fixture', $2)")
            .bind(id).bind(plan).execute(&pool).await.unwrap();
    }
    for embed in EMBEDS {
        assert!(
            render(&pool, embed, Some(raw), Some("true"))
                .await
                .contains("OmniSolo"),
            "{embed:?}: free raw tenant must not borrow encoded lookalike's Pro plan"
        );
    }
    sqlx::query("UPDATE tenants SET plan_tier = CASE WHEN id = $1 THEN 'pro' WHEN id = $2 THEN 'free' ELSE plan_tier END")
        .bind(raw).bind(lookalike).execute(&pool).await.unwrap();
    for embed in EMBEDS {
        assert!(
            !render(&pool, embed, Some(raw), Some("true"))
                .await
                .contains("OmniSolo"),
            "{embed:?}: raw Pro identity must remain eligible"
        );
        assert!(
            render(&pool, embed, Some(lookalike), Some("true"))
                .await
                .contains("OmniSolo"),
            "{embed:?}: literal encoded-looking free identity stays separate"
        );
    }
}
