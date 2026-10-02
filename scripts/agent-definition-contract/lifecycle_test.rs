use super::*;
use server_auth::user_repository::UserRepository;

#[tokio::test]
async fn current_role_revocation_hides_public_content_but_preserves_private_inactive_copy() {
    let f = Fixture::sqlite().await;
    let published = f.publish(publish_request()).await;
    let path = format!(
        "/api/v1/agents/definitions/{}/install",
        published["definition"]["id"].as_str().unwrap()
    );
    let (status, installed) = f
        .request("POST", &path, Some(&f.foreign), install_request(&published))
        .await;
    assert_eq!(status, StatusCode::OK);
    let claims = f.auth.validate_token(&f.owner).await.unwrap();
    let repo = f.auth.portable_repo().unwrap();
    let mut user = repo.get_by_id(&claims.sub, "definition-a").await.unwrap();
    user.roles = vec!["STAFF".into()];
    repo.update_user(user, "definition-a").await.unwrap();
    let own = owner("definition-a", &claims.sub);
    assert!(matches!(
        f.store.publish(&own, &typed_publish()).await,
        Err(crate::agent_definitions::Error::Forbidden)
    ));
    let (_, catalogue) = f
        .request(
            "GET",
            "/api/v1/agents/definitions",
            Some(&f.foreign),
            Value::Null,
        )
        .await;
    assert!(
        !catalogue["definitions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["id"] == published["definition"]["id"])
    );
    assert_eq!(
        catalogue["installations"],
        json!([installed["installation"].clone()])
    );
    let mut exact_replay = install_request(&published);
    exact_replay["request_id"] = installed["request_id"].clone();
    let (status, recovered) = f
        .request("POST", &path, Some(&f.foreign), exact_replay)
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(recovered["replayed"], true);
    assert_eq!(recovered["installation"], installed["installation"]);

    let (status, _) = f
        .request(
            "POST",
            &path,
            Some(&f.same_tenant),
            install_request(&published),
        )
        .await;
    assert_eq!(status, StatusCode::CONFLICT);
    let claims = f.auth.validate_token(&f.foreign).await.unwrap();
    let recovered = f
        .store
        .operation(
            &owner("definition-b", &claims.sub),
            Uuid::parse_str(installed["request_id"].as_str().unwrap()).unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(recovered.installation.unwrap()).unwrap(),
        installed["installation"]
    );
    let (count,): (i64,) =
        sqlx::query_as("SELECT count(*) FROM agent_definition_authorities WHERE eligible=1")
            .fetch_one(&f.sqlite)
            .await
            .unwrap();
    assert_eq!(count, 3); // other two owners plus the explicit file-test owner
}

#[tokio::test]
async fn publisher_disable_delete_and_recreation_do_not_restore_old_discovery() {
    let f = Fixture::sqlite().await;
    let published = f.publish(publish_request()).await;
    let claims = f.auth.validate_token(&f.owner).await.unwrap();
    let repo = f.auth.portable_repo().unwrap();
    let mut user = repo.get_by_id(&claims.sub, "definition-a").await.unwrap();
    let old_key: String =
        sqlx::query_scalar("SELECT marketplace_authority_key FROM users WHERE id=$1")
            .bind(&user.id)
            .fetch_one(&f.sqlite)
            .await
            .unwrap();
    user.active = false;
    repo.update_user(user.clone(), "definition-a")
        .await
        .unwrap();
    let (_, catalogue) = f
        .request(
            "GET",
            "/api/v1/agents/definitions",
            Some(&f.foreign),
            Value::Null,
        )
        .await;
    assert!(
        !catalogue["definitions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["id"] == published["definition"]["id"])
    );
    repo.delete_user(&user.id, "definition-a").await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM agent_definition_operations WHERE user_id=$1"
        )
        .bind(&user.id)
        .fetch_one(&f.sqlite)
        .await
        .unwrap(),
        0
    );
    // This synthetic low-level recreation deliberately reuses an old ID. Normal
    // creation uses fresh UUIDs, but even this cannot rebind the old public key.
    user.active = true;
    // Auth email claims are intentionally not removed by delete_user; the new
    // identity uses a fresh email without weakening that existing ownership rule.
    user.email = "recreated-owner@example.test".into();
    repo.create_user(user.clone(), "definition-a")
        .await
        .unwrap();
    let new_key: String =
        sqlx::query_scalar("SELECT marketplace_authority_key FROM users WHERE id=$1")
            .bind(&user.id)
            .fetch_one(&f.sqlite)
            .await
            .unwrap();
    assert_ne!(old_key, new_key);
    let (_, catalogue) = f
        .request(
            "GET",
            "/api/v1/agents/definitions",
            Some(&f.foreign),
            Value::Null,
        )
        .await;
    assert!(
        !catalogue["definitions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["id"] == published["definition"]["id"])
    );
}

#[tokio::test]
async fn real_sqlite_pool_with_foreign_keys_disabled_fails_before_mutation() {
    let f = Fixture::sqlite().await;
    let claims = f.auth.validate_token(&f.owner).await.unwrap();
    let options = (*f.sqlite.connect_options()).clone().foreign_keys(false);
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>("PRAGMA foreign_keys")
            .fetch_one(&pool)
            .await
            .unwrap(),
        0
    );
    let store = DefinitionStore::Sqlite(pool.clone());
    assert!(matches!(
        store
            .publish(&owner("definition-a", &claims.sub), &typed_publish())
            .await,
        Err(crate::agent_definitions::Error::Unavailable)
    ));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definition_operations")
            .fetch_one(&f.sqlite)
            .await
            .unwrap(),
        0
    );
    pool.close().await;
}

