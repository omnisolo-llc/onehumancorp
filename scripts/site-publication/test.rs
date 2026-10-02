use crate::builder::publication_store::*;
use sea_orm::{ConnectionTrait, Schema};
use serde_json::json;
use sqlx::{PgPool, Row};
use uuid::Uuid;

struct Fixture {
    admin: PgPool,
    pool: PgPool,
    schema: String,
    role: String,
    a: PublicationActor,
    b: PublicationActor,
    product_a: Uuid,
    product_b: Uuid,
}
impl Fixture {
    async fn new(legacy_tenant: bool) -> Self {
        let url =
            std::env::var("OHC_PUBLICATION_TEST_DATABASE_URL").expect("owned PostgreSQL required");
        let schema = format!("publication_store_{}", Uuid::new_v4().simple());
        let role = format!("publication_owner_{}", Uuid::new_v4().simple());
        let password = Uuid::new_v4().simple().to_string();
        let admin = sqlx::postgres::PgPoolOptions::new()
            .max_connections(3)
            .connect_with(
                url.parse::<sqlx::postgres::PgConnectOptions>()
                    .unwrap()
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        let initial = include_str!("../../src/server/migrations/001_initial.sql");
        for table in ["tenants", "users", "products"] {
            let start = initial
                .find(&format!("CREATE TABLE IF NOT EXISTS {table} ("))
                .unwrap();
            let end = initial[start..].find(");").unwrap() + start + 2;
            sqlx::raw_sql(&initial[start..end])
                .execute(&admin)
                .await
                .unwrap();
        }
        let orm = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(admin.clone());
        let backend = sea_orm::DatabaseBackend::Postgres;
        orm.execute(
            backend.build(&Schema::new(backend).create_table_from_entity(
                server_auth::seaorm_store::entities::identity_user_role::Entity,
            )),
        )
        .await
        .unwrap();
        sqlx::raw_sql(include_str!("../../src/server/migrations/009_builder.sql"))
            .execute(&admin)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/1019_site_publication_receipts.sql"
        ))
        .execute(&admin)
        .await
        .unwrap();
        let a = PublicationActor {
            user_id: Uuid::new_v4().to_string(),
            tenant_id: if legacy_tenant {
                format!("legacy-shop-{}", Uuid::new_v4().simple())
            } else {
                Uuid::new_v4().to_string()
            },
        };
        let b = PublicationActor {
            user_id: Uuid::new_v4().to_string(),
            tenant_id: Uuid::new_v4().to_string(),
        };
        for actor in [&a, &b] {
            sqlx::query("INSERT INTO tenants(id,name) VALUES($1,'Owned store')")
                .bind(&actor.tenant_id)
                .execute(&admin)
                .await
                .unwrap();
            sqlx::query("INSERT INTO users(id,username,email,tenant_id,active,roles) VALUES($1,$1,$1,$2,true,ARRAY['ADMIN'])").bind(&actor.user_id).bind(&actor.tenant_id).execute(&admin).await.unwrap();
            sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position) VALUES($1,'ADMIN',$2,0)")
                .bind(&actor.user_id).bind(&actor.tenant_id).execute(&admin).await.unwrap();
        }
        let product_a = Uuid::new_v4();
        let product_b = Uuid::new_v4();
        for (actor, product) in [(&a, product_a), (&b, product_b)] {
            sqlx::query("INSERT INTO products(id,tenant_id,title,price_cents) VALUES($1,$2,'Reviewed catalog item',1234)").bind(product.to_string()).bind(&actor.tenant_id).execute(&admin).await.unwrap();
        }
        for table in ["users", "identity_user_roles", "products"] {
            sqlx::raw_sql(&format!("ALTER TABLE {table} ENABLE ROW LEVEL SECURITY; ALTER TABLE {table} FORCE ROW LEVEL SECURITY; CREATE POLICY scoped ON {table} USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true));")).execute(&admin).await.unwrap();
        }
        sqlx::raw_sql(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}'; GRANT USAGE ON SCHEMA {schema} TO {role}; GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(8)
            .connect_with(
                url.parse::<sqlx::postgres::PgConnectOptions>()
                    .unwrap()
                    .username(&role)
                    .password(&password)
                    .application_name(&schema)
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        let identity:(String,String,bool,bool)=sqlx::query_as("SELECT session_user::text,current_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&pool).await.unwrap();
        assert_eq!(identity, (role.clone(), role.clone(), false, false));
        Self {
            admin,
            pool,
            schema,
            role,
            a,
            b,
            product_a,
            product_b,
        }
    }
    fn snapshot(&self, title: &str) -> SiteSnapshot {
        SiteSnapshot {
            domain: None,
            pages: vec![PublishedPage {
                path: "/".into(),
                title: title.into(),
                seo_metadata: json!({"name":title}),
                blocks: vec![PublishedBlock {
                    block_type: "ProductGridBlock".into(),
                    sort_order: 0,
                    content: json!({"items":[{"product_id":self.product_a.to_string(),"name":title,"price":"12.34"}]}),
                }],
            }],
        }
    }
    async fn counts(&self) -> (i64, i64, i64) {
        (
            sqlx::query_scalar("SELECT COUNT(*) FROM builder_sites")
                .fetch_one(&self.admin)
                .await
                .unwrap(),
            sqlx::query_scalar("SELECT COUNT(*) FROM builder_pages")
                .fetch_one(&self.admin)
                .await
                .unwrap(),
            sqlx::query_scalar("SELECT COUNT(*) FROM builder_publications")
                .fetch_one(&self.admin)
                .await
                .unwrap(),
        )
    }
    async fn finish(self) {
        self.pool.close().await;
        sqlx::raw_sql(&format!(
            "DROP SCHEMA {} CASCADE; DROP ROLE {};",
            self.schema, self.role
        ))
        .execute(&self.admin)
        .await
        .unwrap();
        self.admin.close().await;
    }
}

