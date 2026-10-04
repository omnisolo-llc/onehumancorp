//! Clock writes exercise the actual mounted handler and real canonical stores.
//! Baseline defects must fail assertions, never merely compile against new APIs.
#[path = "staff_timecards_test/fixture.rs"]
pub(crate) mod fixture;
use axum::http::StatusCode;
use chrono::DateTime;
use fixture::{Backend, Fixture, OWNER, STAMP, TENANT, assert_ack, clock, event, identity};
use futures::FutureExt;
use serde_json::json;

macro_rules! on_both_stores {
    ($sqlite:ident, $postgres:ident, $scenario:ident) => {
        #[tokio::test]
        async fn $sqlite() {
            let fixture = Fixture::new(Backend::Sqlite).await;
            let outcome = std::panic::AssertUnwindSafe($scenario(&fixture))
                .catch_unwind()
                .await;
            fixture.finish().await;
            if let Err(panic) = outcome {
                std::panic::resume_unwind(panic);
            }
        }
        #[tokio::test]
        async fn $postgres() {
            let fixture = Fixture::new(Backend::Postgres).await;
            let outcome = std::panic::AssertUnwindSafe($scenario(&fixture))
                .catch_unwind()
                .await;
            fixture.finish().await;
            if let Err(panic) = outcome {
                std::panic::resume_unwind(panic);
            }
        }
    };
}

// Removing the event write, receipt identity, or per-ID acknowledgment must fail.
async fn committed_self_clock(f: &Fixture) {
    let action = clock("self-clock");
    let response = f.post(vec![action.clone()]).await;
    let rows = f.saved().await;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].id, "self-clock");
    assert_eq!(rows[0].staff, OWNER); // No ohc_staff_member row exists for this signed actor.
    assert_eq!(rows[0].kind, "CLOCK_IN");
    assert_eq!(
        rows[0].instant.timestamp_micros(),
        DateTime::parse_from_rfc3339(STAMP)
            .unwrap()
            .timestamp_micros()
    );
    assert_eq!(rows[0].identity, Some(identity(&action, OWNER)));
    assert_ack(&response, &["self-clock"]);
}
on_both_stores!(
    sqlite_self_clock_has_effect_and_receipt,
    postgres_self_clock_has_effect_and_receipt,
    committed_self_clock
);

async fn owned_staff_clock(f: &Fixture) {
    let action = event("staff-clock", "owned-staff", "CLOCK_OUT", STAMP);
    let response = f.post(vec![action.clone()]).await;
    let rows = f.saved().await;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].staff, "owned-staff");
    assert_eq!(rows[0].identity, Some(identity(&action, OWNER)));
    assert_ack(&response, &["staff-clock"]);
}
on_both_stores!(
    sqlite_owner_can_clock_owned_staff,
    postgres_owner_can_clock_owned_staff,
    owned_staff_clock
);

// A duplicate must confirm the same persisted effect without writing it again.
async fn exact_replay(f: &Fixture) {
    let first = f.post(vec![clock("exact-clock")]).await;
    let second = f.post(vec![clock("exact-clock")]).await;
    assert_eq!(f.count().await, 1);
    assert_ack(&first, &["exact-clock"]);
    assert_ack(&second, &["exact-clock"]);
}
on_both_stores!(
    sqlite_exact_clock_replay_is_acknowledged_once,
    postgres_exact_clock_replay_is_acknowledged_once,
    exact_replay
);

async fn concurrent_exact_replay(f: &Fixture) {
    let (first, second) = tokio::join!(
        f.post(vec![clock("concurrent-clock")]),
        f.post(vec![clock("concurrent-clock")])
    );
    assert_eq!(f.count().await, 1);
    assert_ack(&first, &["concurrent-clock"]);
    assert_ack(&second, &["concurrent-clock"]);
}
on_both_stores!(
    sqlite_concurrent_replay_has_one_effect,
    postgres_concurrent_replay_has_one_effect,
    concurrent_exact_replay
);

async fn conflicting_id_rolls_back_batch(f: &Fixture) {
    f.post(vec![clock("zz-existing")]).await;
    let response = f
        .post(vec![
            clock("aa-new"),
            event("zz-existing", OWNER, "CLOCK_OUT", STAMP),
        ])
        .await;
    assert_eq!(response.0, StatusCode::CONFLICT, "{response:?}");
    let rows = f.saved().await;
    assert_eq!(
        rows.len(),
        1,
        "a conflicting second event must roll back the first insert"
    );
    assert_eq!(rows[0].id, "zz-existing");
    assert_eq!(rows[0].kind, "CLOCK_IN");
}
on_both_stores!(
    sqlite_conflicting_clock_id_rolls_back_batch,
    postgres_conflicting_clock_id_rolls_back_batch,
    conflicting_id_rolls_back_batch
);

