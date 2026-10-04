use super::*;
use axum::{
    Router,
    routing::{get, patch},
};
use axum::{
    body::{Body, to_bytes},
    http::Request,
    routing::post,
};
use std::sync::atomic::Ordering;
use tower::ServiceExt;
#[derive(Debug)]
struct WireRequest {
    reference: String,
    amount: i64,
    currency: String,
}
static WIRE_REQUESTS: std::sync::Mutex<Vec<WireRequest>> = std::sync::Mutex::new(vec![]);
struct Fixture {
    pool: PgPool,
    admin: PgPool,
    schema: String,
    id: String,
    version: String,
    provider_task: tokio::task::JoinHandle<()>,
}
impl Fixture {
    async fn new() -> Self {
        let url =
            std::env::var("OHC_QUOTE_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
        let admin = PgPool::connect(&url).await.unwrap();
        let schema = format!("quote_receipt_{}", Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let search = schema.clone();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
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
        sqlx::raw_sql("CREATE TABLE tenants(id TEXT PRIMARY KEY);CREATE TABLE customers(id TEXT PRIMARY KEY,tenant_id TEXT,name TEXT);CREATE TABLE bookings(id TEXT PRIMARY KEY);CREATE TABLE builder_sites(id TEXT PRIMARY KEY);").execute(&pool).await.unwrap();
        for migration in [
            include_str!("../../src/server/migrations/078_quote_engine.sql"),
            include_str!("../../src/server/migrations/107_quote_checkout_url.sql"),
            include_str!("../../src/server/migrations/114_invoicing_agent.sql"),
            include_str!("../../src/server/migrations/166_invoice_quote_link.sql"),
            include_str!("../../src/server/migrations/229_quotes_and_builder_parity.sql"),
            include_str!(
                "../../src/server/migrations/1001_create_omni_inbox_messages_and_quotes_fix.sql"
            ),
            include_str!("../../src/server/migrations/1002_add_created_updated_at_to_quotes.sql"),
            include_str!("../../src/server/migrations/1013_invoice_runtime_contract.sql"),
            include_str!("../../src/server/migrations/1016_quote_line_items_parity.sql"),
            include_str!("../../src/server/migrations/1017_quote_acceptance_receipt.sql"),
        ] {
            sqlx::raw_sql(migration).execute(&pool).await.unwrap();
        }
        sqlx::raw_sql("INSERT INTO tenants VALUES('tenant-a'),('tenant-b');INSERT INTO customers VALUES('customer-a','tenant-a','Fixture');").execute(&pool).await.unwrap();
        let id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO quotes(id,tenant_id,customer_id,status,total_amount_cents,required_deposit_cents,updated_at) VALUES($1,'tenant-a','customer-a','SENT',1234,200,'2026-10-01T00:00:00Z')").bind(&id).execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO quote_line_items(id,quote_id,tenant_id,description,unit_price_cents,quantity)VALUES($1,$2,'tenant-a','Reviewed service',1234,1)").bind(Uuid::new_v4().to_string()).bind(&id).execute(&pool).await.unwrap();
        PROVIDER_CALLS.store(0, Ordering::SeqCst);
        REDIS_CALLS.store(0, Ordering::SeqCst);
        PROVIDER_UNKNOWN.store(false, Ordering::SeqCst);
        PROVIDER_BLOCKED.store(false, Ordering::SeqCst);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        unsafe {
            std::env::set_var(
                "STRIPE_API_KEY",
                "sk_offline_public_fixture_never_a_real_credential",
            );
            std::env::set_var("STRIPE_API_BASE", origin);
            std::env::set_var("PUBLIC_APP_URL", "https://fixture.example");
            std::env::set_var("NO_PROXY", "127.0.0.1,localhost");
        }
        let provider_task = tokio::spawn(async move {
            axum::serve(
                listener,
                Router::new().route("/v1/checkout/sessions", post(checkout_fixture)),
            )
            .await
            .unwrap()
        });
        OPERATIONS.lock().unwrap().clear();
        WIRE_REQUESTS.lock().unwrap().clear();
        let version: DateTime<Utc> =
            sqlx::query_scalar("SELECT updated_at FROM quotes WHERE id=$1")
                .bind(&id)
                .fetch_one(&pool)
                .await
                .unwrap();
        Self {
            pool,
            admin,
            schema,
            id,
            version: version.to_rfc3339(),
            provider_task,
        }
    }
    fn app(&self, tenant: &str) -> Router {
        let claims:server_common::Claims=serde_json::from_value(serde_json::json!({"sub":"fixture-owner","exp":4000000000u64,"organization_id":tenant,"roles":["ADMIN"],"iat":0})).unwrap();
        Router::new()
            .route("/quotes/{id}/accept", post(accept_quote))
            .route("/quotes/{id}", get(get_quote).put(update_quote))
            .route("/quotes/{id}/approve", patch(approve_quote))
            .layer(Extension(claims))
            .with_state(self.pool.clone())
    }
    async fn accept(&self) -> (StatusCode, serde_json::Value) {
        let response = self
            .app("tenant-a")
            .oneshot(
                Request::post(format!("/quotes/{}/accept", self.id))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({"expected_updated_at":self.version}).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (
            status,
            serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
        )
    }
    async fn finish(self) {
        self.provider_task.abort();
        self.pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {} CASCADE", self.schema))
            .execute(&self.admin)
            .await
            .unwrap();
        self.admin.close().await;
    }
}
#[tokio::test]
async fn repeated_acceptance_reuses_one_committed_invoice_and_checkout() {
    let f = Fixture::new().await;
    let (s1, a) = f.accept().await;
    let (s2, b) = f.accept().await;
    assert_eq!(s1, StatusCode::OK);
    assert_eq!(s2, StatusCode::OK);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM invoices")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(count, 1, "replay must not create another committed invoice");
    assert_eq!(a["invoice_id"], b["invoice_id"]);
    assert_eq!(
        PROVIDER_CALLS.load(Ordering::SeqCst),
        1,
        "replay must not create another provider session"
    );
    {
        let wire = WIRE_REQUESTS.lock().unwrap();
        assert_eq!(wire.len(), 1);
        assert_eq!(wire[0].reference, f.id);
        assert_eq!(wire[0].amount, 1234);
        assert_eq!(wire[0].currency, "usd");
    }

    f.finish().await;
}
#[tokio::test]
async fn invoice_failure_prevents_any_provider_effect() {
    let f = Fixture::new().await;
    sqlx::raw_sql("CREATE FUNCTION reject_lines() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN RAISE EXCEPTION 'forced line failure';END$$;CREATE TRIGGER reject_lines BEFORE INSERT ON invoice_line_items FOR EACH ROW EXECUTE FUNCTION reject_lines();").execute(&f.pool).await.unwrap();
    let (status, _) = f.accept().await;
    assert!(status.is_server_error());
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM invoices")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    assert_eq!(
        PROVIDER_CALLS.load(Ordering::SeqCst),
        0,
        "no provider effect can precede the committed local preparation"
    );
    f.finish().await;
}

async fn request(
    app: Router,
    path: String,
    method: &str,
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
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
    )
}
#[tokio::test]
async fn concurrent_acceptance_commits_one_invoice_and_attempt() {
    let f = Fixture::new().await;
    let mut jobs = tokio::task::JoinSet::new();
    for _ in 0..8 {
        let app = f.app("tenant-a");
        let path = format!("/quotes/{}/accept", f.id);
        jobs.spawn(request(
            app,
            path,
            "POST",
            serde_json::json!({"expected_updated_at":f.version}),
        ));
    }
    let mut invoice = None;
    while let Some(result) = jobs.join_next().await {
        let (status, body) = result.unwrap();
        assert_eq!(status, StatusCode::OK);
        if let Some(id) = &invoice {
            assert_eq!(id, &body["invoice_id"])
        } else {
            invoice = Some(body["invoice_id"].clone())
        }
    }
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 1);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM invoices")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    f.finish().await;
}
#[tokio::test]
async fn first_acceptance_requires_the_observed_current_version() {
    let f = Fixture::new().await;
    for body in [
        serde_json::json!({}),
        serde_json::json!({"expected_updated_at":"2026-09-30T00:00:00Z"}),
    ] {
        let (status, _) = request(
            f.app("tenant-a"),
            format!("/quotes/{}/accept", f.id),
            "POST",
            body,
        )
        .await;
        assert_eq!(status, StatusCode::CONFLICT);
    }
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM invoices")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    f.finish().await;
}
#[tokio::test]
async fn foreign_tenant_cannot_accept_or_read_receipt() {
    let f = Fixture::new().await;
    let (status, _) = request(
        f.app("tenant-b"),
        format!("/quotes/{}/accept", f.id),
        "POST",
        serde_json::json!({"expected_updated_at":f.version}),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    let (status, _) = request(
        f.app("tenant-b"),
        format!("/quotes/{}", f.id),
        "GET",
        serde_json::json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    f.finish().await;
}
#[tokio::test]
async fn unknown_provider_outcome_is_durable_and_never_automatically_retried() {
    let f = Fixture::new().await;
    PROVIDER_UNKNOWN.store(true, Ordering::SeqCst);
    let (status, a) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(a["checkout_status"], "reconciliation");
    assert_eq!(a["stripe_payment_link"], "");
    PROVIDER_UNKNOWN.store(false, Ordering::SeqCst);
    let (status, b) = request(
        f.app("tenant-a"),
        format!("/quotes/{}/accept", f.id),
        "POST",
        serde_json::json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(a["invoice_id"], b["invoice_id"]);
    assert_eq!(b["checkout_status"], "reconciliation");
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 1);
    let (status, state) = request(
        f.app("tenant-a"),
        format!("/quotes/{}", f.id),
        "GET",
        serde_json::json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(state["acceptance"], b);
    f.finish().await;
}
#[tokio::test]
async fn owner_put_cannot_fabricate_acceptance_without_an_invoice_receipt() {
    let f = Fixture::new().await;
    for status in ["ACCEPTED", "accepted", "AcCePtEd"] {
        let (response, body) = request(
            f.app("tenant-a"),
            format!("/quotes/{}", f.id),
            "PUT",
            serde_json::json!({"status":status,"expected_updated_at":f.version}),
        )
        .await;
        assert_eq!(
            response,
            StatusCode::CONFLICT,
            "{status} must use the acceptance endpoint"
        );
        assert_eq!(body["reason"], "acceptance_endpoint_required");
        let row: (String, Option<serde_json::Value>) =
            sqlx::query_as("SELECT status,acceptance_receipt FROM quotes WHERE id=$1")
                .bind(&f.id)
                .fetch_one(&f.pool)
                .await
                .unwrap();
        assert_eq!(row.0, "SENT");
        assert!(row.1.is_none());
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM invoices")
            .fetch_one(&f.pool)
            .await
            .unwrap();
        assert_eq!(count, 0);
        assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    }
    f.finish().await;
}

#[tokio::test]
async fn accepted_edits_and_alternate_approval_cannot_reopen_frozen_terms() {
    let f = Fixture::new().await;
    let (status, _) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    let accepted_version = current_version(&f).await;
    for (path, method, body) in [
        (
            format!("/quotes/{}", f.id),
            "PUT",
            serde_json::json!({"expected_updated_at":accepted_version,"status":"SENT","total_amount_cents":1,"line_items":[]}),
        ),
        (
            format!("/quotes/{}/approve", f.id),
            "PATCH",
            serde_json::json!({"expected_updated_at":accepted_version}),
        ),
    ] {
        let (status, result) = request(f.app("tenant-a"), path, method, body).await;
        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(result["reason"], "accepted_quote_is_immutable");
    }
    let (status, total): (String, i64) =
        sqlx::query_as("SELECT status,total_amount_cents FROM quotes")
            .fetch_one(&f.pool)
            .await
            .unwrap();
    assert_eq!((status, total), ("ACCEPTED".into(), 1234));
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 1);
    f.finish().await;
}
#[tokio::test]
async fn old_unbound_or_multiple_invoices_are_not_silently_adopted() {
    let f = Fixture::new().await;
    for index in 0..2 {
        sqlx::query("INSERT INTO invoices(id,tenant_id,customer_id,quote_id,total_amount,currency,status)VALUES($1,'tenant-a','customer-a',$2,12.34,'USD','Draft')").bind(format!("old_{index}")).bind(&f.id).execute(&f.pool).await.unwrap();
        let (status, _) = f.accept().await;
        assert_eq!(status, StatusCode::CONFLICT);
    }
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    f.finish().await;
}
#[tokio::test]
async fn existing_unbound_checkout_never_causes_a_second_provider_session() {
    let mut f = Fixture::new().await;
    sqlx::query("UPDATE quotes SET stripe_payment_link='https://checkout.stripe.com/c/pay/cs_old_unbound' WHERE id=$1").bind(&f.id).execute(&f.pool).await.unwrap();
    f.version = sqlx::query_scalar::<_, DateTime<Utc>>("SELECT updated_at FROM quotes WHERE id=$1")
        .bind(&f.id)
        .fetch_one(&f.pool)
        .await
        .unwrap()
        .to_rfc3339();
    let (status, a) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(a["checkout_status"], "reconciliation");
    assert_eq!(a["stripe_payment_link"], "");
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    f.finish().await;
}
#[tokio::test]
async fn changed_accepted_quote_or_invoice_requires_reconciliation() {
    let f = Fixture::new().await;
    let (status, _) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    sqlx::query("ALTER TABLE quotes DISABLE TRIGGER ohc_quote_acceptance_guard")
        .execute(&f.pool)
        .await
        .unwrap();
    sqlx::query("UPDATE quotes SET total_amount_cents=1235")
        .execute(&f.pool)
        .await
        .unwrap();
    let (status, _) = f.accept().await;
    assert_eq!(status, StatusCode::CONFLICT);
    sqlx::query("UPDATE quotes SET total_amount_cents=1234")
        .execute(&f.pool)
        .await
        .unwrap();
    sqlx::query("ALTER TABLE invoices DISABLE TRIGGER ohc_accepted_invoice_guard")
        .execute(&f.pool)
        .await
        .unwrap();
    sqlx::query("UPDATE invoices SET total_amount=13.34")
        .execute(&f.pool)
        .await
        .unwrap();
    let (status, _) = f.accept().await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 1);
    f.finish().await;
}
#[tokio::test]
async fn authoritative_read_and_replay_preserve_actual_invoice_and_payment_status() {
    let f = Fixture::new().await;
    let (status, a) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(a["invoice_status"], "Draft");
    assert_eq!(a["payment_status"], "unverified");
    sqlx::query("UPDATE invoices SET status='Paid',payment_status='paid'")
        .execute(&f.pool)
        .await
        .unwrap();
    let (status, b) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(b["invoice_id"], a["invoice_id"]);
    assert_eq!(b["invoice_status"], "Paid");
    assert_eq!(b["payment_status"], "paid");
    let (status, state) = request(
        f.app("tenant-a"),
        format!("/quotes/{}?mobile_optimized=true", f.id),
        "GET",
        serde_json::json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(state["quote"]["updated_at"].is_string());
    assert_eq!(state["acceptance"], b);
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 1);
    f.finish().await;
}
#[tokio::test]
async fn missing_checkout_configuration_is_truthful_and_does_not_retry_on_reload() {
    let f = Fixture::new().await;
    unsafe {
        std::env::remove_var("STRIPE_API_KEY");
    }
    let (status, a) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(a["checkout_status"], "not_configured");
    unsafe {
        std::env::set_var(
            "STRIPE_API_KEY",
            "sk_offline_public_fixture_never_a_real_credential",
        );
    }
    let (status, b) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(b["invoice_id"], a["invoice_id"]);
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    f.finish().await;
}
#[tokio::test]
async fn provider_result_write_failure_keeps_the_committed_receipt_for_reconciliation() {
    let f = Fixture::new().await;
    sqlx::raw_sql("CREATE FUNCTION reject_checkout() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN IF NEW.stripe_payment_link IS NOT NULL THEN RAISE EXCEPTION 'forced checkout persistence failure';END IF;RETURN NEW;END$$;CREATE TRIGGER reject_checkout BEFORE UPDATE ON invoices FOR EACH ROW EXECUTE FUNCTION reject_checkout();").execute(&f.pool).await.unwrap();
    let (status, _) = f.accept().await;
    assert!(status.is_server_error());
    let (status, b) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(b["checkout_status"], "reconciliation");
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 1);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM invoices")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
    f.finish().await;
}
#[tokio::test]
async fn signed_line_adjustments_are_preserved_in_exact_minor_units() {
    let mut f = Fixture::new().await;
    sqlx::query("INSERT INTO quote_line_items(id,quote_id,tenant_id,description,unit_price_cents,quantity) VALUES($1,$2,'tenant-a','Reviewed discount',-29,2)").bind(Uuid::new_v4().to_string()).bind(&f.id).execute(&f.pool).await.unwrap();
    f.version = sqlx::query_scalar::<_, DateTime<Utc>>("SELECT updated_at FROM quotes WHERE id=$1")
        .bind(&f.id)
        .fetch_one(&f.pool)
        .await
        .unwrap()
        .to_rfc3339();
    let (status, _) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    let row:(i64,i64)=sqlx::query_as("SELECT unit_price_cents,amount_cents FROM invoice_line_items WHERE description='Reviewed discount'").fetch_one(&f.pool).await.unwrap();
    assert_eq!(row, (-29, -58));
    let (status, _) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    f.finish().await;
}
#[tokio::test]
async fn commit_transport_error_cannot_be_reported_as_success_or_proven_rollback() {
    let response = quote_acceptance::Error::Commit(sqlx::Error::Io(
        std::io::ErrorKind::ConnectionReset.into(),
    ))
    .response();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["success"], false);
    assert_eq!(body["reason"], "commit_outcome_unknown");
    assert_eq!(body["status"], "reconciliation");
}