#[tokio::test]
async fn submission_atomically_persists_reviewed_snapshot_and_pending_job() {
    let f = Fixture::new(false).await;
    let op = Uuid::new_v4();
    let snapshot = f.snapshot("Reviewed");
    let receipt = submit_publication(&f.pool, &f.a, op, None, &snapshot).await;
    let counts = f.counts().await;
    let row=sqlx::query("SELECT tenant_id,owner_id,operation_id,snapshot,product_ids,status FROM builder_publications").fetch_optional(&f.admin).await.unwrap();
    let unscoped: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM builder_publications")
        .fetch_one(&f.pool)
        .await
        .unwrap();
    let expected = (
        f.a.tenant_id.clone(),
        f.a.user_id.clone(),
        f.product_a.to_string(),
    );
    f.finish().await;
    let receipt = receipt.unwrap();
    assert_eq!(receipt.status, PublicationStatus::Pending);
    assert_eq!(receipt.operation_id, op);
    assert!(receipt.public_path.is_none());
    assert_eq!(receipt.version, 1);
    assert_eq!(counts, (1, 1, 1));
    assert_eq!(unscoped, 0);
    let row = row.unwrap();
    assert_eq!(row.get::<String, _>("tenant_id"), expected.0);
    assert_eq!(row.get::<String, _>("owner_id"), expected.1);
    assert_eq!(row.get::<Uuid, _>("operation_id"), op);
    assert_eq!(
        row.get::<serde_json::Value, _>("snapshot"),
        serde_json::to_value(snapshot).unwrap()
    );
    assert_eq!(row.get::<Vec<String>, _>("product_ids"), vec![expected.2]);
    assert_eq!(row.get::<String, _>("status"), "pending");
}

#[tokio::test]
async fn concurrent_same_operation_returns_one_durable_receipt() {
    let f = Fixture::new(false).await;
    let op = Uuid::new_v4();
    let snapshot = f.snapshot("One");
    let mut work = Vec::new();
    for _ in 0..8 {
        let pool = f.pool.clone();
        let actor = f.a.clone();
        let snapshot = snapshot.clone();
        work.push(tokio::spawn(async move {
            submit_publication(&pool, &actor, op, None, &snapshot).await
        }));
    }
    let mut receipts = Vec::new();
    for item in work {
        receipts.push(item.await.unwrap().unwrap());
    }
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(counts, (1, 1, 1));
    assert!(
        receipts
            .iter()
            .all(|r| r.publication_id == receipts[0].publication_id
                && r.site_id == receipts[0].site_id
                && r.version == 1)
    );
}