// Matching receipt text is insufficient when its business effect was changed.
async fn tampered_effect_is_not_a_replay(f: &Fixture) {
    let action = clock("tampered-clock");
    f.post(vec![action.clone()]).await;
    let proof = identity(&action, OWNER).to_string();
    f.execute(&format!("UPDATE ohc_timecard_event SET request_identity='{proof}',event_type='CLOCK_OUT' WHERE id='tampered-clock'")).await;
    let response = f.post(vec![action]).await;
    assert_eq!(
        response.0,
        StatusCode::CONFLICT,
        "a matching identity cannot acknowledge a different stored effect: {response:?}"
    );
    assert_eq!(
        f.saved().await[0].kind,
        "CLOCK_OUT",
        "a conflicting replay must not repair or overwrite history"
    );
}
on_both_stores!(
    sqlite_replay_verifies_effect_beside_identity,
    postgres_replay_verifies_effect_beside_identity,
    tampered_effect_is_not_a_replay
);

async fn changed_request_identity_is_rejected(f: &Fixture) {
    let original = event("same-id", "owned-staff", "CLOCK_IN", STAMP);
    f.post(vec![original.clone()]).await;
    for changed in [
        event("same-id", OWNER, "CLOCK_IN", STAMP),
        event(
            "same-id",
            "owned-staff",
            "CLOCK_IN",
            "2026-10-04T06:01:00.123456Z",
        ),
        event(
            "same-id",
            "owned-staff",
            "CLOCK_IN",
            "2026-10-04T11:30:00.123456+05:30",
        ),
    ] {
        let response = f.post(vec![changed]).await;
        assert_eq!(response.0, StatusCode::CONFLICT, "{response:?}");
    }
    let other = f.token_for("other-owner", TENANT, "ADMIN");
    let response = f.post_token(vec![original], &other).await;
    assert_eq!(
        response.0,
        StatusCode::CONFLICT,
        "a different actor cannot reuse an event receipt: {response:?}"
    );
    assert_eq!(f.count().await, 1);
}
on_both_stores!(
    sqlite_clock_receipt_identity_is_immutable,
    postgres_clock_receipt_identity_is_immutable,
    changed_request_identity_is_rejected
);

async fn legacy_row_is_not_a_receipt(f: &Fixture) {
    f.execute("INSERT INTO ohc_timecard_event(id,tenant_id,staff_id,event_type,event_time) VALUES ('legacy-clock','clock-tenant','clock-owner','CLOCK_IN','2026-10-04T06:00:00.123456Z')").await;
    let response = f.post(vec![clock("legacy-clock")]).await;
    assert_eq!(
        response.0,
        StatusCode::CONFLICT,
        "legacy history is not proof that this actor/request was acknowledged: {response:?}"
    );
    assert!(
        f.saved().await[0].identity.is_none(),
        "never manufacture a receipt for legacy history"
    );
}
on_both_stores!(
    sqlite_legacy_clock_requires_review,
    postgres_legacy_clock_requires_review,
    legacy_row_is_not_a_receipt
);

async fn second_database_error_rolls_back(f: &Fixture) {
    f.fail_second_write().await;
    let response = f.post(vec![clock("aa-first"), clock("zz-fail")]).await;
    assert!(
        !response.0.is_success(),
        "failed storage must never return success: {response:?}"
    );
    assert_eq!(
        f.count().await,
        0,
        "the whole clock batch must roll back on the second write failure"
    );
}
on_both_stores!(
    sqlite_failed_second_clock_rolls_back_first,
    postgres_failed_second_clock_rolls_back_first,
    second_database_error_rolls_back
);

async fn microsecond_and_offset_are_preserved(f: &Fixture) {
    let timestamp = "2026-10-04T11:30:00.123456+05:30";
    let action = event("offset-clock", OWNER, "CLOCK_IN", timestamp);
    let response = f.post(vec![action.clone()]).await;
    let rows = f.saved().await;
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].instant.timestamp_micros(),
        DateTime::parse_from_rfc3339(timestamp)
            .unwrap()
            .timestamp_micros(),
        "timezone offset and microseconds must survive storage exactly"
    );
    assert_eq!(
        rows[0].identity,
        Some(identity(&action, OWNER)),
        "the receipt must preserve the original spelling"
    );
    assert_ack(&response, &["offset-clock"]);
}
on_both_stores!(
    sqlite_clock_offset_preserves_exact_microseconds,
    postgres_clock_offset_preserves_exact_microseconds,
    microsecond_and_offset_are_preserved
);

async fn nonrepresentable_instant_rejects_whole_batch(f: &Fixture) {
    let response = f
        .post(vec![
            clock("aa-valid"),
            event(
                "zz-submicrosecond",
                OWNER,
                "CLOCK_OUT",
                "2026-10-04T06:01:00.123456789Z",
            ),
        ])
        .await;
    assert_eq!(
        response.0,
        StatusCode::BAD_REQUEST,
        "both stores reject precision PostgreSQL cannot represent: {response:?}"
    );
    assert_eq!(
        f.count().await,
        0,
        "precision rejection must happen before any batch write"
    );
}
on_both_stores!(
    sqlite_submicrosecond_clock_is_atomic_rejection,
    postgres_submicrosecond_clock_is_atomic_rejection,
    nonrepresentable_instant_rejects_whole_batch
);

