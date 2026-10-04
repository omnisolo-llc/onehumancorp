use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};

struct ProviderProbe {
    base: String,
    posts: Arc<AtomicUsize>,
    gets: Arc<AtomicUsize>,
    receipt: Arc<tokio::sync::Mutex<Value>>,
    server: tokio::task::JoinHandle<()>,
}
impl ProviderProbe {
    async fn start(unknown: bool) -> Self {
        let posts = Arc::new(AtomicUsize::new(0));
        let gets = Arc::new(AtomicUsize::new(0));
        let receipt = Arc::new(tokio::sync::Mutex::new(
            json!({"status":"SUCCESS","object_state":"VALID","object_id":"purchase_txn","test":true,"label_url":"https://app.goshippo.com/labels/purchase.pdf"}),
        ));
        let (p, r) = (posts.clone(), receipt.clone());
        let (g, read) = (gets.clone(), receipt.clone());
        let list = axum::routing::get(move || {
            let (g, r) = (g.clone(), read.clone());
            async move {
                g.fetch_add(1, Ordering::SeqCst);
                axum::Json(json!({"next":null,"previous":null,"results":[r.lock().await.clone()]}))
            }
        });
        let app = Router::new().route(
            "/transactions",
            list.post(move |axum::Json(request): axum::Json<Value>| {
                let (p, r) = (p.clone(), r.clone());
                async move {
                    p.fetch_add(1, Ordering::SeqCst);
                    let mut value = r.lock().await;
                    value["rate"] = request["rate"].clone();
                    value["metadata"] = request["metadata"].clone();
                    if unknown {
                        (
                            StatusCode::BAD_GATEWAY,
                            "synthetic unknown outcome".to_string(),
                        )
                    } else {
                        (StatusCode::OK, value.to_string())
                    }
                }
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        Self {
            base,
            posts,
            gets,
            receipt,
            server,
        }
    }
}
impl Drop for ProviderProbe {
    fn drop(&mut self) {
        self.server.abort();
    }
}
async fn configured<T>(base: &str, work: impl std::future::Future<Output = T>) -> T {
    temp_env::async_with_vars(
        [
            ("SHIPPO_API_BASE", Some(base)),
            ("SHIPPO_API_TOKEN", Some("shippo_test_synthetic")),
            ("SHIPPO_TENANT_ID", Some("tenant-a")),
            ("SHIPPO_ACCOUNT_NAMESPACE", Some("synthetic-account")),
            ("SHIPPO_WEBHOOK_MODE", Some("test")),
        ],
        work,
    )
    .await
}
fn purchase() -> Option<Value> {
    Some(json!({"orderId":"order-a","rateId":signed_fixture_rate()}))
}

#[tokio::test]
async fn purchase_success_replays_exact_receipt_without_provider_or_fulfillment() {
    let f = Fixture::new().await;
    let p = ProviderProbe::start(false).await;
    configured(&p.base, async {
        let a = response(f.shipping_app().await, "/label", purchase(), &f.headers()).await;
        let b = response(f.shipping_app().await, "/label", purchase(), &f.headers()).await;
        assert_eq!(a.0, StatusCode::OK);
        assert_eq!(
            b, a,
            "a fresh router must replay the exact saved label receipt"
        );
        assert_eq!(p.posts.load(Ordering::SeqCst), 1);
        assert_eq!(p.gets.load(Ordering::SeqCst), 0);
        let order: String = sqlx::query_scalar("SELECT status FROM orders WHERE id='order-a'")
            .fetch_one(&f.owner)
            .await
            .unwrap();
        assert_eq!(order, "paid");
    })
    .await;
    f.finish().await;
}

#[tokio::test]
async fn purchase_unknown_reconciles_exact_metadata_with_read_only_provider_calls() {
    let f = Fixture::new().await;
    let p = ProviderProbe::start(true).await;
    configured(&p.base, async {
        let initial = response(f.shipping_app().await, "/label", purchase(), &f.headers()).await;
        assert_eq!(initial.0, StatusCode::ACCEPTED);
        let status = response(
            f.shipping_app().await,
            "/purchase/order-a",
            None,
            &f.headers(),
        )
        .await;
        assert_eq!(status.0, StatusCode::ACCEPTED);
        assert_eq!(
            p.gets.load(Ordering::SeqCst),
            0,
            "reading local intent must not contact provider"
        );
        let recovered = response(
            f.shipping_app().await,
            "/label/reconcile",
            Some(json!({"orderId":"order-a"})),
            &f.headers(),
        )
        .await;
        assert_eq!(recovered.0, StatusCode::OK);
        assert_eq!(recovered.1["transactionId"], "purchase_txn");
        let replay = response(f.shipping_app().await, "/label", purchase(), &f.headers()).await;
        assert_eq!(replay, recovered);
        assert_eq!(
            p.posts.load(Ordering::SeqCst),
            1,
            "reconciliation can never submit a purchase"
        );
        assert_eq!(p.gets.load(Ordering::SeqCst), 1);
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings")
            .fetch_one(&f.owner)
            .await
            .unwrap();
        assert_eq!(count, 1);
    })
    .await;
    f.finish().await;
}

#[tokio::test]
async fn purchase_reconciliation_rejects_foreign_metadata_rate_and_mode() {
    for field in ["metadata", "rate", "test"] {
        let f = Fixture::new().await;
        let p = ProviderProbe::start(true).await;
        configured(&p.base, async {
            assert_eq!(
                response(f.shipping_app().await, "/label", purchase(), &f.headers())
                    .await
                    .0,
                StatusCode::ACCEPTED
            );
            {
                let mut r = p.receipt.lock().await;
                r[field] = if field == "test" {
                    json!(false)
                } else {
                    json!("foreign")
                };
            }
            let result = response(
                f.shipping_app().await,
                "/label/reconcile",
                Some(json!({"orderId":"order-a"})),
                &f.headers(),
            )
            .await;
            assert_eq!(
                result.0,
                StatusCode::ACCEPTED,
                "foreign {field} must not become a binding"
            );
            assert_eq!(result.1["reconciliationRequired"], true);
            assert_eq!(p.posts.load(Ordering::SeqCst), 1);
            let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings")
                .fetch_one(&f.owner)
                .await
                .unwrap();
            assert_eq!(count, 0);
        })
        .await;
        f.finish().await;
    }
}

#[tokio::test]
async fn purchase_existing_delivery_is_rejected_before_external_dispatch() {
    let f = Fixture::new().await;
    f.task("tenant-a", "order-a", "shippo", "prior-tracking")
        .await;
    let p = ProviderProbe::start(false).await;
    configured(&p.base, async {
        let result = response(f.shipping_app().await, "/label", purchase(), &f.headers()).await;
        assert_eq!(result.0, StatusCode::CONFLICT);
        assert_eq!(p.posts.load(Ordering::SeqCst), 0);
    })
    .await;
    f.finish().await;
}

#[tokio::test]
async fn purchase_different_rate_cannot_replace_unknown_order_intent() {
    let f = Fixture::new().await;
    let p = ProviderProbe::start(true).await;
    configured(&p.base, async {
        assert_eq!(
            response(f.shipping_app().await, "/label", purchase(), &f.headers())
                .await
                .0,
            StatusCode::ACCEPTED
        );
        let mut mac = Hmac::<Sha256>::new_from_slice(b"shippo_test_synthetic").unwrap();
        mac.update(b"tenant-a\0order-a\0another_rate");
        let rate = format!("another_rate.{}", hex::encode(mac.finalize().into_bytes()));
        let result = response(
            f.shipping_app().await,
            "/label",
            Some(json!({"orderId":"order-a","rateId":rate})),
            &f.headers(),
        )
        .await;
        assert_eq!(result.0, StatusCode::CONFLICT);
        assert_eq!(p.posts.load(Ordering::SeqCst), 1);
    })
    .await;
    f.finish().await;
}

#[tokio::test]
async fn purchase_sqlite_reopen_preserves_unknown_fence_and_recovers_without_repurchase() {
    let path = std::env::temp_dir().join(format!("ohc-purchase-{}.sqlite", Uuid::new_v4()));
    let opts = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(&path)
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(3)
        .connect_with(opts.clone())
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("sqlite_schema.sql"))
        .execute(&pool)
        .await
        .unwrap();
    crate::actual_sqlite_shipping_startup(&pool).await.unwrap();
    sqlx::query("INSERT INTO orders(id,tenant_id,status) VALUES('order-a','tenant-a','paid')")
        .execute(&pool)
        .await
        .unwrap();
    let (auth, token) = sqlite_auth(&pool).await;
    let p = ProviderProbe::start(true).await;
    configured(&p.base, async {
        let headers = [("authorization", format!("Bearer {token}"))];
        let app = crate::actual_parent_mount(
            Arc::new(crate::db::DB {
                pool: disconnected(),
                store: crate::db::DbStore::Sqlite(pool.clone()),
            }),
            auth.clone(),
        )
        .await;
        let a = response(app, "/api/v1/shipping/label", purchase(), &headers).await;
        assert_eq!(a.0, StatusCode::ACCEPTED);
        pool.close().await;
        let reopened = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(3)
            .connect_with(opts)
            .await
            .unwrap();
        crate::actual_sqlite_shipping_startup(&reopened)
            .await
            .unwrap();
        let auth = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
            server_auth::seaorm_store::SeaOrmAuthRepository::new(
                sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(reopened.clone()),
            ),
        )));
        let app = crate::actual_parent_mount(
            Arc::new(crate::db::DB {
                pool: disconnected(),
                store: crate::db::DbStore::Sqlite(reopened.clone()),
            }),
            auth,
        )
        .await;
        let b = response(app.clone(), "/api/v1/shipping/label", purchase(), &headers).await;
        assert_eq!(b.0, StatusCode::ACCEPTED);
        assert_eq!(
            p.posts.load(Ordering::SeqCst),
            1,
            "SQLite reopen must not erase admission"
        );
        let recovered = response(
            app,
            "/api/v1/shipping/label/reconcile",
            Some(json!({"orderId":"order-a"})),
            &headers,
        )
        .await;
        assert_eq!(recovered.0, StatusCode::OK);
        assert_eq!(p.posts.load(Ordering::SeqCst), 1);
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings")
            .fetch_one(&reopened)
            .await
            .unwrap();
        assert_eq!(count, 1);
        reopened.close().await;
    })
    .await;
    drop(auth);
    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn purchase_revoked_owner_cannot_dispatch_or_reconcile() {
    let f = Fixture::new().await;
    let p = ProviderProbe::start(true).await;
    configured(&p.base, async {
        assert_eq!(
            response(f.shipping_app().await, "/label", purchase(), &f.headers())
                .await
                .0,
            StatusCode::ACCEPTED
        );
        let claims = f.auth.validate_token(&f.token).await.unwrap();
        f.auth
            .revoke_token(
                claims.jti,
                chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                "tenant-a",
            )
            .await
            .unwrap();
        for (path, body) in [
            ("/label", purchase()),
            ("/label/reconcile", Some(json!({"orderId":"order-a"}))),
        ] {
            let result = response(f.shipping_app().await, path, body, &f.headers()).await;
            assert_eq!(result.0, StatusCode::UNAUTHORIZED);
        }
        assert_eq!(p.posts.load(Ordering::SeqCst), 1);
        assert_eq!(p.gets.load(Ordering::SeqCst), 0);
    })
    .await;
    f.finish().await;
}

