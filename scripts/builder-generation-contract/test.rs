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
        Self::for_authority(response, status, pool, tenants, false).await
    }
    async fn for_authority(
        response: Value,
        status: StatusCode,
        pool: Option<sqlx::PgPool>,
        tenants: Option<[String; 2]>,
        separate_authority: bool,
    ) -> Self {
        Self::for_store_pools(response, status, pool, tenants, separate_authority, None).await
    }
    async fn for_store_pools(
        response: Value,
        status: StatusCode,
        pool: Option<sqlx::PgPool>,
        tenants: Option<[String; 2]>,
        separate_authority: bool,
        authority_pool_override: Option<sqlx::PgPool>,
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
        let authority_pool = authority_pool_override.as_ref().or(if separate_authority {
            None
        } else {
            pool.as_ref()
        });
        let store = Arc::new(match authority_pool {
            Some(pool) => server_auth::Store::with_portable_repo(Arc::new(
                server_auth::seaorm_store::SeaOrmAuthRepository::new(
                    sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone()),
                ),
            )),
            None if separate_authority => server_auth::Store::new(),
            None => {
                let database = crate::persistence::AppDatabase::connect("sqlite::memory:")
                    .await
                    .unwrap();
                crate::persistence::migration::migrate(&database)
                    .await
                    .unwrap();
                server_auth::Store::with_portable_repo(Arc::new(
                    server_auth::seaorm_store::SeaOrmAuthRepository::new(
                        database.connection().clone(),
                    ),
                ))
            }
        });
        let mut tokens = vec![];
        let mut claims = vec![];
        for index in 0..2 {
            let tenant = tenants
                .as_ref()
                .map(|values| values[index].clone())
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
            if let Some(pool) = &pool {
                let mut tx = pool.begin().await.unwrap();
                server_common::auth_utils::set_org_context(&mut *tx, &tenant)
                    .await
                    .unwrap();
                sqlx::query("INSERT INTO tenants(id,name) VALUES($1,'Owned fixture business')")
                    .bind(&tenant)
                    .execute(&mut *tx)
                    .await
                    .unwrap();
                tx.commit().await.unwrap();
            }
            if let Some(authority_pool) = &authority_pool_override {
                let mut tx = authority_pool.begin().await.unwrap();
                server_common::auth_utils::set_org_context(&mut *tx, &tenant)
                    .await
                    .unwrap();
                sqlx::query("INSERT INTO tenants(id,name) VALUES($1,'Owned authority fixture')")
                    .bind(&tenant)
                    .execute(&mut *tx)
                    .await
                    .unwrap();
                tx.commit().await.unwrap();
            }
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
            if let Some(pool) = &pool
                && (separate_authority || authority_pool_override.is_some())
            {
                let tenant = signed.organization_id.as_deref().unwrap();
                let mut tx = pool.begin().await.unwrap();
                server_common::auth_utils::set_org_context(&mut *tx, tenant)
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
        let mut app = match &pool {
            Some(pool) => generation::router(Some(pool.clone())),
            None => crate::non_pg_builder::storage_independent_router(),
        };
        if let Some(pool) = pool {
            app = app.merge(crate::legacy_brand_reader::router(pool));
        }
        let app = app
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
    created_bypass: bool,
}
impl DatabaseFixture {
    async fn new() -> Self {
        let url = std::env::var("OHC_BUILDER_GENERATION_TEST_DATABASE_URL")
            .expect("explicit disposable PostgreSQL is required");
        Self::new_at(&url).await
    }
    async fn new_at(url: &str) -> Self {
        let parsed = reqwest::Url::parse(url).unwrap();
        assert!(
            matches!(parsed.host_str(), Some("127.0.0.1" | "localhost" | "[::1]"))
                && parsed.path().starts_with("/ohc_")
                && parsed.path().ends_with("_test")
                && parsed.query().is_none()
                && parsed.fragment().is_none(),
            "owned disposable loopback database required"
        );
        let admin = sqlx::PgPool::connect(url).await.unwrap();
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
            .connect(url)
            .await
            .unwrap();
        let bypass_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM pg_roles WHERE rolname='ohc_bypassrls')",
        )
        .fetch_one(&scoped)
        .await
        .unwrap();
        if !bypass_exists {
            sqlx::query("CREATE ROLE ohc_bypassrls NOLOGIN BYPASSRLS")
                .execute(&scoped)
                .await
                .unwrap();
        }
        let bypass_mode: (bool, bool) = sqlx::query_as(
            "SELECT rolbypassrls,rolcanlogin FROM pg_roles WHERE rolname='ohc_bypassrls'",
        )
        .fetch_one(&scoped)
        .await
        .unwrap();
        assert_eq!(bypass_mode, (true, false));
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO ohc_bypassrls; ALTER DEFAULT PRIVILEGES IN SCHEMA {schema} GRANT ALL ON TABLES TO ohc_bypassrls;")).execute(&scoped).await.unwrap();
        let initial = include_str!("../../src/server/migrations/001_initial.sql");
        for table in ["tenants", "users", "products"] {
            let start = initial
                .find(&format!("CREATE TABLE IF NOT EXISTS {table} ("))
                .unwrap();
            let end = initial[start..].find(");").unwrap() + start + 2;
            sqlx::raw_sql(&initial[start..end])
                .execute(&scoped)
                .await
                .unwrap();
        }
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/1018_agent_definition_marketplace.sql"
        ))
        .execute(&scoped)
        .await
        .unwrap();
        let auth_database = crate::persistence::AppDatabase::from_connection(
            sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(scoped.clone()),
        );
        crate::persistence::migration::migrate(&auth_database)
            .await
            .unwrap();
        for table in ["users", "identity_user_roles"] {
            sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; ALTER TABLE {table} FORCE ROW LEVEL SECURITY; CREATE POLICY scoped ON {table} USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true));")).execute(&scoped).await.unwrap();
        }
        sqlx::raw_sql(include_str!("../../src/server/migrations/009_builder.sql"))
            .execute(&scoped)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/1019_site_publication_receipts.sql"
        ))
        .execute(&scoped)
        .await
        .unwrap();
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
            created_bypass: !bypass_exists,
        }
    }
    async fn additional_data_pool(&self) -> sqlx::PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .after_connect({
                let schema = self.schema.clone();
                move |connection, _| {
                    let query = format!("SET search_path TO {schema}");
                    Box::pin(async move {
                        sqlx::query(&query).execute(connection).await?;
                        Ok(())
                    })
                }
            })
            .connect_with((*self.pool.connect_options()).clone())
            .await
            .unwrap()
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
        if self.created_bypass {
            sqlx::query("DROP ROLE ohc_bypassrls")
                .execute(&self.admin)
                .await
                .unwrap();
        }
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
    let db = DatabaseFixture::new().await;
    let f = Fixture::new(brand_draft(), StatusCode::OK, Some(db.pool.clone())).await;
    sqlx::query(&format!("DROP TABLE {}.builder_brand_toolboxes", db.schema))
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
    assert_eq!(response.status(), 503);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["code"], "generation_storage_unavailable");
    assert!(body.get("id").is_none());
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
    drop(f);
    db.close().await;
}
#[tokio::test]
async fn absent_generation_context_never_returns_success_or_touches_storage() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_millis(30))
        .connect_lazy("postgresql://invalid:invalid@127.0.0.1:1/unused")
        .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let app = generation::router(Some(pool)).layer(Extension(Claims {
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
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 0);
    drop(f);
    db.close().await;
}

#[tokio::test]
async fn separate_memory_authority_cannot_save_brand_or_consume_a_provider_request() {
    let db = DatabaseFixture::new().await;
    let f = Fixture::for_authority(
        brand_draft(),
        StatusCode::OK,
        Some(db.pool.clone()),
        None,
        true,
    )
    .await;
    assert!(f.store.portable_repo().is_none());
    let response = f
        .post(
            0,
            "/brand_toolbox/generate",
            json!({"description":"Separated authority must not grant a private write"}),
        )
        .await;
    let status = response.status();
    let count: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM {}.builder_brand_toolboxes",
        db.schema
    ))
    .fetch_one(&db.admin)
    .await
    .unwrap();
    let provider_calls = f.provider.calls.load(Ordering::SeqCst);
    drop(f);
    db.close().await;
    assert_eq!(status, 503);
    assert_eq!(provider_calls, 0);
    assert_eq!(count, 0);
}

