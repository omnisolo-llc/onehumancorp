//! Actual order admission, mounted worker and adapter-bound dispatch. No live SMS.
use super::*;

async fn subscribe(f: &Fixture, user: usize) {
    let proof = f.verify(user).await;
    assert_eq!(f.request(user,"sms-preferences",Method::POST,json!({"phone":"+14155550123","verification_id":proof,"urgent_booking":false,"failed_payment":false,"new_order":true})).await.0,StatusCode::OK);
}
async fn order(f: &Fixture, id: &str) {
    sqlx::query("INSERT INTO orders(id,tenant_id,status) VALUES($1,'tenant-a','paid')")
        .bind(id)
        .execute(&f.pool)
        .await
        .unwrap();
}
#[tokio::test]
async fn order_transaction_commit_is_required_before_worker_can_send() {
    let f = Fixture::new().await;
    subscribe(&f, 0).await;
    let mut tx = f.pool.begin().await.unwrap();
    sqlx::query("INSERT INTO orders(id,tenant_id,status) VALUES('rolled-back','tenant-a','paid')")
        .execute(&mut *tx)
        .await
        .unwrap();
    let private_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sms_notification_events")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(private_count, 1);
    let visible: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sms_notification_events")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    assert_eq!(visible, 0);
    tx.rollback().await.unwrap();
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 0);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
}
#[tokio::test]
async fn order_worker_retries_pre_provider_outage_after_real_database_reopen_without_duplicates() {
    let mut f = Fixture::new().await;
    subscribe(&f, 0).await;
    f.service.configured = false;
    order(&f, "restart").await;
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 0);
    f.pool.close().await;
    f.pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(4)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(f._directory.path().join("sms.sqlite"))
                .foreign_keys(true),
        )
        .await
        .unwrap();
    f.store = Arc::new(server_auth::Store::with_portable_repo(Arc::new(
        server_auth::seaorm_store::SeaOrmAuthRepository::new(
            sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(f.pool.clone()),
        ),
    )));
    f.service = SmsService::with_provider(
        f.store.clone(),
        f.provider.clone(),
        "+14155550000".into(),
        true,
    );
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 1);
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 0);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 2);
    let receipt = f
        .service
        .dispatch("tenant-a", "restart", "new_order", "Untrusted replacement")
        .await
        .unwrap();
    assert_eq!(receipt.status, "provider_accepted");
    assert_eq!(receipt.provider_message_ids.len(), 1);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 2);
}
#[tokio::test]
async fn absent_foreign_deleted_and_nonadmitted_historical_orders_never_authorize_sms() {
    let f = Fixture::new().await;
    subscribe(&f, 0).await;
    assert!(
        f.service
            .dispatch("tenant-a", "absent", "new_order", "pretend receipt")
            .await
            .is_err()
    );
    order(&f, "foreign").await;
    assert!(
        f.service
            .dispatch("tenant-b", "foreign", "new_order", "pretend receipt")
            .await
            .is_err()
    );
    order(&f, "deleted").await;
    sqlx::query("DELETE FROM orders WHERE id='deleted'")
        .execute(&f.pool)
        .await
        .unwrap();
    assert!(
        f.service
            .dispatch("tenant-a", "deleted", "new_order", "pretend receipt")
            .await
            .is_err()
    );
    sqlx::query("DROP TRIGGER orders_admit_sms")
        .execute(&f.pool)
        .await
        .unwrap();
    order(&f, "historical").await;
    assert!(
        f.service
            .dispatch("tenant-a", "historical", "new_order", "pretend receipt")
            .await
            .is_err()
    );
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
}
#[tokio::test]
async fn order_worker_freezes_audience_before_later_optin_and_keeps_empty_terminal() {
    let f = Fixture::new().await;
    order(&f, "empty").await;
    subscribe(&f, 0).await;
    order(&f, "one").await;
    subscribe(&f, 1).await;
    let before = f.provider.calls.load(Ordering::SeqCst);
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 1);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), before + 1);
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 0);
    let (message,count):(String,i64)=sqlx::query_as("SELECT e.message,(SELECT COUNT(*) FROM sms_notification_dispatches d WHERE d.event_id=e.event_id) FROM sms_notification_events e WHERE e.event_id='one'").fetch_one(&f.pool).await.unwrap();
    assert_eq!(
        message,
        "A new order has been saved. Open OmniSolo to review it."
    );
    assert_eq!(count, 1);
}
#[tokio::test]
async fn order_worker_holds_unknown_rejected_and_interrupted_claims_without_resending() {
    for outcome in [
        MessageSendError::UnknownOutcome {
            reason: "fixture lost acknowledgement",
        },
        MessageSendError::Rejected { status: 400 },
        MessageSendError::OptedOut,
    ] {
        let f = Fixture::new().await;
        subscribe(&f, 0).await;
        subscribe(&f, 1).await;
        order(&f, "failure").await;
        let before = f.provider.calls.load(Ordering::SeqCst);
        *f.provider.outcome.lock().unwrap() = Err(outcome);
        f.service.drain_order_notifications().await.unwrap();
        assert_eq!(
            f.provider.calls.load(Ordering::SeqCst),
            before + 2,
            "one recipient outcome must not starve another frozen recipient"
        );
        *f.provider.outcome.lock().unwrap() = Ok(MessageReceipt {
            sid: "SM11111111111111111111111111111111".into(),
        });
        assert_eq!(f.service.drain_order_notifications().await.unwrap(), 0);
        assert_eq!(f.provider.calls.load(Ordering::SeqCst), before + 2);
        let held:i64=sqlx::query_scalar("SELECT COUNT(*) FROM sms_notification_dispatches WHERE state IN ('unknown','rejected')").fetch_one(&f.pool).await.unwrap();
        assert_eq!(held, 2);
    }
}
#[tokio::test]
async fn cancelled_order_send_keeps_durable_sending_claim_after_restart() {
    let f = Fixture::new().await;
    subscribe(&f, 0).await;
    order(&f, "interrupted").await;
    f.provider.pause.store(true, Ordering::SeqCst);
    let service = f.service.clone();
    let task = tokio::spawn(async move { service.drain_order_notifications().await });
    f.provider.started.notified().await;
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    f.provider.pause.store(false, Ordering::SeqCst);
    let recreated = SmsService::with_provider(
        f.store.clone(),
        f.provider.clone(),
        "+14155550000".into(),
        true,
    );
    assert_eq!(recreated.drain_order_notifications().await.unwrap(), 0);
    let state: String = sqlx::query_scalar(
        "SELECT state FROM sms_notification_dispatches WHERE event_id='interrupted'",
    )
    .fetch_one(&f.pool)
    .await
    .unwrap();
    assert_eq!(state, "sending");
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 2);
}
#[tokio::test]
async fn order_worker_rechecks_current_optout_role_activity_and_exact_verified_phone() {
    for change in [
        "UPDATE sms_notification_preferences SET new_order=FALSE",
        "UPDATE users SET active=FALSE",
        "UPDATE identity_user_roles SET role_name='MEMBER'",
        "UPDATE sms_notification_preferences SET phone='+14155550999'",
        "UPDATE sms_verification_challenges SET state='superseded'",
    ] {
        let f = Fixture::new().await;
        subscribe(&f, 0).await;
        order(&f, "revoked").await;
        sqlx::query(change).execute(&f.pool).await.unwrap();
        assert_eq!(f.service.drain_order_notifications().await.unwrap(), 1);
        assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
        let state: String = sqlx::query_scalar("SELECT state FROM sms_notification_dispatches")
            .fetch_one(&f.pool)
            .await
            .unwrap();
        assert_eq!(state, "cancelled");
    }
}
#[tokio::test]
async fn order_notifications_availability_requires_live_worker_and_installed_admission() {
    let f = Fixture::new().await;
    assert_eq!(
        f.request(0, "sms-preferences", Method::GET, Value::Null)
            .await
            .1["order_notifications_available"],
        false
    );
    let worker = f.service.start_order_notifications();
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        loop {
            if f.request(0, "sms-preferences", Method::GET, Value::Null)
                .await
                .1["order_notifications_available"]
                == true
            {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    sqlx::query("DROP TRIGGER orders_admit_sms")
        .execute(&f.pool)
        .await
        .unwrap();
    assert_eq!(
        f.request(0, "sms-preferences", Method::GET, Value::Null)
            .await
            .1["order_notifications_available"],
        false
    );
    worker.abort();
    let _ = worker.await;
}
#[tokio::test]
async fn order_receipt_read_distinguishes_queue_acceptance_rejection_and_unknown_without_sending() {
    let f = Fixture::new().await;
    subscribe(&f, 0).await;
    order(&f, "receipt").await;
    assert!(
        f.service
            .order_notification_receipt("tenant-b", "receipt")
            .await
            .is_err()
    );
    assert_eq!(
        f.service
            .order_notification_receipt("tenant-a", "receipt")
            .await
            .unwrap()
            .status,
        "queued"
    );
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
    for (state, wanted) in [
        ("sending", "requires_reconciliation"),
        ("unknown", "requires_reconciliation"),
        ("rejected", "provider_rejected"),
        ("accepted", "provider_accepted"),
        ("cancelled", "no_eligible_recipients"),
    ] {
        sqlx::query("UPDATE sms_notification_dispatches SET state=$1,provider_sid='SM11111111111111111111111111111111' WHERE event_id='receipt'").bind(state).execute(&f.pool).await.unwrap();
        assert_eq!(
            f.service
                .order_notification_receipt("tenant-a", "receipt")
                .await
                .unwrap()
                .status,
            wanted
        );
    }
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
}
#[tokio::test]
async fn order_worker_pages_more_than_one_hundred_frozen_recipients_without_duplicates() {
    let f = Fixture::new().await;
    subscribe(&f, 0).await;
    for n in 0..100 {
        let actor = format!("fixture-owner-{n:03}");
        sqlx::query("INSERT INTO users(id,username,email,password_hash,active,tenant_id,created_at,updated_at) VALUES($1,$1,$2,'fixture only',TRUE,'tenant-a',CURRENT_TIMESTAMP,CURRENT_TIMESTAMP)").bind(&actor).bind(format!("{actor}@example.test")).execute(&f.pool).await.unwrap();
        sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position) VALUES($1,'ADMIN','tenant-a',0)").bind(&actor).execute(&f.pool).await.unwrap();
        sqlx::query("INSERT INTO sms_notification_preferences(tenant_id,actor_id,phone,verification_id,new_order) VALUES('tenant-a',$1,'+14155550123',$1,TRUE)").bind(&actor).execute(&f.pool).await.unwrap();
        sqlx::query("INSERT INTO sms_verification_challenges(tenant_id,actor_id,challenge_id,phone,code_mac,state,provider_sid,created_at,expires_at) VALUES('tenant-a',$1,$1,'+14155550123','','verified','SM11111111111111111111111111111111',0,300)").bind(&actor).execute(&f.pool).await.unwrap();
    }
    order(&f, "large").await;
    assert_eq!(
        f.service
            .dispatch("tenant-a", "large", "new_order", "")
            .await
            .unwrap()
            .status,
        "queued"
    );
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 101);
    assert_eq!(
        f.service
            .dispatch("tenant-a", "large", "new_order", "")
            .await
            .unwrap()
            .status,
        "provider_accepted"
    );
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 102);
    let accepted:i64=sqlx::query_scalar("SELECT COUNT(*) FROM sms_notification_dispatches WHERE event_id='large' AND state='accepted'").fetch_one(&f.pool).await.unwrap();
    assert_eq!(accepted, 101);
    f.service
        .dispatch("tenant-a", "large", "new_order", "")
        .await
        .unwrap();
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 102);
}
#[tokio::test]
async fn failed_order_retry_is_durable_and_does_not_starve_later_tenants() {
    let f = Fixture::new().await;
    subscribe(&f, 0).await;
    for n in 0..32 {
        order(&f, &format!("broken-{n:02}")).await;
    }
    sqlx::query("UPDATE sms_notification_events SET message_hash='fixture corrupted receipt'")
        .execute(&f.pool)
        .await
        .unwrap();
    order(&f, "later-valid").await;
    assert!(f.service.drain_order_notifications().await.is_err());
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 1);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 2);
    let waiting:i64=sqlx::query_scalar("SELECT COUNT(*) FROM sms_notification_events WHERE event_id LIKE 'broken-%' AND next_attempt_at>0").fetch_one(&f.pool).await.unwrap();
    assert_eq!(waiting, 32);
    sqlx::query("UPDATE sms_notification_events SET message_hash='ea077c658f64495269b8e6b40d71658395387f2470d40038bdac298f01abcebd',next_attempt_at=0 WHERE event_id LIKE 'broken-%'").execute(&f.pool).await.unwrap();
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 32);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 34);
}
#[tokio::test]
async fn legacy_sqlite_sms_schema_gains_durable_schedule_without_backfill_or_data_loss() {
    let f = Fixture::new().await;
    subscribe(&f, 0).await;
    order(&f, "existing").await;
    sqlx::query("DROP INDEX sms_order_outbox_pending")
        .execute(&f.pool)
        .await
        .unwrap();
    sqlx::query("ALTER TABLE sms_notification_events DROP COLUMN next_attempt_at")
        .execute(&f.pool)
        .await
        .unwrap();
    let database = crate::persistence::AppDatabase::from_connection(
        sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(f.pool.clone()),
    );
    crate::persistence::migration::migrate(&database)
        .await
        .unwrap();
    let (count, due): (i64, i64) =
        sqlx::query_as("SELECT COUNT(*),MIN(next_attempt_at) FROM sms_notification_events")
            .fetch_one(&f.pool)
            .await
            .unwrap();
    assert_eq!((count, due), (1, 0));
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 1);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 2);
}
#[tokio::test]
async fn reservation_failure_in_a_full_discovery_page_does_not_block_other_orders() {
    let f = Fixture::new().await;
    subscribe(&f, 0).await;
    for n in 0..32 {
        order(&f, &format!("blocked-{n:02}")).await;
    }
    order(&f, "z-valid").await;
    sqlx::query("CREATE TRIGGER fail_schedule BEFORE UPDATE OF next_attempt_at ON sms_notification_events WHEN OLD.event_id LIKE 'blocked-%' BEGIN SELECT RAISE(ABORT,'fixture reservation failure'); END").execute(&f.pool).await.unwrap();
    assert!(f.service.drain_order_notifications().await.is_err());
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 1);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 2);
    sqlx::query("DROP TRIGGER fail_schedule")
        .execute(&f.pool)
        .await
        .unwrap();
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 32);
    assert_eq!(f.provider.calls.load(Ordering::SeqCst), 34);
}
#[tokio::test]
async fn concurrent_order_drains_admit_only_one_provider_effect() {
    let f = Fixture::new().await;
    subscribe(&f, 0).await;
    order(&f, "concurrent").await;
    let other = f.service.clone();
    let (a, b) = tokio::join!(
        f.service.drain_order_notifications(),
        other.drain_order_notifications()
    );
    assert!(a.is_ok() || b.is_ok());
    assert_eq!(
        f.provider.calls.load(Ordering::SeqCst),
        2,
        "one verification and one order notification"
    );
    let (state, count): (String, i64) = sqlx::query_as(
        "SELECT MIN(state),COUNT(*) FROM sms_notification_dispatches WHERE event_id='concurrent'",
    )
    .fetch_one(&f.pool)
    .await
    .unwrap();
    assert_eq!((state, count), ("accepted".into(), 1));
    assert_eq!(f.service.drain_order_notifications().await.unwrap(), 0);
}