async fn invalid_clock_timestamp(f: &Fixture) {
    for timestamp in ["not-a-time", "2026-10-04T06:00:00", "2026-10-04T06:00:60Z"] {
        let response = f
            .post(vec![
                clock("aa-valid"),
                event("zz-invalid", OWNER, "CLOCK_OUT", timestamp),
            ])
            .await;
        assert_eq!(
            response.0,
            StatusCode::BAD_REQUEST,
            "{timestamp}: {response:?}"
        );
        assert_eq!(f.count().await, 0);
    }
}
on_both_stores!(
    sqlite_invalid_clock_timestamp_has_no_effect,
    postgres_invalid_clock_timestamp_has_no_effect,
    invalid_clock_timestamp
);

async fn unsafe_receipt_ids(f: &Fixture) {
    for id in [
        "",
        ".",
        "..",
        "../clock",
        "clock/id",
        "clock%2Fid",
        "clock?id",
        " clock",
        "clock\n",
        "时钟",
    ] {
        let response = f.post(vec![clock(id)]).await;
        assert_eq!(
            response.0,
            StatusCode::BAD_REQUEST,
            "unsafe receipt ID {id:?}: {response:?}"
        );
        assert_eq!(f.count().await, 0);
    }
    let response = f.post(vec![clock(&"x".repeat(129))]).await;
    assert_eq!(response.0, StatusCode::BAD_REQUEST);
    assert_eq!(f.count().await, 0);
}
on_both_stores!(
    sqlite_clock_ids_are_receipt_addressable,
    postgres_clock_ids_are_receipt_addressable,
    unsafe_receipt_ids
);

async fn batch_shape_rejection(f: &Fixture) {
    for events in [
        vec![],
        vec![clock("duplicate"), clock("duplicate")],
        (0..101).map(|n| clock(&format!("clock-{n}"))).collect(),
    ] {
        let response = f.post(events).await;
        assert_eq!(
            response.0,
            StatusCode::BAD_REQUEST,
            "invalid batch shape: {response:?}"
        );
        assert_eq!(f.count().await, 0);
    }
}
on_both_stores!(
    sqlite_invalid_clock_batch_is_rejected,
    postgres_invalid_clock_batch_is_rejected,
    batch_shape_rejection
);

async fn unsupported_event_type(f: &Fixture) {
    for kind in ["", "CLOCK_BREAK", "clock_in", " CLOCK_IN"] {
        let response = f
            .post(vec![
                clock("aa-valid"),
                event("zz-invalid", OWNER, kind, STAMP),
            ])
            .await;
        assert_eq!(response.0, StatusCode::BAD_REQUEST, "{response:?}");
        assert_eq!(f.count().await, 0);
    }
}
on_both_stores!(
    sqlite_only_clock_event_types_are_writable,
    postgres_only_clock_event_types_are_writable,
    unsupported_event_type
);

async fn subject_authority(f: &Fixture) {
    for staff in ["foreign-staff", "missing-staff", "foreign-owner"] {
        let response = f
            .post(vec![
                clock("aa-valid"),
                event("zz-foreign", staff, "CLOCK_IN", STAMP),
            ])
            .await;
        assert_eq!(
            response.0,
            StatusCode::FORBIDDEN,
            "clock subject must be self or a verified same-tenant staff record: {response:?}"
        );
        assert_eq!(f.count().await, 0);
    }
}
on_both_stores!(
    sqlite_clock_subject_requires_same_tenant_authority,
    postgres_clock_subject_requires_same_tenant_authority,
    subject_authority
);

async fn current_owner_role_is_required(f: &Fixture) {
    f.execute("UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id='clock-owner'")
        .await;
    let response = f.post(vec![clock("stale-owner")]).await;
    assert_eq!(
        response.0,
        StatusCode::FORBIDDEN,
        "signed historical OWNER role cannot replace current canonical role: {response:?}"
    );
    assert_eq!(f.count().await, 0);
}
on_both_stores!(
    sqlite_clock_requires_current_owner_role,
    postgres_clock_requires_current_owner_role,
    current_owner_role_is_required
);

async fn member_cannot_clock_by_claiming_staff(f: &Fixture) {
    let token = f.token_for("member", TENANT, "STAFF");
    let response = f
        .post_token(
            vec![event("member-clock", "member", "CLOCK_IN", STAMP)],
            &token,
        )
        .await;
    assert_eq!(
        response.0,
        StatusCode::FORBIDDEN,
        "this repair cannot invent delegated STAFF write authority: {response:?}"
    );
    assert_eq!(f.count().await, 0);
}
on_both_stores!(
    sqlite_staff_role_is_not_owner_authority,
    postgres_staff_role_is_not_owner_authority,
    member_cannot_clock_by_claiming_staff
);

