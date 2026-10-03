use crate::builder::generation::{self, GenerationContext};
use crate::workflow_execution::{AnalysisPolicy, WorkflowExecution};
use axum::{
    Json, Router,
    extract::{Extension, State},
    http::StatusCode,
    routing::post,
};
use serde_json::{Value, json};
use server_common::Claims;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

fn store_draft() -> Value {
    json!({"theme":"Clean","domain":null,"sample_products":[],"shipping_settings":null,"tax_settings":null,"pages":[{"path":"/","title":"Workshop","seo_metadata":{},"blocks":[{"block_type":"HeroBlock","content":{"headline":"Handmade furniture","subtitle":"A draft introduction to our workshop"},"sort_order":0}]}]})
}
fn brand_draft() -> Value {
    json!({"brand_dna":{"name":"Workshop","business_type":"Furniture","positioning":"Handcrafted design proposal","audience":"People seeking handmade furniture","tone_of_voice":["clear"],"colors":["#123456"],"fonts":["Inter"],"image_style":["Natural light"],"do_not_do":["Invent customer reviews"]},"brand_book":[{"title":"Voice","guidance":["Use concrete descriptions"]}],"campaign_ideas":[{"title":"Meet the workshop","goal":"Introduce the making process","channels":["website"],"hook":"See how it is made"}],"social_calendar":[],"assets":[{"asset_type":"copy","channel":"website","title":"Introduction","copy":"Explore the workshop","visual_prompt":"Photograph the actual workshop","editable_fields":["copy"]}],"store_profile":store_draft()})
}
#[derive(Clone)]
struct ProviderState {
    calls: Arc<AtomicUsize>,
    requests: Arc<Mutex<Vec<Value>>>,
    response: Value,
    status: StatusCode,
}
struct Fixture {
    base: String,
    provider: ProviderState,
    tasks: Vec<tokio::task::JoinHandle<()>>,
    store: Arc<server_auth::Store>,
    claims: Vec<Claims>,
    tokens: Vec<String>,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        for task in &self.tasks {
            task.abort();
        }
    }
}
impl Fixture {
    async fn new(response: Value, status: StatusCode, pool: Option<sqlx::PgPool>) -> Self {
        Self::for_tenants(response, status, pool, None).await
    }
    async fn for_tenants(
        response: Value,
        status: StatusCode,
        pool: Option<sqlx::PgPool>,
        tenants: Option<[String; 2]>,
    ) -> Self {
        let provider = ProviderState {
            calls: Arc::new(AtomicUsize::new(0)),
            requests: Arc::new(Mutex::new(vec![])),
            response,
            status,
        };
        let app=Router::new().route("/v1/chat/completions",post(|State(state):State<ProviderState>,Json(body):Json<Value>| async move {
   state.calls.fetch_add(1,Ordering::SeqCst); state.requests.lock().unwrap().push(body);
   (state.status,Json(json!({"id":"local-http-fixture","choices":[{"message":{"role":"assistant","content":state.response.to_string()},"finish_reason":"stop"}],"usage":{"prompt_tokens":25,"completion_tokens":31,"total_tokens":56}})))
  })).with_state(provider.clone());
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/v1", listener.local_addr().unwrap());
        let mut tasks = vec![tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap()
        })];
        let analysis = Arc::new(crate::tenant_analysis::local_http_fixture(&endpoint));
        let store = Arc::new(server_auth::Store::new());
        let mut tokens = vec![];
        let mut claims = vec![];
        for index in 0..2 {
            let tenant = tenants
                .as_ref()
                .map(|values| values[index].clone())
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            let user = store
                .create_user(
                    format!("owner-{tenant}"),
                    format!("owner-{tenant}@example.test"),
                    "public-local-only-password".into(),
                    vec![server_auth::ROLE_ADMIN.into()],
                    tenant,
                )
                .await
                .unwrap();
            let token = store.issue_token(&user).unwrap();
            let signed = store.validate_token(&token).await.unwrap();
            if let Some(pool) = &pool {
                let tenant = signed.organization_id.as_deref().unwrap();
                let mut tx = pool.begin().await.unwrap();
                server_common::auth_utils::set_org_context(&mut *tx, tenant)
                    .await
                    .unwrap();
                sqlx::query("INSERT INTO tenants(id,name) VALUES($1,'Owned fixture business')")
                    .bind(tenant)
                    .execute(&mut *tx)
                    .await
                    .unwrap();
                sqlx::query("INSERT INTO users(id,username,email,tenant_id,active,roles) VALUES($1,$1,$1,$2,true,ARRAY['ADMIN'])").bind(&signed.sub).bind(tenant).execute(&mut *tx).await.unwrap();
                sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position) VALUES($1,'ADMIN',$2,0)").bind(&signed.sub).bind(tenant).execute(&mut *tx).await.unwrap();
                tx.commit().await.unwrap();
            }
            claims.push(signed);
            tokens.push(token);
            let _ = index;
        }
        let policy = AnalysisPolicy::new(
            analysis.provider().into(),
            analysis.model().into(),
            analysis.max_output_tokens(),
        )
        .unwrap();
        let execution = Arc::new(WorkflowExecution::configured(
            store.clone(),
            policy,
            Arc::new(crate::ConfiguredWorkflowInference(analysis)),
        ));
        let context = Arc::new(GenerationContext::new(
            execution,
            claims[0].organization_id.clone(),
        ));
        let auth = store.clone();
        let pool = pool.unwrap_or_else(|| {
            sqlx::postgres::PgPoolOptions::new()
                .acquire_timeout(std::time::Duration::from_millis(100))
                .connect_lazy("postgresql://invalid:invalid@127.0.0.1:1/unused")
                .unwrap()
        });
        let app = generation::router(pool.clone())
            .merge(crate::legacy_brand_reader::router(pool))
            .layer(Extension(context))
            .layer(axum::middleware::from_fn(
                move |mut request: axum::extract::Request, next: axum::middleware::Next| {
                    let auth = auth.clone();
                    async move {
                        if let Some(token) = request
                            .headers()
                            .get("authorization")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|v| v.strip_prefix("Bearer "))
                            && let Ok(claims) = auth.validate_token(token).await
                        {
                            request.extensions_mut().insert(claims);
                        }
                        next.run(request).await
                    }
                },
            ));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        tasks.push(tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap()
        }));
        Self {
            base,
            provider,
            tasks,
            store,
            claims,
            tokens,
        }
    }
    async fn post(&self, index: usize, path: &str, input: Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(format!("{}{path}", self.base))
            .bearer_auth(&self.tokens[index])
            .json(&input)
            .send()
            .await
            .unwrap()
    }
}
#[tokio::test]
async fn signed_owner_receives_only_actual_provider_draft_and_honest_provenance() {
    let f = Fixture::new(store_draft(), StatusCode::OK, None).await;
    let response = f
        .post(
            0,
            "/generate",
            json!({"description":"I make handmade furniture"}),
        )
        .await;
    assert_eq!(response.status(), 200);
    let output: Value = response.json().await.unwrap();
    assert_eq!(output["pages"], store_draft()["pages"]);
    assert_eq!(output["generation"]["kind"], "model_draft");
    assert_eq!(output["generation"]["provider"], "openai-compatible");
    assert_eq!(output["generation"]["model"], "owned-contract-model");
    assert_eq!(output["generation"]["website_fetched"], false);
    assert_eq!(output["generation"]["business_facts_verified"], false);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
    let requests = f.provider.requests.lock().unwrap();
    assert_eq!(requests[0]["model"], "owned-contract-model");
    assert_eq!(requests[0]["max_tokens"], 2048);
    assert!(
        requests[0]
            .get("tools")
            .is_none_or(|v| v.as_array().is_some_and(Vec::is_empty))
    );
    assert!(
        requests[0]
            .to_string()
            .contains("I make handmade furniture")
    );
    assert!(!requests[0].to_string().contains(&f.claims[1].sub));
}
#[tokio::test]
async fn foreign_tenant_and_revoked_owner_do_not_consume_global_provider_configuration() {
    let f = Fixture::new(store_draft(), StatusCode::OK, None).await;
    assert_eq!(
        f.post(1, "/generate", json!({"description":"Foreign request"}))
            .await
            .status(),
        403
    );
    f.store
        .revoke_token(
            f.claims[0].jti.clone(),
            chrono::DateTime::from_timestamp(f.claims[0].exp, 0).unwrap(),
            f.claims[0].organization_id.as_deref().unwrap(),
        )
        .await
        .unwrap();
    assert!(
        !f.post(0, "/generate", json!({"description":"Revoked request"}))
            .await
            .status()
            .is_success()
    );
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 0);
}
#[tokio::test]
async fn urls_and_asset_names_are_rejected_before_any_unfetched_source_claim_or_provider_call() {
    let f = Fixture::new(store_draft(), StatusCode::OK, None).await;
    for input in [
        json!({"description":"Workshop","website_url":"https://example.test"}),
        json!({"description":"Workshop","product_url":"http://127.0.0.1/private"}),
        json!({"description":"Workshop","uploaded_asset_names":["photo.jpg"]}),
    ] {
        for path in ["/generate", "/brand_toolbox/generate"] {
            let response = f.post(0, path, input.clone()).await;
            assert_eq!(response.status(), 422);
            assert_eq!(
                response.json::<Value>().await.unwrap()["code"],
                "source_fetch_unavailable"
            );
        }
    }
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 0);
}
#[tokio::test]
async fn malformed_or_fact_fabricating_provider_output_is_not_a_successful_storefront() {
    for key in [
        "sample_products",
        "shipping_settings",
        "tax_settings",
        "pages",
    ] {
        let mut draft = store_draft();
        draft[key] = match key {
            "sample_products" => json!([{"name":"Invented","price":29}]),
            "pages" => json!([]),
            _ => json!({"default_rate":5}),
        };
        let f = Fixture::new(draft, StatusCode::OK, None).await;
        let response = f
            .post(0, "/generate", json!({"description":"Workshop"}))
            .await;
        assert_eq!(response.status(), 502, "field {key}");
        assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
    }
}
#[tokio::test]
async fn provider_failure_is_not_retried_or_replaced_with_a_template() {
    let f = Fixture::new(json!({}), StatusCode::SERVICE_UNAVAILABLE, None).await;
    let response = f
        .post(0, "/generate", json!({"description":"Workshop"}))
        .await;
    assert_eq!(response.status(), 502);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        response.json::<Value>().await.unwrap()["code"],
        "generation_outcome_unknown"
    );
}
#[tokio::test]
async fn content_heuristic_is_not_reported_as_measured_search_visibility() {
    let f = Fixture::new(store_draft(), StatusCode::OK, None).await;
    let response = f
        .post(
            0,
            "/geo_score",
            json!({"content":"The best bakery in Austin schema.org"}),
        )
        .await;
    assert_eq!(response.status(), 503);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["code"], "visibility_measurement_unavailable");
    assert!(body.get("generative_score").is_none());
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 0);
}