#[tokio::test]
async fn cancellation_after_provider_claim_survives_without_a_second_attempt() {
    let f = Fixture::new().await;
    PROVIDER_BLOCKED.store(true, Ordering::SeqCst);
    let job = tokio::spawn(request(
        f.app("tenant-a"),
        format!("/quotes/{}/accept", f.id),
        "POST",
        serde_json::json!({"expected_updated_at":f.version}),
    ));
    tokio::time::timeout(std::time::Duration::from_secs(3), async {
        while PROVIDER_CALLS.load(Ordering::SeqCst) == 0 {
            tokio::time::sleep(std::time::Duration::from_millis(5)).await
        }
    })
    .await
    .unwrap();
    job.abort();
    assert!(job.await.unwrap_err().is_cancelled());
    PROVIDER_BLOCKED.store(false, Ordering::SeqCst);
    let (status, body) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["checkout_status"], "reconciliation");
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 1);
    f.finish().await;
}
#[tokio::test]
async fn second_phase_database_failure_preserves_committed_acceptance_and_truthful_reason() {
    let f = Fixture::new().await;
    sqlx::raw_sql("CREATE FUNCTION reject_checkout_claim() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN IF OLD.acceptance_receipt->>'checkout_status'='pending' AND NEW.acceptance_receipt->>'checkout_status'='reconciliation' THEN RAISE EXCEPTION 'forced checkout claim rejection';END IF;RETURN NEW;END$$;CREATE TRIGGER reject_checkout_claim BEFORE UPDATE ON quotes FOR EACH ROW EXECUTE FUNCTION reject_checkout_claim();").execute(&f.pool).await.unwrap();
    let (status, body) = f.accept().await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(body["success"], false);
    assert_eq!(body["status"], "reconciliation");
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    let invoice_count: i64 = sqlx::query_scalar("SELECT count(*) FROM invoices")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(
        invoice_count, 1,
        "acceptance committed before the claim failed"
    );
    let quote_status: String = sqlx::query_scalar("SELECT status FROM quotes")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(quote_status, "ACCEPTED");
    assert_eq!(body["reason"], "database_operation_failed");
    let (replay_status, replay) = f.accept().await;
    assert_eq!(replay_status, StatusCode::OK);
    assert_eq!(replay["checkout_status"], "pending");
    // The failed claim never committed; replay exposes its recorded state without retrying.
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    f.finish().await;
}