async fn revoked_session_cannot_write(f: &Fixture) {
    let claims = f.auth.validate_token(&f.token).await.unwrap();
    f.auth
        .revoke_token(
            claims.jti,
            DateTime::from_timestamp(claims.exp, 0).unwrap(),
            TENANT,
        )
        .await
        .unwrap();
    let response = f.post(vec![clock("revoked-clock")]).await;
    assert_eq!(response.0, StatusCode::UNAUTHORIZED);
    assert_eq!(f.count().await, 0);
}
on_both_stores!(
    sqlite_revoked_clock_session_is_rejected,
    postgres_revoked_clock_session_is_rejected,
    revoked_session_cannot_write
);

async fn missing_storage_is_not_success(f: &Fixture) {
    f.execute("DROP TABLE ohc_timecard_event").await;
    let response = f.post(vec![clock("unavailable-clock")]).await;
    assert!(
        !response.0.is_success(),
        "missing persistence cannot acknowledge a clock event: {response:?}"
    );
    assert_ne!(response.1.get("success"), Some(&json!(true)));
}
on_both_stores!(
    sqlite_missing_clock_storage_is_not_success,
    postgres_missing_clock_storage_is_not_success,
    missing_storage_is_not_success
);

// Real database trigger changes the canonical role after the handler's initial
// authentication. Only the owner transaction's commit fence can catch this.
async fn role_change_before_commit_rolls_back(f: &Fixture) {
    f.execute(match f.backend {
        Backend::Sqlite => "CREATE TRIGGER revoke_clock_role AFTER INSERT ON ohc_timecard_event BEGIN UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id='clock-owner'; END",
        Backend::Postgres => "CREATE FUNCTION revoke_clock_role() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id='clock-owner'; RETURN NEW; END $$; CREATE TRIGGER revoke_clock_role AFTER INSERT ON ohc_timecard_event FOR EACH ROW EXECUTE FUNCTION revoke_clock_role()",
    }).await;
    let response = f.post(vec![clock("role-fenced-clock")]).await;
    assert!(
        !response.0.is_success(),
        "clock commit must recheck canonical authority after writes: {response:?}"
    );
    assert_eq!(
        f.count().await,
        0,
        "lost current owner role must roll back the event and its receipt together"
    );
}
on_both_stores!(
    sqlite_clock_rechecks_role_at_commit,
    postgres_clock_rechecks_role_at_commit,
    role_change_before_commit_rolls_back
);

async fn deactivated_owner(f: &Fixture) {
    f.execute("UPDATE users SET active=FALSE WHERE id='clock-owner'")
        .await;
    let response = f.post(vec![clock("inactive-clock")]).await;
    assert!(
        matches!(response.0, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN),
        "{response:?}"
    );
    assert_eq!(f.count().await, 0);
}
on_both_stores!(
    sqlite_inactive_owner_cannot_clock,
    postgres_inactive_owner_cannot_clock,
    deactivated_owner
);

async fn expired_owner_token(f: &Fixture) {
    let response = f
        .post_token(vec![clock("expired-clock")], &f.expired_token().await)
        .await;
    assert_eq!(response.0, StatusCode::UNAUTHORIZED, "{response:?}");
    assert_eq!(f.count().await, 0);
}
on_both_stores!(
    sqlite_expired_owner_cannot_clock,
    postgres_expired_owner_cannot_clock,
    expired_owner_token
);

async fn injected_claims_require_exact_bearer(f: &Fixture) {
    let claims = f.auth.validate_token(&f.token).await.unwrap();
    let missing = f.post_injected(claims.clone(), None, f.db.clone()).await;
    assert_eq!(missing.0, StatusCode::FORBIDDEN, "{missing:?}");
    for field in ["actor", "tenant", "jti", "expiry", "session"] {
        let mut forged = claims.clone();
        match field {
            "actor" => forged.sub = "other-owner".into(),
            "tenant" => forged.organization_id = Some("foreign-tenant".into()),
            "jti" => forged.jti = "another-token".into(),
            "expiry" => forged.exp += 1,
            "session" => forged.session_id = Some("another-session".into()),
            _ => unreachable!(),
        }
        let response = f.post_injected(forged, Some(&f.token), f.db.clone()).await;
        assert_eq!(response.0, StatusCode::FORBIDDEN, "{field}: {response:?}");
    }
    assert_eq!(f.count().await, 0);
}
on_both_stores!(
    sqlite_injected_claims_are_not_authority,
    postgres_injected_claims_are_not_authority,
    injected_claims_require_exact_bearer
);