#[tokio::test]
async fn purchase_revoked_while_admission_waits_does_not_dispatch() {
    let f = Fixture::new().await;
    let p = ProviderProbe::start(false).await;
    configured(&p.base, async {
        let app = f.shipping_app().await;
        let mut lock = f.owner.begin().await.unwrap();
        sqlx::query("SELECT id FROM orders WHERE id='order-a' FOR UPDATE")
            .execute(&mut *lock)
            .await
            .unwrap();
        let headers = [("authorization", format!("Bearer {}", f.token))];
        let task = tokio::spawn(async move { response(app, "/label", purchase(), &headers).await });
        tokio::time::sleep(Duration::from_millis(100)).await;
        let claims = f.auth.validate_token(&f.token).await.unwrap();
        f.auth
            .revoke_token(
                claims.jti,
                chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                "tenant-a",
            )
            .await
            .unwrap();
        lock.commit().await.unwrap();
        let result = task.await.unwrap();
        assert_eq!(
            p.posts.load(Ordering::SeqCst),
            0,
            "revocation before admission commit must prevent dispatch"
        );
        assert_eq!(result.0, StatusCode::FORBIDDEN);
    })
    .await;
    f.finish().await;
}

#[tokio::test]
async fn purchase_intent_cannot_be_adopted_by_another_current_owner() {
    let f = Fixture::new().await;
    let p = ProviderProbe::start(true).await;
    configured(&p.base, async {
        assert_eq!(
            response(f.shipping_app().await, "/label", purchase(), &f.headers())
                .await
                .0,
            StatusCode::ACCEPTED
        );
        sqlx::query("UPDATE identity_user_roles SET role_name='ADMIN' WHERE user_id='member-a'")
            .execute(&f.owner)
            .await
            .unwrap();
        let headers = [("authorization", format!("Bearer {}", f.member))];
        for (path, body) in [
            ("/label", purchase()),
            ("/label/reconcile", Some(json!({"orderId":"order-a"}))),
        ] {
            assert_eq!(
                response(f.shipping_app().await, path, body, &headers)
                    .await
                    .0,
                StatusCode::FORBIDDEN
            );
        }
        assert_eq!(p.posts.load(Ordering::SeqCst), 1);
        assert_eq!(p.gets.load(Ordering::SeqCst), 0);
    })
    .await;
    f.finish().await;
}