struct DatabaseFixture {
    admin: sqlx::PgPool,
    pool: sqlx::PgPool,
    schema: String,
    role: String,
}
impl DatabaseFixture {
    async fn new() -> Self {
        let url = std::env::var("OHC_BUILDER_GENERATION_TEST_DATABASE_URL")
            .expect("explicit disposable PostgreSQL is required");
        let parsed = reqwest::Url::parse(&url).unwrap();
        assert!(
            matches!(parsed.host_str(), Some("127.0.0.1" | "localhost" | "[::1]"))
                && parsed.path().starts_with("/ohc_")
                && parsed.path().ends_with("_test")
                && parsed.query().is_none()
                && parsed.fragment().is_none(),
            "owned disposable loopback database required"
        );
        let admin = sqlx::PgPool::connect(&url).await.unwrap();
        let schema = format!("generation_{}", uuid::Uuid::new_v4().simple());
        let role = format!("generation_role_{}", uuid::Uuid::new_v4().simple());
        let password = uuid::Uuid::new_v4().simple().to_string();
        sqlx::raw_sql(&format!("CREATE SCHEMA {schema};CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS PASSWORD '{password}';")).execute(&admin).await.unwrap();
        let scoped = sqlx::postgres::PgPoolOptions::new()
            .after_connect({
                let schema = schema.clone();
                move |conn, _| {
                    let q = format!("SET search_path TO {schema}");
                    Box::pin(async move {
                        sqlx::query(&q).execute(conn).await?;
                        Ok(())
                    })
                }
            })
            .connect(&url)
            .await
            .unwrap();
        let initial = include_str!("../../src/server/migrations/001_initial.sql");
        for table in ["tenants", "users"] {
            let start = initial
                .find(&format!("CREATE TABLE IF NOT EXISTS {table} ("))
                .unwrap();
            let end = initial[start..].find(");").unwrap() + start + 2;
            sqlx::raw_sql(&initial[start..end])
                .execute(&scoped)
                .await
                .unwrap();
        }
        {
            use sea_orm::ConnectionTrait;
            let orm = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(scoped.clone());
            let backend = sea_orm::DatabaseBackend::Postgres;
            orm.execute(
                backend.build(&sea_orm::Schema::new(backend).create_table_from_entity(
                    server_auth::seaorm_store::entities::identity_user_role::Entity,
                )),
            )
            .await
            .unwrap();
        }
        for table in ["users", "identity_user_roles"] {
            sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; ALTER TABLE {table} FORCE ROW LEVEL SECURITY; CREATE POLICY scoped ON {table} USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true));")).execute(&scoped).await.unwrap();
        }
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/059_brand_toolboxes.sql"
        ))
        .execute(&scoped)
        .await
        .unwrap();
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO {role};GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
        let options = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .unwrap()
            .username(&role)
            .password(&password);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_connect({
                let schema = schema.clone();
                move |conn, _| {
                    let q = format!("SET search_path TO {schema}");
                    Box::pin(async move {
                        sqlx::query(&q).execute(conn).await?;
                        Ok(())
                    })
                }
            })
            .connect_with(options)
            .await
            .unwrap();
        let privileges: (bool, bool) =
            sqlx::query_as("SELECT rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(privileges, (false, false));
        scoped.close().await;
        Self {
            admin,
            pool,
            schema,
            role,
        }
    }
    async fn close(self) {
        self.pool.close().await;
        sqlx::raw_sql(&format!(
            "DROP SCHEMA {} CASCADE; DROP ROLE {};",
            self.schema, self.role
        ))
        .execute(&self.admin)
        .await
        .unwrap();
        self.admin.close().await;
    }
}
#[tokio::test]
async fn actual_brand_draft_is_saved_once_with_provenance_and_cannot_cross_tenants() {
    let db = DatabaseFixture::new().await;
    let f = Fixture::new(brand_draft(), StatusCode::OK, Some(db.pool.clone())).await;
    let response = f
        .post(
            0,
            "/brand_toolbox/generate",
            json!({"description":"We make handmade furniture"}),
        )
        .await;
    assert_eq!(response.status(), 200);
    let toolbox: Value = response.json().await.unwrap();
    let id = uuid::Uuid::parse_str(toolbox["id"].as_str().unwrap()).unwrap();
    assert_eq!(toolbox["catalog"], json!([]));
    assert_eq!(toolbox["logo_concepts"], json!([]));
    assert_eq!(toolbox["photoshoot"]["shots"], json!([]));
    let tenant = uuid::Uuid::parse_str(f.claims[0].organization_id.as_deref().unwrap()).unwrap();
    let record = crate::db::get_brand_toolbox(
        &db.pool,
        tenant,
        f.claims[0].organization_id.as_deref().unwrap(),
        id,
    )
    .await
    .unwrap();
    assert_eq!(record.toolbox["generation"], toolbox["generation"]);
    let saved: generation::BrandToolboxResponse =
        serde_json::from_value(record.toolbox.clone()).unwrap();
    assert!(generation::has_generation_provenance(
        &saved,
        f.claims[0].organization_id.as_deref().unwrap()
    ));
    let mut legacy = saved;
    legacy.generation = None;
    assert!(!generation::has_generation_provenance(
        &legacy,
        f.claims[0].organization_id.as_deref().unwrap()
    ));
    assert_eq!(
        record.toolbox["store_profile"]["pages"],
        store_draft()["pages"]
    );
    let other = uuid::Uuid::parse_str(f.claims[1].organization_id.as_deref().unwrap()).unwrap();
    assert!(
        crate::db::get_brand_toolbox(
            &db.pool,
            other,
            f.claims[1].organization_id.as_deref().unwrap(),
            id
        )
        .await
        .is_err()
    );
    assert!(
        crate::db::list_brand_toolboxes(
            &db.pool,
            other,
            f.claims[1].organization_id.as_deref().unwrap()
        )
        .await
        .unwrap()
        .is_empty()
    );
    assert_eq!(
        crate::db::list_brand_toolboxes(
            &db.pool,
            tenant,
            f.claims[0].organization_id.as_deref().unwrap()
        )
        .await
        .unwrap()
        .len(),
        1
    );
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
    drop(f);
    db.close().await;
}
#[tokio::test]
async fn storage_failure_does_not_return_an_unsaved_successful_toolbox() {
    let f = Fixture::new(brand_draft(), StatusCode::OK, None).await;
    let response = f
        .post(
            0,
            "/brand_toolbox/generate",
            json!({"description":"Workshop"}),
        )
        .await;
    assert_eq!(response.status(), 503);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["code"], "generation_storage_unavailable");
    assert!(body.get("id").is_none());
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
}
#[tokio::test]
async fn absent_generation_context_never_returns_success_or_touches_storage() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(30))
        .connect_lazy("postgresql://invalid:invalid@127.0.0.1:1/unused")
        .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let app = generation::router(pool).layer(Extension(Claims {
        sub: "owner".into(),
        username: "owner".into(),
        email: "owner@example.test".into(),
        roles: vec!["ADMIN".into()],
        organization_id: Some("operator".into()),
        exp: chrono::Utc::now().timestamp() + 3600,
        iat: 0,
        jti: "public-fixture-jti".into(),
        session_id: None,
    }));
    let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    for path in ["/generate", "/brand_toolbox/generate"] {
        let response = reqwest::Client::new()
            .post(format!("{base}{path}"))
            .json(&json!({"description":"Workshop"}))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 503);
        assert_eq!(
            response.json::<Value>().await.unwrap()["code"],
            "generation_unavailable"
        );
    }
    task.abort();
}

