use super::*;
const OFFLINE: &str = "/api/v1/sync/offline";
const INTENTS: &str = "/api/v1/sync/operation-intents";
fn quote(id: &str) -> Value {
    json!({"transaction_id":id,"client_mutation_id":id,"product_id":"a","quantity_deducted":0,"mutation_type":"draft_quote","payload":"Prepare customer follow-up quote for completed appointment a"})
}
fn intent(id: &str) -> Value {
    json!({"id":id,"action_type":"draft_invoice","payload":{"appointment_id":"a","customer_id":"customer-a","notes":"Owner completed job"}})
}
async fn send(f: &Fixture, route: &str, token: &str, item: Value) -> (StatusCode, Value) {
    let body = if route == OFFLINE {
        json!({"mutations":[item]})
    } else {
        json!({"intents":[item]})
    };
    f.request("POST", route, token, body, None).await
}
async fn counts(f: &Fixture) -> (i64, i64, i64, i64) {
    sqlx::query_as("SELECT (SELECT count(*) FROM sync_events),(SELECT count(*) FROM applied_client_mutations),(SELECT count(*) FROM department_tasks),(SELECT count(*) FROM operation_intents)").fetch_one(&f.admin).await.unwrap()
}
#[tokio::test]
async fn child_routes_canonical_owner_receipts_persist_exactly_once() {
    let f = Fixture::new(true).await;
    for (route, item) in [(OFFLINE, quote("quote")), (INTENTS, intent("invoice"))] {
        for _ in 0..2 {
            let r = send(&f, route, &f.token, item.clone()).await;
            assert_eq!(r.0, StatusCode::OK, "{r:?}");
            assert_eq!(r.1["outcomes"][0]["status"], "acknowledged", "{r:?}");
            assert_eq!(r.1["outcomes"][0]["route"], route);
        }
        let mut changed = item;
        changed["payload"] = if route == OFFLINE {
            json!("Changed quote")
        } else {
            json!({"notes":"changed"})
        };
        assert_eq!(
            send(&f, route, &f.token, changed).await.1["outcomes"][0]["status"],
            "reconciliation"
        );
    }
    assert_eq!(counts(&f).await, (2, 1, 1, 1));
    f.finish().await;
}
#[tokio::test]
async fn child_routes_current_nonowners_and_downgraded_tokens_never_write() {
    let f = Fixture::new(true).await;
    for (route, item) in [
        (OFFLINE, quote("denied-quote")),
        (INTENTS, intent("denied-intent")),
    ] {
        let r = send(&f, route, &f.member, item).await;
        assert_eq!(r.0, StatusCode::OK, "{r:?}");
        assert_eq!(r.1["outcomes"][0]["status"], "blocked");
    }
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id='owner-a'")
        .execute(&f.admin)
        .await
        .unwrap();
    for (route, item) in [
        (OFFLINE, quote("downgraded-quote")),
        (INTENTS, intent("downgraded-intent")),
    ] {
        assert_eq!(
            send(&f, route, &f.token, item).await.1["outcomes"][0]["status"],
            "blocked"
        );
    }
    assert_eq!(counts(&f).await, (0, 0, 0, 0));
    f.finish().await;
}
#[tokio::test]
async fn child_route_receipt_commit_failure_rolls_back_all_effects() {
    let f = Fixture::new(true).await;
    sqlx::raw_sql("CREATE FUNCTION reject_child_commit() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN RAISE EXCEPTION 'synthetic deferred failure';END$$;CREATE CONSTRAINT TRIGGER reject_child_commit AFTER INSERT ON sync_events DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_child_commit();").execute(&f.admin).await.unwrap();
    for (route, item) in [
        (OFFLINE, quote("rollback-quote")),
        (INTENTS, intent("rollback-intent")),
    ] {
        let r = send(&f, route, &f.token, item).await;
        assert_eq!(r.1["outcomes"][0]["status"], "blocked", "{r:?}");
    }
    assert_eq!(counts(&f).await, (0, 0, 0, 0));
    f.finish().await;
}
#[tokio::test]
async fn child_routes_revocation_while_effect_waits_rolls_back_and_blocks_replay() {
    for (route, item, table) in [
        (OFFLINE, quote("revoke-quote"), "department_tasks"),
        (INTENTS, intent("revoke-intent"), "operation_intents"),
    ] {
        let f = Fixture::new(true).await;
        sqlx::raw_sql(&format!("CREATE FUNCTION hold_child_effect() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN PERFORM pg_advisory_xact_lock(71893422);RETURN NEW;END$$;CREATE TRIGGER hold_child_effect BEFORE INSERT ON {table} FOR EACH ROW EXECUTE FUNCTION hold_child_effect();")).execute(&f.admin).await.unwrap();
        let app = crate::actual_mount(
            Arc::new(crate::db::DB {
                pool: f.pool.clone(),
            }),
            f.auth.clone(),
        )
        .await;
        let mut gate = f.admin.begin().await.unwrap();
        sqlx::query("SELECT pg_advisory_xact_lock(71893422)")
            .execute(&mut *gate)
            .await
            .unwrap();
        let token = f.token.clone();
        let body = if route == OFFLINE {
            json!({"mutations":[item.clone()]})
        } else {
            json!({"intents":[item.clone()]})
        };
        let pending = tokio::spawn(async move {
            app.oneshot(
                Request::post(route)
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {token}"))
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap()
        });
        tokio::time::timeout(Duration::from_millis(800),async {loop{let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_locks WHERE locktype='advisory' AND objid=71893422 AND NOT granted)").fetch_one(&f.admin).await.unwrap();if waiting{break;}tokio::time::sleep(Duration::from_millis(5)).await;}}).await.expect("write must reach effect before revocation");
        let claims = f.auth.validate_token(&f.token).await.unwrap();
        f.auth
            .revoke_token(
                claims.jti,
                DateTime::from_timestamp(claims.exp, 0).unwrap(),
                "tenant-a",
            )
            .await
            .unwrap();
        gate.commit().await.unwrap();
        let r = pending.await.unwrap();
        let value: Value =
            serde_json::from_slice(&to_bytes(r.into_body(), 1048576).await.unwrap()).unwrap();
        assert_eq!(value["outcomes"][0]["status"], "blocked");
        assert_eq!(
            value["outcomes"][0]["reason"],
            "current_owner_authority_required"
        );
        assert_eq!(counts(&f).await, (0, 0, 0, 0));
        assert_eq!(
            send(&f, route, &f.token, item).await.0,
            StatusCode::UNAUTHORIZED
        );
        f.finish().await;
    }
}
#[tokio::test]
async fn child_routes_expiry_before_commit_discards_receipts_and_effects() {
    for (route, item, table) in [
        (OFFLINE, quote("expire-quote"), "department_tasks"),
        (INTENTS, intent("expire-intent"), "operation_intents"),
    ] {
        let f = Fixture::new(true).await;
        let mut claims = f.auth.validate_token(&f.token).await.unwrap();
        claims.exp = Utc::now().timestamp() + 2;
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(
                std::env::var("JWT_SECRET").unwrap().as_bytes(),
            ),
        )
        .unwrap();
        sqlx::raw_sql(&format!("CREATE FUNCTION delay_child_effect() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN PERFORM pg_sleep(2.1);RETURN NEW;END$$;CREATE TRIGGER delay_child_effect AFTER INSERT ON {table} FOR EACH ROW EXECUTE FUNCTION delay_child_effect();")).execute(&f.admin).await.unwrap();
        let r = send(&f, route, &token, item).await;
        assert_eq!(r.1["outcomes"][0]["status"], "blocked", "{r:?}");
        assert_eq!(
            r.1["outcomes"][0]["reason"],
            "current_owner_authority_required"
        );
        assert_eq!(counts(&f).await, (0, 0, 0, 0));
        f.finish().await;
    }
}
#[tokio::test]
async fn child_routes_reject_alternate_configured_namespace() {
    let f = Fixture::new(true).await;
    let alternate = format!("child_alternate_{}", Uuid::new_v4().simple());
    sqlx::raw_sql(&format!("CREATE SCHEMA {alternate};CREATE TABLE {alternate}.operation_intents (LIKE {}.operation_intents INCLUDING ALL);",f.schema)).execute(&f.admin).await.unwrap();
    let raw = std::env::var("OHC_FIELD_TEST_DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .connect_with(
            target(&raw).options([("search_path", format!("{alternate},{}", f.schema).as_str())]),
        )
        .await
        .unwrap();
    let app = crate::actual_mount(
        Arc::new(crate::db::DB { pool: pool.clone() }),
        f.auth.clone(),
    )
    .await;
    for (route, body, status) in [
        (INTENTS, json!({"intents":[intent("unproven")]}), "blocked"),
        (OFFLINE, json!({"mutations":[quote("proven")]}), "blocked"),
    ] {
        let r = app
            .clone()
            .oneshot(
                Request::post(route)
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", f.token))
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let value: Value =
            serde_json::from_slice(&to_bytes(r.into_body(), 1048576).await.unwrap()).unwrap();
        assert_eq!(value["outcomes"][0]["status"], status);
    }
    assert_eq!(counts(&f).await, (0, 0, 0, 0));
    pool.close().await;
    sqlx::query(&format!("DROP SCHEMA {alternate} CASCADE"))
        .execute(&f.admin)
        .await
        .unwrap();
    f.finish().await;
}
#[tokio::test]
async fn child_routes_inventory_replay_and_foreign_row_boundaries_survive_owner_fencing() {
    let f = Fixture::new(true).await;
    sqlx::query("INSERT INTO products(id,tenant_id)VALUES('p','tenant-a'),('foreign','tenant-b')")
        .execute(&f.admin)
        .await
        .unwrap();
    let mut sale = json!({
        "transaction_id": "stock",
        "client_mutation_id": "stock",
        "product_id": "p",
        "quantity_deducted": 3,
        "mutation_type": "inventory_sale",
        "payload": null
    });
    // Quote prose is not inventory evidence. Reject that malformed identity
    // without reserving its receipt, then accept the explicit inventory request.
    let mut malformed = sale.clone();
    malformed["payload"] = quote("stock")["payload"].clone();
    let rejected = send(&f, OFFLINE, &f.token, malformed).await;
    assert_eq!(rejected.0, StatusCode::OK, "{rejected:?}");
    assert_eq!(rejected.1["outcomes"][0]["status"], "blocked");
    assert_eq!(
        rejected.1["outcomes"][0]["reason"],
        "explicit_consistent_operation_required"
    );
    assert_eq!(counts(&f).await, (0, 0, 0, 0));
    for _ in 0..2 {
        let accepted = send(&f, OFFLINE, &f.token, sale.clone()).await;
        assert_eq!(accepted.0, StatusCode::OK, "{accepted:?}");
        assert_eq!(
            accepted.1["outcomes"][0]["status"],
            "acknowledged",
            "{accepted:?}"
        );
    }
    assert_eq!(
        sqlx::query_scalar::<_, i32>("SELECT inventory_count FROM products WHERE id='p'")
            .fetch_one(&f.admin)
            .await
            .unwrap(),
        7
    );
    sale["transaction_id"] = json!("foreign");
    sale["client_mutation_id"] = json!("foreign");
    sale["product_id"] = json!("foreign");
    assert_eq!(
        send(&f, OFFLINE, &f.token, sale).await.1["outcomes"][0]["status"],
        "blocked"
    );
    assert_eq!(
        sqlx::query_scalar::<_, i32>("SELECT inventory_count FROM products WHERE id='foreign'")
            .fetch_one(&f.admin)
            .await
            .unwrap(),
        10
    );
    assert_eq!(counts(&f).await, (1, 1, 0, 0));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM ohc_job_queue")
            .fetch_one(&f.admin)
            .await
            .unwrap(),
        1
    );
    f.finish().await;
}
#[tokio::test]
async fn child_routes_missing_or_malformed_signed_auth_cannot_acknowledge() {
    let f = Fixture::new(true).await;
    for token in ["", "unsigned"] {
        for (route, item) in [(OFFLINE, quote("unsigned")), (INTENTS, intent("unsigned"))] {
            assert_eq!(
                send(&f, route, token, item).await.0,
                StatusCode::UNAUTHORIZED
            );
        }
    }
    assert_eq!(counts(&f).await, (0, 0, 0, 0));
    f.finish().await;
}
#[tokio::test]
async fn actual_completion_receipt_releases_both_exact_quote_child_routes_once() {
    let f = Fixture::new(true).await;
    let completion = json!({"id":"completion-chain","entity_id":"a","entity_type":"appointment","action_type":"UpdateStatus","base_version":1,"payload":{"status":"Completed","expected_status":"Scheduled","expected_updated_at":"2026-10-03T00:00:00Z","expected_notes":"stored","notes":"Requested estimate"}});
    let parent = f
        .request(
            "POST",
            "/api/v1/sync/events",
            &f.token,
            json!({"events":[completion]}),
            None,
        )
        .await;
    assert_eq!(
        parent.1["outcomes"][0],
        json!({"id":"completion-chain","route":"/api/v1/sync/events","status":"acknowledged","result_version":2})
    );
    // Exact new fieldCompletionFollowups + planRoutes payloads, with one stable
    // child ID shared by its two independently scoped receipt routes.
    let id = "field-completion-completion-chain-quote";
    let notes = "Follow up quote requested by field op for job a. Notes: Requested estimate";
    let operation = json!({"id":id,"action_type":"draft_quote","payload":{"notes":notes},"timestamp":"2026-10-03T00:00:01Z"});
    let mutation = json!({"timestamp":"2026-10-03T00:00:01Z","transaction_id":id,"quantity_deducted":0,"amount":null,"payment_method":null,"payment_intent_id":null,"currency":"usd","product_id":"draft_quote","mutation_type":"draft_quote","payload":notes});
    for _ in 0..2 {
        for (route, item) in [(INTENTS, operation.clone()), (OFFLINE, mutation.clone())] {
            let r = send(&f, route, &f.token, item).await;
            assert_eq!(r.0, StatusCode::OK, "{r:?}");
            assert_eq!(r.1["success"], true);
            assert_eq!(
                r.1["outcomes"][0],
                json!({"id":id,"route":route,"status":"acknowledged"})
            );
        }
    }
    assert_eq!(counts(&f).await, (3, 1, 2, 1));
    let payload: Value = sqlx::query_scalar(
        "SELECT payload FROM operation_intents WHERE id=$1 AND tenant_id='tenant-a'",
    )
    .bind(id)
    .fetch_one(&f.admin)
    .await
    .unwrap();
    assert_eq!(payload, json!({"notes":notes}));
    let task:Value=sqlx::query_scalar("SELECT payload FROM department_tasks WHERE tenant_id='tenant-a' AND event_type='tenant.omnichannel.message.received'").fetch_one(&f.admin).await.unwrap();
    assert_eq!(task["message"], notes);
    assert_eq!(task["client_mutation_id"], id);
    let claims = f.auth.validate_token(&f.token).await.unwrap();
    f.auth
        .revoke_token(
            claims.jti,
            DateTime::from_timestamp(claims.exp, 0).unwrap(),
            "tenant-a",
        )
        .await
        .unwrap();
    for (route, item) in [(INTENTS, operation), (OFFLINE, mutation)] {
        assert_eq!(
            send(&f, route, &f.token, item).await.0,
            StatusCode::UNAUTHORIZED
        );
    }
    assert_eq!(counts(&f).await, (3, 1, 2, 1));
    f.finish().await;
}

#[tokio::test]
async fn child_routes_prove_independent_configured_pool_and_write_canonically() {
    let f = Fixture::new(true).await;
    assert!(!std::ptr::eq(f.pool.options(), f.admin.options()));
    let app = crate::actual_mount(
        Arc::new(crate::db::DB {
            pool: f.admin.clone(),
        }),
        f.auth.clone(),
    )
    .await;
    for (route, body) in [
        (INTENTS, json!({"intents":[intent("proved-intent")]})),
        (OFFLINE, json!({"mutations":[quote("proved-quote")]})),
    ] {
        let r = app
            .clone()
            .oneshot(
                Request::post(route)
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", f.token))
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let value: Value =
            serde_json::from_slice(&to_bytes(r.into_body(), 1048576).await.unwrap()).unwrap();
        assert_eq!(value["outcomes"][0]["status"], "acknowledged", "{value:?}");
    }
    assert_eq!(counts(&f).await, (2, 1, 1, 1));
    f.finish().await;
}