#[tokio::test]
async fn conflicting_reuse_does_not_change_the_committed_snapshot() {
    let f = Fixture::new(false).await;
    let op = Uuid::new_v4();
    let first = submit_publication(&f.pool, &f.a, op, None, &f.snapshot("Original"))
        .await
        .unwrap();
    let result = submit_publication(&f.pool, &f.a, op, None, &f.snapshot("Changed")).await;
    let saved: String = sqlx::query_scalar(
        "SELECT snapshot->'pages'->0->>'title' FROM builder_publications WHERE publication_id=$1",
    )
    .bind(first.publication_id)
    .fetch_one(&f.admin)
    .await
    .unwrap();
    let counts = f.counts().await;
    f.finish().await;
    assert!(matches!(result, Err(PublicationError::Conflict)));
    assert_eq!(saved, "Original");
    assert_eq!(counts, (1, 1, 1));
}

#[tokio::test]
async fn current_authority_and_product_ownership_are_required_before_writes() {
    let f = Fixture::new(false).await;
    let mut snapshot = f.snapshot("Foreign");
    snapshot.pages[0].blocks[0].content["items"][0]["product_id"] = json!(f.product_b.to_string());
    let foreign = submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &snapshot).await;
    sqlx::query("UPDATE users SET roles=ARRAY['MEMBER'] WHERE id=$1")
        .bind(&f.a.user_id)
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query("UPDATE identity_user_roles SET role_name='MEMBER' WHERE user_id=$1")
        .bind(&f.a.user_id)
        .execute(&f.admin)
        .await
        .unwrap();
    let revoked =
        submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &f.snapshot("Denied")).await;
    let counts = f.counts().await;
    f.finish().await;
    assert!(matches!(foreign, Err(PublicationError::NotFound)));
    assert!(matches!(revoked, Err(PublicationError::Unauthorized)));
    assert_eq!(counts, (0, 0, 0));
}

#[tokio::test]
async fn legacy_string_tenant_mapping_keeps_real_owner_and_catalog_binding() {
    let f = Fixture::new(true).await;
    let result =
        submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &f.snapshot("Legacy")).await;
    let mapped: Option<Uuid> = sqlx::query_scalar("SELECT tenant_id FROM builder_sites")
        .fetch_optional(&f.admin)
        .await
        .unwrap();
    let expected = Uuid::new_v5(&Uuid::NAMESPACE_DNS, f.a.tenant_id.as_bytes());
    f.finish().await;
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(mapped, Some(expected));
}

#[tokio::test]
async fn owner_reassignment_cannot_replay_or_disclose_a_prior_receipt() {
    let f = Fixture::new(false).await;
    let op = Uuid::new_v4();
    let snapshot = f.snapshot("Owned");
    let first = submit_publication(&f.pool, &f.a, op, None, &snapshot)
        .await
        .unwrap();
    sqlx::query("UPDATE users SET tenant_id=$2 WHERE id=$1")
        .bind(&f.a.user_id)
        .bind(&f.b.tenant_id)
        .execute(&f.admin)
        .await
        .unwrap();
    let replay = submit_publication(&f.pool, &f.a, op, None, &snapshot).await;
    let foreign = submit_publication(
        &f.pool,
        &f.b,
        Uuid::new_v4(),
        Some(first.site_id),
        &SiteSnapshot {
            domain: None,
            pages: vec![PublishedPage {
                path: "/".into(),
                title: "Foreign".into(),
                seo_metadata: json!({}),
                blocks: vec![],
            }],
        },
    )
    .await;
    let counts = f.counts().await;
    f.finish().await;
    assert!(matches!(replay, Err(PublicationError::Unauthorized)));
    assert!(matches!(foreign, Err(PublicationError::NotFound)));
    assert_eq!(counts, (1, 1, 1));
}

