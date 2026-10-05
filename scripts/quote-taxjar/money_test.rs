use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
    routing::post,
};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

static PROVIDER_ENVIRONMENT: Mutex<()> = Mutex::new(());

struct ProviderEnvironment {
    _guard: std::sync::MutexGuard<'static, ()>,
    previous_origin: Option<std::ffi::OsString>,
    previous_no_proxy: Option<std::ffi::OsString>,
}

impl ProviderEnvironment {
    fn lock() -> Self {
        let guard = PROVIDER_ENVIRONMENT
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        Self {
            _guard: guard,
            previous_origin: std::env::var_os("TAXJAR_API_BASE"),
            previous_no_proxy: std::env::var_os("NO_PROXY"),
        }
    }

    fn install(&self, origin: String) {
        // Every test mutating these variables holds this process-wide lock.
        unsafe {
            std::env::set_var("TAXJAR_API_BASE", origin);
            std::env::set_var("NO_PROXY", "127.0.0.1,localhost");
        }
    }
}

impl Drop for ProviderEnvironment {
    fn drop(&mut self) {
        for (key, previous) in [
            ("TAXJAR_API_BASE", self.previous_origin.take()),
            ("NO_PROXY", self.previous_no_proxy.take()),
        ] {
            unsafe {
                if let Some(value) = previous {
                    std::env::set_var(key, value);
                } else {
                    std::env::remove_var(key);
                }
            }
        }
    }
}

struct Fixture {
    admin: sqlx::PgPool,
    pool: sqlx::PgPool,
    schema: String,
    customer: uuid::Uuid,
    response: Arc<Mutex<(StatusCode, String)>>,
    requests: Arc<Mutex<Vec<String>>>,
    server: tokio::task::JoinHandle<()>,
    _environment: ProviderEnvironment,
}