async fn copied_identity_is_not_storage_binding(f: &Fixture) {
    let foreign = Fixture::new(f.backend).await;
    let claims = f.auth.validate_token(&f.token).await.unwrap();
    let response = f
        .post_injected(claims, Some(&f.token), foreign.db.clone())
        .await;
    let original_count = f.count().await;
    let foreign_count = foreign.count().await;
    foreign.finish().await;
    assert_eq!(
        response.0,
        StatusCode::SERVICE_UNAVAILABLE,
        "equal user/tenant rows cannot bind independent business storage: {response:?}"
    );
    assert_eq!((original_count, foreign_count), (0, 0));
}
on_both_stores!(
    sqlite_copied_database_is_not_canonical,
    postgres_other_schema_is_not_canonical,
    copied_identity_is_not_storage_binding
);

async fn tampered_staff_and_time_are_not_receipts(f: &Fixture) {
    for (index, assignment) in [
        "staff_id='owned-staff'",
        "event_time='2026-10-04T06:00:01.123456Z'",
    ]
    .into_iter()
    .enumerate()
    {
        let id = format!("tampered-effect-{index}");
        assert_ack(&f.post(vec![clock(&id)]).await, &[&id]);
        f.execute(&format!(
            "UPDATE ohc_timecard_event SET {assignment} WHERE id='{id}'"
        ))
        .await;
        let response = f
            .post(vec![clock(&id), clock("aa-new-must-rollback")])
            .await;
        assert_eq!(response.0, StatusCode::CONFLICT, "{response:?}");
        assert_eq!(f.count().await, index as i64 + 1);
    }
}
on_both_stores!(
    sqlite_tampered_staff_or_time_cannot_ack,
    postgres_tampered_staff_or_time_cannot_ack,
    tampered_staff_and_time_are_not_receipts
);

async fn insert_trigger_cannot_fabricate_success(f: &Fixture) {
    f.execute(match f.backend {
        Backend::Sqlite => "CREATE TRIGGER corrupt_clock AFTER INSERT ON ohc_timecard_event BEGIN UPDATE ohc_timecard_event SET event_type='CLOCK_OUT' WHERE id=NEW.id; END",
        Backend::Postgres => "CREATE FUNCTION corrupt_clock() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN UPDATE ohc_timecard_event SET event_type='CLOCK_OUT' WHERE id=NEW.id; RETURN NEW; END $$; CREATE TRIGGER corrupt_clock AFTER INSERT ON ohc_timecard_event FOR EACH ROW EXECUTE FUNCTION corrupt_clock()",
    }).await;
    let response = f.post(vec![clock("corrupt-clock")]).await;
    assert_eq!(response.0, StatusCode::CONFLICT, "{response:?}");
    assert_eq!(f.count().await, 0);
}
on_both_stores!(
    sqlite_trigger_changed_effect_rolls_back,
    postgres_trigger_changed_effect_rolls_back,
    insert_trigger_cannot_fabricate_success
);

async fn arbitrarily_precise_spelling_must_be_exact(f: &Fixture) {
    for stamp in [
        "2026-10-04T06:00:00.1234560001Z",
        "2026-10-04T06:00:00.00000000000001Z",
    ] {
        let response = f
            .post(vec![
                clock("aa-valid"),
                event("zz-too-precise", OWNER, "CLOCK_IN", stamp),
            ])
            .await;
        assert_eq!(response.0, StatusCode::BAD_REQUEST, "{response:?}");
        assert_eq!(f.count().await, 0);
    }
    let response = f
        .post(vec![event(
            "exact-trailing-zero",
            OWNER,
            "CLOCK_IN",
            "2026-10-04T06:00:00.1234560000Z",
        )])
        .await;
    assert_ack(&response, &["exact-trailing-zero"]);
}
on_both_stores!(
    sqlite_subnanosecond_spelling_is_not_truncated,
    postgres_subnanosecond_spelling_is_not_truncated,
    arbitrarily_precise_spelling_must_be_exact
);

#[tokio::test]
async fn sqlite_unsafe_connection_pragmas_cannot_write() {
    for (set, restore) in [
        ("PRAGMA foreign_keys=OFF", "PRAGMA foreign_keys=ON"),
        ("PRAGMA query_only=ON", "PRAGMA query_only=OFF"),
        ("PRAGMA read_uncommitted=ON", "PRAGMA read_uncommitted=OFF"),
    ] {
        let f = Fixture::new(Backend::Sqlite).await;
        f.execute(set).await;
        let response = f.post(vec![clock("unsafe-connection")]).await;
        f.execute(restore).await;
        let count = f.count().await;
        f.finish().await;
        assert_eq!(
            response.0,
            StatusCode::SERVICE_UNAVAILABLE,
            "{set}: {response:?}"
        );
        assert_eq!(count, 0);
    }
}