#[tokio::test]
async fn canonical_token_revocation_writer_waits_for_brand_final_commit_fence() {
    let db = DatabaseFixture::new().await;
    let f = Fixture::new(brand_draft(), StatusCode::OK, Some(db.pool.clone())).await;
    assert!(f.store.portable_repo().is_some());
    let gate: i64 = 846_012_735;
    sqlx::raw_sql(&format!("CREATE FUNCTION {}.brand_final_commit_gate() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN PERFORM pg_catalog.pg_advisory_xact_lock({gate}); RETURN NEW; END $$; CREATE CONSTRAINT TRIGGER owned_brand_commit_gate AFTER INSERT ON {}.builder_brand_toolboxes DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION {}.brand_final_commit_gate();",db.schema,db.schema,db.schema)).execute(&db.admin).await.unwrap();
    let mut blocker = db.admin.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(gate)
        .execute(&mut *blocker)
        .await
        .unwrap();
    let base = f.base.clone();
    let token = f.tokens[0].clone();
    let request = tokio::spawn(async move {
        reqwest::Client::new()
            .post(format!("{base}/brand_toolbox/generate"))
            .bearer_auth(token)
            .json(&json!({"description":"Owned final-commit fixture"}))
            .send()
            .await
            .unwrap()
    });
    tokio::time::timeout(std::time::Duration::from_secs(3),async {
        loop {
            let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE usename=$1 AND wait_event_type='Lock' AND query ILIKE 'COMMIT%')").bind(&db.role).fetch_one(&db.admin).await.unwrap();
            if waiting {break;}tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }).await.expect("owned deferred trigger must block the actual final COMMIT");
    let store = f.store.clone();
    let claims = f.claims[0].clone();
    let revoke = tokio::spawn(async move {
        store
            .revoke_token(
                claims.jti,
                chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                claims.organization_id.as_deref().unwrap(),
            )
            .await
    });
    let writer_waited=tokio::time::timeout(std::time::Duration::from_millis(500),async {
        loop {
            if revoke.is_finished() {break false;}
            let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE usename=$1 AND wait_event_type='Lock' AND query LIKE '%auth_revoked_tokens%')").bind(&db.role).fetch_one(&db.admin).await.unwrap();
            if waiting {break true;}tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }).await.unwrap_or(false);
    blocker.commit().await.unwrap();
    let status = request.await.unwrap().status();
    revoke.await.unwrap().unwrap();
    assert!(f.store.validate_token(&f.tokens[0]).await.is_err());
    drop(f);
    db.close().await;
    assert!(
        writer_waited,
        "canonical revoked-token INSERT committed inside an unfenced data COMMIT"
    );
    assert_eq!(
        status, 200,
        "a transaction ordered before revocation remains acknowledged"
    );
}