#[tokio::test]
async fn legacy_tenant_and_its_uuid_alias_cannot_read_each_others_toolboxes() {
    let db = DatabaseFixture::new().await;
    let raw = format!("legacy-shop-{}", uuid::Uuid::new_v4().simple());
    let alias = uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_DNS, raw.as_bytes()).to_string();
    let f = Fixture::for_tenants(
        brand_draft(),
        StatusCode::OK,
        Some(db.pool.clone()),
        Some([raw.clone(), alias]),
    )
    .await;
    let response = f
        .post(
            0,
            "/brand_toolbox/generate",
            json!({"description":"Private workshop draft"}),
        )
        .await;
    assert_eq!(response.status(), 200);
    let output: Value = response.json().await.unwrap();
    let id = output["id"].as_str().unwrap();
    let client = reqwest::Client::new();
    let own = client
        .get(format!("{}/brand_toolbox/{id}", f.base))
        .bearer_auth(&f.tokens[0])
        .send()
        .await
        .unwrap();
    assert_eq!(own.status(), 200);
    let foreign = client
        .get(format!("{}/brand_toolbox/{id}", f.base))
        .bearer_auth(&f.tokens[1])
        .send()
        .await
        .unwrap();
    assert_eq!(
        foreign.status(),
        404,
        "Raw tenant identity must survive the UUID compatibility mapping"
    );
    let foreign = client
        .get(format!("{}/brand_toolbox", f.base))
        .bearer_auth(&f.tokens[1])
        .send()
        .await
        .unwrap();
    assert_eq!(foreign.status(), 200);
    assert_eq!(foreign.json::<Value>().await.unwrap(), json!([]));
    drop(f);
    db.close().await;
}