async fn deferred_failure_never_acknowledges(f: &Fixture) {
    f.execute(match f.backend {
        Backend::Sqlite => "CREATE TABLE deferred_parent(id TEXT PRIMARY KEY); CREATE TABLE deferred_child(id TEXT PRIMARY KEY,parent TEXT REFERENCES deferred_parent(id) DEFERRABLE INITIALLY DEFERRED); CREATE TRIGGER reject_clock_commit AFTER INSERT ON ohc_timecard_event BEGIN INSERT INTO deferred_child VALUES(NEW.id,'missing-parent'); END",
        Backend::Postgres => "CREATE FUNCTION reject_clock_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'synthetic deferred constraint' USING ERRCODE='23514'; END $$; CREATE CONSTRAINT TRIGGER reject_clock_commit AFTER INSERT ON ohc_timecard_event DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_clock_commit()",
    }).await;
    let response = f.post(vec![clock("commit-rejected")]).await;
    assert_eq!(response.0, StatusCode::SERVICE_UNAVAILABLE, "{response:?}");
    // SQLite reaches COMMIT; PostgreSQL flushes its deferred callbacks before
    // the final persisted-effect verification. Neither failure may acknowledge.
    let expected = match f.backend {
        Backend::Sqlite => "reconciliation",
        Backend::Postgres => "blocked",
    };
    assert_eq!(response.1["outcomes"][0]["status"], json!(expected));
    assert_eq!(
        f.count().await,
        0,
        "event and receipt must roll back on actual deferred failure"
    );
}
on_both_stores!(
    sqlite_commit_error_is_not_acknowledged,
    postgres_deferred_constraint_error_is_not_acknowledged,
    deferred_failure_never_acknowledges
);

#[tokio::test]
async fn postgres_receipt_migration_is_additive_idempotent_and_preserves_legacy() {
    let f = Fixture::new(Backend::Postgres).await;
    f.execute("ALTER TABLE ohc_timecard_event DROP COLUMN request_identity")
        .await;
    f.execute("INSERT INTO ohc_timecard_event(id,tenant_id,staff_id,event_type,event_time) VALUES('legacy-schema','clock-tenant','clock-owner','CLOCK_IN','2026-10-04T06:00:00.123456Z')").await;
    let migration = include_str!("../migrations/1035_staff_timecard_receipts.sql");
    f.execute(migration).await;
    f.execute(migration).await;
    let saved = f.saved().await;
    let response = f.post(vec![clock("legacy-schema")]).await;
    f.finish().await;
    assert_eq!(saved.len(), 1);
    assert_eq!(saved[0].identity, None);
    assert_eq!(
        response.0,
        StatusCode::CONFLICT,
        "migration must not fabricate an old receipt: {response:?}"
    );
}

async fn multi_event_batch_commits_every_receipt(f: &Fixture) {
    let first = event(
        "zz-batch-clock",
        "owned-staff",
        "CLOCK_OUT",
        "2026-10-04T12:00:00.123456+05:30",
    );
    let second = clock("aa-batch-clock");
    let response = f.post(vec![first.clone(), second.clone()]).await;
    assert_ack(&response, &["zz-batch-clock", "aa-batch-clock"]);
    let saved = f.saved().await;
    assert_eq!(saved.len(), 2);
    assert_eq!(saved[0].identity, Some(identity(&second, OWNER)));
    assert_eq!(saved[1].identity, Some(identity(&first, OWNER)));
    assert_ack(
        &f.post(vec![second, first]).await,
        &["aa-batch-clock", "zz-batch-clock"],
    );
    assert_eq!(f.count().await, 2);
}
on_both_stores!(
    sqlite_multi_event_batch_and_reordered_replay,
    postgres_multi_event_batch_and_reordered_replay,
    multi_event_batch_commits_every_receipt
);

async fn foreign_global_id_is_a_private_atomic_conflict(f: &Fixture) {
    let token = f.token_for("foreign-owner", "foreign-tenant", "OWNER");
    let foreign = event("zz-global-clock", "foreign-owner", "CLOCK_IN", STAMP);
    assert_ack(
        &f.post_token(vec![foreign.clone()], &token).await,
        &["zz-global-clock"],
    );
    let response = f
        .post(vec![clock("aa-new-clock"), clock("zz-global-clock")])
        .await;
    assert_eq!(response.0, StatusCode::CONFLICT, "{response:?}");
    assert!(!response.1.to_string().contains("foreign-owner"));
    assert!(!response.1.to_string().contains("foreign-tenant"));
    let saved = f.saved().await;
    assert_eq!(saved.len(), 1);
    assert_eq!(saved[0].identity, Some(identity(&foreign, "foreign-owner")));
}
on_both_stores!(
    sqlite_foreign_global_id_rolls_back_without_disclosure,
    postgres_foreign_global_id_rolls_back_without_disclosure,
    foreign_global_id_is_a_private_atomic_conflict
);

