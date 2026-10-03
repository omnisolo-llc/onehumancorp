use super::*;
use axum::body::to_bytes;
use chrono::{DateTime, Utc};
use sea_orm::{ConnectionTrait, Schema};
use serde_json::Value;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use uuid::Uuid;
fn target(raw: &str) -> PgConnectOptions {
    let u = url::Url::parse(raw).unwrap();
    assert!(
        matches!(u.scheme(), "postgres" | "postgresql")
            && u.host_str().is_some_and(|h| h
                .trim_matches(['[', ']'])
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback()))
            && u.path() == "/ohc_field_test"
            && u.query().is_none()
            && u.fragment().is_none(),
        "explicit owned field database required"
    );
    raw.parse().unwrap()
}
struct Fixture {
    admin: PgPool,
    pool: PgPool,
    auth: Arc<server_auth::Store>,
    schema: String,
    role: String,
    token: String,
    foreign: String,
    member: String,
}
impl Fixture {
    async fn new(restricted: bool) -> Self {
        let raw = std::env::var("OHC_FIELD_TEST_DATABASE_URL").expect("owned PostgreSQL required");
        let schema = format!("field_case_{}", Uuid::new_v4().simple());
        let role = format!("field_role_{}", Uuid::new_v4().simple());
        let password = Uuid::new_v4().to_string();
        let opts = target(&raw).options([("search_path", schema.as_str())]);
        let admin = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(opts.clone())
            .await
            .unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!("schema.sql"))
            .execute(&admin)
            .await
            .unwrap();
        let orm = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(admin.clone());
        let backend = sea_orm::DatabaseBackend::Postgres;
        orm.execute(
            backend.build(&Schema::new(backend).create_table_from_entity(
                server_auth::seaorm_store::entities::identity_user_role::Entity,
            )),
        )
        .await
        .unwrap();
        orm.execute(backend.build(
            &Schema::new(backend).create_table_from_entity(
                server_auth::seaorm_store::entities::revoked_token::Entity,
            ),
        ))
        .await
        .unwrap();
        for table in ["users", "identity_user_roles", "auth_revoked_tokens"] {
            sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY;ALTER TABLE {table} FORCE ROW LEVEL SECURITY;CREATE POLICY scoped ON {table} USING(tenant_id=current_setting('app.current_tenant',true)) WITH CHECK(tenant_id=current_setting('app.current_tenant',true));")).execute(&admin).await.unwrap();
        }
        for suffix in ["a", "b"] {
            sqlx::query("INSERT INTO tenants(id,name)VALUES($1,$1)")
                .bind(format!("tenant-{suffix}"))
                .execute(&admin)
                .await
                .unwrap();
            sqlx::query("INSERT INTO appointments(id,tenant_id,status,notes,scheduled_start_time,scheduled_end_time,updated_at)VALUES($1,$2,'Scheduled','stored', '2026-10-04T10:00:00Z','2026-10-04T11:00:00Z','2026-10-03T00:00:00Z')").bind(suffix).bind(format!("tenant-{suffix}")).execute(&admin).await.unwrap();
            sqlx::query(
                "INSERT INTO service_routes(id,tenant_id,route_date)VALUES($1,$2,CURRENT_DATE)",
            )
            .bind(format!("r-{suffix}"))
            .bind(format!("tenant-{suffix}"))
            .execute(&admin)
            .await
            .unwrap();
            sqlx::query("INSERT INTO job_locations(id,tenant_id,service_route_id,appointment_id,sequence_order,updated_at)VALUES($1,$2,$3,$4,0,'2026-10-03T00:00:00Z')").bind(format!("j-{suffix}")).bind(format!("tenant-{suffix}")).bind(format!("r-{suffix}")).bind(suffix).execute(&admin).await.unwrap();
        }
        sqlx::raw_sql(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}';GRANT USAGE ON SCHEMA {schema} TO {role};GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
        let rejected = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(1))
            .connect_with(
                opts.clone()
                    .username(&role)
                    .password("incorrect-fixture-secret"),
            )
            .await;
        assert!(rejected.is_err(), "owned cluster must enforce SCRAM");
        let scoped = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(2))
            .connect_with(opts.username(&role).password(&password))
            .await
            .unwrap();
        let identity: (bool, bool) =
            sqlx::query_as("SELECT rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user")
                .fetch_one(&scoped)
                .await
                .unwrap();
        assert_eq!(identity, (false, false));
        let pool = if restricted {
            scoped.clone()
        } else {
            admin.clone()
        };
        let repo = Arc::new(server_auth::seaorm_store::SeaOrmAuthRepository::new(
            sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone()),
        ));
        let auth = Arc::new(server_auth::Store::with_portable_repo(repo));
        let mut tokens = vec![];
        for (id, tenant, role_name) in [
            ("owner-a", "tenant-a", "ADMIN"),
            ("owner-b", "tenant-b", "ADMIN"),
            ("member-a", "tenant-a", "MEMBER"),
        ] {
            let user = server_auth::User {
                id: id.into(),
                username: id.into(),
                email: format!("{id}@example.test"),
                password_hash: "fixture-unused".into(),
                roles: vec![role_name.into()],
                active: true,
                organization_id: Some(tenant.into()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                oidc_subject: None,
            };
            sqlx::query("INSERT INTO users(id,username,email,tenant_id)VALUES($1,$1,$2,$3)")
                .bind(id)
                .bind(&user.email)
                .bind(tenant)
                .execute(&admin)
                .await
                .unwrap();
            sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position)VALUES($1,$2,$3,0)").bind(id).bind(role_name).bind(tenant).execute(&admin).await.unwrap();
            tokens.push(auth.issue_token(&user).unwrap());
        }
        if !restricted {
            scoped.close().await;
        }
        Self {
            admin,
            pool,
            auth,
            schema,
            role,
            token: tokens.remove(0),
            foreign: tokens.remove(0),
            member: tokens.remove(0),
        }
    }
    async fn request(
        &self,
        method: &str,
        path: &str,
        token: &str,
        body: Value,
        key: Option<&str>,
    ) -> (StatusCode, Value) {
        let mut req = Request::builder()
            .method(method)
            .uri(path)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {token}"))
            .header("x-tenant-id", "tenant-b");
        if let Some(key) = key {
            req = req.header("idempotency-key", key)
        }
        let app = crate::actual_mount(
            Arc::new(crate::db::DB {
                pool: self.pool.clone(),
            }),
            self.auth.clone(),
        )
        .await;
        let r = app
            .oneshot(req.body(Body::from(body.to_string())).unwrap())
            .await
            .unwrap();
        let status = r.status();
        let bytes = to_bytes(r.into_body(), 1048576).await.unwrap();
        (
            status,
            serde_json::from_slice(&bytes)
                .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).to_string())),
        )
    }
    fn update(&self, id: &str) -> Value {
        json!({"id":id,"status":"In-Progress","expected_updated_at":"2026-10-03T00:00:00Z","notes":"saved","location_lat":12.5,"location_lng":-20.0,"scheduled_start_time":"2026-10-04T10:30:00Z","scheduled_end_time":"2026-10-04T11:30:00Z"})
    }
    async fn finish(self) {
        if !std::ptr::eq(self.pool.options(), self.admin.options()) {
            self.pool.close().await;
        }
        sqlx::raw_sql(&format!(
            "DROP SCHEMA {} CASCADE;DROP ROLE {};",
            self.schema, self.role
        ))
        .execute(&self.admin)
        .await
        .unwrap();
        self.admin.close().await;
    }
}
#[tokio::test]
async fn current_owner_mutation_persists_all_requested_fields_and_observed_timestamp() {
    let f = Fixture::new(true).await;
    let (status, body) = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            f.update("a"),
            None,
        )
        .await;
    let row: (String, Option<f64>, DateTime<Utc>, DateTime<Utc>) = sqlx::query_as(
        "SELECT status,location_lat,scheduled_start_time,updated_at FROM appointments WHERE id='a'",
    )
    .fetch_one(&f.admin)
    .await
    .unwrap();
    f.finish().await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(row.0, "In-Progress");
    assert_eq!(row.1, Some(12.5));
    assert_eq!(row.2.to_rfc3339(), "2026-10-04T10:30:00+00:00");
    assert_eq!(body["updated_at"], json!(row.3));
}
#[tokio::test]
async fn tenant_owner_cannot_mutate_foreign_or_missing_appointment_even_with_forged_header() {
    let f = Fixture::new(false).await;
    for id in ["b", "missing"] {
        let r = f
            .request(
                "POST",
                "/api/v1/field-ops/appointments",
                &f.token,
                f.update(id),
                None,
            )
            .await;
        assert_eq!(r.0, StatusCode::NOT_FOUND, "{id}:{}", r.1)
    }
    let status: String = sqlx::query_scalar("SELECT status FROM appointments WHERE id='b'")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(status, "Scheduled");
}
#[tokio::test]
async fn signed_member_cannot_update_appointment() {
    let f = Fixture::new(false).await;
    let r = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.member,
            f.update("a"),
            None,
        )
        .await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::FORBIDDEN, "{}", r.1);
}
#[tokio::test]
async fn stale_appointment_update_conflicts_without_overwriting() {
    let f = Fixture::new(false).await;
    let mut body = f.update("a");
    body["expected_updated_at"] = json!("2026-10-02T00:00:00Z");
    let r = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            body,
            None,
        )
        .await;
    let stored: String = sqlx::query_scalar("SELECT status FROM appointments WHERE id='a'")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(r.0, StatusCode::CONFLICT, "{}", r.1);
    assert_eq!(stored, "Scheduled");
}
#[tokio::test]
async fn terminal_appointment_cannot_be_reopened() {
    let f = Fixture::new(false).await;
    sqlx::query("UPDATE appointments SET status='Completed' WHERE id='a'")
        .execute(&f.admin)
        .await
        .unwrap();
    let observed: DateTime<Utc> =
        sqlx::query_scalar("SELECT updated_at FROM appointments WHERE id='a'")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    let mut body = f.update("a");
    body["expected_updated_at"] = json!(observed);
    let r = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            body,
            None,
        )
        .await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::CONFLICT, "{}", r.1);
}
#[tokio::test]
async fn appointment_idempotency_returns_exact_receipt_and_conflicts_on_changed_request() {
    let f = Fixture::new(false).await;
    let body = f.update("a");
    let first = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            body.clone(),
            Some("save-1"),
        )
        .await;
    let replay = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            body.clone(),
            Some("save-1"),
        )
        .await;
    let mut changed = body;
    changed["notes"] = json!("changed");
    let conflict = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            changed,
            Some("save-1"),
        )
        .await;
    f.finish().await;
    assert_eq!(first.0, StatusCode::OK, "{}", first.1);
    assert_eq!(first, replay);
    assert_eq!(conflict.0, StatusCode::CONFLICT, "{}", conflict.1);
}
#[tokio::test]
async fn route_read_uses_signed_tenant_and_real_job_relationships() {
    let f = Fixture::new(false).await;
    let r = f
        .request(
            "GET",
            "/api/v1/field-service-routing/routes/today",
            &f.token,
            json!(null),
            None,
        )
        .await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK, "{}", r.1);
    assert_eq!(r.1["routes"].as_array().unwrap().len(), 1);
    assert_eq!(r.1["routes"][0]["id"], "r-a");
    assert_eq!(r.1["routes"][0]["jobs"][0]["id"], "j-a");
}
#[tokio::test]
async fn routing_job_uses_signed_tenant_and_cas_and_committed_response() {
    let f = Fixture::new(true).await;
    let body = json!({"status":"en_route","expected_updated_at":"2026-10-03T00:00:00Z"});
    let r = f
        .request(
            "POST",
            "/api/v1/field-service-routing/jobs/j-a/status",
            &f.token,
            body.clone(),
            None,
        )
        .await;
    let stale = f
        .request(
            "POST",
            "/api/v1/field-service-routing/jobs/j-a/status",
            &f.token,
            body,
            None,
        )
        .await;
    let stored: String = sqlx::query_scalar("SELECT status FROM job_locations WHERE id='j-a'")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK, "{}", r.1);
    assert_eq!(stored, "en_route");
    assert!(r.1["updated_at"].is_string());
    assert_eq!(stale.0, StatusCode::CONFLICT);
}
#[tokio::test]
async fn optimizer_commit_uses_owned_persisted_appointments_and_actual_schema() {
    let f = Fixture::new(true).await;
    let read = f
        .request(
            "GET",
            "/api/v1/field-ops/appointments",
            &f.token,
            json!(null),
            None,
        )
        .await;
    assert_eq!(read.0, StatusCode::OK, "{}", read.1);
    let body = json!({"tenantId":"tenant-a","commit":true,"appointments":read.1["appointments"],"currentLocationLat":1.0,"currentLocationLng":1.0});
    let r = f
        .request(
            "POST",
            "/api/v1/field-ops/optimize-route",
            &f.token,
            body,
            Some("route-1"),
        )
        .await;
    let n: i64 =
        sqlx::query_scalar("SELECT count(*) FROM service_routes WHERE tenant_id='tenant-a'")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK, "{}", r.1);
    assert_eq!(r.1["committed"], true);
    assert!(r.1["routeId"].is_string());
    assert_eq!(n, 2);
}
#[tokio::test]
async fn optimizer_preview_is_explicitly_uncommitted() {
    let f = Fixture::new(true).await;
    let read = f
        .request(
            "GET",
            "/api/v1/field-ops/appointments",
            &f.token,
            json!(null),
            None,
        )
        .await;
    let r = f
        .request(
            "POST",
            "/api/v1/field-ops/optimize-route",
            &f.token,
            json!({"commit":false,"appointments":read.1["appointments"]}),
            None,
        )
        .await;
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM service_routes")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK, "{}", r.1);
    assert_eq!(r.1["committed"], false);
    assert_eq!(n, 2);
}
#[tokio::test]
async fn foreign_signed_owner_cannot_mutate_routing_job() {
    let f = Fixture::new(false).await;
    let r = f
        .request(
            "POST",
            "/api/v1/field-service-routing/jobs/j-a/status",
            &f.foreign,
            json!({"status":"done","expected_updated_at":"2026-10-03T00:00:00Z"}),
            None,
        )
        .await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::NOT_FOUND, "{}", r.1);
}
#[tokio::test]
async fn current_downgraded_owner_is_denied_despite_old_signed_admin_claim() {
    let f = Fixture::new(false).await;
    sqlx::query("UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id='owner-a'")
        .execute(&f.admin)
        .await
        .unwrap();
    let r = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            f.update("a"),
            None,
        )
        .await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::FORBIDDEN, "{}", r.1);
}
#[tokio::test]
async fn revoked_and_disabled_owner_cannot_update_field_records() {
    for revoked in [true, false] {
        let f = Fixture::new(false).await;
        if revoked {
            let claims = f.auth.validate_token(&f.token).await.unwrap();
            sqlx::query("INSERT INTO auth_revoked_tokens(jti,tenant_id,expires_at)VALUES($1,'tenant-a',CURRENT_TIMESTAMP+INTERVAL '1 hour')").bind(claims.jti).execute(&f.admin).await.unwrap();
        } else {
            sqlx::query("UPDATE users SET active=false WHERE id='owner-a'")
                .execute(&f.admin)
                .await
                .unwrap();
        }
        let r = f
            .request(
                "POST",
                "/api/v1/field-ops/appointments",
                &f.token,
                f.update("a"),
                None,
            )
            .await;
        f.finish().await;
        assert_eq!(r.0, StatusCode::UNAUTHORIZED, "{}", r.1);
    }
}
#[tokio::test]
async fn routing_job_commit_failure_does_not_report_success() {
    let f = Fixture::new(false).await;
    sqlx::raw_sql("CREATE FUNCTION reject_field_commit() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN RAISE EXCEPTION 'synthetic deferred failure';END$$;CREATE CONSTRAINT TRIGGER reject_field_commit AFTER UPDATE ON job_locations DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_field_commit();").execute(&f.admin).await.unwrap();
    let r = f
        .request(
            "POST",
            "/api/v1/field-service-routing/jobs/j-a/status",
            &f.token,
            json!({"status":"done","expected_updated_at":"2026-10-03T00:00:00Z"}),
            None,
        )
        .await;
    let actual: String = sqlx::query_scalar("SELECT status FROM job_locations WHERE id='j-a'")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(r.0, StatusCode::SERVICE_UNAVAILABLE, "{}", r.1);
    assert_eq!(actual, "pending");
}
#[tokio::test]
async fn canonical_pool_proof_rejects_alternate_schema_and_other_database() {
    let f = Fixture::new(false).await;
    let repo = f.auth.portable_repo().unwrap();
    let raw = std::env::var("OHC_FIELD_TEST_DATABASE_URL").unwrap();
    let equivalent = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(target(&raw).options([("search_path", f.schema.as_str())]))
        .await
        .unwrap();
    let good = server_auth::commit_authority::canonical_pg_data_pool(
        repo.connection(),
        &equivalent,
        &["appointments", "service_routes", "job_locations"],
    )
    .await
    .unwrap();
    assert!(std::ptr::eq(good.options(), f.pool.options()));
    let other_schema = format!("field_other_{}", Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE SCHEMA {other_schema}"))
        .execute(&f.admin)
        .await
        .unwrap();
    let alternate = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(target(&raw).options([("search_path", other_schema.as_str())]))
        .await
        .unwrap();
    assert!(
        server_auth::commit_authority::canonical_pg_data_pool(
            repo.connection(),
            &alternate,
            &["appointments", "service_routes", "job_locations"]
        )
        .await
        .is_err()
    );
    let other_db = format!("ohc_field_{}_test", Uuid::new_v4().simple());
    sqlx::query(&format!("CREATE DATABASE {other_db}"))
        .execute(&f.admin)
        .await
        .unwrap();
    let foreign = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(target(&raw).database(&other_db))
        .await
        .unwrap();
    assert!(
        server_auth::commit_authority::canonical_pg_data_pool(
            repo.connection(),
            &foreign,
            &["appointments", "service_routes", "job_locations"]
        )
        .await
        .is_err()
    );
    foreign.close().await;
    alternate.close().await;
    equivalent.close().await;
    // The entire cluster belongs to this test wrapper. SQLx background connection
    // closure can outlive Pool::close; only this randomly named owned probe DB is
    // forcibly removed, never the configured destination or another worker's DB.
    sqlx::query(&format!("DROP DATABASE {other_db} WITH (FORCE)"))
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query(&format!("DROP SCHEMA {other_schema}"))
        .execute(&f.admin)
        .await
        .unwrap();
    f.finish().await;
}
#[tokio::test]
async fn mounted_appointment_reads_reject_unproven_data_schema() {
    let f = Fixture::new(false).await;
    let other = Fixture::new(false).await;
    let app = crate::actual_mount(
        Arc::new(crate::db::DB {
            pool: other.pool.clone(),
        }),
        f.auth.clone(),
    )
    .await;
    let request = Request::builder()
        .uri("/api/v1/field-ops/appointments")
        .header("authorization", format!("Bearer {}", f.token))
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    other.finish().await;
    f.finish().await;
    assert_eq!(
        status,
        StatusCode::SERVICE_UNAVAILABLE,
        "unproven private storage must not expose identically named tenant rows"
    );
}
#[tokio::test]
async fn concurrent_observed_version_updates_have_exactly_one_winner() {
    let f = Fixture::new(true).await;
    let first = f.update("a");
    let mut second = first.clone();
    second["notes"] = json!("second writer");
    let (a, b) = tokio::join!(
        f.request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            first,
            None
        ),
        f.request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            second,
            None
        )
    );
    let mut statuses = [a.0.as_u16(), b.0.as_u16()];
    statuses.sort();
    f.finish().await;
    assert_eq!(statuses, [200, 409]);
}
#[tokio::test]
async fn completion_receipt_replay_enqueues_exactly_one_owned_department_task() {
    let f = Fixture::new(true).await;
    let mut body = f.update("a");
    body["status"] = json!("Completed");
    let a = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            body.clone(),
            Some("complete-1"),
        )
        .await;
    let b = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            body,
            Some("complete-1"),
        )
        .await;
    let rows: Vec<(String, Value)> =
        sqlx::query_as("SELECT tenant_id,payload FROM department_tasks")
            .fetch_all(&f.admin)
            .await
            .unwrap();
    f.finish().await;
    assert_eq!(a.0, StatusCode::OK, "{}", a.1);
    assert_eq!(a, b);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].0, "tenant-a");
    assert_eq!(rows[0].1["appointment_id"], "a");
}
#[tokio::test]
async fn deferred_appointment_commit_failure_rolls_back_status_receipt_and_task() {
    let f = Fixture::new(true).await;
    sqlx::raw_sql("CREATE FUNCTION reject_appointment_commit() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN RAISE EXCEPTION 'synthetic deferred failure';END$$;CREATE CONSTRAINT TRIGGER reject_appointment_commit AFTER UPDATE ON appointments DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_appointment_commit();").execute(&f.admin).await.unwrap();
    let mut body = f.update("a");
    body["status"] = json!("Completed");
    let r = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            body,
            Some("failed-complete"),
        )
        .await;
    let counts:(i64,i64,String)=sqlx::query_as("SELECT (SELECT count(*) FROM field_mutation_receipts),(SELECT count(*) FROM department_tasks),(SELECT status FROM appointments WHERE id='a')").fetch_one(&f.admin).await.unwrap();
    f.finish().await;
    assert_eq!(r.0, StatusCode::SERVICE_UNAVAILABLE, "{}", r.1);
    assert_eq!(counts, (0, 0, "Scheduled".into()));
}
#[tokio::test]
async fn route_commit_replay_is_exact_and_foreign_snapshot_is_rejected() {
    let f = Fixture::new(true).await;
    let read = f
        .request(
            "GET",
            "/api/v1/field-ops/appointments",
            &f.token,
            json!(null),
            None,
        )
        .await;
    let body = json!({"commit":true,"appointments":read.1["appointments"]});
    let first = f
        .request(
            "POST",
            "/api/v1/field-ops/optimize-route",
            &f.token,
            body.clone(),
            Some("route-replay"),
        )
        .await;
    let second = f
        .request(
            "POST",
            "/api/v1/field-ops/optimize-route",
            &f.token,
            body.clone(),
            Some("route-replay"),
        )
        .await;
    let mut foreign = body;
    foreign["appointments"][0]["id"] = json!("b");
    let denied = f
        .request(
            "POST",
            "/api/v1/field-ops/optimize-route",
            &f.token,
            foreign,
            Some("foreign-route"),
        )
        .await;
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM service_routes")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(first.0, StatusCode::OK, "{}", first.1);
    assert_eq!(first, second);
    assert_eq!(denied.0, StatusCode::NOT_FOUND);
    assert_eq!(n, 3);
}
#[tokio::test]
async fn route_job_insert_failure_rolls_back_route_and_receipt() {
    let f = Fixture::new(true).await;
    sqlx::raw_sql("CREATE FUNCTION reject_route_job() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN RAISE EXCEPTION 'synthetic route write failure';END$$;CREATE TRIGGER reject_route_job BEFORE INSERT ON job_locations FOR EACH ROW EXECUTE FUNCTION reject_route_job();").execute(&f.admin).await.unwrap();
    let read = f
        .request(
            "GET",
            "/api/v1/field-ops/appointments",
            &f.token,
            json!(null),
            None,
        )
        .await;
    let r = f
        .request(
            "POST",
            "/api/v1/field-ops/optimize-route",
            &f.token,
            json!({"commit":true,"appointments":read.1["appointments"]}),
            Some("failed-route"),
        )
        .await;
    let counts:(i64,i64,i64)=sqlx::query_as("SELECT (SELECT count(*) FROM service_routes),(SELECT count(*) FROM job_locations),(SELECT count(*) FROM field_mutation_receipts)").fetch_one(&f.admin).await.unwrap();
    f.finish().await;
    assert_eq!(r.0, StatusCode::SERVICE_UNAVAILABLE, "{}", r.1);
    assert_eq!(counts, (2, 2, 0));
}
#[tokio::test]
async fn delay_preview_uses_real_schedule_and_leaves_storage_unchanged() {
    let f = Fixture::new(true).await;
    sqlx::query("INSERT INTO appointments(id,tenant_id,status,scheduled_start_time,scheduled_end_time,updated_at)SELECT 'a2',tenant_id,status,scheduled_start_time+INTERVAL '2 hours',scheduled_end_time+INTERVAL '2 hours',updated_at FROM appointments WHERE id='a'").execute(&f.admin).await.unwrap();
    let read = f
        .request(
            "GET",
            "/api/v1/field-ops/appointments",
            &f.token,
            json!(null),
            None,
        )
        .await;
    let mut input = read.1["appointments"].clone();
    input[1]["scheduled_start_time"] = json!("2099-01-01T00:00:00Z");
    let r = f
        .request(
            "POST",
            "/api/v1/field-ops/running-late",
            &f.token,
            json!({"appointments":input,"delayJobId":"a"}),
            None,
        )
        .await;
    let stored: DateTime<Utc> =
        sqlx::query_scalar("SELECT scheduled_start_time FROM appointments WHERE id='a2'")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK, "{}", r.1);
    assert_eq!(r.1["committed"], false);
    assert_eq!(r.1["subsequentCount"], 1);
    assert_eq!(
        r.1["optimizedRoute"][1]["scheduled_start_time"],
        json!(stored + chrono::Duration::minutes(30))
    );
    assert_eq!(stored.to_rfc3339(), "2026-10-04T12:00:00+00:00");
}
#[tokio::test]
async fn malformed_route_relationship_cannot_return_foreign_appointment_details() {
    let f = Fixture::new(false).await;
    sqlx::query("UPDATE job_locations SET appointment_id='b' WHERE id='j-a'")
        .execute(&f.admin)
        .await
        .unwrap();
    let read = f
        .request(
            "GET",
            "/api/v1/field-service-routing/routes/today",
            &f.token,
            json!(null),
            None,
        )
        .await;
    let update = f
        .request(
            "POST",
            "/api/v1/field-service-routing/jobs/j-a/status",
            &f.token,
            json!({"status":"done","expected_updated_at":"2026-10-03T00:00:00Z"}),
            None,
        )
        .await;
    f.finish().await;
    assert_eq!(read.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(update.0, StatusCode::NOT_FOUND);
}
#[tokio::test]
async fn route_read_does_not_expose_foreign_customer_identifier_from_legacy_relation() {
    let f = Fixture::new(false).await;
    sqlx::query("INSERT INTO customers(id,tenant_id,name)VALUES('foreign-customer','tenant-b','Private customer')").execute(&f.admin).await.unwrap();
    sqlx::query("UPDATE appointments SET customer_id='foreign-customer' WHERE id='a'")
        .execute(&f.admin)
        .await
        .unwrap();
    let r = f
        .request(
            "GET",
            "/api/v1/field-service-routing/routes/today",
            &f.token,
            json!(null),
            None,
        )
        .await;
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK, "{}", r.1);
    assert!(
        r.1["routes"][0]["jobs"][0]["customer_id"].is_null(),
        "foreign customer ID must not cross ownership boundary: {}",
        r.1
    );
}
#[tokio::test]
async fn normalizing_existing_completed_status_does_not_create_another_completion_task() {
    let f = Fixture::new(true).await;
    sqlx::query("UPDATE appointments SET status='completed' WHERE id='a'")
        .execute(&f.admin)
        .await
        .unwrap();
    let observed: DateTime<Utc> =
        sqlx::query_scalar("SELECT updated_at FROM appointments WHERE id='a'")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    let mut body = f.update("a");
    body["expected_updated_at"] = json!(observed);
    body["status"] = json!("Completed");
    let r = f
        .request(
            "POST",
            "/api/v1/field-ops/appointments",
            &f.token,
            body,
            Some("normalize-completed"),
        )
        .await;
    let n: i64 = sqlx::query_scalar("SELECT count(*) FROM department_tasks")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(r.0, StatusCode::OK, "{}", r.1);
    assert_eq!(
        n, 0,
        "case normalization must not fabricate a completion transition"
    );
}
#[tokio::test]
async fn saved_routes_preserve_terminal_legacy_statuses() {
    let f = Fixture::new(true).await;
    for (index, status) in ["completed", "COMPLETED", "done", "cancelled", "canceled"]
        .into_iter()
        .enumerate()
    {
        sqlx::query("INSERT INTO appointments(id,tenant_id,status,updated_at)VALUES($1,'tenant-a',$2,'2026-10-03T00:00:00Z')").bind(format!("terminal-{index}")).bind(status).execute(&f.admin).await.unwrap();
    }
    let read = f
        .request(
            "GET",
            "/api/v1/field-ops/appointments",
            &f.token,
            json!(null),
            None,
        )
        .await;
    let saved = f
        .request(
            "POST",
            "/api/v1/field-ops/optimize-route",
            &f.token,
            json!({"commit":true,"appointments":read.1["appointments"]}),
            Some("legacy-status-route"),
        )
        .await;
    assert_eq!(saved.0, StatusCode::OK, "{}", saved.1);
    let jobs:Vec<(String,String)>=sqlx::query_as("SELECT a.status,j.status FROM job_locations j JOIN appointments a ON a.id=j.appointment_id WHERE j.service_route_id=$1 AND a.id LIKE 'terminal-%'").bind(saved.1["routeId"].as_str().unwrap()).fetch_all(&f.admin).await.unwrap();
    f.finish().await;
    assert_eq!(jobs.len(), 5);
    for (original, saved) in jobs {
        assert_eq!(
            saved,
            if original.to_lowercase().starts_with("cancel") {
                "cancelled"
            } else {
                "done"
            },
            "terminal {original} must not be reopened by route creation"
        );
    }
}
#[tokio::test]
async fn saving_a_new_route_replaces_historical_order_on_appointment_reload() {
    let f = Fixture::new(true).await;
    sqlx::query("INSERT INTO appointments(id,tenant_id,status,scheduled_start_time,updated_at)VALUES('a2','tenant-a','Scheduled','2026-10-04T12:00:00Z','2026-10-03T00:00:00Z')").execute(&f.admin).await.unwrap();
    let read = f
        .request(
            "GET",
            "/api/v1/field-ops/appointments",
            &f.token,
            json!(null),
            None,
        )
        .await;
    let mut rows = read.1["appointments"].as_array().unwrap().clone();
    let first = f
        .request(
            "POST",
            "/api/v1/field-ops/optimize-route",
            &f.token,
            json!({"commit":true,"appointments":rows}),
            Some("initial-order"),
        )
        .await;
    assert_eq!(first.0, StatusCode::OK, "{}", first.1);
    rows.reverse();
    let second = f
        .request(
            "POST",
            "/api/v1/field-ops/optimize-route",
            &f.token,
            json!({"commit":true,"appointments":rows}),
            Some("updated-order"),
        )
        .await;
    assert_eq!(second.0, StatusCode::OK, "{}", second.1);
    let reload = f
        .request(
            "GET",
            "/api/v1/field-ops/appointments",
            &f.token,
            json!(null),
            None,
        )
        .await;
    f.finish().await;
    assert_eq!(reload.0, StatusCode::OK, "{}", reload.1);
    assert_eq!(
        reload.1["appointments"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["id"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["a2", "a"]
    );
}
#[tokio::test]
async fn appointment_notifications_keep_flat_identity_and_only_follow_new_commits() {
    let f = Fixture::new(true).await;
    let mesh = Arc::new(crate::mesh::transport::InProcessTransport::new());
    let app: axum::Router = crate::field_ops::configured_router(
        f.pool.clone(),
        Some(f.pool.clone()),
        mesh.clone(),
        f.auth.clone(),
    );
    let send = |body: Value, key: &str| {
        let request = Request::builder()
            .method("POST")
            .uri("/appointments")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", f.token))
            .header("idempotency-key", key)
            .body(Body::from(body.to_string()))
            .unwrap();
        app.clone().oneshot(request)
    };
    let body =
        json!({"id":"a","status":"In-Progress","expected_updated_at":"2026-10-03T00:00:00Z"});
    let first = send(body.clone(), "event-once").await.unwrap();
    assert_eq!(first.status(), StatusCode::OK);
    let first: Value =
        serde_json::from_slice(&to_bytes(first.into_body(), 65536).await.unwrap()).unwrap();
    let replay = send(body, "event-once").await.unwrap();
    assert_eq!(replay.status(), StatusCode::OK);
    sqlx::raw_sql("CREATE FUNCTION fail_event_commit() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN RAISE EXCEPTION 'synthetic event rollback';END$$;CREATE CONSTRAINT TRIGGER fail_event_commit AFTER UPDATE ON appointments DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION fail_event_commit();").execute(&f.admin).await.unwrap();
    let failed = send(
        json!({"id":"a","status":"Completed","expected_updated_at":first["updated_at"]}),
        "event-failed",
    )
    .await
    .unwrap();
    assert_eq!(failed.status(), StatusCode::SERVICE_UNAVAILABLE);
    let (count, topic, event) = {
        let events = mesh.events.lock().unwrap();
        (
            events.len(),
            events[0].0.clone(),
            serde_json::from_slice::<Value>(&events[0].1.payload).unwrap(),
        )
    };
    f.finish().await;
    assert_eq!(
        count, 1,
        "replay and rollback must not publish status events"
    );
    assert_eq!(topic, "job:status_changed");
    assert_eq!(event["id"], "a");
    assert_eq!(event["status"], "In-Progress");
    assert_eq!(event["tenant_id"], "tenant-a");
    assert_eq!(event["updated_at"], first["updated_at"]);
    assert!(
        event["notes"].is_null(),
        "an event must not add stored notes that were absent from the request"
    );
}

#[path = "offline_test.rs"]
mod offline_test;

#[path="offline_children_test.rs"]
mod offline_children_test;