#[tokio::test]
async fn separate_postgres_authority_cannot_use_copied_roles_to_write_another_store() {
    let authority = DatabaseFixture::new().await;
    let data = DatabaseFixture::new().await;
    let f = Fixture::for_store_pools(
        brand_draft(),
        StatusCode::OK,
        Some(data.pool.clone()),
        None,
        true,
        Some(authority.pool.clone()),
    )
    .await;
    assert!(f.store.portable_repo().is_some());
    assert!(!std::ptr::eq(authority.pool.options(), data.pool.options()));
    let response = f
        .post(
            0,
            "/brand_toolbox/generate",
            json!({"description":"Copied identities do not establish atomic authority"}),
        )
        .await;
    let status = response.status();
    let count: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(*) FROM {}.builder_brand_toolboxes",
        data.schema
    ))
    .fetch_one(&data.admin)
    .await
    .unwrap();
    let provider_calls = f.provider.calls.load(Ordering::SeqCst);
    drop(f);
    data.close().await;
    authority.close().await;
    assert_eq!(status, 503);
    assert_eq!(provider_calls, 0);
    assert_eq!(count, 0);
}

#[tokio::test]
async fn application_builder_mount_uses_the_existing_canonical_pool() {
    let db = DatabaseFixture::new().await;
    let canonical = crate::persistence::AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(db.pool.clone()),
    );
    let legacy = db.additional_data_pool().await;
    assert!(!std::ptr::eq(legacy.options(), db.pool.options()));
    let selected = crate::application_builder_pool(legacy.clone(), &canonical)
        .await
        .unwrap();
    let same_pool = std::ptr::eq(selected.options(), db.pool.options());
    let current_user: String = sqlx::query_scalar("SELECT current_user::text")
        .fetch_one(&selected)
        .await
        .unwrap();
    let expected_role = db.role.clone();
    drop(selected);
    legacy.close().await;
    db.close().await;
    assert!(
        same_pool,
        "run_server must mount builder persistence on the existing canonical auth pool"
    );
    assert_eq!(current_user, expected_role);
}