#[tokio::test]
async fn postgres_deferred_effect_change_cannot_be_acknowledged() {
    let f = Fixture::new(Backend::Postgres).await;
    f.execute("CREATE FUNCTION corrupt_deferred_clock() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN UPDATE ohc_timecard_event SET event_type='CLOCK_OUT' WHERE id=NEW.id; RETURN NEW; END $$; CREATE CONSTRAINT TRIGGER corrupt_deferred_clock AFTER INSERT ON ohc_timecard_event DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION corrupt_deferred_clock()").await;
    let response = f.post(vec![clock("deferred-corrupt-clock")]).await;
    let count = f.count().await;
    f.finish().await;
    assert_eq!(
        response.0,
        StatusCode::CONFLICT,
        "deferred callbacks must run before effect certification: {response:?}"
    );
    assert_eq!(count, 0);
}

// Receipt recovery reads the already committed effect; it never resends a clock.
async fn committed_clock_has_read_only_receipt(f: &Fixture) {
    let action = event(
        "receipt-clock",
        OWNER,
        "CLOCK_OUT",
        "2026-10-04T11:30:00.123456+05:30",
    );
    assert_ack(&f.post(vec![action.clone()]).await, &["receipt-clock"]);
    f.execute(match f.backend {
        Backend::Sqlite=>"CREATE TRIGGER receipt_no_insert BEFORE INSERT ON ohc_timecard_event BEGIN SELECT RAISE(ABORT,'receipt must not insert'); END; CREATE TRIGGER receipt_no_update BEFORE UPDATE ON ohc_timecard_event BEGIN SELECT RAISE(ABORT,'receipt must not update'); END; CREATE TRIGGER receipt_no_delete BEFORE DELETE ON ohc_timecard_event BEGIN SELECT RAISE(ABORT,'receipt must not delete'); END",
        Backend::Postgres=>"CREATE FUNCTION receipt_no_write() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'receipt must not mutate' USING ERRCODE='23514'; END $$; CREATE TRIGGER receipt_no_write BEFORE INSERT OR UPDATE OR DELETE ON ohc_timecard_event FOR EACH ROW EXECUTE FUNCTION receipt_no_write()",
    }).await;
    for _ in 0..2 {
        let response = f.receipt("receipt-clock").await;
        assert_ack(&response, &["receipt-clock"]);
        assert_eq!(response.1["receipt"], identity(&action, OWNER));
    }
    assert_eq!(f.count().await, 1);
}
on_both_stores!(
    sqlite_receipt_lookup_confirms_without_writes,
    postgres_receipt_lookup_confirms_without_writes,
    committed_clock_has_read_only_receipt
);

async fn missing_and_legacy_receipts_are_unconfirmed(f: &Fixture) {
    let missing = f.receipt("missing-receipt").await;
    assert_eq!(missing.0, StatusCode::NOT_FOUND);
    assert_ack(
        &f.post(vec![clock("legacy-receipt")]).await,
        &["legacy-receipt"],
    );
    f.execute("UPDATE ohc_timecard_event SET request_identity=NULL WHERE id='legacy-receipt'")
        .await;
    let legacy = f.receipt("legacy-receipt").await;
    assert_eq!(legacy.0, StatusCode::NOT_FOUND);
    assert_ne!(legacy.1["success"], json!(true));
    assert_eq!(f.saved().await[0].identity, None);
}
on_both_stores!(
    sqlite_missing_or_legacy_receipt_cannot_ack,
    postgres_missing_or_legacy_receipt_cannot_ack,
    missing_and_legacy_receipts_are_unconfirmed
);

async fn receipt_is_tenant_and_issuing_actor_scoped(f: &Fixture) {
    assert_ack(
        &f.post(vec![clock("private-receipt")]).await,
        &["private-receipt"],
    );
    for (actor, tenant, role) in [
        ("other-owner", TENANT, "ADMIN"),
        ("foreign-owner", "foreign-tenant", "OWNER"),
    ] {
        let response = f
            .receipt_token("private-receipt", &f.token_for(actor, tenant, role))
            .await;
        assert_eq!(response.0, StatusCode::NOT_FOUND, "{response:?}");
        assert!(response.1.get("receipt").is_none());
    }
    assert_eq!(f.count().await, 1);
}
on_both_stores!(
    sqlite_receipt_requires_current_tenant_and_issuer,
    postgres_receipt_requires_current_tenant_and_issuer,
    receipt_is_tenant_and_issuing_actor_scoped
);

async fn receipt_verifies_all_persisted_effect_fields(f: &Fixture) {
    for (index, assignment) in [
        "staff_id='owned-staff'",
        "event_type='CLOCK_OUT'",
        "event_time='2026-10-04T06:00:01.123456Z'",
    ]
    .into_iter()
    .enumerate()
    {
        let id = format!("receipt-tamper-{index}");
        assert_ack(&f.post(vec![clock(&id)]).await, &[&id]);
        f.execute(&format!(
            "UPDATE ohc_timecard_event SET {assignment} WHERE id='{id}'"
        ))
        .await;
        let response = f.receipt(&id).await;
        assert_eq!(response.0, StatusCode::CONFLICT, "{response:?}");
        assert!(response.1.get("receipt").is_none());
    }
    assert_eq!(f.count().await, 3);
}
on_both_stores!(
    sqlite_receipt_checks_persisted_effect,
    postgres_receipt_checks_persisted_effect,
    receipt_verifies_all_persisted_effect_fields
);

