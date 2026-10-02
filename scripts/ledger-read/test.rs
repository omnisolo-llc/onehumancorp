use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
    routing::get,
};
use std::sync::Arc;
use tower::ServiceExt;

struct Fixture {
    admin: sqlx::PgPool,
    schema: String,
    role: String,
    scoped: sqlx::PgPool,
    store: Arc<server_auth::Store>,
}

fn user(tenant: &str) -> server_auth::User {
    server_auth::User {
        id: format!("owner-{tenant}"),
        username: format!("owner-{tenant}"),
        email: format!("{tenant}@example.test"),
        password_hash: String::new(),
        roles: vec!["owner".into()],
        active: true,
        organization_id: Some(tenant.into()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        oidc_subject: None,
    }
}

impl Fixture {
    async fn new() -> Self {
        let url =
            std::env::var("OHC_LEDGER_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
        let schema = format!("ledger_read_{}", uuid::Uuid::new_v4().simple());
        let role = format!("ledger_reader_{}", uuid::Uuid::new_v4().simple());
        let password = uuid::Uuid::new_v4().simple().to_string();
        let admin = sqlx::postgres::PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                url.parse::<sqlx::postgres::PgConnectOptions>()
                    .unwrap()
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("../../src/server/migrations/080_ledger.sql"))
            .execute(&admin)
            .await
            .unwrap();
        sqlx::raw_sql("CREATE TABLE users(id TEXT,username TEXT,email TEXT,password_hash TEXT,roles TEXT[],active BOOLEAN,tenant_id TEXT,oidc_subject TEXT,created_at TIMESTAMPTZ,updated_at TIMESTAMPTZ); CREATE TABLE revoked_tokens(jti TEXT,tenant_id TEXT,expires_at TIMESTAMPTZ);").execute(&admin).await.unwrap();
        for tenant in ["tenant-a", "tenant-b", "tenant-empty"] {
            let u = user(tenant);
            sqlx::query("INSERT INTO users VALUES($1,$2,$3,'',ARRAY['owner'],true,$4,NULL,CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)").bind(&u.id).bind(&u.username).bind(&u.email).bind(tenant).execute(&admin).await.unwrap();
        }
        for (tenant, id, amount, currency) in [
            ("tenant-a", "eur", 12.34, "EUR"),
            ("tenant-a", "jpy", 500.0, "JPY"),
            ("tenant-b", "foreign", 987.65, "USD"),
        ] {
            sqlx::query("INSERT INTO ledger_accounts(tenant_id,account_id,currency,balance)VALUES($1,$2,$3,$4)").bind(tenant).bind(id).bind(currency).bind(amount).execute(&admin).await.unwrap();
            sqlx::query("INSERT INTO ledger_transactions(tenant_id,tx_id,amount,currency)VALUES($1,$2,$3,$4)").bind(tenant).bind(id).bind(amount).bind(currency).execute(&admin).await.unwrap();
            sqlx::query("INSERT INTO ledger_entries(tenant_id,entry_id,tx_id,account_id,direction,amount)VALUES($1,$2,$2,$2,'CREDIT',$3)").bind(tenant).bind(id).bind(amount).execute(&admin).await.unwrap();
        }
        sqlx::query(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}'")).execute(&admin).await.unwrap();
        for table in ["ledger_entries", "ledger_transactions", "ledger_accounts"] {
            sqlx::query(&format!("ALTER TABLE {table} FORCE ROW LEVEL SECURITY"))
                .execute(&admin)
                .await
                .unwrap();
        }
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT ON ledger_entries,ledger_transactions,ledger_accounts TO {role};")).execute(&admin).await.unwrap();
        let scoped = db::secure_pg_pool_options()
            .max_connections(1)
            .connect_with(
                url.parse::<sqlx::postgres::PgConnectOptions>()
                    .unwrap()
                    .username(&role)
                    .password(&password)
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        let (session,current,is_super,bypass):(String,String,bool,bool) = sqlx::query_as("SELECT session_user::text,current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&scoped).await.unwrap();
        assert_eq!(session, role);
        assert_eq!(current, role);
        assert!(!is_super && !bypass);
        *db::POOL.write().unwrap() = Some(scoped.clone());
        let store = Arc::new(server_auth::Store::with_repo(Arc::new(
            server_auth::postgres_store::PgUserRepository::new(admin.clone()),
        )));
        Self {
            admin,
            schema,
            role,
            scoped,
            store,
        }
    }
    async fn read(&self, tenant: Option<&str>) -> (StatusCode, serde_json::Value) {
        let app = Router::new()
            .route("/api/v1/ledger/entries", get(get_entries))
            .route_layer(axum::middleware::from_fn_with_state(
                self.store.clone(),
                server_auth::strict_bearer_auth_middleware,
            ));
        let mut request = Request::builder()
            .uri("/api/v1/ledger/entries?tenant=tenant-b")
            .header("x-tenant-id", "tenant-b")
            .header("x-spiffe-id", "spiffe://ohc/org/tenant-b/agent/forged");
        if let Some(tenant) = tenant {
            request = request.header(
                "authorization",
                format!("Bearer {}", self.store.issue_token(&user(tenant)).unwrap()),
            );
        }
        let response = app
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
        )
    }
    async fn finish(self) {
        *db::POOL.write().unwrap() = None;
        self.scoped.close().await;
        sqlx::query(&format!("DROP SCHEMA {} CASCADE", self.schema))
            .execute(&self.admin)
            .await
            .unwrap();
        sqlx::query(&format!("DROP ROLE {}", self.role))
            .execute(&self.admin)
            .await
            .unwrap();
        self.admin.close().await;
    }
}

#[tokio::test]
async fn signed_tenant_gets_its_actual_entries_and_currency_units_under_forced_rls() {
    let f = Fixture::new().await;
    let (status, body) = f.read(Some("tenant-a")).await;
    f.finish().await;
    assert_eq!(status, StatusCode::OK);
    let entries = body["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 2);
    let eur = entries.iter().find(|x| x["id"] == "eur").unwrap();
    assert_eq!(eur["amount"], 12.34);
    assert_eq!(eur["currency"], "EUR");
    assert_eq!(eur["entry_type"], "credit");
    let jpy = entries.iter().find(|x| x["id"] == "jpy").unwrap();
    assert_eq!(jpy["amount"], 500.0);
    assert_eq!(jpy["currency"], "JPY");
    assert!(entries.iter().all(|x| x["id"] != "foreign"));
}

#[tokio::test]
async fn pooled_read_switches_tenants_and_empty_is_an_actual_empty_result() {
    let f = Fixture::new().await;
    let a = f.read(Some("tenant-a")).await;
    let b = f.read(Some("tenant-b")).await;
    let empty = f.read(Some("tenant-empty")).await;
    let again = f.read(Some("tenant-a")).await;
    let context: Option<String> =
        sqlx::query_scalar("SELECT current_setting('app.current_tenant',true)")
            .fetch_one(&f.scoped)
            .await
            .unwrap();
    f.finish().await;
    assert_eq!(a.1["entries"].as_array().unwrap().len(), 2);
    assert_eq!(b.1["entries"][0]["id"], "foreign");
    assert_eq!(empty, (StatusCode::OK, serde_json::json!({"entries":[]})));
    assert_eq!(again.1, a.1);
    assert!(context.as_deref().unwrap_or("").is_empty());
}

#[tokio::test]
async fn forged_headers_without_bearer_never_authorize_the_read() {
    let f = Fixture::new().await;
    let response = f.read(None).await;
    f.finish().await;
    assert_eq!(response.0, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn database_failure_is_unavailable_instead_of_empty_activity() {
    let f = Fixture::new().await;
    f.scoped.close().await;
    let response = f.read(Some("tenant-a")).await;
    f.finish().await;
    assert_eq!(response.0, StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn a_stored_row_that_cannot_be_decoded_is_not_silently_removed() {
    let f = Fixture::new().await;
    sqlx::query("UPDATE ledger_entries SET created_at=NULL WHERE tenant_id='tenant-a'")
        .execute(&f.admin)
        .await
        .unwrap();
    let response = f.read(Some("tenant-a")).await;
    f.finish().await;
    assert_eq!(response.0, StatusCode::SERVICE_UNAVAILABLE);
}