#[tokio::test]
async fn sqlite_builder_mount_never_adopts_a_separate_postgres_database() {
    let db = DatabaseFixture::new().await;
    let sqlite = crate::persistence::AppDatabase::connect("sqlite::memory:")
        .await
        .unwrap();
    let selected = crate::application_builder_pool(db.pool.clone(), &sqlite).await;
    let rejected = selected.is_none();
    drop(selected);
    db.close().await;
    assert!(
        rejected,
        "a SQLite authority must not silently select the legacy dummy PostgreSQL data pool"
    );
}

#[tokio::test]
async fn missing_postgres_storage_rejects_private_routes_before_provider_work() {
    let f = Fixture::new(store_draft(), StatusCode::OK, None).await;
    use sea_orm::ConnectionTrait;
    assert_eq!(
        f.store
            .portable_repo()
            .unwrap()
            .connection()
            .get_database_backend(),
        sea_orm::DatabaseBackend::Sqlite
    );
    for (method, path) in [
        (reqwest::Method::GET, "/sites"),
        (reqwest::Method::POST, "/sites"),
        (
            reqwest::Method::POST,
            "/sites/00000000-0000-4000-8000-000000000001/pages",
        ),
        (reqwest::Method::GET, "/brand_toolbox"),
        (reqwest::Method::POST, "/publish_draft"),
        (reqwest::Method::POST, "/brand_toolbox/generate"),
    ] {
        let response = reqwest::Client::new()
            .request(method, format!("{}{path}", f.base))
            .bearer_auth(&f.tokens[0])
            .json(&json!({"description":"Owner supplied draft text"}))
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), 503, "{path}");
    }
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 0);
    let response = f
        .post(
            0,
            "/generate",
            json!({"description":"Text drafting remains usable"}),
        )
        .await;
    assert_eq!(response.status(), 200);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn startup_private_storage_cannot_relocate_a_separate_configured_pg_schema() {
    let auth = DatabaseFixture::new().await;
    let data = DatabaseFixture::new().await;
    let canonical = crate::persistence::AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(auth.pool.clone()),
    );
    let selected = crate::application_builder_pool(data.pool.clone(), &canonical).await;
    let rejected = selected.is_none();
    drop(selected);
    let data_exists: bool = sqlx::query_scalar("SELECT to_regclass($1) IS NOT NULL")
        .bind(format!("{}.builder_brand_toolboxes", data.schema))
        .fetch_one(&data.admin)
        .await
        .unwrap();
    data.close().await;
    auth.close().await;
    assert!(
        rejected,
        "private writes must not move from the configured business schema into the auth schema"
    );
    assert!(
        data_exists,
        "classification cannot delete or relocate the configured data"
    );
}

#[tokio::test]
async fn startup_missing_business_relation_proof_does_not_authorize_private_storage() {
    let db = DatabaseFixture::new().await;
    let data = db.additional_data_pool().await;
    let canonical = crate::persistence::AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(db.pool.clone()),
    );
    sqlx::query(&format!("DROP TABLE {}.builder_brand_toolboxes", db.schema))
        .execute(&db.admin)
        .await
        .unwrap();
    let selected = crate::application_builder_pool(data.clone(), &canonical).await;
    let rejected = selected.is_none();
    drop(selected);
    data.close().await;
    db.close().await;
    assert!(
        rejected,
        "missing relation identity is not proof of a common business store"
    );
}