#[tokio::test]
async fn purchase_concurrent_reconciliation_records_one_binding_and_replays_one_receipt() {
    let f = Fixture::new().await;
    let p = ProviderProbe::start(true).await;
    configured(&p.base, async {
        assert_eq!(
            response(f.shipping_app().await, "/label", purchase(), &f.headers())
                .await
                .0,
            StatusCode::ACCEPTED
        );
        let app = f.shipping_app().await;
        let headers = f.headers();
        let body = Some(json!({"orderId":"order-a"}));
        let (a, b) = tokio::join!(
            response(app.clone(), "/label/reconcile", body.clone(), &headers),
            response(app, "/label/reconcile", body, &headers)
        );
        assert_eq!(a.0, StatusCode::OK);
        assert_eq!(a, b);
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM delivery_provider_bindings")
            .fetch_one(&f.owner)
            .await
            .unwrap();
        assert_eq!(count, 1);
        assert_eq!(p.posts.load(Ordering::SeqCst), 1);
    })
    .await;
    f.finish().await;
}

const IDENTITY_MUTATIONS: &[&str] = &[
    "id='changed'",
    "organization_id='tenant-b'",
    "actor_id='member-a'",
    "order_id='another-order'",
    "rate_id='another-rate'",
    "account_namespace='another-account'",
    "is_test=false",
    "transaction_id='different-transaction'",
    "receipt_json='{}'",
    "status='dispatched',receipt_json=NULL",
    "created_at='2001-01-01 00:00:00'",
];