#[tokio::test]
async fn deferred_commit_failure_cannot_start_checkout() {
    let f = Fixture::new().await;
    sqlx::raw_sql("CREATE FUNCTION reject_commit() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN RAISE EXCEPTION 'forced deferred rejection';END$$;CREATE CONSTRAINT TRIGGER reject_commit AFTER INSERT ON invoices DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_commit();").execute(&f.pool).await.unwrap();
    let (status, body) = f.accept().await;
    assert!(status.is_server_error());
    assert_eq!(body["success"], false);
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM invoices")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    let status: String = sqlx::query_scalar("SELECT status FROM quotes")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(status, "SENT");
    f.finish().await;
}
#[tokio::test]
async fn forced_rls_preserves_tenant_boundary_and_pool_reuse() {
    let f = Fixture::new().await;
    let role = format!("quote_test_{}", Uuid::new_v4().simple());
    let password = Uuid::new_v4().simple().to_string();
    sqlx::query(&format!(
        "CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS PASSWORD '{password}'"
    ))
    .execute(&f.admin)
    .await
    .unwrap();
    for table in [
        "quotes",
        "quote_line_items",
        "invoices",
        "invoice_line_items",
    ] {
        sqlx::query(&format!("ALTER TABLE {table} FORCE ROW LEVEL SECURITY"))
            .execute(&f.pool)
            .await
            .unwrap();
    }
    sqlx::raw_sql(&format!("GRANT USAGE ON SCHEMA {} TO {role}; GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {} TO {role};",f.schema,f.schema)).execute(&f.admin).await.unwrap();
    let schema = f.schema.clone();
    let scoped = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .after_connect(move |connection, _| {
            let search = format!("SET search_path TO {schema}");
            Box::pin(async move {
                sqlx::query(&search).execute(&mut *connection).await?;
                Ok(())
            })
        })
        .connect_with(
            std::env::var("OHC_QUOTE_TEST_DATABASE_URL")
                .unwrap()
                .parse::<sqlx::postgres::PgConnectOptions>()
                .unwrap()
                .username(&role)
                .password(&password),
        )
        .await
        .unwrap();
    let (session,current,superuser,bypass):(String,String,bool,bool)=sqlx::query_as("SELECT session_user::text,current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&scoped).await.unwrap();
    assert_eq!(session, role);
    assert_eq!(current, role);
    assert!(!superuser && !bypass);
    let claims = |tenant: &str| {
        serde_json::from_value::<server_common::Claims>(serde_json::json!({"sub":"fixture-owner","exp":4000000000u64,"iat":0,"organization_id":tenant})).unwrap()
    };
    let app = |tenant: &str| {
        Router::new()
            .route("/quotes/{id}/accept", post(accept_quote))
            .route("/quotes/{id}", get(get_quote))
            .layer(Extension(claims(tenant)))
            .with_state(scoped.clone())
    };
    let (status, a) = request(
        app("tenant-a"),
        format!("/quotes/{}/accept", f.id),
        "POST",
        serde_json::json!({"expected_updated_at":f.version}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let (status, _) = request(
        app("tenant-b"),
        format!("/quotes/{}", f.id),
        "GET",
        serde_json::json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (status, b) = request(
        app("tenant-a"),
        format!("/quotes/{}/accept", f.id),
        "POST",
        serde_json::json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(a["invoice_id"], b["invoice_id"]);
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 1);
    let (session, current): (String, String) =
        sqlx::query_as("SELECT session_user::text,current_user::text")
            .fetch_one(&scoped)
            .await
            .unwrap();
    assert_eq!(session, role);
    assert_eq!(current, role);
    scoped.close().await;
    sqlx::raw_sql(&format!("DROP OWNED BY {role}; DROP ROLE {role};"))
        .execute(&f.admin)
        .await
        .unwrap();
    f.finish().await;
}

#[tokio::test]
async fn current_customer_tenant_binding_is_checked_before_acceptance() {
    let f = Fixture::new().await;
    sqlx::query("UPDATE customers SET tenant_id='tenant-b' WHERE id='customer-a'")
        .execute(&f.pool)
        .await
        .unwrap();
    let (status, _) = f.accept().await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "an old quote reference must not authorize a new invoice for a now-foreign customer"
    );
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM invoices")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
    f.finish().await;
}

#[tokio::test]
async fn accepted_terms_are_immutable_for_existing_direct_sql_writers() {
    let f = Fixture::new().await;
    assert_eq!(f.accept().await.0, StatusCode::OK);
    for statement in [
        "UPDATE quotes SET status='SENT',updated_at=NOW()",
        "UPDATE quotes SET total_amount_cents=1",
        "UPDATE quote_line_items SET unit_price_cents=1",
        "DELETE FROM quote_line_items",
        "UPDATE invoices SET total_amount=0.01,total_amount_cents=1",
        "UPDATE invoice_line_items SET unit_price=0.01,amount=0.01",
        "DELETE FROM invoice_line_items",
    ] {
        assert!(
            sqlx::query(statement).execute(&f.pool).await.is_err(),
            "accepted economic terms must reject direct writer: {statement}"
        );
    }
    sqlx::query("UPDATE invoices SET status='Paid',payment_status='paid'")
        .execute(&f.pool)
        .await
        .unwrap();
    let (status, receipt) = f.accept().await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(receipt["payment_status"], "paid");
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 1);
    f.finish().await;
}
#[tokio::test]
async fn line_edits_advance_the_review_token_before_first_acceptance() {
    let f = Fixture::new().await;
    sqlx::query("UPDATE quote_line_items SET description='Changed after customer review'")
        .execute(&f.pool)
        .await
        .unwrap();
    let (status, _) = f.accept().await;
    assert_eq!(
        status,
        StatusCode::CONFLICT,
        "an unchanged parent token must not authorize unseen line changes"
    );
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    f.finish().await;
}

async fn checkout_fixture(
    headers: axum::http::HeaderMap,
    axum::extract::Form(form): axum::extract::Form<std::collections::HashMap<String, String>>,
) -> axum::response::Response {
    assert!(
        headers
            .get("authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("Basic ")
    );
    PROVIDER_CALLS.fetch_add(1, Ordering::SeqCst);
    if !headers.contains_key("idempotency-key") {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    let operation = headers
        .get("idempotency-key")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    if operation.starts_with("checkout:") {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    assert!(operation.starts_with("ohc_quote_accept_"));
    OPERATIONS.lock().unwrap().push(operation);
    assert_eq!(form["mode"], "payment");
    assert_eq!(form["line_items[0][price_data][currency]"], "usd");
    assert!(Uuid::parse_str(&form["client_reference_id"]).is_ok());
    let amount: i64 = form["line_items[0][price_data][unit_amount]"]
        .parse()
        .unwrap();
    WIRE_REQUESTS.lock().unwrap().push(WireRequest {
        reference: form["client_reference_id"].clone(),
        amount,
        currency: form["line_items[0][price_data][currency]"].clone(),
    });
    while PROVIDER_BLOCKED.load(Ordering::SeqCst) {
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    }
    if PROVIDER_UNKNOWN.load(Ordering::SeqCst) {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    Json(serde_json::json!({"id":"cs_public_fixture","url":"https://checkout.stripe.com/c/pay/cs_public_fixture","amount_total":amount,"currency":"usd","payment_status":"unpaid"})).into_response()
}

#[tokio::test]
async fn reviewed_revision_before_acceptance_keeps_the_canonical_text_id_schema_working() {
    let mut f = Fixture::new().await;
    let(status,body)=request(f.app("tenant-a"),format!("/quotes/{}",f.id),"PUT",serde_json::json!({"expected_updated_at":f.version,"status":"DRAFT","total_amount_cents":1500,"line_items":[{"description":"New reviewed service","unit_price_cents":500,"quantity":3,"is_optional":false}]})).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let (status, state) = request(
        f.app("tenant-a"),
        format!("/quotes/{}?mobile_optimized=true", f.id),
        "GET",
        serde_json::json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_ne!(state["quote"]["updated_at"], f.version);
    assert_eq!(
        state["line_items"][0]["description"],
        "New reviewed service"
    );
    f.version = state["quote"]["updated_at"].as_str().unwrap().into();
    assert_eq!(f.accept().await.0, StatusCode::OK);
    f.finish().await;
}
#[tokio::test]
async fn parent_deletion_preserves_the_existing_cascade_lifecycle() {
    let f = Fixture::new().await;
    assert_eq!(f.accept().await.0, StatusCode::OK);
    sqlx::query("DELETE FROM tenants WHERE id='tenant-a'")
        .execute(&f.pool)
        .await
        .unwrap();
    let quotes: i64 = sqlx::query_scalar("SELECT count(*) FROM quotes")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    let invoices: i64 = sqlx::query_scalar("SELECT count(*) FROM invoices")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!((quotes, invoices), (0, 0));
    f.finish().await;
}

// These assertions execute the extracted mounted handlers against real PostgreSQL.
// The provider listener and booking-lock sentinel must stay untouched on rejection.
async fn quote_state(f: &Fixture) -> serde_json::Value {
    sqlx::query_scalar("SELECT jsonb_build_object('quote',(SELECT to_jsonb(q) FROM quotes q WHERE id=$1),'lines',(SELECT COALESCE(jsonb_agg(to_jsonb(l) ORDER BY id),'[]'::jsonb) FROM quote_line_items l WHERE quote_id=$1),'invoices',(SELECT COALESCE(jsonb_agg(to_jsonb(i) ORDER BY id),'[]'::jsonb) FROM invoices i))")
        .bind(&f.id).fetch_one(&f.pool).await.unwrap()
}
async fn current_version(f: &Fixture) -> String {
    sqlx::query_scalar::<_, DateTime<Utc>>("SELECT updated_at FROM quotes WHERE id=$1")
        .bind(&f.id)
        .fetch_one(&f.pool)
        .await
        .unwrap()
        .to_rfc3339()
}
fn revision_body(version: serde_json::Value) -> serde_json::Value {
    serde_json::json!({"expected_updated_at":version,"status":"DRAFT","total_amount_cents":1500,"line_items":[{"description":"New reviewed service","unit_price_cents":500,"quantity":3,"is_optional":false}]})
}
async fn record_quote_write_attempts(f: &Fixture) {
    if !sqlx::query_scalar::<_, bool>("SELECT to_regclass('write_attempts') IS NOT NULL")
        .fetch_one(&f.pool)
        .await
        .unwrap()
    {
        sqlx::raw_sql("CREATE SEQUENCE write_attempts;CREATE FUNCTION record_write_attempt() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN PERFORM nextval('write_attempts');IF TG_OP='DELETE' THEN RETURN OLD;END IF;RETURN NEW;END$$;CREATE TRIGGER record_write_attempt BEFORE INSERT OR UPDATE OR DELETE ON quotes FOR EACH ROW EXECUTE FUNCTION record_write_attempt();CREATE TRIGGER record_write_attempt BEFORE INSERT OR UPDATE OR DELETE ON quote_line_items FOR EACH ROW EXECUTE FUNCTION record_write_attempt();").execute(&f.pool).await.unwrap();
    }
}
async fn assert_version_conflict(f: &Fixture, method: &str, body: serde_json::Value) {
    record_quote_write_attempts(f).await;
    let before = quote_state(f).await;
    let attempts: (i64, bool) = sqlx::query_as("SELECT last_value,is_called FROM write_attempts")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    let path = if method == "PUT" {
        format!("/quotes/{}", f.id)
    } else {
        format!("/quotes/{}/approve", f.id)
    };
    let (status, response) = request(f.app("tenant-a"), path, method, body).await;
    assert_eq!(status, StatusCode::CONFLICT, "{method}: {response}");
    assert_eq!(response["reason"], "reviewed_quote_version_required");
    assert_eq!(response["success"], false);
    assert_eq!(
        quote_state(f).await,
        before,
        "rejection must preserve every stored field and line"
    );
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(REDIS_CALLS.load(Ordering::SeqCst), 0);
    let after: (i64, bool) = sqlx::query_as("SELECT last_value,is_called FROM write_attempts")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(
        after, attempts,
        "rejection must precede even rolled-back SQL write attempts"
    );
}
#[tokio::test]
async fn owner_update_requires_current_version_before_database_provider_or_redis_effects() {
    let f = Fixture::new().await;
    sqlx::query("UPDATE quotes SET proposed_slot_id='slot-fixture' WHERE id=$1")
        .bind(&f.id)
        .execute(&f.pool)
        .await
        .unwrap();
    for version in [
        None,
        Some(serde_json::Value::Null),
        Some(serde_json::json!(f.version)),
    ] {
        let mut body = revision_body(version.clone().unwrap_or(serde_json::Value::Null));
        body["status"] = serde_json::json!("SENT");
        if version.is_none() {
            body.as_object_mut().unwrap().remove("expected_updated_at");
        }
        assert_version_conflict(&f, "PUT", body).await;
    }
    f.finish().await;
}
#[tokio::test]
async fn owner_approval_requires_current_version_and_an_optional_empty_body_is_a_conflict() {
    let f = Fixture::new().await;
    for body in [
        serde_json::json!({}),
        serde_json::json!({"expected_updated_at":null}),
        serde_json::json!({"expected_updated_at":"2026-01-01T00:00:00Z"}),
    ] {
        assert_version_conflict(&f, "PATCH", body).await;
    }
    let before = quote_state(&f).await;
    let response = f
        .app("tenant-a")
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/quotes/{}/approve", f.id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
    let body: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body["reason"], "reviewed_quote_version_required");
    assert_eq!(quote_state(&f).await, before);
    f.finish().await;
}
#[tokio::test]
async fn reviewed_version_rejects_changed_status_commercial_terms_and_line_items() {
    let f = Fixture::new().await;
    for change in [
        "UPDATE quotes SET status='DRAFTING' WHERE id=$1",
        "UPDATE quotes SET total_amount_cents=5555,required_deposit_cents=444 WHERE id=$1",
        "UPDATE quote_line_items SET description='Worker changed scope' WHERE quote_id=$1",
    ] {
        let version = current_version(&f).await;
        sqlx::query(change)
            .bind(&f.id)
            .execute(&f.pool)
            .await
            .unwrap();
        assert_ne!(current_version(&f).await, version);
        for method in ["PUT", "PATCH"] {
            assert_version_conflict(&f, method, revision_body(serde_json::json!(version))).await;
        }
    }
    f.finish().await;
}
#[tokio::test]
async fn approval_rejects_ineligible_status_even_with_its_current_version() {
    let f = Fixture::new().await;
    for status in ["DRAFTING", "DECLINED", "CANCELLED", "EXPIRED"] {
        sqlx::query("UPDATE quotes SET status=$2 WHERE id=$1")
            .bind(&f.id)
            .bind(status)
            .execute(&f.pool)
            .await
            .unwrap();
        let before = quote_state(&f).await;
        let (response, body) = request(
            f.app("tenant-a"),
            format!("/quotes/{}/approve", f.id),
            "PATCH",
            serde_json::json!({"expected_updated_at":current_version(&f).await}),
        )
        .await;
        assert_eq!(response, StatusCode::CONFLICT, "{status}: {body}");
        assert_eq!(body["reason"], "quote_not_open_for_approval");
        assert_eq!(quote_state(&f).await, before);
    }
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(REDIS_CALLS.load(Ordering::SeqCst), 0);
    f.finish().await;
}
#[tokio::test]
async fn version_comparison_preserves_microseconds_and_accepts_equivalent_offsets() {
    for method in ["PUT", "PATCH"] {
        let f = Fixture::new().await;
        // Insert a deterministic timestamp directly as initial fixture data, without bypassing a guard.
        sqlx::query("DELETE FROM quotes WHERE id=$1")
            .bind(&f.id)
            .execute(&f.pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO quotes(id,tenant_id,customer_id,status,total_amount_cents,updated_at) VALUES($1,'tenant-a','customer-a','DRAFT',1234,'2026-10-01T12:00:00.123456Z')").bind(&f.id).execute(&f.pool).await.unwrap();
        for stale in [
            "2026-10-01T12:00:00.123455Z",
            "2026-10-01T12:00:00.123457Z",
            "2026-10-01T12:00:00.123Z",
        ] {
            assert_version_conflict(&f, method, revision_body(serde_json::json!(stale))).await;
        }
        let path = if method == "PUT" {
            format!("/quotes/{}", f.id)
        } else {
            format!("/quotes/{}/approve", f.id)
        };
        let (status, body) = request(
            f.app("tenant-a"),
            path,
            method,
            revision_body(serde_json::json!("2026-10-01T17:30:00.123456+05:30")),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{method}: {body}");
        f.finish().await;
    }
}
#[tokio::test]
async fn concurrent_same_version_owner_mutations_have_exactly_one_winner() {
    for methods in [["PUT", "PUT"], ["PATCH", "PATCH"], ["PUT", "PATCH"]] {
        let f = Fixture::new().await;
        let mut jobs = tokio::task::JoinSet::new();
        for method in methods {
            let path = if method == "PUT" {
                format!("/quotes/{}", f.id)
            } else {
                format!("/quotes/{}/approve", f.id)
            };
            jobs.spawn(request(
                f.app("tenant-a"),
                path,
                method,
                revision_body(serde_json::json!(f.version)),
            ));
        }
        let mut successes = 0;
        let mut conflicts = 0;
        while let Some(result) = jobs.join_next().await {
            let (status, body) = result.unwrap();
            match status {
                StatusCode::OK => successes += 1,
                StatusCode::CONFLICT => {
                    conflicts += 1;
                    assert_eq!(body["reason"], "reviewed_quote_version_required");
                }
                _ => panic!("unexpected response {status}: {body}"),
            }
        }
        assert_eq!((successes, conflicts), (1, 1), "{methods:?}");
        assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
        assert_eq!(REDIS_CALLS.load(Ordering::SeqCst), 0);
        f.finish().await;
    }
}
#[tokio::test]
async fn owner_save_returns_final_line_trigger_version_and_that_token_can_approve() {
    let f = Fixture::new().await;
    // Record every actual quote update, including those from deleting/inserting lines.
    sqlx::raw_sql("CREATE TABLE version_audit(seq BIGSERIAL,version TIMESTAMPTZ);CREATE FUNCTION record_version() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN INSERT INTO version_audit(version)VALUES(NEW.updated_at);RETURN NEW;END$$;CREATE TRIGGER record_version AFTER UPDATE ON quotes FOR EACH ROW EXECUTE FUNCTION record_version();").execute(&f.pool).await.unwrap();
    let (status, saved) = request(
        f.app("tenant-a"),
        format!("/quotes/{}", f.id),
        "PUT",
        revision_body(serde_json::json!(f.version)),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{saved}");
    assert_eq!(saved["success"], true);
    let (_, state) = request(
        f.app("tenant-a"),
        format!("/quotes/{}", f.id),
        "GET",
        serde_json::json!({}),
    )
    .await;
    assert!(
        saved["updated_at"].is_string(),
        "PUT must return the final server token"
    );
    assert_eq!(saved["updated_at"], state["quote"]["updated_at"]);
    assert_ne!(saved["updated_at"], f.version);
    let versions: Vec<DateTime<Utc>> =
        sqlx::query_scalar("SELECT version FROM version_audit ORDER BY seq")
            .fetch_all(&f.pool)
            .await
            .unwrap();
    assert_eq!(
        versions.len(),
        3,
        "one parent update, one line delete and one line insert"
    );
    assert!(versions.windows(2).all(|pair| pair[1] > pair[0]));
    assert_eq!(
        DateTime::parse_from_rfc3339(saved["updated_at"].as_str().unwrap())
            .unwrap()
            .with_timezone(&Utc),
        *versions.last().unwrap()
    );
    let (status, approved) = request(
        f.app("tenant-a"),
        format!("/quotes/{}/approve", f.id),
        "PATCH",
        serde_json::json!({"expected_updated_at":saved["updated_at"]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{approved}");
    assert_eq!(approved["quote"]["status"], "SENT");
    assert_ne!(approved["quote"]["updated_at"], saved["updated_at"]);
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(REDIS_CALLS.load(Ordering::SeqCst), 0);
    f.finish().await;
}

#[tokio::test]
async fn owner_put_cannot_approve_an_ineligible_record_with_a_fresh_token() {
    let f = Fixture::new().await;
    sqlx::query("UPDATE quotes SET status='DRAFTING',proposed_slot_id='slot-fixture' WHERE id=$1")
        .bind(&f.id)
        .execute(&f.pool)
        .await
        .unwrap();
    for target in ["SENT", "APPROVED", "sent", "approved"] {
        let before = quote_state(&f).await;
        let mut body = revision_body(serde_json::json!(current_version(&f).await));
        body["status"] = serde_json::json!(target);
        let (status, response) =
            request(f.app("tenant-a"), format!("/quotes/{}", f.id), "PUT", body).await;
        assert_eq!(status, StatusCode::CONFLICT, "{target}: {response}");
        assert_eq!(response["reason"], "quote_not_open_for_approval");
        assert_eq!(quote_state(&f).await, before);
        assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
        assert_eq!(REDIS_CALLS.load(Ordering::SeqCst), 0);
    }
    f.finish().await;
}

#[tokio::test]
async fn owner_mutations_require_the_quote_customer_to_remain_in_the_same_tenant() {
    let f = Fixture::new().await;
    for tenant in ["tenant-b", "tenant-a"] {
        if tenant == "tenant-a" {
            sqlx::query("UPDATE customers SET tenant_id='tenant-b' WHERE id='customer-a'")
                .execute(&f.pool)
                .await
                .unwrap();
        }
        for method in ["PUT", "PATCH"] {
            let before = quote_state(&f).await;
            let path = if method == "PUT" {
                format!("/quotes/{}", f.id)
            } else {
                format!("/quotes/{}/approve", f.id)
            };
            let (status, body) = request(
                f.app(tenant),
                path,
                method,
                revision_body(serde_json::json!(f.version)),
            )
            .await;
            assert_eq!(status, StatusCode::NOT_FOUND, "{tenant}/{method}: {body}");
            assert_eq!(quote_state(&f).await, before);
            assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
            assert_eq!(REDIS_CALLS.load(Ordering::SeqCst), 0);
        }
    }
    f.finish().await;
}
#[tokio::test]
async fn owned_quote_lock_prevents_customer_reassignment_until_the_mutation_finishes() {
    let f = Fixture::new().await;
    let mut tx = f.pool.begin().await.unwrap();
    lock_owned_quote(
        &mut tx,
        &TenantAuthority("tenant-a".into()),
        Uuid::parse_str(&f.id).unwrap(),
    )
    .await
    .unwrap()
    .unwrap();
    let mut competing = f.pool.acquire().await.unwrap();
    sqlx::query("SET lock_timeout='100ms'")
        .execute(&mut *competing)
        .await
        .unwrap();
    let result = sqlx::query("UPDATE customers SET tenant_id='tenant-b' WHERE id='customer-a'")
        .execute(&mut *competing)
        .await;
    let error = result.expect_err("the owned customer must remain SHARE-locked during mutation");
    assert_eq!(
        error
            .as_database_error()
            .and_then(|error| error.code())
            .as_deref(),
        Some("55P03")
    );
    drop(competing);
    tx.rollback().await.unwrap();
    f.finish().await;
}

#[tokio::test]
async fn actual_http_quote_save_and_approval_enforce_the_observed_version() {
    let f = Fixture::new().await;
    sqlx::query("UPDATE quotes SET proposed_slot_id='http-slot-fixture' WHERE id=$1")
        .bind(&f.id)
        .execute(&f.pool)
        .await
        .unwrap();
    // Real loopback HTTP and PostgreSQL, with explicit verified claims instead
    // of a JWT middleware/full-server fixture. The mounted handlers are unchanged.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let quote_url = format!(
        "http://{}/api/v1/quotes/{}",
        listener.local_addr().unwrap(),
        f.id
    );
    let approve_url = format!("{quote_url}/approve");
    let app = Router::new().nest("/api/v1", f.app("tenant-a"));
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let client = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .connect_timeout(std::time::Duration::from_secs(2))
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap();

    let read = client.get(&quote_url).send().await.unwrap();
    assert_eq!(read.status(), StatusCode::OK);
    let observed: serde_json::Value = read.json().await.unwrap();
    let original_version = observed["quote"]["updated_at"].clone();
    assert!(original_version.is_string());
    let save = client
        .put(&quote_url)
        .json(&revision_body(original_version.clone()))
        .send()
        .await
        .unwrap();
    assert_eq!(save.status(), StatusCode::OK);
    let saved: serde_json::Value = save.json().await.unwrap();
    assert_eq!(saved["success"], true);
    assert!(saved["updated_at"].is_string());
    assert_ne!(saved["updated_at"], original_version);
    let readback = client.get(&quote_url).send().await.unwrap();
    assert_eq!(readback.status(), StatusCode::OK);
    let readback: serde_json::Value = readback.json().await.unwrap();
    assert_eq!(saved["updated_at"], readback["quote"]["updated_at"]);
    assert_eq!(readback["quote"]["total_amount_cents"], 1500);
    assert_eq!(
        readback["line_items"][0]["description"],
        "New reviewed service"
    );
    assert_eq!(readback["line_items"][0]["quantity"], 3);

    record_quote_write_attempts(&f).await;
    // A nontransactional sequence records even writes that later roll back.
    // Missing/stale requests must not reach either SQL writes or providers.
    for method in [reqwest::Method::PATCH, reqwest::Method::PUT] {
        for version in [Some(original_version.clone()), None] {
            let before = quote_state(&f).await;
            let attempts: (i64, bool) =
                sqlx::query_as("SELECT last_value,is_called FROM write_attempts")
                    .fetch_one(&f.pool)
                    .await
                    .unwrap();
            let mut body =
                serde_json::json!({"status":"SENT","total_amount_cents":999,"line_items":[]});
            if let Some(version) = version {
                body["expected_updated_at"] = version;
            }
            let url = if method == reqwest::Method::PATCH {
                &approve_url
            } else {
                &quote_url
            };
            let rejected = client
                .request(method.clone(), url)
                .json(&body)
                .send()
                .await
                .unwrap();
            assert_eq!(rejected.status(), StatusCode::CONFLICT, "{method}");
            let rejected: serde_json::Value = rejected.json().await.unwrap();
            assert_eq!(rejected["success"], false);
            assert_eq!(rejected["reason"], "reviewed_quote_version_required");
            assert_eq!(quote_state(&f).await, before);
            let after: (i64, bool) =
                sqlx::query_as("SELECT last_value,is_called FROM write_attempts")
                    .fetch_one(&f.pool)
                    .await
                    .unwrap();
            assert_eq!(
                after, attempts,
                "{method} rejection must precede any SQL mutation"
            );
            assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
            assert_eq!(REDIS_CALLS.load(Ordering::SeqCst), 0);
        }
    }
    let approved = client
        .patch(&approve_url)
        .json(&serde_json::json!({"expected_updated_at":saved["updated_at"]}))
        .send()
        .await
        .unwrap();
    assert_eq!(approved.status(), StatusCode::OK);
    let approved: serde_json::Value = approved.json().await.unwrap();
    assert_eq!(approved["quote"]["status"], "SENT");
    assert_eq!(approved["quote"]["total_amount_cents"], 1500);
    assert!(approved["quote"]["updated_at"].is_string());
    assert_ne!(approved["quote"]["updated_at"], saved["updated_at"]);
    let fresh = client.get(&quote_url).send().await.unwrap();
    assert_eq!(fresh.status(), StatusCode::OK);
    let fresh: serde_json::Value = fresh.json().await.unwrap();
    assert_eq!(fresh["quote"], approved["quote"]);
    assert_eq!(fresh["line_items"], readback["line_items"]);
    assert_eq!(PROVIDER_CALLS.load(Ordering::SeqCst), 0);
    assert_eq!(REDIS_CALLS.load(Ordering::SeqCst), 0);

    drop(client);
    stop.send(()).unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
    f.finish().await;
}