#[tokio::test]
async fn a_new_review_gets_a_higher_site_version_without_mutating_the_old_snapshot() {
    let f = Fixture::new(false).await;
    let first = submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &f.snapshot("First"))
        .await
        .unwrap();
    let second = submit_publication(
        &f.pool,
        &f.a,
        Uuid::new_v4(),
        Some(first.site_id),
        &f.snapshot("Second"),
    )
    .await
    .unwrap();
    let rows:Vec<(i64,String)>=sqlx::query_as("SELECT site_version,snapshot->'pages'->0->>'title' FROM builder_publications ORDER BY site_version").fetch_all(&f.admin).await.unwrap();
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(first.site_id, second.site_id);
    assert_eq!(second.version, 2);
    assert_eq!(rows, vec![(1, "First".into()), (2, "Second".into())]);
    assert_eq!(counts, (1, 1, 2));
}

#[tokio::test]
async fn deferred_commit_failure_rolls_back_draft_and_receipt_together() {
    let f = Fixture::new(false).await;
    sqlx::raw_sql(&format!("CREATE SEQUENCE publication_commit_attempt; GRANT USAGE ON SEQUENCE publication_commit_attempt TO {}; CREATE FUNCTION reject_publication_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN PERFORM nextval('publication_commit_attempt'); RAISE EXCEPTION 'owned commit rejection'; END $$; CREATE CONSTRAINT TRIGGER reject_publication_commit AFTER INSERT ON builder_publications DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_publication_commit();", f.role)).execute(&f.admin).await.unwrap();
    let result =
        submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &f.snapshot("Rollback")).await;
    let counts = f.counts().await;
    let commit_observed: bool =
        sqlx::query_scalar("SELECT is_called FROM publication_commit_attempt")
            .fetch_one(&f.admin)
            .await
            .unwrap();
    f.finish().await;
    assert!(
        matches!(result, Err(PublicationError::Database(ref error)) if error.as_database_error().is_some_and(|e| e.message() == "owned commit rejection")),
        "{result:?}"
    );
    assert!(
        commit_observed,
        "the deferred commit trigger must actually execute"
    );
    assert_eq!(counts, (0, 0, 0));
}

#[tokio::test]
async fn protocol_relative_document_paths_are_rejected_without_a_receipt() {
    let f = Fixture::new(false).await;
    let mut snapshot = f.snapshot("Local");
    let mut external = snapshot.pages[0].clone();
    external.path = "//other.example/path".into();
    snapshot.pages.push(external);
    let result = submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &snapshot).await;
    let counts = f.counts().await;
    f.finish().await;
    assert!(
        matches!(result, Err(PublicationError::Invalid(_))),
        "{result:?}"
    );
    assert_eq!(counts, (0, 0, 0));
}

#[tokio::test]
async fn canonical_roles_authorize_publication_despite_an_empty_legacy_mirror() {
    let f = Fixture::new(false).await;
    sqlx::query("UPDATE users SET roles=ARRAY[]::TEXT[] WHERE id=$1")
        .bind(&f.a.user_id)
        .execute(&f.admin)
        .await
        .unwrap();
    let result = submit_publication(
        &f.pool,
        &f.a,
        Uuid::new_v4(),
        None,
        &f.snapshot("Canonical"),
    )
    .await;
    let counts = f.counts().await;
    f.finish().await;
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(counts, (1, 1, 1));
}

#[tokio::test]
async fn stale_legacy_admin_cannot_publish_after_canonical_role_revocation() {
    let f = Fixture::new(false).await;
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id=$1")
        .bind(&f.a.user_id)
        .execute(&f.admin)
        .await
        .unwrap();
    let result =
        submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &f.snapshot("Revoked")).await;
    let counts = f.counts().await;
    f.finish().await;
    assert!(
        matches!(result, Err(PublicationError::Unauthorized)),
        "{result:?}"
    );
    assert_eq!(counts, (0, 0, 0));
}