#[tokio::test]
async fn startup_identical_single_connection_pool_needs_no_second_connection() {
    let db = DatabaseFixture::new().await;
    let pool = db.additional_data_pool().await;
    let canonical = crate::persistence::AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone()),
    );
    let held = pool.acquire().await.unwrap();
    let selected = tokio::time::timeout(
        std::time::Duration::from_millis(250),
        crate::application_builder_pool(pool.clone(), &canonical),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(std::ptr::eq(selected.options(), pool.options()));
    drop(held);
    drop(selected);
    pool.close().await;
    db.close().await;
}

#[tokio::test]
async fn startup_inconclusive_pool_probe_is_bounded_and_releases_its_transaction() {
    let db = DatabaseFixture::new().await;
    let data = db.additional_data_pool().await;
    let canonical = crate::persistence::AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(db.pool.clone()),
    );
    let held = data.acquire().await.unwrap();
    let began = std::time::Instant::now();
    let selected = tokio::time::timeout(
        std::time::Duration::from_secs(4),
        crate::application_builder_pool(data.clone(), &canonical),
    )
    .await
    .unwrap();
    let rejected = selected.is_none();
    drop(selected);
    drop(held);
    assert!(began.elapsed() < std::time::Duration::from_secs(4));
    tokio::time::timeout(std::time::Duration::from_secs(2),async {
        loop {
            let open:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE usename=$1 AND state='idle in transaction')")
                .bind(&db.role).fetch_one(&db.admin).await.unwrap();
            if !open {break;}
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }).await.expect("a timed-out probe must release its transaction");
    data.close().await;
    db.close().await;
    assert!(
        rejected,
        "inconclusive pool identity cannot authorize private writes"
    );
}

#[tokio::test]
async fn startup_sqlite_business_store_never_selects_its_dummy_pg_handle() {
    let auth = DatabaseFixture::new().await;
    let canonical = crate::persistence::AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(auth.pool.clone()),
    );
    let sqlite = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    let configured = crate::startup_db::DB {
        pool: sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unavailable@127.0.0.1:1/unused")
            .unwrap(),
        store: crate::startup_db::DbStore::Sqlite(sqlite),
    };
    let selected = crate::application_builder_pool_from_store(configured, &canonical).await;
    let rejected = selected.is_none();
    drop(selected);
    auth.close().await;
    assert!(
        rejected,
        "a real SQLite data store must not be replaced by its dummy PG handle or an unrelated auth database"
    );
}

async fn wait_owned_database_disconnect(
    control: &sqlx::PgPool,
    database: &str,
) -> Result<(), serde_json::Value> {
    let drained = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let connected: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE datname=$1 AND backend_type IS DISTINCT FROM 'autovacuum worker')",
            )
            .bind(database)
            .fetch_one(control)
            .await
            .unwrap();
            if !connected {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await;
    if drained.is_err() {
        // Never log query text or connection credentials. These fields identify
        // a leaked client versus a PostgreSQL background backend on this exact
        // test-owned database without guessing from a timeout alone.
        let remaining: serde_json::Value = sqlx::query_scalar(
            "SELECT COALESCE(jsonb_agg(jsonb_build_object('pid',pid,'database',datname,'role',usename,'application',application_name,'backend_type',backend_type,'state',state,'wait_type',wait_event_type,'wait_event',wait_event)), '[]'::jsonb) FROM pg_stat_activity WHERE datname=$1",
        ).bind(database).fetch_one(control).await.unwrap();
        return Err(remaining);
    }
    Ok(())
}

