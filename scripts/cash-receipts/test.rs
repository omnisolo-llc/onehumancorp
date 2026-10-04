use super::*;
use axum::{
    Extension, Router,
    body::{Body, to_bytes},
    http::Request,
    routing::{get, post},
};
use tower::ServiceExt;

struct Fixture {
    pool: sqlx::PgPool,
    admin: sqlx::PgPool,
    schema: String,
    tenant: String,
    hub: Arc<Hub>,
}
impl Fixture {
    async fn new(key: Option<&str>) -> Self {
        let url = std::env::var("OHC_CASH_TEST_DATABASE_URL").expect("owned PostgreSQL required");
        let redis_url = std::env::var("OHC_CASH_TEST_REDIS_URL").expect("owned Redis required");
        let admin = sqlx::PgPool::connect(&url).await.unwrap();
        let schema = format!("cash_receipt_{}", uuid::Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let search = schema.clone();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(8)
            .after_connect(move |c, _| {
                let query = format!("SET search_path TO {search}");
                Box::pin(async move {
                    sqlx::query(&query).execute(c).await?;
                    Ok(())
                })
            })
            .connect(&url)
            .await
            .unwrap();
        sqlx::raw_sql("CREATE TABLE tenants(id TEXT PRIMARY KEY);CREATE TABLE customers(id TEXT PRIMARY KEY,tenant_id TEXT);CREATE TABLE products(id TEXT PRIMARY KEY,tenant_id TEXT,title TEXT,price_cents BIGINT,is_subscribable BOOLEAN DEFAULT false,subscription_frequency TEXT,subscription_discount_percent INT DEFAULT 0,inventory_count INT,available_quantity INT,locked_quantity INT DEFAULT 0);
CREATE TABLE orders(id TEXT PRIMARY KEY,tenant_id TEXT,customer_id TEXT REFERENCES customers(id),total_amount NUMERIC,status TEXT);
CREATE TABLE order_items(id TEXT PRIMARY KEY,tenant_id TEXT,order_id TEXT REFERENCES orders(id),product_id TEXT REFERENCES products(id),quantity INT,price NUMERIC);
CREATE TABLE department_tasks(id TEXT PRIMARY KEY,tenant_id TEXT,department TEXT,event_type TEXT,payload JSONB,status TEXT);
CREATE TABLE ohc_universal_ledger(id TEXT PRIMARY KEY,tenant_id TEXT,department TEXT,action_type TEXT,state_change JSONB);
CREATE TABLE agent_action_requests(id TEXT PRIMARY KEY,tenant_id TEXT,action_type TEXT,status TEXT,confidence_score FLOAT,product_id TEXT,payload JSONB,source TEXT,agent_type TEXT,created_at TIMESTAMPTZ,updated_at TIMESTAMPTZ);
CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT,event_source TEXT,context_payload JSONB,proposed_action JSONB,lifecycle_state TEXT);").execute(&pool).await.unwrap();
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/210_centralized_inventory.sql"
        ))
        .execute(&pool)
        .await
        .unwrap();
        let migration = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../src/server/migrations/1033_terminal_cash_receipts.sql");
        if migration.exists() {
            sqlx::raw_sql(&std::fs::read_to_string(migration).unwrap())
                .execute(&pool)
                .await
                .unwrap();
        }
        let tenant = format!("tenant-{}", uuid::Uuid::new_v4());
        sqlx::query("INSERT INTO tenants VALUES($1),('other-tenant')")
            .bind(&tenant)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO products(id,tenant_id,title,price_cents,inventory_count,available_quantity) VALUES('product-a',$1,'Cash fixture',1999,10,10)").bind(&tenant).execute(&pool).await.unwrap();
        *POOL.write().unwrap() = Some(pool.clone());
        let hub = Arc::new(Hub {
            pool: pool.clone(),
            tracker: hub::Tracker {
                stripe_client: key
                    .map(|k| integrations::stripe::client::StripeClient::new(k.into())),
            },
            redis: redis::Client::open(redis_url).unwrap(),
            events: std::sync::Mutex::new(vec![]),
        });
        Self {
            pool,
            admin,
            schema,
            tenant,
            hub,
        }
    }
    fn app(&self, tenant: &str) -> Router {
        Router::new()
            .route("/reserve", post(reserve_inventory_handler))
            .route("/commit", post(commit_inventory_handler))
            .route("/commit/{operation_id}", get(read_cash_receipt_handler))
            .route("/checkout", post(create_checkout_session_handler))
            .layer(Extension(orchestration::AuthInfo {
                org_id: tenant.into(),
                spiffe_id: "signed-fixture".into(),
                agent_id: "fixture-owner".into(),
            }))
            .with_state(self.hub.clone())
    }
    async fn request(
        &self,
        method: &str,
        path: &str,
        body: serde_json::Value,
    ) -> (StatusCode, serde_json::Value) {
        request(self.app(&self.tenant), method, path, body).await
    }
    async fn reserve(&self) -> String {
        let (status,body)=self.request("POST","/reserve",serde_json::json!({"tenant_id":self.tenant,"product_id":"product-a","quantity":2,"ttl_seconds":300})).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["success"], true, "{body}");
        body["lock_id"].as_str().unwrap().into()
    }
    fn sale(&self, lock: &str) -> serde_json::Value {
        serde_json::json!({"tenant_id":self.tenant,"operation_id":uuid::Uuid::new_v4().to_string(),"product_id":"product-a","quantity":2,"lock_id":lock,"amount_cents":3998})
    }
    async fn stock(&self) -> (i32, i32, i32) {
        sqlx::query_as("SELECT inventory_count,available_quantity,locked_quantity FROM products WHERE id='product-a'").fetch_one(&self.pool).await.unwrap()
    }
    async fn counts(&self) -> (i64, i64) {
        sqlx::query_as("SELECT (SELECT COUNT(*) FROM orders),(SELECT COUNT(*) FROM order_items)")
            .fetch_one(&self.pool)
            .await
            .unwrap()
    }
    async fn redis_lock(&self) -> Option<String> {
        let mut conn = self
            .hub
            .redis
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        redis::cmd("GET")
            .arg(format!("ohc:lock:{}:inventory:product-a", self.tenant))
            .query_async(&mut conn)
            .await
            .unwrap()
    }
    async fn finish(self) {
        let mut conn = self
            .hub
            .redis
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        let _: i32 = redis::cmd("DEL")
            .arg(format!("ohc:lock:{}:inventory:product-a", self.tenant))
            .query_async(&mut conn)
            .await
            .unwrap();
        self.pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {} CASCADE", self.schema))
            .execute(&self.admin)
            .await
            .unwrap();
        self.admin.close().await;
    }
}
async fn request(
    app: Router,
    method: &str,
    path: &str,
    body: serde_json::Value,
) -> (StatusCode, serde_json::Value) {
    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
    )
}

