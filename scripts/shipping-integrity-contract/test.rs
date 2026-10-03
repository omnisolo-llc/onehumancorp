use axum::{
    Extension, Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use hmac::{Hmac, Mac};
use sea_orm::{ConnectionTrait, Schema};
use serde_json::{Value, json};
use sha2::Sha256;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{sync::Arc, time::Duration};
use tower::ServiceExt;
use uuid::Uuid;
fn claims(tenant: Option<&str>) -> server_common::Claims {
    server_common::Claims {
        sub: "synthetic-owner".into(),
        exp: i64::MAX,
        iat: 0,
        organization_id: tenant.map(str::to_owned),
        username: String::new(),
        email: String::new(),
        roles: vec!["ADMIN".into()],
        session_id: None,
        jti: "fixture-jti".into(),
    }
}
fn disconnected() -> sqlx::PgPool {
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(30))
        .connect_lazy("postgresql://synthetic@127.0.0.1:1/ohc_shipping_test")
        .unwrap()
}
async fn response(
    app: Router,
    path: &str,
    body: Option<Value>,
    headers: &[(&str, String)],
) -> (StatusCode, Value) {
    let mut request = Request::builder()
        .uri(path)
        .method(if body.is_some() { "POST" } else { "GET" })
        .header("content-type", "application/json");
    for (name, value) in headers {
        request = request.header(*name, value);
    }
    let response = app
        .oneshot(
            request
                .body(Body::from(body.map(|v| v.to_string()).unwrap_or_default()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
}
fn signature(secret: &str, body: &Value, now: i64) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(format!("{now}.{body}").as_bytes());
    format!("t={now},v1={}", hex::encode(mac.finalize().into_bytes()))
}
struct Fixture {
    pool: sqlx::PgPool,
    admin: sqlx::PgPool,
    owner: sqlx::PgPool,
    schema: String,
    role: String,
    auth: Arc<server_auth::Store>,
    token: String,
    member: String,
}
impl Fixture {
    async fn new() -> Self {
        let raw =
            std::env::var("OHC_SHIPPING_TEST_DATABASE_URL").expect("disposable test URL required");
        let url = url::Url::parse(&raw).unwrap();
        assert!(
            url.host_str()
                .unwrap()
                .parse::<std::net::IpAddr>()
                .unwrap()
                .is_loopback()
        );
        assert!(
            url.path().starts_with("/ohc_")
                && url.path().ends_with("_test")
                && url.query().is_none()
        );
        let opts: PgConnectOptions = raw.parse().unwrap();
        let admin = PgPoolOptions::new()
            .connect_with(opts.clone())
            .await
            .unwrap();
        let schema = format!("shipping_{}", Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let pool = PgPoolOptions::new()
            .connect_with(opts.clone().options([("search_path", schema.as_str())]))
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("schema.sql"))
            .execute(&pool)
            .await
            .unwrap();
        sqlx::raw_sql("INSERT INTO tenants(id,name) VALUES ('tenant-a','Synthetic A'),('tenant-b','Synthetic B'),('default','Synthetic default');INSERT INTO orders(id,tenant_id,status) VALUES ('order-a','tenant-a','paid'),('order-b','tenant-b','paid');").execute(&pool).await.unwrap();
        let orm = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone());
        let backend = sea_orm::DatabaseBackend::Postgres;
        for statement in [
            Schema::new(backend).create_table_from_entity(
                server_auth::seaorm_store::entities::identity_user_role::Entity,
            ),
            Schema::new(backend).create_table_from_entity(
                server_auth::seaorm_store::entities::revoked_token::Entity,
            ),
        ] {
            orm.execute(backend.build(&statement)).await.unwrap();
        }
        for table in ["users", "identity_user_roles", "auth_revoked_tokens"] {
            sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; ALTER TABLE {table} FORCE ROW LEVEL SECURITY; CREATE POLICY scoped ON {table} USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));")).execute(&pool).await.unwrap();
        }
        let owner = pool;
        let password = format!("synthetic-{}", Uuid::new_v4().simple());
        let role = format!("shipping_role_{}", Uuid::new_v4().simple());
        sqlx::raw_sql(&format!("CREATE ROLE {role} LOGIN PASSWORD '{password}' NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE;GRANT USAGE ON SCHEMA {schema} TO {role};GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect_with(
                opts.username(&role)
                    .password(&password)
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
            server_auth::seaorm_store::SeaOrmAuthRepository::new(
                sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone()),
            ),
        )));
        let mut tokens = Vec::new();
        for (id, role_name) in [("owner-a", "ADMIN"), ("member-a", "MEMBER")] {
            let user = server_auth::User {
                id: id.into(),
                username: id.into(),
                email: format!("{id}@example.test"),
                password_hash: "unused-fixture".into(),
                roles: vec![role_name.into()],
                active: true,
                organization_id: Some("tenant-a".into()),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                oidc_subject: None,
            };
            sqlx::query(
                "INSERT INTO users(id,username,email,tenant_id)VALUES($1,$1,$2,'tenant-a')",
            )
            .bind(id)
            .bind(&user.email)
            .execute(&owner)
            .await
            .unwrap();
            sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position)VALUES($1,$2,'tenant-a',0)").bind(id).bind(role_name).execute(&owner).await.unwrap();
            tokens.push(auth.issue_token(&user).unwrap());
        }
        Self {
            pool,
            owner,
            admin,
            schema,
            role,
            auth,
            token: tokens.remove(0),
            member: tokens.remove(0),
        }
    }
    async fn finish(self) {
        self.pool.close().await;
        self.owner.close().await;
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
    async fn task(&self, tenant: &str, order: &str, provider: &str, id: &str) -> Uuid {
        let task = Uuid::new_v4();
        sqlx::query("INSERT INTO delivery_tasks(id,organization_id,order_id,provider,provider_delivery_id,status) VALUES($1,$2,$3,$4,$5,'LABEL_CREATED')").bind(task).bind(tenant).bind(order).bind(provider).bind(id).execute(&self.owner).await.unwrap();
        task
    }
}
#[tokio::test]
async fn unconfigured_shippo_webhook_fails_closed_before_payload_processing() {
    temp_env::async_with_vars([("SHIPPO_WEBHOOK_SECRET", None::<&str>)], async {
        let (status, _) = response(
            crate::fulfillment::webhook_router(disconnected()),
            "/webhook/shippo",
            Some(json!({})),
            &[],
        )
        .await;
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    })
    .await;
}
#[tokio::test]
async fn official_shippo_signature_is_accepted_and_old_unsigned_header_rejected() {
    let event = json!({"event":"transaction_created","data":{}});
    temp_env::async_with_vars(
        [
            ("SHIPPO_WEBHOOK_SECRET", Some("synthetic-secret")),
            ("SHIPPO_TENANT_ID", Some("tenant-a")),
            ("SHIPPO_ACCOUNT_NAMESPACE", Some("synthetic-account")),
            ("SHIPPO_WEBHOOK_MODE", Some("test")),
        ],
        async {
            let sig = signature("synthetic-secret", &event, chrono::Utc::now().timestamp());
            let (status, _) = response(
                crate::fulfillment::webhook_router(disconnected()),
                "/webhook/shippo",
                Some(event.clone()),
                &[("shippo-auth-signature", sig)],
            )
            .await;
            assert_eq!(status, StatusCode::OK);
            let (status, _) = response(
                crate::fulfillment::webhook_router(disconnected()),
                "/webhook/shippo",
                Some(event),
                &[],
            )
            .await;
            assert_eq!(status, StatusCode::UNAUTHORIZED);
        },
    )
    .await;
}
#[tokio::test]
async fn unconfigured_doordash_does_not_accept_claimed_tenant() {
    temp_env::async_with_vars([("DOORDASH_WEBHOOK_AUTHORIZATION",None::<&str>)],async {let (status,_)=response(crate::fulfillment::webhook_router(disconnected()),"/webhook/doordash",Some(json!({"external_delivery_id":"order-a","delivery_status":"delivered","tenant_id":"tenant-a"})),&[("x-tenant-id","tenant-a".into())]).await;assert_eq!(status,StatusCode::SERVICE_UNAVAILABLE);}).await;
}
#[tokio::test]
async fn missing_tenant_claim_does_not_read_default_fixtures() {
    let (status, _) = response(
        crate::fulfillment::router(Arc::new(
            crate::shipping::authority::ShippingAccess::configured(
                &crate::db::DB {
                    pool: disconnected(),
                    store: crate::db::DbStore::Postgres,
                },
                Arc::new(server_auth::Store::new()),
            )
            .await,
        ))
        .layer(Extension(claims(None))),
        "/",
        None,
        &[],
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
#[tokio::test]
async fn empty_database_queue_does_not_fabricate_customer_orders() {
    let f = Fixture::new().await;
    let (status, body) = response(f.fulfillment_app().await, "/", None, &f.headers()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["to_pack"], json!([]));
    assert_eq!(body["awaiting_pickup"], json!([]));
    f.finish().await;
}
#[tokio::test]
async fn shippo_tracking_never_matches_order_id_or_changes_provider() {
    let f = Fixture::new().await;
    let id = f
        .task("tenant-a", "order-a", "doordash", "different-provider-id")
        .await;
    let update = crate::fulfillment::parse_shippo_tracking_webhook(&shippo_event(
        None,
        "order-a",
        "DELIVERED",
        1000,
    ))
    .unwrap();
    let scope = scope();
    let result =
        crate::fulfillment::storage::apply_tracking(&f.pool, &scope, &update, "synthetic-digest")
            .await;
    assert!(matches!(
        result,
        Err(crate::fulfillment::storage::Error::Conflict(_))
    ));
    let status: String = sqlx::query_scalar("SELECT status FROM delivery_tasks WHERE id=$1")
        .bind(id)
        .fetch_one(&f.owner)
        .await
        .unwrap();
    assert_eq!(status, "LABEL_CREATED");
    f.finish().await;
}
#[tokio::test]
async fn successful_label_purchase_is_not_shipping_or_order_fulfillment() {
    let f = Fixture::new().await;
    let app=Router::new().route("/transactions",axum::routing::post(||async{axum::Json(json!({"status":"SUCCESS","object_state":"VALID","object_id":"transaction_a","test":true,"label_url":"https://app.goshippo.com/labels/actual.pdf","tracking_number":"tracking_a","tracking_carrier":"usps"}))}));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    temp_env::async_with_vars([("SHIPPO_API_BASE",Some(base.as_str())),("SHIPPO_API_TOKEN",Some("shippo_test_synthetic")),("SHIPPO_TENANT_ID",Some("tenant-a")),("SHIPPO_ACCOUNT_NAMESPACE",Some("synthetic-account")),("SHIPPO_WEBHOOK_MODE",Some("test"))],async {let mut mac=Hmac::<Sha256>::new_from_slice(b"shippo_test_synthetic").unwrap();mac.update(b"tenant-a");mac.update(&[0]);mac.update(b"order-a");mac.update(&[0]);mac.update(b"rate_a");let rate=format!("rate_a.{}",hex::encode(mac.finalize().into_bytes()));let(status,_)=response(f.shipping_app().await,"/label",Some(json!({"orderId":"order-a","rateId":rate})),&f.headers()).await;assert_eq!(status,StatusCode::OK);let status:String=sqlx::query_scalar("SELECT status FROM delivery_tasks WHERE organization_id='tenant-a' AND order_id='order-a'").fetch_one(&f.owner).await.unwrap();assert_eq!(status,"LABEL_CREATED");let order:String=sqlx::query_scalar("SELECT status FROM orders WHERE id='order-a'").fetch_one(&f.owner).await.unwrap();assert_eq!(order,"paid");}).await;
    server.abort();
    f.finish().await;
}

fn scope() -> crate::fulfillment::authentication::ProviderScope {
    crate::fulfillment::authentication::ProviderScope {
        tenant_id: "tenant-a".into(),
        account_namespace: "synthetic-account".into(),
        is_test: true,
    }
}
fn shippo_event(transaction: Option<&str>, number: &str, status: &str, millis: i64) -> Value {
    json!({"event":"track_updated","test":true,"data":{"transaction":transaction,"tracking_number":number,"carrier":"usps","tracking_status":{"status":status,"status_date":chrono::DateTime::from_timestamp_millis(millis).unwrap().to_rfc3339()}}})
}
impl Fixture {
    async fn bind(
        &self,
        id: Uuid,
        scope: &crate::fulfillment::authentication::ProviderScope,
        provider: &str,
        object: &str,
        carrier: Option<&str>,
        tracking: Option<&str>,
    ) {
        sqlx::query("INSERT INTO delivery_provider_bindings(delivery_task_id,organization_id,provider,account_namespace,provider_object_id,carrier,tracking_number,is_test) VALUES($1,$2,$3,$4,$5,$6,$7,$8)")
  .bind(id).bind(&scope.tenant_id).bind(provider).bind(&scope.account_namespace).bind(object).bind(carrier).bind(tracking).bind(scope.is_test).execute(&self.owner).await.unwrap();
    }
    async fn apply(
        &self,
        scope: &crate::fulfillment::authentication::ProviderScope,
        event: Value,
    ) -> Result<crate::fulfillment::storage::Outcome, crate::fulfillment::storage::Error> {
        let update = crate::fulfillment::parse_shippo_tracking_webhook(&event).unwrap();
        crate::fulfillment::storage::apply_tracking(
            &self.pool,
            scope,
            &update,
            &hex::encode(<Sha256 as sha2::Digest>::digest(
                event.to_string().as_bytes(),
            )),
        )
        .await
    }
    async fn status(&self, id: Uuid) -> String {
        sqlx::query_scalar("SELECT status FROM delivery_tasks WHERE id=$1")
            .bind(id)
            .fetch_one(&self.owner)
            .await
            .unwrap()
    }
}
#[tokio::test]
async fn exact_bound_tracking_survives_new_router_and_updates_real_order() {
    let f = Fixture::new().await;
    let id = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    assert_eq!(
        f.apply(
            &scope(),
            shippo_event(Some("txn_a"), "tracking_a", "TRANSIT", 1000)
        )
        .await
        .unwrap(),
        crate::fulfillment::storage::Outcome::Applied
    );
    let order: String = sqlx::query_scalar("SELECT status FROM orders WHERE id='order-a'")
        .fetch_one(&f.owner)
        .await
        .unwrap();
    assert_eq!(order, "shipped");
    let (status, queue) = response(f.fulfillment_app().await, "/", None, &f.headers()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(queue["awaiting_pickup"][0]["status"], "TRANSIT");
    assert!(queue["awaiting_pickup"][0]["customer_name"].is_null());
    assert_eq!(queue["awaiting_pickup"][0]["items"], json!([]));
    assert_eq!(
        f.apply(
            &scope(),
            shippo_event(Some("txn_a"), "tracking_a", "DELIVERED", 2000)
        )
        .await
        .unwrap(),
        crate::fulfillment::storage::Outcome::Applied
    );
    let order: String = sqlx::query_scalar("SELECT status FROM orders WHERE id='order-a'")
        .fetch_one(&f.owner)
        .await
        .unwrap();
    assert_eq!(order, "fulfilled");
    f.finish().await;
}
#[tokio::test]
async fn provider_namespace_tenant_mode_transaction_and_carrier_are_all_bound() {
    let f = Fixture::new().await;
    let id = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    let mut wrong = scope();
    wrong.tenant_id = "tenant-b".into();
    assert!(
        f.apply(
            &wrong,
            shippo_event(Some("txn_a"), "tracking_a", "DELIVERED", 1000)
        )
        .await
        .is_err()
    );
    wrong = scope();
    wrong.account_namespace = "foreign-account".into();
    assert!(
        f.apply(
            &wrong,
            shippo_event(Some("txn_a"), "tracking_a", "DELIVERED", 1000)
        )
        .await
        .is_err()
    );
    wrong = scope();
    wrong.is_test = false;
    assert!(
        f.apply(
            &wrong,
            shippo_event(Some("txn_a"), "tracking_a", "DELIVERED", 1000)
        )
        .await
        .is_err()
    );
    assert!(
        f.apply(
            &scope(),
            shippo_event(Some("wrong_txn"), "tracking_a", "DELIVERED", 1000)
        )
        .await
        .is_err()
    );
    let mut event = shippo_event(Some("txn_a"), "tracking_a", "DELIVERED", 1000);
    event["data"]["carrier"] = json!("ups");
    assert!(f.apply(&scope(), event).await.is_err());
    assert_eq!(f.status(id).await, "LABEL_CREATED");
    f.finish().await;
}
#[tokio::test]
async fn missing_transaction_uses_only_unique_actual_carrier_tracking_binding() {
    let f = Fixture::new().await;
    let id = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    assert_eq!(
        f.apply(&scope(), shippo_event(None, "tracking_a", "TRANSIT", 1000))
            .await
            .unwrap(),
        crate::fulfillment::storage::Outcome::Applied
    );
    let duplicate = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    f.bind(
        duplicate,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_b",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    assert!(
        f.apply(
            &scope(),
            shippo_event(None, "tracking_a", "DELIVERED", 2000)
        )
        .await
        .is_err()
    );
    assert_eq!(f.status(id).await, "TRANSIT");
    assert_eq!(f.status(duplicate).await, "LABEL_CREATED");
    f.finish().await;
}
#[tokio::test]
async fn optional_carrier_and_tracking_are_learned_only_from_exact_authenticated_transaction() {
    let f = Fixture::new().await;
    let id = f.task("tenant-a", "order-a", "shippo", "").await;
    sqlx::query("UPDATE delivery_tasks SET provider_delivery_id=NULL WHERE id=$1")
        .bind(id)
        .execute(&f.owner)
        .await
        .unwrap();
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        None,
        None,
    )
    .await;
    assert!(
        f.apply(&scope(), shippo_event(None, "tracking_a", "TRANSIT", 1000))
            .await
            .is_err()
    );
    assert_eq!(
        f.apply(
            &scope(),
            shippo_event(Some("txn_a"), "tracking_a", "TRANSIT", 1000)
        )
        .await
        .unwrap(),
        crate::fulfillment::storage::Outcome::Applied
    );
    let pair: (Option<String>, Option<String>) = sqlx::query_as(
        "SELECT carrier,tracking_number FROM delivery_provider_bindings WHERE delivery_task_id=$1",
    )
    .bind(id)
    .fetch_one(&f.owner)
    .await
    .unwrap();
    assert_eq!(pair, (Some("usps".into()), Some("tracking_a".into())));
    f.finish().await;
}
#[tokio::test]
async fn duplicate_stale_and_terminal_events_never_regress_delivery() {
    let f = Fixture::new().await;
    let id = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    let event = shippo_event(Some("txn_a"), "tracking_a", "TRANSIT", 2000);
    assert_eq!(
        f.apply(&scope(), event.clone()).await.unwrap(),
        crate::fulfillment::storage::Outcome::Applied
    );
    assert_eq!(
        f.apply(&scope(), event).await.unwrap(),
        crate::fulfillment::storage::Outcome::Ignored
    );
    for event in [
        shippo_event(Some("txn_a"), "tracking_a", "PRE_TRANSIT", 1000),
        shippo_event(Some("txn_a"), "tracking_a", "PRE_TRANSIT", 3000),
        shippo_event(Some("txn_a"), "tracking_a", "FAILURE", 2000),
    ] {
        assert_eq!(
            f.apply(&scope(), event).await.unwrap(),
            crate::fulfillment::storage::Outcome::Ignored
        );
    }
    assert_eq!(
        f.apply(
            &scope(),
            shippo_event(Some("txn_a"), "tracking_a", "DELIVERED", 4000)
        )
        .await
        .unwrap(),
        crate::fulfillment::storage::Outcome::Applied
    );
    assert_eq!(
        f.apply(
            &scope(),
            shippo_event(Some("txn_a"), "tracking_a", "TRANSIT", 5000)
        )
        .await
        .unwrap(),
        crate::fulfillment::storage::Outcome::Ignored
    );
    assert_eq!(f.status(id).await, "DELIVERED");
    f.finish().await;
}
#[tokio::test]
async fn valid_failure_to_transit_recovery_remains_supported() {
    let f = Fixture::new().await;
    let id = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    f.apply(
        &scope(),
        shippo_event(Some("txn_a"), "tracking_a", "FAILURE", 1000),
    )
    .await
    .unwrap();
    assert_eq!(
        f.apply(
            &scope(),
            shippo_event(Some("txn_a"), "tracking_a", "TRANSIT", 2000)
        )
        .await
        .unwrap(),
        crate::fulfillment::storage::Outcome::Applied
    );
    assert_eq!(f.status(id).await, "TRANSIT");
    f.finish().await;
}
#[tokio::test]
async fn canceled_and_terminal_orders_are_never_resurrected() {
    let f = Fixture::new().await;
    let id = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    for status in ["cancelled", "canceled", "fulfilled", "returned"] {
        sqlx::query("UPDATE orders SET status=$1 WHERE id='order-a'")
            .bind(status)
            .execute(&f.owner)
            .await
            .unwrap();
        assert_eq!(
            f.apply(
                &scope(),
                shippo_event(Some("txn_a"), "tracking_a", "DELIVERED", 2000)
            )
            .await
            .unwrap(),
            crate::fulfillment::storage::Outcome::Ignored
        );
        assert_eq!(f.status(id).await, "LABEL_CREATED");
    }
    f.finish().await;
}
#[tokio::test]
async fn concurrent_duplicates_apply_once() {
    let f = Fixture::new().await;
    let id = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    let s = scope();
    let event = shippo_event(Some("txn_a"), "tracking_a", "TRANSIT", 1000);
    let (a, b) = tokio::join!(f.apply(&s, event.clone()), f.apply(&s, event));
    let applied = [a.unwrap(), b.unwrap()]
        .into_iter()
        .filter(|outcome| *outcome == crate::fulfillment::storage::Outcome::Applied)
        .count();
    assert_eq!(applied, 1);
    f.finish().await;
}
#[tokio::test]
async fn real_doordash_event_requires_bound_account_and_does_not_trust_tenant_payload() {
    let f = Fixture::new().await;
    let id = f
        .task("tenant-a", "order-a", "doordash", "delivery_a")
        .await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "doordash",
        "delivery_a",
        None,
        None,
    )
    .await;
    let update=crate::fulfillment::parse_doordash_tracking_webhook(&json!({"event_name":"DASHER_PICKED_UP","created_at":"2026-10-03T00:00:00Z","external_delivery_id":"delivery_a","dasher_id":1234,"dasher_location":{"lat":37.7,"lng":-122.4},"tenant_id":"tenant-b"})).unwrap();
    assert_eq!(
        crate::fulfillment::storage::apply_tracking(&f.pool, &scope(), &update, "dd-event")
            .await
            .unwrap(),
        crate::fulfillment::storage::Outcome::Applied
    );
    let row: (String, Option<String>) =
        sqlx::query_as("SELECT status,driver_id FROM delivery_tasks WHERE id=$1")
            .bind(id)
            .fetch_one(&f.owner)
            .await
            .unwrap();
    assert_eq!(row, ("TRANSIT".into(), Some("1234".into())));
    f.finish().await;
}
#[tokio::test]
async fn mounted_owner_routes_require_real_bearer_auth_and_aliases_are_not_bypasses() {
    let db = Arc::new(crate::db::DB {
        pool: disconnected(),
        store: crate::db::DbStore::Postgres,
    });
    let auth = Arc::new(server_auth::Store::new());
    let app = crate::actual_parent_mount(db, auth).await;
    for (path, body) in [
        ("/api/v1/fulfillment", None),
        (
            "/api/v1/fulfillment/rates",
            Some(json!({"orderId":"order-a","weight":"16","dimensions":"1x1x1"})),
        ),
        (
            "/api/v1/fulfillment/label",
            Some(json!({"orderId":"order-a","rateId":"forged"})),
        ),
        (
            "/api/v1/fulfillment/execute/order-a",
            Some(json!({"action":"hand_off"})),
        ),
    ] {
        let (status, _) = response(
            app.clone(),
            path,
            body,
            &[("x-tenant-id", "tenant-a".into())],
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED, "{path}");
    }
}
#[tokio::test]
async fn manual_handoff_is_persisted_without_claiming_delivery_or_driver_dispatch() {
    let f = Fixture::new().await;
    let id = f
        .task("tenant-a", "order-a", "doordash", "delivery_a")
        .await;
    sqlx::query("UPDATE delivery_tasks SET status='PENDING' WHERE id=$1")
        .bind(id)
        .execute(&f.owner)
        .await
        .unwrap();
    let app = f.fulfillment_app().await;
    let (status, body) = response(
        app.clone(),
        "/execute/order-a",
        Some(json!({"action":"mark_ready"})),
        &f.headers(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ReadyForPickup");
    let (status, _) = response(
        app.clone(),
        "/execute/order-a",
        Some(json!({"action":"request_driver"})),
        &f.headers(),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
    let (status, body) = response(
        app,
        "/execute/order-a",
        Some(json!({"action":"hand_off"})),
        &f.headers(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "HANDED_OFF");
    assert_eq!(f.status(id).await, "HANDED_OFF");
    let order: String = sqlx::query_scalar("SELECT status FROM orders WHERE id='order-a'")
        .fetch_one(&f.owner)
        .await
        .unwrap();
    assert_eq!(order, "paid");
    f.finish().await;
}

#[tokio::test]
async fn sqlite_forward_schema_preserves_unbound_legacy_data_and_records_real_label_atomically() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("sqlite_schema.sql"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql("INSERT INTO orders(id,tenant_id,status) VALUES('order-a','tenant-a','paid'); INSERT INTO delivery_tasks(id,organization_id,order_id,status) VALUES('old-task','tenant-a','legacy-order','SHIPPED')").execute(&pool).await.unwrap();
    crate::actual_sqlite_shipping_startup(&pool).await.unwrap();
    crate::actual_sqlite_shipping_startup(&pool).await.unwrap();
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    let old: String = sqlx::query_scalar("SELECT status FROM delivery_tasks WHERE id='old-task'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(old, "SHIPPED");
    let db = crate::db::DB {
        pool: disconnected(),
        store: crate::db::DbStore::Sqlite(pool.clone()),
    };
    let (auth, token) = sqlite_auth(&pool).await;
    let receipt=crate::integrations::shippo::client::ShippoClient::parse_label_response(&json!({"status":"SUCCESS","object_id":"actual_txn","test":true,"label_url":"https://app.goshippo.com/labels/actual.pdf"})).unwrap();
    assert!(receipt.tracking_number.is_none() && receipt.carrier.is_none());
    crate::shipping::labels::record(
        authorize_db(&db, auth.clone(), &token).await,
        &scope(),
        "order-a",
        &receipt,
    )
    .await
    .unwrap();
    let saved = crate::shipping::labels::read(
        authorize_db(&db, auth.clone(), &token).await,
        &scope(),
        "actual_txn",
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(saved.label.transaction_id, "actual_txn");
    assert_eq!(saved.fulfillment_status, "LABEL_CREATED");
    assert!(saved.label.tracking_number.is_none());
    let order: String = sqlx::query_scalar("SELECT status FROM orders WHERE id='order-a'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(order, "paid");
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    let mut other = scope();
    other.tenant_id = "tenant-b".into();
    assert!(
        crate::shipping::labels::read(
            authorize_db(&db, auth.clone(), &token).await,
            &other,
            "actual_txn"
        )
        .await
        .is_err()
    );
    assert!(
        crate::shipping::labels::record(
            authorize_db(&db, auth.clone(), &token).await,
            &scope(),
            "order-a",
            &receipt
        )
        .await
        .is_err()
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    pool.close().await;
}
#[tokio::test]
async fn failed_label_binding_rolls_back_task_change_without_fulfilling_order() {
    let f = Fixture::new().await;
    let db = crate::db::DB {
        pool: f.pool.clone(),
        store: crate::db::DbStore::Postgres,
    };
    let receipt=crate::integrations::shippo::client::ShippoClient::parse_label_response(&json!({"status":"SUCCESS","object_id":"actual_txn","test":true,"label_url":"https://app.goshippo.com/labels/actual.pdf"})).unwrap();
    sqlx::query("ALTER TABLE delivery_provider_bindings ADD CONSTRAINT synthetic_refuse_label CHECK (provider <> 'shippo')").execute(&f.owner).await.unwrap();
    assert!(
        crate::shipping::labels::record(
            authorize_db(&db, f.auth.clone(), &f.token).await,
            &scope(),
            "order-a",
            &receipt
        )
        .await
        .is_err()
    );
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_tasks")
        .fetch_one(&f.owner)
        .await
        .unwrap();
    assert_eq!(count, 0);
    let status: String = sqlx::query_scalar("SELECT status FROM orders WHERE id='order-a'")
        .fetch_one(&f.owner)
        .await
        .unwrap();
    assert_eq!(status, "paid");
    f.finish().await;
}
#[test]
fn provider_success_requires_real_receipt_fields_and_does_not_invent_optional_tracking() {
    let valid = json!({"status":"SUCCESS","object_id":"actual_txn","test":true,"label_url":"https://app.goshippo.com/labels/actual.pdf"});
    let receipt =
        crate::integrations::shippo::client::ShippoClient::parse_label_response(&valid).unwrap();
    assert!(receipt.tracking_number.is_none() && receipt.carrier.is_none());
    for (key, value) in [
        ("status", json!("ERROR")),
        ("object_state", json!("INVALID")),
        ("object_id", Value::Null),
        ("test", Value::Null),
    ] {
        let mut invalid = valid.clone();
        invalid[key] = value;
        assert!(
            crate::integrations::shippo::client::ShippoClient::parse_label_response(&invalid)
                .is_err(),
            "{key}"
        );
    }
}

#[tokio::test]
async fn postgres_bindings_enforce_rls_under_non_owner_application_role() {
    let f = Fixture::new().await;
    let a = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    let b = f.task("tenant-b", "order-b", "shippo", "tracking_b").await;
    f.bind(
        a,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    f.bind(
        b,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-b".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_b",
        Some("usps"),
        Some("tracking_b"),
    )
    .await;
    let privileged: bool = sqlx::query_scalar(
        "SELECT rolsuper OR rolbypassrls FROM pg_roles WHERE rolname=current_user",
    )
    .fetch_one(&f.pool)
    .await
    .unwrap();
    assert!(!privileged);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    for tenant in ["tenant-a", "tenant-b"] {
        let mut tx = f.pool.begin().await.unwrap();
        server_common::auth_utils::set_org_context(&mut *tx, tenant)
            .await
            .unwrap();
        let tenants: Vec<String> =
            sqlx::query_scalar("SELECT organization_id FROM delivery_provider_bindings")
                .fetch_all(&mut *tx)
                .await
                .unwrap();
        assert_eq!(tenants, vec![tenant.to_owned()]);
        tx.commit().await.unwrap();
    }
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    f.finish().await;
}
#[tokio::test]
async fn authenticated_webhook_uses_configured_tenant_despite_forged_hints_and_rejects_bad_auth() {
    let f = Fixture::new().await;
    let id = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    temp_env::async_with_vars(
        [
            ("SHIPPO_WEBHOOK_SECRET", Some("synthetic-secret")),
            ("SHIPPO_TENANT_ID", Some("tenant-a")),
            ("SHIPPO_ACCOUNT_NAMESPACE", Some("synthetic-account")),
            ("SHIPPO_WEBHOOK_MODE", Some("test")),
        ],
        async {
            let mut event = shippo_event(Some("txn_a"), "tracking_a", "TRANSIT", 1000);
            event["tenant_id"] = json!("tenant-b");
            let sig = signature("synthetic-secret", &event, chrono::Utc::now().timestamp());
            let app = crate::fulfillment::webhook_router(f.pool.clone());
            let (status, _) = response(
                app.clone(),
                "/webhook/shippo",
                Some(event.clone()),
                &[
                    ("x-tenant-id", "tenant-b".into()),
                    ("shippo-auth-signature", sig),
                ],
            )
            .await;
            assert_eq!(status, StatusCode::OK);
            assert_eq!(f.status(id).await, "TRANSIT");
            let mut event = shippo_event(Some("txn_a"), "tracking_a", "DELIVERED", 2000);
            event["test"] = json!(false);
            let sig = signature("synthetic-secret", &event, chrono::Utc::now().timestamp());
            let (status, _) = response(
                app.clone(),
                "/webhook/shippo",
                Some(event),
                &[("shippo-auth-signature", sig)],
            )
            .await;
            assert_eq!(status, StatusCode::CONFLICT);
            assert_eq!(f.status(id).await, "TRANSIT");
            let event = shippo_event(Some("txn_a"), "tracking_a", "DELIVERED", 2000);
            let stale = signature(
                "synthetic-secret",
                &event,
                chrono::Utc::now().timestamp() - 301,
            );
            let (status, _) = response(
                app.clone(),
                "/webhook/shippo",
                Some(event.clone()),
                &[("shippo-auth-signature", stale)],
            )
            .await;
            assert_eq!(status, StatusCode::UNAUTHORIZED);
            let (status, _) = response(
                app,
                "/webhook/shippo",
                Some(event),
                &[("x-shippo-signature", "not-the-official-signature".into())],
            )
            .await;
            assert_eq!(status, StatusCode::UNAUTHORIZED);
            assert_eq!(f.status(id).await, "TRANSIT");
        },
    )
    .await;
    f.finish().await;
}
#[tokio::test]
async fn doordash_http_authenticates_before_applying_real_bound_events() {
    let f = Fixture::new().await;
    let id = f
        .task("tenant-a", "order-a", "doordash", "delivery_a")
        .await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "doordash",
        "delivery_a",
        None,
        None,
    )
    .await;
    temp_env::async_with_vars([("DOORDASH_WEBHOOK_AUTHORIZATION",Some("Basic synthetic-authorized-header")),("DOORDASH_TENANT_ID",Some("tenant-a")),("DOORDASH_ACCOUNT_NAMESPACE",Some("synthetic-account")),("DOORDASH_WEBHOOK_MODE",Some("test"))],async{
 let event=json!({"event_name":"DASHER_PICKED_UP","created_at":"2026-10-03T00:00:00Z","external_delivery_id":"delivery_a","dasher_id":4321,"tenant_id":"tenant-b"});let app=crate::fulfillment::webhook_router(f.pool.clone());
 for headers in [vec![],vec![("authorization","Bearer forged".into())],vec![("authorization","Basic synthetic-authorized-header".into()),("authorization","Basic synthetic-authorized-header".into())]]{let(status,_)=response(app.clone(),"/webhook/doordash",Some(event.clone()),&headers).await;assert_eq!(status,StatusCode::UNAUTHORIZED);}
 let(status,body)=response(app,"/webhook/doordash",Some(event),&[("authorization","Basic synthetic-authorized-header".into()),("x-tenant-id","tenant-b".into())]).await;assert_eq!(status,StatusCode::OK);assert_eq!(body["applied"],true);assert_eq!(f.status(id).await,"TRANSIT");
 }).await;
    f.finish().await;
}
#[tokio::test]
async fn fresh_bearer_queue_reads_real_tenant_and_deleted_user_token_fails() {
    let f = Fixture::new().await;
    let id = f.task("tenant-a", "order-a", "shippo", "tracking_a").await;
    f.bind(
        id,
        &crate::fulfillment::authentication::ProviderScope {
            tenant_id: "tenant-a".into(),
            account_namespace: "synthetic-account".into(),
            is_test: true,
        },
        "shippo",
        "txn_a",
        Some("usps"),
        Some("tracking_a"),
    )
    .await;
    f.task("tenant-b", "order-b", "shippo", "tracking_b").await;
    let auth = f.auth.clone();
    let token = f.token.clone();
    let db = Arc::new(crate::db::DB {
        pool: f.pool.clone(),
        store: crate::db::DbStore::Postgres,
    });
    let app = crate::actual_parent_mount(db, auth.clone()).await;
    let headers = [
        ("authorization", format!("Bearer {token}")),
        ("x-tenant-id", "tenant-b".into()),
    ];
    let (status, queue) = response(app.clone(), "/api/v1/fulfillment", None, &headers).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(queue["awaiting_pickup"].as_array().unwrap().len(), 1);
    assert_eq!(queue["awaiting_pickup"][0]["id"], "order-a");
    auth.delete_user("owner-a", "tenant-a").await.unwrap();
    let (status, _) = response(app, "/api/v1/fulfillment", None, &headers).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    f.finish().await;
}

#[tokio::test]
async fn provider_webhook_is_outside_global_owner_authentication_boundary() {
    temp_env::async_with_vars(
        [
            ("SHIPPO_WEBHOOK_SECRET", Some("synthetic-secret")),
            ("SHIPPO_TENANT_ID", Some("tenant-a")),
            ("SHIPPO_ACCOUNT_NAMESPACE", Some("synthetic-account")),
            ("SHIPPO_WEBHOOK_MODE", Some("test")),
        ],
        async {
            let db = Arc::new(crate::db::DB {
                pool: disconnected(),
                store: crate::db::DbStore::Postgres,
            });
            let auth = Arc::new(server_auth::Store::new());
            let app = crate::actual_parent_mount(db, auth).await;
            let event = json!({"event":"transaction_created"});
            let sig = signature("synthetic-secret", &event, chrono::Utc::now().timestamp());
            let (status, body) = response(
                app.clone(),
                "/api/v1/fulfillment/webhook/shippo",
                Some(event.clone()),
                &[("shippo-auth-signature", sig)],
            )
            .await;
            assert_eq!(status, StatusCode::OK);
            assert_eq!(body["applied"], false);
            let (status, _) =
                response(app, "/api/v1/fulfillment/webhook/shippo", Some(event), &[]).await;
            assert_eq!(status, StatusCode::UNAUTHORIZED);
        },
    )
    .await;
}

#[tokio::test]
async fn authority_member_cannot_read_queue_or_mutate_fulfillment() {
    let f = Fixture::new().await;
    let id = f
        .task("tenant-a", "order-a", "doordash", "delivery_a")
        .await;
    sqlx::query("UPDATE delivery_tasks SET status='PENDING' WHERE id=$1")
        .bind(id)
        .execute(&f.owner)
        .await
        .unwrap();
    let app = crate::actual_parent_mount(
        Arc::new(crate::db::DB {
            pool: f.pool.clone(),
            store: crate::db::DbStore::Postgres,
        }),
        f.auth.clone(),
    )
    .await;
    let headers = [("authorization", format!("Bearer {}", f.member))];
    for (path, body) in [
        ("/api/v1/fulfillment", None),
        (
            "/api/v1/fulfillment/execute/order-a",
            Some(json!({"action":"mark_ready"})),
        ),
    ] {
        let (status, _) = response(app.clone(), path, body, &headers).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{path}");
    }
    assert_eq!(f.status(id).await, "PENDING");
    f.finish().await;
}
#[tokio::test]
async fn authority_distinct_schema_with_matching_tenant_and_order_is_rejected() {
    let canonical = Fixture::new().await;
    let data = Fixture::new().await;
    let id = data
        .task("tenant-a", "order-a", "doordash", "delivery_a")
        .await;
    let app = crate::actual_parent_mount(
        Arc::new(crate::db::DB {
            pool: data.pool.clone(),
            store: crate::db::DbStore::Postgres,
        }),
        canonical.auth.clone(),
    )
    .await;
    let headers = [("authorization", format!("Bearer {}", canonical.token))];
    let (status, _) = response(app, "/api/v1/fulfillment", None, &headers).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(data.status(id).await, "LABEL_CREATED");
    canonical.finish().await;
    data.finish().await;
}
#[tokio::test]
async fn authority_revoked_during_provider_response_never_records_local_success() {
    let f = Fixture::new().await;
    let signed = f.auth.validate_token(&f.token).await.unwrap();
    let auth = f.auth.clone();
    let app=Router::new().route("/transactions",axum::routing::post(move || {let auth=auth.clone();let signed=signed.clone();async move {
        auth.revoke_token(signed.jti,chrono::DateTime::from_timestamp(signed.exp,0).unwrap(),"tenant-a").await.unwrap();
        axum::Json(json!({"status":"SUCCESS","object_id":"transaction_revoked","test":true,"label_url":"https://app.goshippo.com/labels/actual.pdf"}))
    }}));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    temp_env::async_with_vars(
        [
            ("SHIPPO_API_BASE", Some(base.as_str())),
            ("SHIPPO_API_TOKEN", Some("shippo_test_synthetic")),
            ("SHIPPO_TENANT_ID", Some("tenant-a")),
            ("SHIPPO_ACCOUNT_NAMESPACE", Some("synthetic-account")),
            ("SHIPPO_WEBHOOK_MODE", Some("test")),
        ],
        async {
            let mut mac = Hmac::<Sha256>::new_from_slice(b"shippo_test_synthetic").unwrap();
            mac.update(b"tenant-a\0order-a\0rate_a");
            let rate = format!("rate_a.{}", hex::encode(mac.finalize().into_bytes()));
            let app = crate::actual_parent_mount(
                Arc::new(crate::db::DB {
                    pool: f.pool.clone(),
                    store: crate::db::DbStore::Postgres,
                }),
                f.auth.clone(),
            )
            .await;
            let (status, body) = response(
                app,
                "/api/v1/fulfillment/label",
                Some(json!({"orderId":"order-a","rateId":rate})),
                &[("authorization", format!("Bearer {}", f.token))],
            )
            .await;
            assert_eq!(status, StatusCode::ACCEPTED);
            assert_eq!(body["reconciliationRequired"], true);
            assert_eq!(body["transactionId"], "transaction_revoked");
            let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings")
                .fetch_one(&f.owner)
                .await
                .unwrap();
            assert_eq!(count, 0);
        },
    )
    .await;
    server.abort();
    f.finish().await;
}

#[tokio::test]
async fn authority_revoked_while_manual_write_waits_cannot_commit() {
    let f = Fixture::new().await;
    let id = f
        .task("tenant-a", "order-a", "doordash", "delivery_a")
        .await;
    sqlx::query("UPDATE delivery_tasks SET status='PENDING' WHERE id=$1")
        .bind(id)
        .execute(&f.owner)
        .await
        .unwrap();
    let mut barrier = f.owner.begin().await.unwrap();
    sqlx::query("SELECT id FROM delivery_tasks WHERE id=$1 FOR UPDATE")
        .bind(id)
        .execute(&mut *barrier)
        .await
        .unwrap();
    let app = crate::actual_parent_mount(
        Arc::new(crate::db::DB {
            pool: f.pool.clone(),
            store: crate::db::DbStore::Postgres,
        }),
        f.auth.clone(),
    )
    .await;
    let headers = [("authorization", format!("Bearer {}", f.token))];
    let pending = tokio::spawn(async move {
        response(
            app,
            "/api/v1/fulfillment/execute/order-a",
            Some(json!({"action":"mark_ready"})),
            &headers,
        )
        .await
    });
    tokio::time::timeout(Duration::from_secs(3),async {
        loop {
            let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE usename=$1 AND wait_event_type='Lock')").bind(&f.role).fetch_one(&f.admin).await.unwrap();
            if waiting {break;}
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    }).await.expect("request must reach the blocked real database write");
    let signed = f.auth.validate_token(&f.token).await.unwrap();
    f.auth
        .revoke_token(
            signed.jti,
            chrono::DateTime::from_timestamp(signed.exp, 0).unwrap(),
            "tenant-a",
        )
        .await
        .unwrap();
    barrier.commit().await.unwrap();
    let (status, _) = pending.await.unwrap();
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(f.status(id).await, "PENDING");
    f.finish().await;
}

#[tokio::test]
async fn authority_shipping_routes_and_aliases_require_current_owner_before_work() {
    let f = Fixture::new().await;
    temp_env::async_with_vars(
        [
            ("SHIPPO_API_TOKEN", Some("shippo_test_synthetic")),
            ("SHIPPO_TENANT_ID", Some("tenant-a")),
            ("SHIPPO_ACCOUNT_NAMESPACE", Some("synthetic-account")),
            ("SHIPPO_WEBHOOK_MODE", Some("test")),
        ],
        async {
            let app = crate::actual_parent_mount(
                Arc::new(crate::db::DB {
                    pool: f.pool.clone(),
                    store: crate::db::DbStore::Postgres,
                }),
                f.auth.clone(),
            )
            .await;
            let headers = [("authorization", format!("Bearer {}", f.member))];
            for prefix in ["/api/v1/shipping", "/api/v1/fulfillment"] {
                for (path, body) in [
                    (
                        format!("{prefix}/rates"),
                        Some(json!({"orderId":"order-a","weight":"invalid","dimensions":"1x1x1"})),
                    ),
                    (
                        format!("{prefix}/label"),
                        Some(json!({"orderId":"order-a","rateId":"invalid"})),
                    ),
                    (format!("{prefix}/label/transaction_a"), None),
                ] {
                    let (status, _) = response(app.clone(), &path, body, &headers).await;
                    assert_eq!(status, StatusCode::FORBIDDEN, "{path}");
                }
            }
        },
    )
    .await;
    f.finish().await;
}

async fn sqlite_auth(pool: &sqlx::SqlitePool) -> (Arc<server_auth::Store>, String) {
    let orm = sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());
    let backend = sea_orm::DatabaseBackend::Sqlite;
    for statement in [
        Schema::new(backend)
            .create_table_from_entity(server_auth::seaorm_store::entities::user::Entity),
        Schema::new(backend).create_table_from_entity(
            server_auth::seaorm_store::entities::identity_user_role::Entity,
        ),
        Schema::new(backend)
            .create_table_from_entity(server_auth::seaorm_store::entities::revoked_token::Entity),
    ] {
        orm.execute(backend.build(&statement)).await.unwrap();
    }
    let user = server_auth::User {
        id: "owner-a".into(),
        username: "owner-a".into(),
        email: "owner-a@example.test".into(),
        password_hash: "unused".into(),
        roles: vec!["ADMIN".into()],
        active: true,
        organization_id: Some("tenant-a".into()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        oidc_subject: None,
    };
    sqlx::query("INSERT INTO users(id,username,email,password_hash,active,tenant_id,created_at,updated_at)VALUES('owner-a','owner-a','owner-a@example.test','unused',1,'tenant-a',?,?)").bind(user.created_at).bind(user.updated_at).execute(pool).await.unwrap();
    sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position)VALUES('owner-a','ADMIN','tenant-a',0)").execute(pool).await.unwrap();
    let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
        server_auth::seaorm_store::SeaOrmAuthRepository::new(orm),
    )));
    let token = auth.issue_token(&user).unwrap();
    (auth, token)
}
#[tokio::test]
async fn authority_independent_sqlite_business_pool_fails_closed() {
    let canonical = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    let business = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    let (auth, token) = sqlite_auth(&canonical).await;
    sqlx::raw_sql(include_str!("sqlite_schema.sql"))
        .execute(&business)
        .await
        .unwrap();
    crate::actual_sqlite_shipping_startup(&business)
        .await
        .unwrap();
    let app = crate::actual_parent_mount(
        Arc::new(crate::db::DB {
            pool: disconnected(),
            store: crate::db::DbStore::Sqlite(business.clone()),
        }),
        auth,
    )
    .await;
    temp_env::async_with_vars(
        [
            ("SHIPPO_TENANT_ID", Some("tenant-a")),
            ("SHIPPO_ACCOUNT_NAMESPACE", Some("synthetic-account")),
            ("SHIPPO_WEBHOOK_MODE", Some("test")),
        ],
        async {
            let (status, _) = response(
                app,
                "/api/v1/shipping/label/unrecorded",
                None,
                &[("authorization", format!("Bearer {token}"))],
            )
            .await;
            assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
        },
    )
    .await;
    canonical.close().await;
    business.close().await;
}

#[tokio::test]
async fn authority_separate_postgres_pools_for_same_actual_relations_are_supported() {
    let f = Fixture::new().await;
    let separate = PgPoolOptions::new()
        .max_connections(3)
        .connect_with((*f.pool.connect_options()).clone())
        .await
        .unwrap();
    let app = crate::actual_parent_mount(
        Arc::new(crate::db::DB {
            pool: separate.clone(),
            store: crate::db::DbStore::Postgres,
        }),
        f.auth.clone(),
    )
    .await;
    let (status, body) = response(
        app,
        "/api/v1/fulfillment",
        None,
        &[("authorization", format!("Bearer {}", f.token))],
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["to_pack"], json!([]));
    separate.close().await;
    f.finish().await;
}

async fn authorize_db(
    db: &crate::db::DB,
    auth: Arc<server_auth::Store>,
    token: &str,
) -> crate::shipping::authority::AuthorizedOwner {
    let claims = auth.validate_token(token).await.unwrap();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {token}").parse().unwrap());
    crate::shipping::authority::ShippingAccess::configured(db, auth)
        .await
        .authorize(&claims, &headers)
        .await
        .unwrap()
}
impl Fixture {
    fn headers(&self) -> [(&str, String); 1] {
        [("authorization", format!("Bearer {}", self.token))]
    }
    async fn access(&self) -> Arc<crate::shipping::authority::ShippingAccess> {
        Arc::new(
            crate::shipping::authority::ShippingAccess::configured(
                &crate::db::DB {
                    pool: self.pool.clone(),
                    store: crate::db::DbStore::Postgres,
                },
                self.auth.clone(),
            )
            .await,
        )
    }
    async fn fulfillment_app(&self) -> Router {
        crate::fulfillment::router(self.access().await).route_layer(
            axum::middleware::from_fn_with_state(
                self.auth.clone(),
                server_auth::strict_bearer_auth_middleware,
            ),
        )
    }
    async fn shipping_app(&self) -> Router {
        crate::shipping::router(self.access().await).route_layer(
            axum::middleware::from_fn_with_state(
                self.auth.clone(),
                server_auth::strict_bearer_auth_middleware,
            ),
        )
    }
}

fn signed_fixture_rate() -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(b"shippo_test_synthetic").unwrap();
    mac.update(b"tenant-a\0order-a\0rate_a");
    format!("rate_a.{}", hex::encode(mac.finalize().into_bytes()))
}
async fn sqlite_mounted_purchase(revoke: bool) {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("sqlite_schema.sql"))
        .execute(&pool)
        .await
        .unwrap();
    crate::actual_sqlite_shipping_startup(&pool).await.unwrap();
    sqlx::query("INSERT INTO orders(id,tenant_id,status)VALUES('order-a','tenant-a','paid')")
        .execute(&pool)
        .await
        .unwrap();
    let (auth, token) = sqlite_auth(&pool).await;
    let signed = auth.validate_token(&token).await.unwrap();
    let provider_auth = auth.clone();
    let provider=Router::new().route("/transactions",axum::routing::post(move||{let auth=provider_auth.clone();let signed=signed.clone();async move {
        if revoke {auth.revoke_token(signed.jti,chrono::DateTime::from_timestamp(signed.exp,0).unwrap(),"tenant-a").await.unwrap();}
        axum::Json(json!({"status":"SUCCESS","object_id":"transaction_sqlite","test":true,"label_url":"https://app.goshippo.com/labels/actual.pdf"}))
    }}));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, provider).await.unwrap() });
    temp_env::async_with_vars([("SHIPPO_API_BASE",Some(base.as_str())),("SHIPPO_API_TOKEN",Some("shippo_test_synthetic")),("SHIPPO_TENANT_ID",Some("tenant-a")),("SHIPPO_ACCOUNT_NAMESPACE",Some("synthetic-account")),("SHIPPO_WEBHOOK_MODE",Some("test"))],async{
        let app=crate::actual_parent_mount(Arc::new(crate::db::DB{pool:disconnected(),store:crate::db::DbStore::Sqlite(pool.clone())}),auth.clone()).await;
        let headers=[("authorization",format!("Bearer {token}"))];
        let(status,body)=response(app.clone(),"/api/v1/shipping/label",Some(json!({"orderId":"order-a","rateId":signed_fixture_rate()})),&headers).await;
        assert_eq!(status,if revoke {StatusCode::ACCEPTED}else{StatusCode::OK});
        assert_eq!(body["transactionId"],"transaction_sqlite");
        let count:i64=sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings").fetch_one(&pool).await.unwrap();assert_eq!(count,if revoke {0}else{1});
        let order:String=sqlx::query_scalar("SELECT status FROM orders WHERE id='order-a'").fetch_one(&pool).await.unwrap();assert_eq!(order,"paid");
        if revoke {assert_eq!(body["reconciliationRequired"],true);} else {
            let(status,receipt)=response(app.clone(),"/api/v1/shipping/label/transaction_sqlite",None,&headers).await;
            assert_eq!(status,StatusCode::OK);assert_eq!(receipt["fulfillmentStatus"],"LABEL_CREATED");assert!(receipt["trackingNumber"].is_null());
            let(status,_)=response(app,"/api/v1/fulfillment",None,&headers).await;assert_eq!(status,StatusCode::SERVICE_UNAVAILABLE,"SQLite fulfillment must fail explicitly before using the disconnected PostgreSQL pool");
        }
    }).await;
    server.abort();
    pool.close().await;
}
#[tokio::test]
async fn authority_same_pool_sqlite_supports_mounted_label_purchase_and_receipt() {
    sqlite_mounted_purchase(false).await;
}
#[tokio::test]
async fn authority_sqlite_revocation_during_provider_response_requires_reconciliation() {
    sqlite_mounted_purchase(true).await;
}

