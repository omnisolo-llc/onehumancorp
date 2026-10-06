use super::*;

#[test]
fn shared_pool_deduction_is_distributed_once_and_preserves_committed_stock() {
    let p = json!({"inventory_count":8,"locked_quantity":2});
    let levels = vec![
        Level {
            id: "a".into(),
            available_count: 2,
            committed_count: 1,
        },
        Level {
            id: "b".into(),
            available_count: 4,
            committed_count: 1,
        },
    ];
    let result = plan(&p, &levels, -3).unwrap();
    assert_eq!(result.stock, 3);
    assert_eq!(result.total, 5);
    assert_eq!(
        result.level_changes,
        vec![("a".into(), -2), ("b".into(), -1)]
    );
    let added = plan(&p, &levels, 3).unwrap();
    assert_eq!(added.stock, 9);
    assert_eq!(added.level_changes, vec![("a".into(), 3)]);
}
#[test]
fn adjustments_reject_shortage_overflow_and_inconsistent_counters() {
    assert!(plan(&json!({"inventory_count":3,"locked_quantity":2}), &[], -2).is_err());
    assert!(plan(&json!({"inventory_count":2147483647}), &[], 1).is_err());
    assert!(
        plan(
            &json!({"inventory_count":3,"pn_counter_p":10,"pn_counter_n":1}),
            &[],
            1
        )
        .is_err()
    );
    let result = plan(
        &json!({"inventory_count":3,"locked_quantity":1,"pn_counter_p":5,"pn_counter_n":2}),
        &[],
        -1,
    )
    .unwrap();
    assert_eq!(result.stock, 2);
    assert_eq!(result.available, 1);
    assert_eq!(result.counters, Some((5, 3)));
}
#[test]
fn mutation_identity_requires_a_version_and_checked_integer_quantity() {
    assert!(serde_json::from_value::<Mutation>(json!({"id":"m","payload":{"item_id":"p","quantity_change":2147483648_i64,"expected_version":"a".repeat(64)}})).is_err());
    assert!(
        serde_json::from_value::<Mutation>(
            json!({"id":"m","payload":{"item_id":"p","quantity_change":1}})
        )
        .is_err()
    );
    let mutation = Mutation {
        id: "m".into(),
        payload: Adjustment {
            item_id: "p".into(),
            quantity_change: 1,
            expected_version: "a".repeat(64),
            location_id: None,
            is_sold_out: None,
        },
    };
    assert!(mutation.valid());
    let mut bad = mutation;
    bad.id.clear();
    assert!(!bad.valid());
}

#[test]
fn rounded_mysql_timestamps_cannot_recreate_a_prior_adjustment_version() {
    let before = mysql_state(3, Some("2026-10-06 15:00:00".into()), 0);
    let after_cycle = mysql_state(3, Some("2026-10-06 15:00:00".into()), 2);
    assert_ne!(
        version("a", "product", &before, &[]),
        version("a", "product", &after_cycle, &[])
    );
}

#[test]
fn pn_oversell_debt_is_blocked_without_creating_stock_or_resetting_counters() {
    let debt = json!({"inventory_count":0,"available_quantity":0,"locked_quantity":0,"pn_counter_p":5,"pn_counter_n":8});
    for delta in [1, 3, 4] {
        assert!(matches!(
            plan(&debt, &[], delta),
            Err(Error::Blocked("inventory_debt_requires_reconciliation"))
        ));
    }
    assert_eq!(debt["pn_counter_p"], 5);
    assert_eq!(debt["pn_counter_n"], 8);
    let balanced = plan(
        &json!({"inventory_count":0,"pn_counter_p":5,"pn_counter_n":5}),
        &[],
        1,
    )
    .unwrap();
    assert_eq!(balanced.stock, 1);
    assert_eq!(balanced.counters, Some((6, 5)));
}