#[tokio::test]
async fn immutable_receipt_terms_and_revoked_state_survive_direct_sql_writers() {
    let f = Fixture::new(false).await;
    let receipt = submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &f.snapshot("Original"))
        .await
        .unwrap();
    let altered = sqlx::query("UPDATE builder_publications SET snapshot=jsonb_set(snapshot,'{pages,0,title}', '\"Altered\"') WHERE publication_id=$1")
        .bind(receipt.publication_id).execute(&f.admin).await;
    sqlx::query("UPDATE builder_publications SET status='published',rendered_pages='{}',rendered_sha256=$2 WHERE publication_id=$1")
        .bind(receipt.publication_id).bind("a".repeat(64)).execute(&f.admin).await.unwrap();
    let rerender = sqlx::query("UPDATE builder_publications SET rendered_pages='{\"/\":\"changed\"}' WHERE publication_id=$1")
        .bind(receipt.publication_id).execute(&f.admin).await;
    sqlx::query(
        "UPDATE builder_publications SET status='revoked',revoked_at=now() WHERE publication_id=$1",
    )
    .bind(receipt.publication_id)
    .execute(&f.admin)
    .await
    .unwrap();
    let resurrect = sqlx::query("UPDATE builder_publications SET status='published',revoked_at=NULL WHERE publication_id=$1")
        .bind(receipt.publication_id).execute(&f.admin).await;
    let state: (String, String) = sqlx::query_as("SELECT status,snapshot->'pages'->0->>'title' FROM builder_publications WHERE publication_id=$1")
        .bind(receipt.publication_id).fetch_one(&f.admin).await.unwrap();
    f.finish().await;
    assert!(altered.is_err());
    assert!(rerender.is_err());
    assert!(
        resurrect.is_err(),
        "revocation must not be undone by a stale writer"
    );
    assert_eq!(state, ("revoked".into(), "Original".into()));
}

#[tokio::test]
async fn pending_authority_change_is_rechecked_after_row_lock_wait() {
    for change_kind in ["account", "product", "role"] {
        let f = Fixture::new(false).await;
        let mut change = f.admin.begin().await.unwrap();
        if change_kind == "product" {
            sqlx::query("UPDATE products SET tenant_id=$2 WHERE id=$1")
                .bind(f.product_a.to_string())
                .bind(&f.b.tenant_id)
                .execute(&mut *change)
                .await
                .unwrap();
        } else if change_kind == "role" {
            sqlx::query("DELETE FROM identity_user_roles WHERE user_id=$1")
                .bind(&f.a.user_id)
                .execute(&mut *change)
                .await
                .unwrap();
        } else {
            sqlx::query("UPDATE users SET active=false WHERE id=$1")
                .bind(&f.a.user_id)
                .execute(&mut *change)
                .await
                .unwrap();
        }
        let pool = f.pool.clone();
        let actor = f.a.clone();
        let snapshot = f.snapshot("Racing");
        let task = tokio::spawn(async move {
            submit_publication(&pool, &actor, Uuid::new_v4(), None, &snapshot).await
        });
        let waiting = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            loop {
                let blocked: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE application_name=$1 AND wait_event_type='Lock')")
                    .bind(&f.schema).fetch_one(&f.admin).await.unwrap();
                if blocked { break; }
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        }).await;
        change.commit().await.unwrap();
        let result = task.await.unwrap();
        let counts = f.counts().await;
        f.finish().await;
        assert!(
            waiting.is_ok(),
            "publication must have waited for the actual authority row lock"
        );
        if change_kind == "product" {
            assert!(
                matches!(result, Err(PublicationError::NotFound)),
                "{result:?}"
            );
        } else {
            assert!(
                matches!(result, Err(PublicationError::Unauthorized)),
                "{result:?}"
            );
        }
        assert_eq!(counts, (0, 0, 0));
    }
}