#[tokio::test]
async fn authority_role_removed_during_rates_response_does_not_return_signed_rates() {
    let f = Fixture::new().await;
    let owner = f.owner.clone();
    let provider=Router::new().route("/shipments",axum::routing::post(move||{let owner=owner.clone();async move {
        sqlx::query("UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id='owner-a'").execute(&owner).await.unwrap();
        axum::Json(json!({"rates":[{"object_id":"rate_a","provider":"USPS","servicelevel":{"name":"Ground"},"amount":"4.50","estimated_days":3}]}))
    }}));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, provider).await.unwrap() });
    temp_env::async_with_vars(
        [
            ("SHIPPO_API_BASE", Some(base.as_str())),
            ("SHIPPO_API_TOKEN", Some("shippo_test_synthetic")),
            ("SHIPPO_TENANT_ID", Some("tenant-a")),
            ("SHIPPO_ACCOUNT_NAMESPACE", Some("synthetic-account")),
            ("SHIPPO_WEBHOOK_MODE", Some("test")),
            ("SHIPPO_ADDRESS_FROM_JSON", Some("{}")),
            ("SHIPPO_ADDRESS_TO_JSON", Some("{}")),
        ],
        async {
            let (status, body) = response(
                f.shipping_app().await,
                "/rates",
                Some(json!({"orderId":"order-a","weight":"16","dimensions":"1x1x1"})),
                &f.headers(),
            )
            .await;
            assert_eq!(status, StatusCode::FORBIDDEN);
            assert!(body.get("rates").is_none());
        },
    )
    .await;
    server.abort();
    f.finish().await;
}