async fn close_owned_single_connection_pool(pool: &sqlx::PgPool, expected_pid: i32) -> i32 {
    // The pinned pool can publish a returning connection after its close loop's
    // last idle-queue check. Own and close this fixture's sole known connection
    // before marking the pool closed; never terminate an arbitrary backend.
    assert_eq!(pool.options().get_max_connections(), 1);
    let mut connection = tokio::time::timeout(std::time::Duration::from_secs(3), pool.acquire())
        .await
        .unwrap()
        .unwrap();
    let actual_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *connection)
        .await
        .unwrap();
    connection.close().await.unwrap();
    pool.close().await;
    assert_eq!(
        actual_pid, expected_pid,
        "close the exact fixture-owned backend"
    );
    actual_pid
}

#[tokio::test]
async fn startup_cloned_database_relation_ids_cannot_replace_the_real_lock_namespace() {
    let base = std::env::var("OHC_BUILDER_GENERATION_TEST_DATABASE_URL").unwrap();
    let mut control_url = reqwest::Url::parse(&base).unwrap();
    assert!(matches!(
        control_url.host_str(),
        Some("localhost" | "127.0.0.1" | "[::1]")
    ));
    control_url.set_path("/postgres");
    let control = sqlx::PgPool::connect(control_url.as_str()).await.unwrap();
    let suffix = uuid::Uuid::new_v4().simple().to_string();
    let source_name = format!("ohc_authority_source_{suffix}_test");
    let clone_name = format!("ohc_authority_clone_{suffix}_test");
    sqlx::query(&format!("CREATE DATABASE {source_name} TEMPLATE template0"))
        .execute(&control)
        .await
        .unwrap();
    let mut source_url = control_url.clone();
    source_url.set_path(&format!("/{source_name}"));
    let mut source = DatabaseFixture::new_at(source_url.as_str()).await;
    source.pool.close().await;
    source.admin.close().await;
    wait_owned_database_disconnect(&control, &source_name)
        .await
        .expect("owned source clients must close before template cloning");
    sqlx::query(&format!(
        "CREATE DATABASE {clone_name} TEMPLATE {source_name}"
    ))
    .execute(&control)
    .await
    .unwrap();
    let options = (*source.pool.connect_options()).clone();
    source.pool = source.additional_data_pool().await;
    source.admin = sqlx::PgPool::connect(source_url.as_str()).await.unwrap();
    let data = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .after_connect({
            let schema = source.schema.clone();
            move |connection, _| {
                let query = format!("SET search_path TO {schema}");
                Box::pin(async move {
                    sqlx::query(&query).execute(connection).await?;
                    Ok(())
                })
            }
        })
        .connect_with(options.database(&clone_name))
        .await
        .unwrap();
    let identities = "SELECT ARRAY[to_regclass('users')::oid::bigint,to_regclass('auth_revoked_tokens')::oid::bigint,to_regclass('builder_brand_toolboxes')::oid::bigint]";
    let canonical_ids: Vec<i64> = sqlx::query_scalar(identities)
        .fetch_one(&source.pool)
        .await
        .unwrap();
    let cloned_ids: Vec<i64> = sqlx::query_scalar(identities)
        .fetch_one(&data)
        .await
        .unwrap();
    let data_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&data)
        .await
        .unwrap();
    assert_eq!(
        canonical_ids, cloned_ids,
        "fixture must preserve catalog IDs across distinct real databases"
    );
    let canonical = crate::persistence::AppDatabase::from_connection(
        sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(source.pool.clone()),
    );
    let selected = crate::application_builder_pool(data.clone(), &canonical).await;
    let rejected = selected.is_none();
    drop(selected);
    let before_close = (data.size(), data.num_idle(), data.is_closed());
    let closing_pid = close_owned_single_connection_pool(&data, data_pid).await;
    let after_close = (data.size(), data.num_idle(), data.is_closed());
    let clone_drained = wait_owned_database_disconnect(&control, &clone_name).await;
    let independent_observation = if clone_drained.is_err() {
        use sqlx::Connection;
        let mut observer = sqlx::PgConnection::connect_with(&control.connect_options())
            .await
            .unwrap();
        let current: serde_json::Value = sqlx::query_scalar("SELECT COALESCE(jsonb_agg(jsonb_build_object('pid',pid,'backend_type',backend_type,'role',usename,'state',state,'wait_event',wait_event)), '[]'::jsonb) FROM pg_stat_activity WHERE datname=$1")
            .bind(&clone_name).fetch_one(&mut observer).await.unwrap();
        observer.close().await.unwrap();
        Some(current)
    } else {
        None
    };
    assert!(
        clone_drained.is_ok(),
        "owned clone close: known_pid={data_pid}, closing_pid={closing_pid}, before={before_close:?}, after={after_close:?}, remaining={clone_drained:?}, independent={independent_observation:?}"
    );
    sqlx::query(&format!("DROP DATABASE {clone_name}"))
        .execute(&control)
        .await
        .unwrap();
    source.close().await;
    wait_owned_database_disconnect(&control, &source_name)
        .await
        .expect("owned source connections must finish closing before drop");
    sqlx::query(&format!("DROP DATABASE {source_name}"))
        .execute(&control)
        .await
        .unwrap();
    control.close().await;
    assert_eq!(
        data_pid, closing_pid,
        "the max1 fixture must close its own known backend"
    );
    assert_eq!(after_close, (0, 0, true));
    assert!(
        rejected,
        "cloned schema/catalog identity is insufficient without the same actual advisory-lock namespace"
    );
}