#[tokio::test]
async fn published_rendering_cannot_be_reopened_then_replaced_in_two_updates() {
    for status in ["pending", "failed", "processing"] {
        let f = Fixture::new(false).await;
        let receipt = submit_publication(
            &f.pool,
            &f.a,
            Uuid::new_v4(),
            None,
            &f.snapshot("Immutable"),
        )
        .await
        .unwrap();
        sqlx::query("UPDATE builder_publications SET status='published',rendered_pages='{\"/\":\"original\"}',rendered_sha256=$2 WHERE publication_id=$1")
            .bind(receipt.publication_id).bind("a".repeat(64)).execute(&f.admin).await.unwrap();
        let reopen = sqlx::query("UPDATE builder_publications SET status=$2,lease_token=$3,lease_until=now()+interval '1 minute' WHERE publication_id=$1")
            .bind(receipt.publication_id).bind(status).bind(Uuid::new_v4()).execute(&f.admin).await;
        let replace = sqlx::query("UPDATE builder_publications SET rendered_pages='{\"/\":\"replacement\"}',rendered_sha256=$2 WHERE publication_id=$1")
            .bind(receipt.publication_id).bind("b".repeat(64)).execute(&f.admin).await;
        let stored: (String, serde_json::Value, String) = sqlx::query_as("SELECT status,rendered_pages,rendered_sha256 FROM builder_publications WHERE publication_id=$1")
            .bind(receipt.publication_id).fetch_one(&f.admin).await.unwrap();
        f.finish().await;
        assert!(reopen.is_err(), "published receipt reopened as {status}");
        assert!(
            replace.is_err(),
            "published rendering changed after {status} transition"
        );
        assert_eq!(
            stored,
            ("published".into(), json!({"/":"original"}), "a".repeat(64))
        );
    }
}

#[tokio::test]
async fn distinct_raw_tenants_cannot_share_authority_through_the_builder_uuid_mapping() {
    let mut f = Fixture::new(true).await;
    let first = submit_publication(
        &f.pool,
        &f.a,
        Uuid::new_v4(),
        None,
        &f.snapshot("First owner"),
    )
    .await
    .unwrap();
    let mapped = Uuid::new_v5(&Uuid::NAMESPACE_DNS, f.a.tenant_id.as_bytes()).to_string();
    assert_ne!(mapped, f.a.tenant_id);
    sqlx::query("INSERT INTO tenants(id,name) VALUES($1,'Distinct UUID tenant')")
        .bind(&mapped)
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query("UPDATE users SET tenant_id=$2 WHERE id=$1")
        .bind(&f.b.user_id)
        .bind(&mapped)
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query("UPDATE identity_user_roles SET tenant_id=$2 WHERE user_id=$1")
        .bind(&f.b.user_id)
        .bind(&mapped)
        .execute(&f.admin)
        .await
        .unwrap();
    f.b.tenant_id = mapped;
    let mut foreign_snapshot = f.snapshot("Other tenant rewrite");
    foreign_snapshot.pages[0].blocks.clear();
    let result = submit_publication(
        &f.pool,
        &f.b,
        Uuid::new_v4(),
        Some(first.site_id),
        &foreign_snapshot,
    )
    .await;
    let title: String = sqlx::query_scalar("SELECT title FROM builder_pages WHERE site_id=$1")
        .bind(first.site_id)
        .fetch_one(&f.admin)
        .await
        .unwrap();
    let counts = f.counts().await;
    f.finish().await;
    assert!(
        matches!(result, Err(PublicationError::NotFound)),
        "{result:?}"
    );
    assert_eq!(title, "First owner");
    assert_eq!(counts, (1, 1, 1));
}

#[tokio::test]
async fn a_legacy_site_without_raw_tenant_binding_is_not_silently_adopted() {
    let f = Fixture::new(true).await;
    let site = Uuid::new_v4();
    let mapped = Uuid::new_v5(&Uuid::NAMESPACE_DNS, f.a.tenant_id.as_bytes());
    sqlx::query("INSERT INTO builder_sites(id,tenant_id) VALUES($1,$2)")
        .bind(site)
        .bind(mapped)
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query("INSERT INTO builder_pages(id,tenant_id,site_id,path,title,seo_metadata) VALUES($1,$2,$3,'/','Legacy draft','{}')")
        .bind(Uuid::new_v4()).bind(mapped).bind(site).execute(&f.admin).await.unwrap();
    let result = submit_publication(
        &f.pool,
        &f.a,
        Uuid::new_v4(),
        Some(site),
        &f.snapshot("New review"),
    )
    .await;
    let title: String = sqlx::query_scalar("SELECT title FROM builder_pages WHERE site_id=$1")
        .bind(site)
        .fetch_one(&f.admin)
        .await
        .unwrap();
    let counts = f.counts().await;
    f.finish().await;
    assert!(
        matches!(result, Err(PublicationError::NotFound)),
        "{result:?}"
    );
    assert_eq!(title, "Legacy draft");
    assert_eq!(counts, (1, 1, 0));
}