async fn receipt_identity_must_match_lookup_id_and_schema(f: &Fixture) {
    assert_ack(&f.post(vec![clock("lookup-id")]).await, &["lookup-id"]);
    for mutate in ["id", "version", "extra"] {
        let mut value = identity(&clock("lookup-id"), OWNER);
        match mutate {
            "id" => value["id"] = json!("different-id"),
            "version" => value["version"] = json!(2),
            "extra" => value["unexpected"] = json!(true),
            _ => unreachable!(),
        }
        f.execute(&format!(
            "UPDATE ohc_timecard_event SET request_identity='{}' WHERE id='lookup-id'",
            value.to_string().replace('\'', "''")
        ))
        .await;
        let response = f.receipt("lookup-id").await;
        assert!(!response.0.is_success(), "{response:?}");
        assert!(response.1.get("receipt").is_none());
    }
    assert_eq!(f.count().await, 1);
}
on_both_stores!(
    sqlite_receipt_rejects_mismatched_or_unversioned_identity,
    postgres_receipt_rejects_mismatched_or_unversioned_identity,
    receipt_identity_must_match_lookup_id_and_schema
);

async fn stale_owner_cannot_read_receipt(f: &Fixture) {
    assert_ack(
        &f.post(vec![clock("role-receipt")]).await,
        &["role-receipt"],
    );
    f.execute("UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id='clock-owner'")
        .await;
    let response = f.receipt("role-receipt").await;
    assert_eq!(response.0, StatusCode::FORBIDDEN, "{response:?}");
    assert!(response.1.get("receipt").is_none());
    assert_eq!(f.count().await, 1);
}
on_both_stores!(
    sqlite_receipt_requires_current_owner_role,
    postgres_receipt_requires_current_owner_role,
    stale_owner_cannot_read_receipt
);

async fn revoked_owner_cannot_read_receipt(f: &Fixture) {
    assert_ack(
        &f.post(vec![clock("revoked-receipt")]).await,
        &["revoked-receipt"],
    );
    let claims = f.auth.validate_token(&f.token).await.unwrap();
    f.auth
        .revoke_token(
            claims.jti,
            DateTime::from_timestamp(claims.exp, 0).unwrap(),
            TENANT,
        )
        .await
        .unwrap();
    let response = f.receipt("revoked-receipt").await;
    assert_eq!(response.0, StatusCode::UNAUTHORIZED, "{response:?}");
    assert_eq!(f.count().await, 1);
}
on_both_stores!(
    sqlite_revoked_session_cannot_get_receipt,
    postgres_revoked_session_cannot_get_receipt,
    revoked_owner_cannot_read_receipt
);

async fn receipt_storage_failure_is_explicit(f: &Fixture) {
    f.execute("DROP TABLE ohc_timecard_event").await;
    let response = f.receipt("unavailable-receipt").await;
    assert_eq!(response.0, StatusCode::SERVICE_UNAVAILABLE, "{response:?}");
    assert!(response.1.get("receipt").is_none());
}
on_both_stores!(
    sqlite_unavailable_receipt_storage_cannot_ack,
    postgres_unavailable_receipt_storage_cannot_ack,
    receipt_storage_failure_is_explicit
);

async fn receipt_ids_keep_canonical_path_rules(f: &Fixture) {
    for id in [".", "..", "a%2Fb", "a%25b", "a%20b", "a%5Cb"] {
        let response = f.receipt(id).await;
        assert!(
            matches!(response.0, StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND),
            "{id}: {response:?}"
        );
        assert_ne!(response.1["success"], json!(true));
    }
    assert_eq!(f.count().await, 0);
}
on_both_stores!(
    sqlite_receipt_ids_are_unambiguous,
    postgres_receipt_ids_are_unambiguous,
    receipt_ids_keep_canonical_path_rules
);

async fn receipt_rejects_encoded_alias_of_safe_id(f: &Fixture) {
    assert_ack(
        &f.post(vec![clock("receipt-clock"), clock("receipt_clock")])
            .await,
        &["receipt-clock", "receipt_clock"],
    );
    for alias in ["%72eceipt-clock", "receipt%2Dclock", "receipt%5Fclock"] {
        let response = f.receipt(alias).await;
        assert_eq!(
            response.0,
            StatusCode::BAD_REQUEST,
            "raw path aliases cannot acknowledge a canonical receipt: {alias}: {response:?}"
        );
        assert!(response.1.get("receipt").is_none());
    }
    assert_eq!(f.count().await, 2);
}
on_both_stores!(
    sqlite_receipt_rejects_percent_encoded_safe_ids,
    postgres_receipt_rejects_percent_encoded_safe_ids,
    receipt_rejects_encoded_alias_of_safe_id
);