async fn create_owned_drain_database() -> (sqlx::PgPool, String, reqwest::Url) {
    let base = std::env::var("OHC_BUILDER_GENERATION_TEST_DATABASE_URL").unwrap();
    let mut url = reqwest::Url::parse(&base).unwrap();
    assert!(matches!(
        url.host_str(),
        Some("localhost" | "127.0.0.1" | "[::1]")
    ));
    url.set_path("/postgres");
    let control = sqlx::PgPool::connect(url.as_str()).await.unwrap();
    let name = format!("ohc_drain_{}_test", uuid::Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE DATABASE {name} TEMPLATE template0"))
        .execute(&control)
        .await
        .unwrap();
    url.set_path(&format!("/{name}"));
    (control, name, url)
}

#[tokio::test]
async fn owned_single_connection_shutdown_drains_a_concurrently_returning_real_backend() {
    let (control, name, url) = create_owned_drain_database().await;
    let armed = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let entered = Arc::new(tokio::sync::Notify::new());
    let release = Arc::new(tokio::sync::Notify::new());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .after_release({
            let armed = armed.clone();
            let entered = entered.clone();
            let release = release.clone();
            move |_, _| {
                let pause = armed.swap(false, Ordering::SeqCst);
                let entered = entered.clone();
                let release = release.clone();
                Box::pin(async move {
                    if pause {
                        entered.notify_one();
                        release.notified().await;
                    }
                    Ok(true)
                })
            }
        })
        .connect(url.as_str())
        .await
        .unwrap();
    let mut connection = pool.acquire().await.unwrap();
    let expected_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *connection)
        .await
        .unwrap();
    armed.store(true, Ordering::SeqCst);
    drop(connection);
    tokio::time::timeout(std::time::Duration::from_secs(3), entered.notified())
        .await
        .unwrap();
    // Force the actual SQLx return_to_pool/close interleaving: its release hook
    // has passed the closed check but has not published the idle connection.
    let mut shutdown = Box::pin(close_owned_single_connection_pool(&pool, expected_pid));
    std::future::poll_fn(|context| {
        assert!(std::future::Future::poll(shutdown.as_mut(), context).is_pending());
        std::task::Poll::Ready(())
    })
    .await;
    release.notify_one();
    let closed_pid = shutdown.await;
    let state = (pool.size(), pool.num_idle(), pool.is_closed());
    let still_connected: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE datname=$1 AND pid=$2)",
    )
    .bind(&name)
    .bind(expected_pid)
    .fetch_one(&control)
    .await
    .unwrap();
    // Safe fixture cleanup even when the old close routine returned too early.
    pool.close().await;
    wait_owned_database_disconnect(&control, &name)
        .await
        .unwrap();
    sqlx::query(&format!("DROP DATABASE {name}"))
        .execute(&control)
        .await
        .unwrap();
    control.close().await;
    assert_eq!(closed_pid, expected_pid);
    assert_eq!(
        state,
        (0, 0, true),
        "fixture shutdown must not leave its known client in the closed pool; connected={still_connected}"
    );
}