#[tokio::test]
async fn repeated_portable_migration_must_not_regrant_revoked_normalized_role() {
    let path =
        std::env::temp_dir().join(format!("ohc-authority-migration-{}.sqlite", Uuid::new_v4()));
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
    sqlite_schema(&pool, &url).await;
    let old: String =
        sqlx::query_scalar("SELECT marketplace_authority_key FROM users WHERE id='disk-user'")
            .fetch_one(&pool)
            .await
            .unwrap();
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id='disk-user'")
        .execute(&pool)
        .await
        .unwrap();
    assert!(
        !sqlx::query_scalar::<_, bool>(
            "SELECT marketplace_eligible FROM users WHERE id='disk-user'"
        )
        .fetch_one(&pool)
        .await
        .unwrap()
    );
    let db = crate::persistence::AppDatabase::connect(&url)
        .await
        .unwrap();
    crate::persistence::migration::migrate(&db).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM identity_user_roles WHERE user_id='disk-user'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        0,
        "stale legacy mirror restored a revoked canonical role"
    );
    assert_eq!(
        sqlx::query_scalar::<_, String>(
            "SELECT marketplace_authority_key FROM users WHERE id='disk-user'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        old
    );
    assert!(
        !sqlx::query_scalar::<_, bool>(
            "SELECT marketplace_eligible FROM users WHERE id='disk-user'"
        )
        .fetch_one(&pool)
        .await
        .unwrap()
    );
    db.connection().clone().close().await.unwrap();
    pool.close().await;
    drop(pool);
    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn waiting_install_rechecks_revocation_with_inherited_repeatable_read_default() {
    let pg = PgFixture::new().await;
    let a = owner("pg-a", "owner");
    let b = owner("pg-b", "user-c");
    let published = pg
        .store
        .publish(&a, &typed_publish())
        .await
        .unwrap()
        .definition
        .unwrap();
    let pool = crate::db::secure_pg_pool_options()
        .max_connections(1)
        .after_connect(|c, _| {
            Box::pin(async move {
                sqlx::query("SET default_transaction_isolation TO 'repeatable read'")
                    .execute(c)
                    .await?;
                Ok(())
            })
        })
        .connect_with((*pg.pool.connect_options()).clone())
        .await
        .unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, String>("SHOW default_transaction_isolation")
            .fetch_one(&pool)
            .await
            .unwrap(),
        "repeatable read"
    );
    let mut revoke = pg.pool.begin().await.unwrap();
    server_common::auth_utils::set_org_context(&mut *revoke, "pg-a")
        .await
        .unwrap();
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id='owner'")
        .execute(&mut *revoke)
        .await
        .unwrap();
    let store = DefinitionStore::Postgres(pool.clone());
    let request = crate::agent_definitions::InstallRequest {
        request_id: Uuid::new_v4(),
        version: 1,
        digest: published.digest,
    };
    let task = tokio::spawn(async move {
        store
            .install(&b, Uuid::parse_str(&published.id).unwrap(), &request)
            .await
    });
    tokio::time::timeout(std::time::Duration::from_secs(5),async{loop{let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_locks l JOIN pg_stat_activity a ON a.pid=l.pid WHERE a.usename=$1 AND l.locktype='advisory' AND NOT l.granted)").bind(&pg.role).fetch_one(&pg.admin).await.unwrap();if waiting{break}tokio::task::yield_now().await;}}).await.unwrap();
    revoke.commit().await.unwrap();
    assert!(matches!(
        task.await.unwrap(),
        Err(crate::agent_definitions::Error::Conflict)
    ));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_definition_installations")
            .fetch_one(&pg.admin)
            .await
            .unwrap(),
        0
    );
    pool.close().await;
    pg.close().await;
}

#[tokio::test]
async fn unicode_newline_receipt_uses_the_actual_rust_digest_tuple() {
    let f = Fixture::sqlite().await;
    let mut request = publish_request();
    request["name"] = json!("研究者 🦀");
    request["description"] = json!("Line one\nQuoted \"text\" and café");
    request["role"] = json!("技術 writer");
    request["system_prompt"] = json!("Preserve\nUnicode 界 and slash / literally.\nNo tools.");
    let receipt = f.publish(request).await;
    let bytes = serde_json::to_vec_pretty(&receipt).unwrap();
    if let Ok(path) = std::env::var("OHC_AGENT_DEFINITION_GOLDEN_PATH") {
        std::fs::write(path, bytes).unwrap();
    }
    assert_eq!(receipt["definition"]["name"], "研究者 🦀");
}

#[tokio::test]
async fn initial_role_conversion_and_version_marker_commit_or_rollback_together() {
    use sea_orm::ConnectionTrait;
    let path =
        std::env::temp_dir().join(format!("ohc-authority-initial-{}.sqlite", Uuid::new_v4()));
    let url = format!("sqlite://{}?mode=rwc", path.display());
    let pool = sqlx::SqlitePool::connect(&url).await.unwrap();
    sqlx::raw_sql(include_str!("core_sqlite.sql"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!(
        "../../src/server/persistence/agent_definitions_sqlite.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO tenants(id,name) VALUES('legacy','legacy')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO users(id,tenant_id,username,email,roles) VALUES('legacy-user','legacy','legacy-user','legacy@example.test','[\"ADMIN\"]')").execute(&pool).await.unwrap();
    let db = crate::persistence::AppDatabase::connect(&url)
        .await
        .unwrap();
    let schema = sea_orm::Schema::new(sea_orm::DatabaseBackend::Sqlite);
    db.connection()
        .execute(
            sea_orm::DatabaseBackend::Sqlite.build(&schema.create_table_from_entity(
                server_auth::seaorm_store::entities::identity_user_role::Entity,
            )),
        )
        .await
        .unwrap();
    sqlx::raw_sql("CREATE TRIGGER reject_initial_role BEFORE INSERT ON identity_user_roles BEGIN SELECT RAISE(ABORT,'synthetic initial migration failure'); END;").execute(&pool).await.unwrap();
    assert!(crate::persistence::migration::migrate(&db).await.is_err());
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM onehumancorp_schema_versions WHERE id=$1"
        )
        .bind(crate::persistence::migration::PORTABLE_ROLE_SCHEMA_VERSION)
        .fetch_one(&pool)
        .await
        .unwrap(),
        0
    );
    sqlx::query("DROP TRIGGER reject_initial_role")
        .execute(&pool)
        .await
        .unwrap();
    crate::persistence::migration::migrate(&db).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, String>(
            "SELECT role_name FROM identity_user_roles WHERE user_id='legacy-user'"
        )
        .fetch_one(&pool)
        .await
        .unwrap(),
        "ADMIN"
    );
    assert!(
        sqlx::query_scalar::<_, bool>(
            "SELECT marketplace_eligible FROM users WHERE id='legacy-user'"
        )
        .fetch_one(&pool)
        .await
        .unwrap()
    );
    db.connection().clone().close().await.unwrap();
    pool.close().await;
    drop(pool);
    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn postgres_restart_does_not_restore_a_canonical_revocation_from_legacy_roles() {
    let pg = PgFixture::new().await;
    let mut t = pg.pool.begin().await.unwrap();
    server_common::auth_utils::set_org_context(&mut *t, "pg-a")
        .await
        .unwrap();
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id='owner'")
        .execute(&mut *t)
        .await
        .unwrap();
    t.commit().await.unwrap();
    let url = std::env::var("OHC_AGENT_DEFINITION_TEST_DATABASE_URL").unwrap();
    let db = crate::persistence::AppDatabase::connect(&format!(
        "{url}?options=-csearch_path%3D{}",
        pg.schema
    ))
    .await
    .unwrap();
    crate::persistence::migration::migrate(&db).await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM identity_user_roles WHERE user_id='owner'"
        )
        .fetch_one(&pg.admin)
        .await
        .unwrap(),
        0
    );
    assert!(
        !sqlx::query_scalar::<_, bool>("SELECT marketplace_eligible FROM users WHERE id='owner'")
            .fetch_one(&pg.admin)
            .await
            .unwrap()
    );
    db.connection().clone().close().await.unwrap();
    pg.close().await;
}