impl Fixture {
    async fn new() -> Self {
        let environment = ProviderEnvironment::lock();
        let url =
            std::env::var("OHC_QUOTE_TEST_DATABASE_URL").expect("disposable PostgreSQL required");
        let admin = sqlx::PgPool::connect(&url).await.unwrap();
        let schema = format!("quote_money_{}", uuid::Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let search = schema.clone();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .after_connect(move |connection, _| {
                let command = format!("SET search_path TO {search}");
                Box::pin(async move {
                    sqlx::query(&command).execute(connection).await?;
                    Ok(())
                })
            })
            .connect(&url)
            .await
            .unwrap();
        sqlx::raw_sql("
            CREATE TABLE customers (id UUID PRIMARY KEY, tenant_id TEXT NOT NULL);
            CREATE TABLE integrations (tenant_id TEXT, provider_id TEXT, api_token TEXT);
            CREATE TABLE quotes (id UUID PRIMARY KEY, tenant_id TEXT, customer_id UUID, status TEXT, total_amount_cents BIGINT, required_deposit_cents BIGINT, stripe_payment_link TEXT, proposed_slot_id TEXT, service_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ);
            CREATE TABLE quote_line_items (id UUID PRIMARY KEY, quote_id UUID, tenant_id TEXT, description TEXT, unit_price_cents BIGINT, quantity INTEGER, is_optional BOOLEAN, service_item_id UUID, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ);
            INSERT INTO integrations VALUES ('tenant-money','taxjar','offline-public-test-token');
        ").execute(&pool).await.unwrap();
        let customer = uuid::Uuid::new_v4();
        sqlx::query("INSERT INTO customers VALUES ($1, 'tenant-money')")
            .bind(customer)
            .execute(&pool)
            .await
            .unwrap();
        let response = Arc::new(Mutex::new((
            StatusCode::OK,
            r#"{"tax":{"amount_to_collect":0.29,"rate":0.0725}}"#.to_string(),
        )));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let response_handler = response.clone();
        let request_handler = requests.clone();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        environment.install(format!("http://{}", listener.local_addr().unwrap()));
        let server = tokio::spawn(async move {
            axum::serve(
                listener,
                Router::new().route(
                    "/taxes",
                    post(move |body: String| {
                        let response = response_handler.clone();
                        let requests = request_handler.clone();
                        async move {
                            requests.lock().unwrap().push(body);
                            let (status, body) = response.lock().unwrap().clone();
                            (status, [("content-type", "application/json")], body)
                        }
                    }),
                ),
            )
            .await
            .unwrap();
        });
        Self {
            admin,
            pool,
            schema,
            customer,
            response,
            requests,
            server,
            _environment: environment,
        }
    }

    fn tax(&self, value: &str) {
        *self.response.lock().unwrap() = (
            StatusCode::OK,
            format!(r#"{{"tax":{{"amount_to_collect":{value},"rate":0.0725}}}}"#),
        );
    }

    async fn create(&self, lines: &[(i64, i32)]) -> axum::response::Response {
        let claims = server_common::Claims {
            sub: "offline-owner".into(),
            exp: i64::MAX,
            iat: 1,
            organization_id: Some("tenant-money".into()),
            username: String::new(),
            email: String::new(),
            roles: vec![],
            session_id: None,
            jti: String::new(),
        };
        let payload = serde_json::json!({
            "customer_id": self.customer.to_string(),
            "line_items": lines.iter().map(|(price, quantity)| serde_json::json!({
                "description": "Fixture item", "unit_price_cents": price,
                "quantity": quantity, "is_optional": false
            })).collect::<Vec<_>>()
        });
        Router::new()
            .route("/", post(create_quote))
            .with_state(self.pool.clone())
            .layer(axum::Extension(claims))
            .oneshot(
                Request::post("/")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap()
    }

    async fn saved(&self, response: axum::response::Response) -> (i64, Vec<i64>) {
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = to_bytes(response.into_body(), 4096).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = uuid::Uuid::parse_str(json["id"].as_str().unwrap()).unwrap();
        let total = sqlx::query_scalar("SELECT total_amount_cents FROM quotes WHERE id=$1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .unwrap();
        let taxes = sqlx::query_scalar("SELECT unit_price_cents FROM quote_line_items WHERE quote_id=$1 AND description='Automated Sales Tax (TaxJar)'").bind(id).fetch_all(&self.pool).await.unwrap();
        (total, taxes)
    }

    async fn close(self) {
        self.server.abort();
        self.pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {} CASCADE", self.schema))
            .execute(&self.admin)
            .await
            .unwrap();
        self.admin.close().await;
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        self.server.abort();
    }
}

#[tokio::test]
async fn quote_persists_every_provider_cent() {
    let fixture = Fixture::new().await;
    for (json, cents) in [
        ("0.29", 29),
        ("0.57", 57),
        ("1.13", 113),
        ("8.03", 803),
        ("2.9e-1", 29),
    ] {
        fixture.tax(json);
        assert_eq!(
            fixture
                .saved(fixture.create(&[(29, 3), (57, 2)]).await)
                .await,
            (201 + cents, vec![cents]),
            "TaxJar amount {json}"
        );
    }
    fixture.close().await;
}

#[tokio::test]
async fn quote_request_preserves_large_integer_cents() {
    let fixture = Fixture::new().await;
    fixture.tax("0");
    assert_eq!(
        fixture.saved(fixture.create(&[(i64::MAX, 1)]).await).await,
        (i64::MAX, vec![])
    );
    #[derive(serde::Deserialize)]
    struct Amount {
        amount: Box<serde_json::value::RawValue>,
        shipping: Box<serde_json::value::RawValue>,
    }
    let requests = fixture.requests.lock().unwrap().clone();
    let sent: Amount = serde_json::from_str(&requests[0]).unwrap();
    assert_eq!(sent.amount.get(), "92233720368547758.07");
    assert_eq!(sent.shipping.get(), "0.00");
    fixture.close().await;
}

#[tokio::test]
async fn quote_invalid_tax_never_becomes_a_tax_line() {
    let fixture = Fixture::new().await;
    for json in [
        "-0.29",
        "0.291",
        "92233720368547758.08",
        "null",
        "\"0.29\"",
        "0.29000000000000000000000000001",
        "2.9000000000000000000000000001e-1",
    ] {
        fixture.tax(json);
        assert_eq!(
            fixture.saved(fixture.create(&[(100, 1)]).await).await,
            (100, vec![]),
            "invalid TaxJar amount {json}"
        );
    }
    *fixture.response.lock().unwrap() = (
        StatusCode::SERVICE_UNAVAILABLE,
        r#"{"error":"offline fixture failure"}"#.into(),
    );
    assert_eq!(
        fixture.saved(fixture.create(&[(100, 1)]).await).await,
        (100, vec![])
    );
    fixture.close().await;
}

#[tokio::test]
async fn quote_rejects_overflow_before_provider_or_persistence() {
    let fixture = Fixture::new().await;
    for lines in [
        vec![(i64::MAX, 2)],
        vec![(i64::MAX, 1), (1, 1)],
        vec![(-1, 1)],
        vec![(100, -1)],
    ] {
        assert_eq!(
            fixture.create(&lines).await.status(),
            StatusCode::BAD_REQUEST
        );
    }
    assert!(fixture.requests.lock().unwrap().is_empty());
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM quotes")
        .fetch_one(&fixture.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    fixture.close().await;
}

#[tokio::test]
async fn quote_rejects_overflow_after_adding_valid_tax() {
    let fixture = Fixture::new().await;
    fixture.tax("0.01");
    assert_eq!(
        fixture.create(&[(i64::MAX, 1)]).await.status(),
        StatusCode::BAD_REQUEST
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM quotes")
        .fetch_one(&fixture.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    fixture.close().await;
}

#[tokio::test]
async fn quote_preserves_maximum_tax_from_original_http_json() {
    let fixture = Fixture::new().await;
    fixture.tax("92233720368547758.07");
    assert_eq!(
        fixture.saved(fixture.create(&[(0, 1)]).await).await,
        (i64::MAX, vec![i64::MAX])
    );
    fixture.close().await;
}