struct Fixture {
    pool: PgPool,
    admin: PgPool,
    schema: String,
    role: String,
    password: String,
    url: String,
}
impl Fixture {
    async fn open() -> Self {
        let url = std::env::var("OHC_INVENTORY_TEST_DATABASE_URL")
            .expect("Explicit isolated inventory test database required");
        let parsed = reqwest::Url::parse(&url).unwrap();
        assert!(matches!(parsed.scheme(), "postgres" | "postgresql"));
        assert!(
            parsed
                .host_str()
                .unwrap()
                .parse::<std::net::IpAddr>()
                .unwrap()
                .is_loopback()
        );
        assert!(parsed.path().starts_with("/ohc_") && parsed.path().ends_with("_test"));
        assert!(parsed.query().is_none() && parsed.fragment().is_none());
        let base = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&url)
            .await
            .unwrap();
        let schema = format!("inventory_{}", uuid::Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&base)
            .await
            .unwrap();
        let selected = schema.clone();
        let admin = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .after_connect(move |connection, _| {
                let selected = selected.clone();
                Box::pin(async move {
                    sqlx::query(&format!("SET search_path TO {selected}"))
                        .execute(connection)
                        .await?;
                    Ok(())
                })
            })
            .connect(&url)
            .await
            .unwrap();
        base.close().await;
        sqlx::raw_sql("CREATE TABLE products(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,title TEXT,description TEXT,price_cents BIGINT,currency TEXT,inventory_count INT NOT NULL,available_quantity INT NOT NULL,locked_quantity INT NOT NULL DEFAULT 0,is_sold_out BOOLEAN NOT NULL DEFAULT FALSE,updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP);
            CREATE TABLE inventory_levels(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,variant_id TEXT NOT NULL REFERENCES products(id),available_count INT NOT NULL,committed_count INT NOT NULL DEFAULT 0,updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP);
            CREATE TABLE inventory_transactions(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,inventory_level_id TEXT REFERENCES inventory_levels(id),type TEXT,quantity_change INT);
            CREATE TABLE applied_client_mutations(client_mutation_id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL);
            INSERT INTO products(id,tenant_id,title,price_cents,currency,inventory_count,available_quantity) VALUES('owned','a','Actual product',1200,'USD',6,6),('foreign','b','Private product',400,'USD',8,8);")
            .execute(&admin).await.unwrap();
        sqlx::raw_sql(include_str!(
            "../migrations/1041_inventory_adjustment_receipts.sql"
        ))
        .execute(&admin)
        .await
        .unwrap();
        for table in [
            "products",
            "inventory_levels",
            "inventory_transactions",
            "applied_client_mutations",
        ] {
            sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; ALTER TABLE {table} FORCE ROW LEVEL SECURITY; CREATE POLICY tenant_inventory ON {table} USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));")).execute(&admin).await.unwrap();
        }
        sqlx::query("ALTER TABLE inventory_adjustment_receipts FORCE ROW LEVEL SECURITY")
            .execute(&admin)
            .await
            .unwrap();
        let role = format!("inventory_login_{}", uuid::Uuid::new_v4().simple());
        let password = uuid::Uuid::new_v4().simple().to_string();
        sqlx::query(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE NOINHERIT PASSWORD '{password}'")).execute(&admin).await.unwrap();
        sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT,INSERT,UPDATE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
        let options = url
            .parse::<sqlx::postgres::PgConnectOptions>()
            .unwrap()
            .username(&role)
            .password(&password)
            .options([("search_path", schema.as_str())]);
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(8)
            .connect_with(options)
            .await
            .unwrap();
        let verified:(String,bool,bool)=sqlx::query_as("SELECT current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&pool).await.unwrap();
        assert_eq!(verified, (role.clone(), false, false));
        Self {
            pool,
            admin,
            schema,
            role,
            password,
            url,
        }
    }
    async fn request(&self, id: &str, delta: i32) -> Mutation {
        let rows = read_postgres(&self.pool, "a").await.unwrap();
        Mutation {
            id: id.into(),
            payload: Adjustment {
                item_id: "owned".into(),
                quantity_change: delta,
                expected_version: rows[0]["inventory_version"].as_str().unwrap().into(),
                location_id: None,
                is_sold_out: None,
            },
        }
    }
    async fn counts(&self) -> (i64, i64, i64) {
        let products = sqlx::query_scalar("SELECT count(*) FROM products")
            .fetch_one(&self.admin)
            .await
            .unwrap();
        let receipts = sqlx::query_scalar("SELECT count(*) FROM inventory_adjustment_receipts")
            .fetch_one(&self.admin)
            .await
            .unwrap();
        let markers = sqlx::query_scalar("SELECT count(*) FROM applied_client_mutations")
            .fetch_one(&self.admin)
            .await
            .unwrap();
        (products, receipts, markers)
    }
    async fn close(self) {
        self.pool.close().await;
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
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_missing_and_foreign_products_never_create_stock_or_receipts() {
    let f = Fixture::open().await;
    for product in ["missing", "foreign", "e2e-product-cake"] {
        let mut request = f.request("unknown", 1).await;
        request.payload.item_id = product.into();
        assert!(matches!(
            apply_postgres(&f.pool, "a", &request).await,
            Err(Error::Blocked("product_not_found"))
        ));
    }
    assert_eq!(f.counts().await, (2, 0, 0));
    f.close().await;
}
#[tokio::test]
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_duplicate_replay_survives_new_connection_and_changed_identity_is_blocked() {
    let f = Fixture::open().await;
    let request = f.request("same", 2).await;
    let one = apply_postgres(&f.pool, "a", &request).await.unwrap();
    f.pool.close().await;
    let options = f
        .url
        .parse::<sqlx::postgres::PgConnectOptions>()
        .unwrap()
        .username(&f.role)
        .password(&f.password)
        .options([("search_path", f.schema.as_str())]);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_with(options)
        .await
        .unwrap();
    let two = apply_postgres(&pool, "a", &request).await.unwrap();
    assert_eq!(
        serde_json::to_value(one).unwrap(),
        serde_json::to_value(two).unwrap()
    );
    let mut changed = request;
    changed.payload.quantity_change = 3;
    assert!(matches!(
        apply_postgres(&pool, "a", &changed).await,
        Err(Error::Unconfirmed("request_identity_changed"))
    ));
    assert_eq!(read_postgres(&pool, "a").await.unwrap()[0]["stock"], 8);
    pool.close().await;
    f.close().await;
}
#[tokio::test]
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_concurrent_same_request_changes_stock_once() {
    let f = Fixture::open().await;
    let request = f.request("duplicate", 1).await;
    let (one, two) = tokio::join!(
        apply_postgres(&f.pool, "a", &request),
        apply_postgres(&f.pool, "a", &request)
    );
    assert_eq!(one.unwrap().stock, 7);
    assert_eq!(two.unwrap().stock, 7);
    assert_eq!(f.counts().await, (2, 1, 1));
    f.close().await;
}
#[tokio::test]
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_concurrent_different_requests_with_same_version_cannot_both_apply() {
    let f = Fixture::open().await;
    let one = f.request("one", 1).await;
    let mut two = one.clone();
    two.id = "two".into();
    let (one, two) = tokio::join!(
        apply_postgres(&f.pool, "a", &one),
        apply_postgres(&f.pool, "a", &two)
    );
    assert_ne!(one.is_ok(), two.is_ok());
    assert_eq!(read_postgres(&f.pool, "a").await.unwrap()[0]["stock"], 7);
    assert_eq!(f.counts().await, (2, 1, 1));
    f.close().await;
}
#[tokio::test]
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_shared_pool_shortage_and_success_preserve_location_totals() {
    let f = Fixture::open().await;
    sqlx::query("INSERT INTO inventory_levels(id,tenant_id,variant_id,available_count,committed_count)VALUES('a','a','owned',2,1),('b','a','owned',4,1)").execute(&f.admin).await.unwrap();
    let too_much = f.request("shortage", -7).await;
    assert!(matches!(
        apply_postgres(&f.pool, "a", &too_much).await,
        Err(Error::Blocked("insufficient_or_overflowing_stock"))
    ));
    assert_eq!(f.counts().await, (2, 0, 0));
    let request = f.request("deduct", -3).await;
    assert_eq!(
        apply_postgres(&f.pool, "a", &request).await.unwrap().stock,
        3
    );
    let levels: Vec<(i32, i32)> =
        sqlx::query_as("SELECT available_count,committed_count FROM inventory_levels ORDER BY id")
            .fetch_all(&f.admin)
            .await
            .unwrap();
    assert_eq!(levels, vec![(0, 1), (3, 1)]);
    let sum: i64 =
        sqlx::query_scalar("SELECT SUM(quantity_change)::bigint FROM inventory_transactions")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    assert_eq!(sum, -3);
    f.close().await;
}
#[tokio::test]
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_receipt_failure_rolls_back_stock_marker_and_ledger() {
    let f = Fixture::open().await;
    sqlx::raw_sql("CREATE FUNCTION deny_receipt() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'owned fixture rejection'; END; $$;CREATE TRIGGER deny_receipt BEFORE INSERT ON inventory_adjustment_receipts FOR EACH ROW EXECUTE FUNCTION deny_receipt();").execute(&f.admin).await.unwrap();
    let request = f.request("failed", 1).await;
    assert!(matches!(
        apply_postgres(&f.pool, "a", &request).await,
        Err(Error::Database(_))
    ));
    assert_eq!(read_postgres(&f.pool, "a").await.unwrap()[0]["stock"], 6);
    assert_eq!(f.counts().await, (2, 0, 0));
    f.close().await;
}
#[tokio::test]
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_legacy_marker_is_not_upgraded_into_a_new_effect() {
    let f = Fixture::open().await;
    sqlx::query("INSERT INTO applied_client_mutations VALUES('legacy','a')")
        .execute(&f.admin)
        .await
        .unwrap();
    let request = f.request("legacy", 1).await;
    assert!(matches!(
        apply_postgres(&f.pool, "a", &request).await,
        Err(Error::Unconfirmed("legacy_adjustment_requires_review"))
    ));
    assert_eq!(read_postgres(&f.pool, "a").await.unwrap()[0]["stock"], 6);
    assert_eq!(f.counts().await, (2, 0, 1));
    f.close().await;
}
#[tokio::test]
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_zero_stock_and_empty_tenant_are_real_and_not_fixtures() {
    let f = Fixture::open().await;
    assert!(read_postgres(&f.pool, "empty").await.unwrap().is_empty());
    let request = f.request("zero", -6).await;
    assert_eq!(
        apply_postgres(&f.pool, "a", &request).await.unwrap().stock,
        0
    );
    assert_eq!(read_postgres(&f.pool, "a").await.unwrap()[0]["stock"], 0);
    assert_eq!(f.counts().await, (2, 1, 1));
    f.close().await;
}

#[tokio::test]
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_receipt_lookup_and_replay_survive_later_product_deletion() {
    let f = Fixture::open().await;
    assert!(
        receipt_postgres(&f.pool, "a", "not-recorded")
            .await
            .unwrap()
            .is_none()
    );
    assert_eq!(f.counts().await, (2, 0, 0));
    let request = f.request("committed-before-deletion", 1).await;
    let original = apply_postgres(&f.pool, "a", &request).await.unwrap();
    sqlx::query("DELETE FROM products WHERE id='owned'")
        .execute(&f.admin)
        .await
        .unwrap();
    let found = receipt_postgres(&f.pool, "a", &request.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        serde_json::to_value(&original).unwrap(),
        serde_json::to_value(found).unwrap()
    );
    let replayed = apply_postgres(&f.pool, "a", &request).await.unwrap();
    assert_eq!(
        serde_json::to_value(&original).unwrap(),
        serde_json::to_value(replayed).unwrap()
    );
    assert!(
        receipt_postgres(&f.pool, "b", &request.id)
            .await
            .unwrap()
            .is_none()
    );
    let mut changed = request;
    changed.payload.quantity_change = 2;
    assert!(matches!(
        apply_postgres(&f.pool, "a", &changed).await,
        Err(Error::Unconfirmed("request_identity_changed"))
    ));
    assert_eq!(f.counts().await, (1, 1, 1));
    f.close().await;
}

#[tokio::test]
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_pn_debt_does_not_invent_stock_or_clear_pending_shortage() {
    let f = Fixture::open().await;
    sqlx::raw_sql("ALTER TABLE products ADD COLUMN pn_counter_p BIGINT;ALTER TABLE products ADD COLUMN pn_counter_n BIGINT;UPDATE products SET inventory_count=0,available_quantity=0,pn_counter_p=5,pn_counter_n=8 WHERE id='owned';
        CREATE TABLE department_tasks(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,event_type TEXT,payload JSONB,status TEXT);
        ALTER TABLE department_tasks ENABLE ROW LEVEL SECURITY;ALTER TABLE department_tasks FORCE ROW LEVEL SECURITY;CREATE POLICY shortage_tenant ON department_tasks USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));
        INSERT INTO department_tasks VALUES('shortage','a','inventory.sync.conflict',jsonb_build_object('shortage',3),'PENDING');")
        .execute(&f.admin).await.unwrap();
    for delta in [1, 4] {
        let request = f.request(&format!("debt-{delta}"), delta).await;
        assert!(matches!(
            apply_postgres(&f.pool, "a", &request).await,
            Err(Error::Blocked("inventory_debt_requires_reconciliation"))
        ));
    }
    let actual: (i32,i32,i64,i64) = sqlx::query_as("SELECT inventory_count,available_quantity,pn_counter_p,pn_counter_n FROM products WHERE id='owned'").fetch_one(&f.admin).await.unwrap();
    assert_eq!(actual, (0, 0, 5, 8));
    let shortage: (String, Value) =
        sqlx::query_as("SELECT status,payload FROM department_tasks WHERE id='shortage'")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    assert_eq!(shortage, ("PENDING".into(), json!({"shortage":3})));
    assert_eq!(f.counts().await, (2, 0, 0));
    f.close().await;
}

async fn wait_for_inventory_blocker(admin: &PgPool, waiter: i32, blocker: i32) {
    tokio::time::timeout(std::time::Duration::from_secs(15), async {
        loop {
            let blockers: Vec<i32> = sqlx::query_scalar("SELECT pg_blocking_pids($1)")
                .bind(waiter)
                .fetch_one(admin)
                .await
                .unwrap();
            if blockers.contains(&blocker) {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("Expected PostgreSQL lock queue barrier was not observed");
}

#[tokio::test]
#[ignore = "requires OHC_INVENTORY_TEST_DATABASE_URL isolated PostgreSQL"]
async fn postgres_queued_delete_ahead_of_retry_still_returns_original_committed_receipt() {
    let f = Fixture::open().await;
    let lock_key =
        i64::from_be_bytes(uuid::Uuid::new_v4().as_bytes()[..8].try_into().unwrap()) & i64::MAX;
    sqlx::raw_sql(&format!("CREATE FUNCTION pause_inventory_receipt() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN PERFORM pg_advisory_xact_lock({lock_key}); RETURN NEW; END; $$;CREATE TRIGGER pause_inventory_receipt BEFORE INSERT ON inventory_adjustment_receipts FOR EACH ROW EXECUTE FUNCTION pause_inventory_receipt();"))
        .execute(&f.admin).await.unwrap();
    let options = f
        .url
        .parse::<sqlx::postgres::PgConnectOptions>()
        .unwrap()
        .username(&f.role)
        .password(&f.password)
        .options([("search_path", f.schema.as_str())]);
    let original_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.clone())
        .await
        .unwrap();
    let retry_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    let original_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&original_pool)
        .await
        .unwrap();
    let retry_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&retry_pool)
        .await
        .unwrap();
    let mut barrier = f.admin.begin().await.unwrap();
    let barrier_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *barrier)
        .await
        .unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(lock_key)
        .execute(&mut *barrier)
        .await
        .unwrap();
    let request = f.request("commit-delete-retry", 1).await;
    let first_pool = original_pool.clone();
    let first_request = request.clone();
    let original =
        tokio::spawn(async move { apply_postgres(&first_pool, "a", &first_request).await });
    wait_for_inventory_blocker(&f.admin, original_pid, barrier_pid).await;

    // Queue the DELETE behind the original writer before allowing the retry to
    // perform its first receipt read and wait for the same product row.
    let mut delete_connection = f.admin.acquire().await.unwrap();
    let delete_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *delete_connection)
        .await
        .unwrap();
    let deletion = tokio::spawn(async move {
        sqlx::query("DELETE FROM products WHERE id='owned'")
            .execute(&mut *delete_connection)
            .await
            .unwrap()
            .rows_affected()
    });
    wait_for_inventory_blocker(&f.admin, delete_pid, original_pid).await;
    let second_pool = retry_pool.clone();
    let second_request = request.clone();
    let retry =
        tokio::spawn(async move { apply_postgres(&second_pool, "a", &second_request).await });
    wait_for_inventory_blocker(&f.admin, retry_pid, delete_pid).await;
    barrier.commit().await.unwrap();

    let saved = original.await.unwrap().unwrap();
    assert_eq!(deletion.await.unwrap(), 1);
    let recovered = retry.await.unwrap().unwrap();
    assert_eq!(
        serde_json::to_value(&saved).unwrap(),
        serde_json::to_value(recovered).unwrap()
    );
    assert_eq!(f.counts().await, (1, 1, 1));
    assert!(read_postgres(&f.pool, "a").await.unwrap().is_empty());
    original_pool.close().await;
    retry_pool.close().await;
    f.close().await;
}