#[tokio::test]
async fn purchase_postgres_identity_and_recorded_receipt_are_immutable_and_tenant_scoped() {
    let f = Fixture::new().await;
    let p = ProviderProbe::start(false).await;
    configured(&p.base, async {
        assert_eq!(
            response(f.shipping_app().await, "/label", purchase(), &f.headers())
                .await
                .0,
            StatusCode::OK
        );
        for mutation in IDENTITY_MUTATIONS {
            assert!(
                sqlx::query(&format!(
                    "UPDATE shipping_purchase_intents SET {mutation} WHERE order_id='order-a'"
                ))
                .execute(&f.owner)
                .await
                .is_err(),
                "identity mutation allowed: {mutation}"
            );
        }
        let mut tx = f.pool.begin().await.unwrap();
        sqlx::query("SELECT set_config('app.current_tenant','tenant-b',true)")
            .execute(&mut *tx)
            .await
            .unwrap();
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM shipping_purchase_intents")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        assert_eq!(count, 0);
        tx.rollback().await.unwrap();
        assert_eq!(
            response(f.shipping_app().await, "/label", purchase(), &f.headers())
                .await
                .0,
            StatusCode::OK
        );
        assert_eq!(p.posts.load(Ordering::SeqCst), 1);
    })
    .await;
    f.finish().await;
}

