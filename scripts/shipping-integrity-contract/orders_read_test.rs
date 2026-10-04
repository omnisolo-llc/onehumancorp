use super::*;

async fn app(f: &Fixture) -> Router {
    // Prove predicates and transaction-local RLS using a non-bypass role.
    for table in ["orders", "customers"] {
        sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; ALTER TABLE {table} FORCE ROW LEVEL SECURITY; CREATE POLICY scoped ON {table} USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));"))
            .execute(&f.owner).await.unwrap();
    }
    crate::actual_parent_mount(
        Arc::new(crate::db::DB {
            pool: f.pool.clone(),
            store: crate::db::DbStore::Postgres,
        }),
        f.auth.clone(),
    )
    .await
}

#[tokio::test]
async fn committed_order_list_reads_do_not_return_a_primed_snapshot() {
    let f = Fixture::new().await;
    let app = app(&f).await;
    let headers = [("authorization", format!("Bearer {}", f.token))];
    for path in [
        "/api/v1/ui/orders",
        "/api/v1/ui/orders?mobile_optimized=true",
    ] {
        let (status, before) = response(app.clone(), path, None, &headers).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(before.as_array().unwrap().len(), 1);
    }
    sqlx::raw_sql("INSERT INTO customers(id,tenant_id,name) VALUES('fresh-customer','tenant-a','Committed customer'); INSERT INTO orders(id,tenant_id,customer_id,total_amount,status)VALUES('fresh-order','tenant-a','fresh-customer',42.50,'pending'); UPDATE orders SET status='fulfilled' WHERE id='order-a';").execute(&f.owner).await.unwrap();
    for path in [
        "/api/v1/ui/orders",
        "/api/v1/ui/orders?mobile_optimized=true",
    ] {
        let (status, after) = response(app.clone(), path, None, &headers).await;
        assert_eq!(status, StatusCode::OK);
        let rows = after.as_array().unwrap();
        assert_eq!(rows.len(), 2);
        assert!(
            rows.iter()
                .any(|r| r["id"] == "fresh-order" && r["total_amount"] == 42.5)
        );
        assert!(
            rows.iter()
                .any(|r| r["id"] == "order-a" && r["status"] == "fulfilled")
        );
    }
    f.finish().await;
}

#[tokio::test]
async fn exact_order_detail_is_independent_of_newest_fifty_pagination() {
    let f = Fixture::new().await;
    let app = app(&f).await;
    sqlx::raw_sql("UPDATE orders SET created_at='2000-01-01' WHERE id='order-a';INSERT INTO orders(id,tenant_id,status,created_at) SELECT 'newer-'||n,'tenant-a','pending',CURRENT_TIMESTAMP FROM generate_series(1,60)n;").execute(&f.owner).await.unwrap();
    let headers = [("authorization", format!("Bearer {}", f.token))];
    let (status, rows) = response(app.clone(), "/api/v1/ui/orders", None, &headers).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(rows.as_array().unwrap().len(), 50);
    assert!(
        !rows
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["id"] == "order-a")
    );
    let (status, order) = response(app.clone(), "/api/v1/ui/orders/order-a", None, &headers).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(order["id"], "order-a");
    assert_eq!(order["status"], "paid");
    sqlx::query("UPDATE orders SET status='pending' WHERE id='order-a'")
        .execute(&f.owner)
        .await
        .unwrap();
    let (_, order) = response(app, "/api/v1/ui/orders/order-a", None, &headers).await;
    assert_eq!(order["status"], "pending");
    f.finish().await;
}