#[tokio::test]
async fn authority_unbound_business_cannot_invoke_any_shipping_provider_work() {
    let canonical = Fixture::new().await;
    let data = Fixture::new().await;
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = calls.clone();
    let provider=Router::new().fallback(move ||{let calls=observed.clone();async move{calls.fetch_add(1,std::sync::atomic::Ordering::SeqCst);axum::Json(json!({"status":"SUCCESS","object_id":"unexpected","test":true,"label_url":"https://app.goshippo.com/labels/actual.pdf","rates":[]}))}});
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, provider).await.unwrap() });
    temp_env::async_with_vars(
        [
            ("SHIPPO_API_BASE", Some(base.as_str())),
            ("SHIPPO_API_TOKEN", Some("shippo_test_synthetic")),
            ("SHIPPO_TENANT_ID", Some("tenant-a")),
            ("SHIPPO_ACCOUNT_NAMESPACE", Some("synthetic-account")),
            ("SHIPPO_WEBHOOK_MODE", Some("test")),
            ("SHIPPO_ADDRESS_FROM_JSON", Some("{}")),
            ("SHIPPO_ADDRESS_TO_JSON", Some("{}")),
        ],
        async {
            let app = crate::actual_parent_mount(
                Arc::new(crate::db::DB {
                    pool: data.pool.clone(),
                    store: crate::db::DbStore::Postgres,
                }),
                canonical.auth.clone(),
            )
            .await;
            for (path, body) in [
                (
                    "/api/v1/shipping/rates",
                    json!({"orderId":"order-a","weight":"16","dimensions":"1x1x1"}),
                ),
                (
                    "/api/v1/fulfillment/label",
                    json!({"orderId":"order-a","rateId":signed_fixture_rate()}),
                ),
            ] {
                let (status, _) =
                    response(app.clone(), path, Some(body), &canonical.headers()).await;
                assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
            }
            assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 0);
        },
    )
    .await;
    server.abort();
    canonical.finish().await;
    data.finish().await;
}