#[tokio::test]
async fn purchase_sqlite_identity_and_recorded_receipt_are_immutable() {
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
    sqlx::query("INSERT INTO orders(id,tenant_id,status) VALUES('order-a','tenant-a','paid')")
        .execute(&pool)
        .await
        .unwrap();
    let (auth, token) = sqlite_auth(&pool).await;
    let p = ProviderProbe::start(false).await;
    configured(&p.base, async {
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
            response(app.clone(), "/api/v1/shipping/label", purchase(), &headers)
                .await
                .0,
            StatusCode::OK
        );
        for mutation in IDENTITY_MUTATIONS {
            assert!(
                sqlx::query(&format!(
                    "UPDATE shipping_purchase_intents SET {mutation} WHERE order_id='order-a'"
                ))
                .execute(&pool)
                .await
                .is_err(),
                "identity mutation allowed: {mutation}"
            );
        }
        assert_eq!(
            response(app, "/api/v1/shipping/label", purchase(), &headers)
                .await
                .0,
            StatusCode::OK
        );
        assert_eq!(p.posts.load(Ordering::SeqCst), 1);
    })
    .await;
    pool.close().await;
}

#[tokio::test]
async fn purchase_aborted_handler_leaves_restart_fence_and_recovers_lost_response() {
    let f = Fixture::new().await;
    let calls = Arc::new(AtomicUsize::new(0));
    let reached = Arc::new(tokio::sync::Notify::new());
    let receipt = Arc::new(tokio::sync::Mutex::new(Value::Null));
    let (posts, ready, saved) = (calls.clone(), reached.clone(), receipt.clone());
    let provider=Router::new().route("/transactions",axum::routing::post(move |axum::Json(request):axum::Json<Value>| {
        let (posts,ready,saved)=(posts.clone(),ready.clone(),saved.clone());
        async move {
            posts.fetch_add(1,Ordering::SeqCst);
            *saved.lock().await=json!({"metadata":request["metadata"],"rate":request["rate"],"status":"SUCCESS","object_id":"lost_response_txn","test":true,"label_url":"https://app.goshippo.com/labels/lost.pdf"});
            ready.notify_one();
            std::future::pending::<StatusCode>().await
        }
    }).get(move || {let receipt=receipt.clone();async move {axum::Json(json!({"next":null,"previous":null,"results":[receipt.lock().await.clone()]}))}}));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move { axum::serve(listener, provider).await.unwrap() });
    configured(&base, async {
        let app = f.shipping_app().await;
        let headers = [("authorization", format!("Bearer {}", f.token))];
        let request =
            tokio::spawn(async move { response(app, "/label", purchase(), &headers).await });
        tokio::time::timeout(Duration::from_secs(3), reached.notified())
            .await
            .unwrap();
        request.abort();
        assert!(request.await.unwrap_err().is_cancelled());
        assert_eq!(
            response(f.shipping_app().await, "/label", purchase(), &f.headers())
                .await
                .0,
            StatusCode::ACCEPTED
        );
        let recovered = response(
            f.shipping_app().await,
            "/label/reconcile",
            Some(json!({"orderId":"order-a"})),
            &f.headers(),
        )
        .await;
        assert_eq!(recovered.0, StatusCode::OK);
        assert_eq!(recovered.1["transactionId"], "lost_response_txn");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    })
    .await;
    server.abort();
    f.finish().await;
}