#[tokio::test]
async fn revocation_while_storage_is_blocked_prevents_commit_and_success() {
    let db = DatabaseFixture::new().await;
    let f = Fixture::new(brand_draft(), StatusCode::OK, Some(db.pool.clone())).await;
    let mut blocker = db.admin.begin().await.unwrap();
    sqlx::query(&format!(
        "LOCK TABLE {}.builder_brand_toolboxes IN ACCESS EXCLUSIVE MODE",
        db.schema
    ))
    .execute(&mut *blocker)
    .await
    .unwrap();
    let base = f.base.clone();
    let token = f.tokens[0].clone();
    let request = tokio::spawn(async move {
        reqwest::Client::new()
            .post(format!("{base}/brand_toolbox/generate"))
            .bearer_auth(token)
            .json(&json!({"description":"Workshop"}))
            .send()
            .await
            .unwrap()
    });
    tokio::time::timeout(std::time::Duration::from_secs(3),async {
        loop {
            let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE usename=$1 AND wait_event_type='Lock' AND query LIKE '%INSERT INTO builder_brand_toolboxes%')").bind(&db.role).fetch_one(&db.admin).await.unwrap();
            if waiting {break;} tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }).await.expect("actual INSERT must be waiting on the owned table lock");
    f.store
        .revoke_token(
            f.claims[0].jti.clone(),
            chrono::DateTime::from_timestamp(f.claims[0].exp, 0).unwrap(),
            f.claims[0].organization_id.as_deref().unwrap(),
        )
        .await
        .unwrap();
    blocker.commit().await.unwrap();
    let response = request.await.unwrap();
    assert_eq!(response.status(), 403);
    let count: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM {}.builder_brand_toolboxes",
        db.schema
    ))
    .fetch_one(&db.admin)
    .await
    .unwrap();
    assert_eq!(count, 0);
    drop(f);
    db.close().await;
}

#[tokio::test]
async fn canonical_database_role_revocation_blocks_persisting_a_cached_actor() {
    let db = DatabaseFixture::new().await;
    let f = Fixture::new(brand_draft(), StatusCode::OK, Some(db.pool.clone())).await;
    sqlx::query(&format!(
        "DELETE FROM {}.identity_user_roles WHERE user_id=$1",
        db.schema
    ))
    .bind(&f.claims[0].sub)
    .execute(&db.admin)
    .await
    .unwrap();
    let response = f
        .post(
            0,
            "/brand_toolbox/generate",
            json!({"description":"Workshop"}),
        )
        .await;
    assert_eq!(response.status(), 403);
    let count: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM {}.builder_brand_toolboxes",
        db.schema
    ))
    .fetch_one(&db.admin)
    .await
    .unwrap();
    assert_eq!(count, 0);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
    drop(f);
    db.close().await;
}