#[tokio::test]
async fn owned_database_drain_reports_a_real_retained_client_and_waits_for_release() {
    let (control, name, url) = create_owned_drain_database().await;
    let options = url
        .as_str()
        .parse::<sqlx::postgres::PgConnectOptions>()
        .unwrap()
        .application_name("ohc-owned-retained-client");
    let client = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    let mut held = client.acquire().await.unwrap();
    let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *held)
        .await
        .unwrap();
    let blocked = wait_owned_database_disconnect(&control, &name).await;
    drop(held);
    client.close().await;
    let released = wait_owned_database_disconnect(&control, &name).await;
    sqlx::query(&format!("DROP DATABASE {name}"))
        .execute(&control)
        .await
        .unwrap();
    control.close().await;
    let remaining = blocked.expect_err("an owned client is still connected");
    assert!(
        remaining.as_array().unwrap().iter().any(|backend| {
            backend["pid"] == pid
                && backend["backend_type"] == "client backend"
                && backend["application"] == "ohc-owned-retained-client"
        }),
        "safe diagnostics must identify the actual retained client: {remaining}"
    );
    assert!(
        released.is_ok(),
        "client release must clear the drain: {released:?}"
    );
}

#[tokio::test]
async fn owned_database_drain_does_not_misreport_real_autovacuum_as_a_leaked_client() {
    let (control, name, url) = create_owned_drain_database().await;
    let writer = sqlx::PgPool::connect(url.as_str()).await.unwrap();
    sqlx::raw_sql("CREATE TABLE drain_vacuum_probe(id bigint, payload text) WITH (autovacuum_vacuum_threshold=1,autovacuum_vacuum_scale_factor=0,autovacuum_vacuum_cost_delay=100,autovacuum_vacuum_cost_limit=1); INSERT INTO drain_vacuum_probe SELECT value, repeat('x',200) FROM generate_series(1,5000) AS value; DELETE FROM drain_vacuum_probe;")
        .execute(&writer).await.unwrap();
    sqlx::query("SELECT pg_stat_force_next_flush()")
        .execute(&writer)
        .await
        .unwrap();
    writer.close().await;
    // Default PostgreSQL schedules autovacuum approximately once a minute.
    // The local owned-cluster reproduction can shorten that startup setting;
    // this test never changes persisted or shared server configuration.
    let observed = tokio::time::timeout(std::time::Duration::from_secs(75), async {
        loop {
            let active: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE datname=$1 AND backend_type='autovacuum worker')")
                .bind(&name).fetch_one(&control).await.unwrap();
            if active { break; }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }).await;
    let drained = if observed.is_ok() {
        wait_owned_database_disconnect(&control, &name).await
    } else {
        Err(serde_json::json!({"fixture":"no actual autovacuum observed"}))
    };
    let clone_name = name.replacen("ohc_drain_", "ohc_drain_clone_", 1);
    let cloned = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        sqlx::query(&format!("CREATE DATABASE {clone_name} TEMPLATE {name}")).execute(&control),
    )
    .await;
    let clone_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname=$1)")
            .bind(&clone_name)
            .fetch_one(&control)
            .await
            .unwrap();
    if clone_exists {
        sqlx::query(&format!("DROP DATABASE {clone_name}"))
            .execute(&control)
            .await
            .unwrap();
    }
    // Ordinary PostgreSQL DROP handles its own maintenance worker on this
    // exact uniquely-created database. No pg_terminate_backend is used.
    tokio::time::timeout(
        std::time::Duration::from_secs(10),
        sqlx::query(&format!("DROP DATABASE {name}")).execute(&control),
    )
    .await
    .unwrap()
    .unwrap();
    control.close().await;
    assert!(
        observed.is_ok(),
        "real autovacuum fixture must run; no simulated backend is accepted"
    );
    assert!(
        drained.is_ok(),
        "all client pools closed; a maintenance worker is not a leaked connection: {drained:?}"
    );
    assert!(
        matches!(cloned, Ok(Ok(_))),
        "normal PostgreSQL template cloning must still succeed: {cloned:?}"
    );
}