#[tokio::test]
async fn missing_provider_does_not_reserve_inventory() {
    let f = Fixture::new(None).await;
    let (s, _) = f
        .request(
            "POST",
            "/checkout",
            serde_json::json!({"product_id":"product-a","quantity":2}),
        )
        .await;
    assert_eq!(s, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        f.stock().await,
        (10, 10, 0),
        "missing Stripe must not allocate stock"
    );
    assert!(f.redis_lock().await.is_none());
    f.finish().await;
}
#[tokio::test]
async fn invalid_provider_does_not_reserve_inventory() {
    let f = Fixture::new(Some("sk_test_123")).await;
    let (s, _) = f
        .request(
            "POST",
            "/checkout",
            serde_json::json!({"product_id":"product-a","quantity":2}),
        )
        .await;
    assert_eq!(s, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(f.stock().await, (10, 10, 0));
    assert!(f.redis_lock().await.is_none());
    f.finish().await;
}
#[tokio::test]
async fn sale_receipt_replays_once_and_reads_back_after_lost_response() {
    let f = Fixture::new(None).await;
    let lock = f.reserve().await;
    let sale = f.sale(&lock);
    let (s, a) = f.request("POST", "/commit", sale.clone()).await;
    assert_eq!(s, StatusCode::OK, "{a}");
    assert_eq!(a["status"], "completed", "{a}");
    let receipt = &a["receipt"];
    for field in [
        "operation_id",
        "product_id",
        "quantity",
        "amount_cents",
        "lock_id",
    ] {
        assert_eq!(receipt[field], sale[field], "{field}");
    }
    assert_eq!(receipt["tenant_id"], f.tenant);
    assert!(!receipt["order_id"].as_str().unwrap().is_empty());
    let (s, b) = f.request("POST", "/commit", sale.clone()).await;
    assert_eq!(s, StatusCode::OK, "{b}");
    assert_eq!(b["receipt"], *receipt);
    let (s, c) = f
        .request(
            "GET",
            &format!("/commit/{}", sale["operation_id"].as_str().unwrap()),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(s, StatusCode::OK, "{c}");
    assert_eq!(c["receipt"], *receipt);
    assert_eq!(f.stock().await, (8, 8, 0));
    assert_eq!(f.counts().await, (1, 1));
    assert!(f.redis_lock().await.is_none());
    {
        let events = f.hub.events.lock().unwrap();
        assert_eq!(
            events.len(),
            1,
            "publish the real completion only once after commit"
        );
        let event: serde_json::Value = serde_json::from_slice(&events[0].payload).unwrap();
        assert_eq!(event["payload"]["amount"], 39.98);
    }
    f.finish().await;
}
#[tokio::test]
async fn concurrent_replay_deducts_stock_once() {
    let f = Fixture::new(None).await;
    let lock = f.reserve().await;
    let sale = f.sale(&lock);
    let (a, b) = tokio::join!(
        f.request("POST", "/commit", sale.clone()),
        f.request("POST", "/commit", sale)
    );
    assert_eq!(a.0, StatusCode::OK, "{a:?}");
    assert_eq!(b.0, StatusCode::OK, "{b:?}");
    assert_eq!(a.1["receipt"], b.1["receipt"]);
    assert!(a.1["receipt"].is_object());
    assert_eq!(f.stock().await, (8, 8, 0));
    assert_eq!(f.counts().await, (1, 1));
    f.finish().await;
}
#[tokio::test]
async fn changed_replay_and_reused_lock_are_rejected() {
    let f = Fixture::new(None).await;
    let lock = f.reserve().await;
    let sale = f.sale(&lock);
    let (s, _) = f.request("POST", "/commit", sale.clone()).await;
    assert_eq!(s, StatusCode::OK);
    for field in [
        "amount_cents",
        "quantity",
        "product_id",
        "lock_id",
        "operation_id",
    ] {
        let mut changed = sale.clone();
        changed[field] = match field {
            "amount_cents" => 3999.into(),
            "quantity" => 1.into(),
            _ => "different".into(),
        };
        let (s, b) = f.request("POST", "/commit", changed).await;
        assert_eq!(s, StatusCode::CONFLICT, "{field}: {b}");
        assert_ne!(b["success"], true);
    }
    assert_eq!(f.stock().await, (8, 8, 0));
    assert_eq!(f.counts().await, (1, 1));
    f.finish().await;
}
async fn rejected_transaction(trigger: &str) {
    let f = Fixture::new(None).await;
    let lock = f.reserve().await;
    let sale = f.sale(&lock);
    sqlx::raw_sql(&format!("CREATE FUNCTION reject_cash() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'owned cash failure'; END $$;{trigger}")).execute(&f.pool).await.unwrap();
    let (s, b) = f.request("POST", "/commit", sale.clone()).await;
    assert!(
        !s.is_success(),
        "failed transaction must not report success: {b}"
    );
    assert_ne!(b["success"], true);
    assert_eq!(
        f.stock().await,
        (10, 8, 2),
        "stock must roll back with order"
    );
    assert_eq!(f.counts().await, (0, 0));
    assert_eq!(
        f.redis_lock().await.as_deref(),
        Some(lock.as_str()),
        "retain reservation after failed commit"
    );
    assert!(
        f.hub.events.lock().unwrap().is_empty(),
        "no completed event before commit"
    );
    let (s, b) = f
        .request(
            "GET",
            &format!("/commit/{}", sale["operation_id"].as_str().unwrap()),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(s, StatusCode::NOT_FOUND, "{b}");
    sqlx::raw_sql("DROP FUNCTION reject_cash() CASCADE")
        .execute(&f.pool)
        .await
        .unwrap();
    let (s, b) = f.request("POST", "/commit", sale).await;
    assert_eq!(s, StatusCode::OK, "{b}");
    assert_eq!(b["status"], "completed");
    assert_eq!(f.stock().await, (8, 8, 0));
    assert_eq!(f.counts().await, (1, 1));
    f.finish().await;
}
#[tokio::test]
async fn order_insert_failure_rolls_back_stock() {
    rejected_transaction("CREATE TRIGGER reject_sale BEFORE INSERT ON orders FOR EACH ROW EXECUTE FUNCTION reject_cash();").await;
}
#[tokio::test]
async fn item_insert_failure_rolls_back_stock_and_order() {
    rejected_transaction("CREATE TRIGGER reject_item BEFORE INSERT ON order_items FOR EACH ROW EXECUTE FUNCTION reject_cash();").await;
}
#[tokio::test]
async fn deferred_commit_failure_rolls_back_stock_and_order() {
    rejected_transaction("CREATE CONSTRAINT TRIGGER reject_commit AFTER INSERT ON orders DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_cash();").await;
}
#[tokio::test]
async fn price_mismatch_is_rejected_before_stock_changes() {
    let f = Fixture::new(None).await;
    let lock = f.reserve().await;
    let mut sale = f.sale(&lock);
    sale["amount_cents"] = 1.into();
    let (s, b) = f.request("POST", "/commit", sale).await;
    assert_eq!(s, StatusCode::CONFLICT, "{b}");
    assert_eq!(f.stock().await, (10, 8, 2));
    assert_eq!(f.counts().await, (0, 0));
    f.finish().await;
}
#[tokio::test]
async fn unknown_readback_and_other_tenant_do_not_claim_sale() {
    let f = Fixture::new(None).await;
    let (s, b) = f
        .request("GET", "/commit/unknown", serde_json::Value::Null)
        .await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    assert_eq!(b["status"], "not_found");
    let lock = f.reserve().await;
    let sale = f.sale(&lock);
    let (s, _) = f.request("POST", "/commit", sale.clone()).await;
    assert_eq!(s, StatusCode::OK);
    let (s, b) = request(
        f.app("other-tenant"),
        "GET",
        &format!("/commit/{}", sale["operation_id"].as_str().unwrap()),
        serde_json::Value::Null,
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND, "{b}");
    assert_ne!(b["success"], true);
    f.finish().await;
}

fn direct_sale(f: &Fixture, quantity: i32) -> serde_json::Value {
    serde_json::json!({"tenant_id":f.tenant,"operation_id":uuid::Uuid::new_v4().to_string(),"amount_cents":1999*i64::from(quantity),"items":[{"product_id":"product-a","quantity":quantity,"amount_cents":1999*i64::from(quantity)}]})
}
#[tokio::test]
async fn multi_item_cash_commits_one_order_and_replays_normalized_cart() {
    let f = Fixture::new(None).await;
    sqlx::query("INSERT INTO products(id,tenant_id,title,price_cents,inventory_count,available_quantity) VALUES('product-b',$1,'Second item',500,4,4)").bind(&f.tenant).execute(&f.pool).await.unwrap();
    let mut sale = direct_sale(&f, 2);
    sale["items"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({"product_id":"product-b","quantity":3,"amount_cents":1500}));
    sale["amount_cents"] = 5498.into();
    let (s, a) = f.request("POST", "/commit", sale.clone()).await;
    assert_eq!(s, StatusCode::OK, "{a}");
    assert_eq!(a["receipt"]["items"].as_array().unwrap().len(), 2);
    sale["items"].as_array_mut().unwrap().reverse();
    let (s, b) = f.request("POST", "/commit", sale).await;
    assert_eq!(s, StatusCode::OK, "{b}");
    assert_eq!(a["receipt"], b["receipt"]);
    assert_eq!(f.stock().await, (8, 8, 0));
    assert_eq!(f.counts().await, (1, 2));
    let second:(i32,i32,i32)=sqlx::query_as("SELECT inventory_count,available_quantity,locked_quantity FROM products WHERE id='product-b'").fetch_one(&f.pool).await.unwrap();
    assert_eq!(second, (1, 1, 0));
    let total: String = sqlx::query_scalar("SELECT total_amount::text FROM orders")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(total.parse::<f64>().unwrap(), 54.98);
    f.finish().await;
}
#[tokio::test]
async fn later_cart_item_failure_rolls_back_every_earlier_item() {
    let f = Fixture::new(None).await;
    sqlx::query("INSERT INTO products(id,tenant_id,title,price_cents,inventory_count,available_quantity) VALUES('product-b',$1,'Unavailable item',500,0,0)").bind(&f.tenant).execute(&f.pool).await.unwrap();
    let mut sale = direct_sale(&f, 2);
    sale["items"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({"product_id":"product-b","quantity":1,"amount_cents":500}));
    sale["amount_cents"] = 4498.into();
    let (s, b) = f.request("POST", "/commit", sale).await;
    assert_eq!(s, StatusCode::CONFLICT, "{b}");
    assert_eq!(b["status"], "rejected");
    assert_eq!(f.stock().await, (10, 10, 0));
    assert_eq!(f.counts().await, (0, 0));
    f.finish().await;
}
#[tokio::test]
async fn concurrent_cash_requests_compete_for_last_available_item() {
    let f = Fixture::new(None).await;
    sqlx::query("UPDATE products SET inventory_count=1,available_quantity=1")
        .execute(&f.pool)
        .await
        .unwrap();
    let (a, b) = tokio::join!(
        f.request("POST", "/commit", direct_sale(&f, 1)),
        f.request("POST", "/commit", direct_sale(&f, 1))
    );
    let statuses = [a.0, b.0];
    assert_eq!(
        statuses.iter().filter(|s| s.is_success()).count(),
        1,
        "{a:?} {b:?}"
    );
    assert!(statuses.contains(&StatusCode::CONFLICT));
    assert_eq!(f.stock().await, (0, 0, 0));
    assert_eq!(f.counts().await, (1, 1));
    f.finish().await;
}
#[tokio::test]
async fn cash_sale_cannot_consume_online_reserved_stock() {
    let f = Fixture::new(None).await;
    sqlx::query("UPDATE products SET inventory_count=2,available_quantity=2")
        .execute(&f.pool)
        .await
        .unwrap();
    let lock = f.reserve().await;
    let (s, b) = f.request("POST", "/commit", direct_sale(&f, 1)).await;
    assert_eq!(s, StatusCode::CONFLICT, "{b}");
    assert_eq!(f.stock().await, (2, 0, 2));
    assert_eq!(f.redis_lock().await.as_deref(), Some(lock.as_str()));
    assert_eq!(f.counts().await, (0, 0));
    f.finish().await;
}
#[tokio::test]
async fn centralized_empty_stock_cannot_fall_back_to_stale_legacy_counts() {
    let f = Fixture::new(None).await;
    sqlx::query("INSERT INTO inventory_levels(id,tenant_id,variant_id,location_id,available_count,committed_count) VALUES('level-a',$1,'product-a','main',0,2)").bind(&f.tenant).execute(&f.pool).await.unwrap();
    let (s, b) = f.request("POST", "/commit", direct_sale(&f, 1)).await;
    assert_eq!(s, StatusCode::CONFLICT, "{b}");
    assert_eq!(f.stock().await, (10, 10, 0));
    assert_eq!(f.counts().await, (0, 0));
    f.finish().await;
}
#[tokio::test]
async fn centralized_cash_deducts_available_without_touching_reservation() {
    let f = Fixture::new(None).await;
    sqlx::query("INSERT INTO inventory_levels(id,tenant_id,variant_id,location_id,available_count,committed_count) VALUES('level-a',$1,'product-a','main',3,2)").bind(&f.tenant).execute(&f.pool).await.unwrap();
    let (s, b) = f.request("POST", "/commit", direct_sale(&f, 2)).await;
    assert_eq!(s, StatusCode::OK, "{b}");
    let stock: (i32, i32) =
        sqlx::query_as("SELECT available_count,committed_count FROM inventory_levels")
            .fetch_one(&f.pool)
            .await
            .unwrap();
    assert_eq!(stock, (1, 2));
    assert_eq!(f.stock().await, (10, 10, 0));
    assert_eq!(f.counts().await, (1, 1));
    f.finish().await;
}
#[tokio::test]
async fn readback_is_private_uncached_and_unsigned_headers_are_rejected() {
    let f = Fixture::new(None).await;
    let response = f
        .app(&f.tenant)
        .oneshot(
            Request::builder()
                .uri("/commit/missing")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        response.headers().get("cache-control").unwrap(),
        "private, no-store"
    );
    let unsigned = Router::new()
        .route("/commit", post(commit_inventory_handler))
        .with_state(f.hub.clone());
    let response = unsigned
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/commit")
                .header("content-type", "application/json")
                .header(
                    "x-spiffe-id",
                    format!("spiffe://ohc/org/{}/agent/fake", f.tenant),
                )
                .header("x-tenant-id", &f.tenant)
                .body(Body::from(direct_sale(&f, 1).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(f.stock().await, (10, 10, 0));
    f.finish().await;
}

#[tokio::test]
async fn centralized_cash_allocates_quantity_once_across_locations() {
    let f = Fixture::new(None).await;
    sqlx::query("INSERT INTO inventory_levels(id,tenant_id,variant_id,location_id,available_count,committed_count) VALUES('level-a',$1,'product-a','first',3,2),('level-b',$1,'product-a','second',4,3)").bind(&f.tenant).execute(&f.pool).await.unwrap();
    let (s, b) = f.request("POST", "/commit", direct_sale(&f, 5)).await;
    assert_eq!(s, StatusCode::OK, "{b}");
    let counts: (i64, i64) =
        sqlx::query_as("SELECT SUM(available_count),SUM(committed_count) FROM inventory_levels")
            .fetch_one(&f.pool)
            .await
            .unwrap();
    assert_eq!(counts, (2, 5));
    let deducted: i64 =
        sqlx::query_scalar("SELECT -SUM(quantity_change) FROM inventory_transactions")
            .fetch_one(&f.pool)
            .await
            .unwrap();
    assert_eq!(deducted, 5);
    assert_eq!(f.stock().await, (10, 10, 0));
    assert_eq!(f.counts().await, (1, 1));
    f.finish().await;
}

#[tokio::test]
async fn paid_online_last_unit_commits_existing_reservation_without_lock_token() {
    let f = Fixture::new(None).await;
    sqlx::query("UPDATE products SET inventory_count=1,available_quantity=1")
        .execute(&f.pool)
        .await
        .unwrap();
    let reservation = inventory::InventoryService::new(Some(f.hub.redis.clone()))
        .reserve_inventory(&f.tenant, "product-a", 1, 300)
        .await
        .unwrap();
    assert!(reservation.success, "{reservation:?}");
    assert_eq!(f.stock().await, (1, 0, 1));
    // billing_webhook's verified paid-completion path invokes this exact shared
    // entry point with no Redis client and an empty token after checkout reserved.
    let result = inventory::InventoryService::new(None)
        .commit_inventory(&f.tenant, "product-a", 1, "")
        .await
        .unwrap();
    assert!(result.success, "{result:?}");
    assert_eq!(f.stock().await, (0, 0, 0));
    f.finish().await;
}
#[tokio::test]
async fn paid_online_centralized_last_unit_commits_existing_reservation() {
    let f = Fixture::new(None).await;
    sqlx::query("INSERT INTO inventory_levels(id,tenant_id,variant_id,location_id,available_count,committed_count) VALUES('level-a',$1,'product-a','main',1,0)").bind(&f.tenant).execute(&f.pool).await.unwrap();
    let reservation = inventory::InventoryService::new(Some(f.hub.redis.clone()))
        .reserve_inventory(&f.tenant, "product-a", 1, 300)
        .await
        .unwrap();
    assert!(reservation.success, "{reservation:?}");
    let result = inventory::InventoryService::new(None)
        .commit_inventory(&f.tenant, "product-a", 1, "")
        .await
        .unwrap();
    assert!(result.success, "{result:?}");
    let stock: (i32, i32) =
        sqlx::query_as("SELECT available_count,committed_count FROM inventory_levels")
            .fetch_one(&f.pool)
            .await
            .unwrap();
    assert_eq!(stock, (0, 0));
    f.finish().await;
}
async fn multilocation_fixture() -> Fixture {
    let f = Fixture::new(None).await;
    sqlx::query("INSERT INTO inventory_levels(id,tenant_id,variant_id,location_id,available_count,committed_count) VALUES('level-a',$1,'product-a','first',3,0),('level-b',$1,'product-a','second',4,0)").bind(&f.tenant).execute(&f.pool).await.unwrap();
    f
}
async fn central_counts(f: &Fixture) -> (i64, i64) {
    sqlx::query_as("SELECT SUM(available_count),SUM(committed_count) FROM inventory_levels")
        .fetch_one(&f.pool)
        .await
        .unwrap()
}
#[tokio::test]
async fn centralized_reserve_and_release_restore_exact_total_across_locations() {
    let f = multilocation_fixture().await;
    let service = inventory::InventoryService::new(Some(f.hub.redis.clone()));
    let held = service
        .reserve_inventory(&f.tenant, "product-a", 2, 300)
        .await
        .unwrap();
    assert!(held.success, "{held:?}");
    assert_eq!(
        central_counts(&f).await,
        (5, 2),
        "one reserve2 must never hold4"
    );
    let released = service
        .release_inventory(&f.tenant, "product-a", 2, &held.lock_id)
        .await
        .unwrap();
    assert!(released.success, "{released:?}");
    assert_eq!(central_counts(&f).await, (7, 0));
    assert!(f.redis_lock().await.is_none());
    f.finish().await;
}
#[tokio::test]
async fn centralized_reserve_spans_locations_then_commit_consumes_all_hold() {
    let f = multilocation_fixture().await;
    let service = inventory::InventoryService::new(Some(f.hub.redis.clone()));
    let held = service
        .reserve_inventory(&f.tenant, "product-a", 5, 300)
        .await
        .unwrap();
    assert!(held.success, "{held:?}");
    assert_eq!(central_counts(&f).await, (2, 5));
    let completed = service
        .commit_inventory(&f.tenant, "product-a", 5, &held.lock_id)
        .await
        .unwrap();
    assert!(completed.success, "{completed:?}");
    assert_eq!(central_counts(&f).await, (2, 0));
    assert!(f.redis_lock().await.is_none());
    f.finish().await;
}
#[tokio::test]
async fn centralized_release_never_releases_quantity_from_every_location() {
    let f = multilocation_fixture().await;
    sqlx::query("UPDATE inventory_levels SET available_count=available_count-1,committed_count=1")
        .execute(&f.pool)
        .await
        .unwrap();
    let service = inventory::InventoryService::new(Some(f.hub.redis.clone()));
    let released = service
        .release_inventory(&f.tenant, "product-a", 2, "")
        .await
        .unwrap();
    assert!(released.success, "{released:?}");
    assert_eq!(central_counts(&f).await, (7, 0));
    let levels: Vec<(i32, i32)> =
        sqlx::query_as("SELECT available_count,committed_count FROM inventory_levels ORDER BY id")
            .fetch_all(&f.pool)
            .await
            .unwrap();
    assert_eq!(levels, vec![(3, 0), (4, 0)]);
    f.finish().await;
}
#[tokio::test]
async fn replay_uses_rust_order_even_when_database_collation_orders_ids_differently() {
    let f = Fixture::new(None).await;
    sqlx::raw_sql("CREATE COLLATION cash_replay_order (provider=icu,locale='en-US');ALTER TABLE terminal_cash_receipt_items ALTER COLUMN product_id TYPE TEXT COLLATE cash_replay_order;").execute(&f.pool).await.expect("owned PostgreSQL must provide ICU collation for replay regression");
    let ids = ["Z-product", "a-product", "_product"];
    for id in ids {
        sqlx::query("INSERT INTO products(id,tenant_id,title,price_cents,inventory_count,available_quantity) VALUES($1,$2,'Collation fixture',100,3,3)").bind(id).bind(&f.tenant).execute(&f.pool).await.unwrap();
    }
    let operation = uuid::Uuid::new_v4().to_string();
    let sale = serde_json::json!({"tenant_id":f.tenant,"operation_id":operation,"amount_cents":300,"items":ids.map(|id|serde_json::json!({"product_id":id,"quantity":1,"amount_cents":100}))});
    let (status, first) = f.request("POST", "/commit", sale.clone()).await;
    assert_eq!(status, StatusCode::OK, "{first}");
    let database_order: Vec<String> = sqlx::query_scalar(
        "SELECT product_id FROM terminal_cash_receipt_items ORDER BY product_id",
    )
    .fetch_all(&f.pool)
    .await
    .unwrap();
    let mut rust_order = database_order.clone();
    rust_order.sort();
    assert_ne!(
        database_order, rust_order,
        "fixture must exercise an actual collation disagreement"
    );
    let (status, replay) = f.request("POST", "/commit", sale).await;
    assert_eq!(status, StatusCode::OK, "{replay}");
    assert_eq!(first["receipt"], replay["receipt"]);
    let (status, readback) = f
        .request(
            "GET",
            &format!("/commit/{operation}"),
            serde_json::Value::Null,
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(first["receipt"], readback["receipt"]);
    assert_eq!(f.counts().await, (1, 3));
    f.finish().await;
}