#[tokio::test]
async fn exact_detail_and_customer_join_cannot_follow_forged_tenants() {
    let f = Fixture::new().await;
    let app = app(&f).await;
    sqlx::raw_sql("INSERT INTO customers(id,tenant_id,name)VALUES('private-customer','tenant-b','Foreign private customer');UPDATE orders SET customer_id='private-customer' WHERE id='order-a';").execute(&f.owner).await.unwrap();
    let headers = [
        ("authorization", format!("Bearer {}", f.token)),
        ("x-tenant-id", "tenant-b".into()),
    ];
    for path in [
        "/api/v1/ui/orders/order-b",
        "/api/v1/ui/orders/order-b?tenant_id=tenant-b",
        "/api/v1/ui/orders/missing",
    ] {
        let (status, body) = response(app.clone(), path, None, &headers).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body, json!({"error":"Order not found"}));
    }
    let (status, order) = response(
        app.clone(),
        "/api/v1/ui/orders/order-a?tenant_id=tenant-b",
        None,
        &headers,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(order["id"], "order-a");
    assert_eq!(order["customer_name"], "");
    let (status, rows) = response(
        app,
        "/api/v1/ui/orders?tenant_id=tenant-b&fields=id,status",
        None,
        &headers,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(rows, json!([{"id":"order-a","status":"paid"}]));
    f.finish().await;
}

#[tokio::test]
async fn order_reads_keep_current_strict_bearer_access_after_priming() {
    let f = Fixture::new().await;
    let app = app(&f).await;
    let headers = [("authorization", format!("Bearer {}", f.token))];
    for path in ["/api/v1/ui/orders", "/api/v1/ui/orders/order-a"] {
        assert_eq!(
            response(app.clone(), path, None, &[]).await.0,
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            response(app.clone(), path, None, &headers).await.0,
            StatusCode::OK
        );
    }
    sqlx::query("UPDATE users SET active=false WHERE id='owner-a'")
        .execute(&f.owner)
        .await
        .unwrap();
    for path in ["/api/v1/ui/orders", "/api/v1/ui/orders/order-a"] {
        assert_eq!(
            response(app.clone(), path, None, &headers).await.0,
            StatusCode::UNAUTHORIZED
        );
    }
    f.finish().await;
}

#[tokio::test]
async fn committed_storage_failure_is_not_a_cached_order_or_false_missing() {
    let f = Fixture::new().await;
    let app = app(&f).await;
    let headers = [("authorization", format!("Bearer {}", f.token))];
    for path in ["/api/v1/ui/orders", "/api/v1/ui/orders/order-a"] {
        assert_eq!(
            response(app.clone(), path, None, &headers).await.0,
            StatusCode::OK
        );
    }
    sqlx::query("DROP TABLE customers CASCADE")
        .execute(&f.owner)
        .await
        .unwrap();
    for path in ["/api/v1/ui/orders", "/api/v1/ui/orders/order-a"] {
        let (status, body) = response(app.clone(), path, None, &headers).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body, json!({"error":"Order data is unavailable"}));
    }
    f.finish().await;
}

#[tokio::test]
async fn order_list_and_exact_detail_responses_are_private_and_not_storable() {
    let f = Fixture::new().await;
    let app = app(&f).await;
    for path in [
        "/api/v1/ui/orders",
        "/api/v1/ui/orders/order-a",
        "/api/v1/ui/orders/missing",
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(path)
                    .header("authorization", format!("Bearer {}", f.token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.headers()["cache-control"], "private, no-store");
        assert_eq!(response.headers()["vary"], "Authorization");
    }
    f.finish().await;
}

#[tokio::test]
async fn sqlite_exact_details_and_post_commit_lists_use_the_same_authoritative_contract() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    let (auth, token) = sqlite_auth(&pool).await;
    sqlx::raw_sql("CREATE TABLE customers(id TEXT PRIMARY KEY,tenant_id TEXT,name TEXT); CREATE TABLE orders(id TEXT PRIMARY KEY,tenant_id TEXT,customer_id TEXT,total_amount REAL,status TEXT,created_at TEXT);INSERT INTO customers VALUES('customer-a','tenant-a','Owned customer'),('customer-b','tenant-b','Private customer');INSERT INTO orders VALUES('order-a','tenant-a','customer-a',42.5,'paid','2000-01-01'),('order-b','tenant-b','customer-b',10,'paid','2026-01-01');").execute(&pool).await.unwrap();
    let app = crate::actual_parent_mount(
        Arc::new(crate::db::DB {
            pool: disconnected(),
            store: crate::db::DbStore::Sqlite(pool.clone()),
        }),
        auth,
    )
    .await;
    let headers = [("authorization", format!("Bearer {token}"))];
    assert_eq!(
        response(app.clone(), "/api/v1/ui/orders", None, &headers)
            .await
            .1
            .as_array()
            .unwrap()
            .len(),
        1
    );
    sqlx::raw_sql("WITH RECURSIVE n(i)AS(SELECT 1 UNION ALL SELECT i+1 FROM n WHERE i<60)INSERT INTO orders SELECT 'newer-'||i,'tenant-a','customer-a',1,'pending','2026-10-04' FROM n;").execute(&pool).await.unwrap();
    let (_, rows) = response(app.clone(), "/api/v1/ui/orders", None, &headers).await;
    assert_eq!(rows.as_array().unwrap().len(), 50);
    assert!(
        !rows
            .as_array()
            .unwrap()
            .iter()
            .any(|r| r["id"] == "order-a")
    );
    let (status, order) = response(app.clone(), "/api/v1/ui/orders/order-a", None, &headers).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(order["customer_name"], "Owned customer");
    assert_eq!(
        response(
            app.clone(),
            "/api/v1/ui/orders/order-b?tenant_id=tenant-b",
            None,
            &headers
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    sqlx::query("UPDATE orders SET status='pending' WHERE id='order-a'")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        response(app.clone(), "/api/v1/ui/orders/order-a", None, &headers)
            .await
            .1["status"],
        "pending"
    );
    sqlx::query("DROP TABLE customers")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        response(app, "/api/v1/ui/orders/order-a", None, &headers)
            .await
            .0,
        StatusCode::INTERNAL_SERVER_ERROR
    );
    pool.close().await;
}
