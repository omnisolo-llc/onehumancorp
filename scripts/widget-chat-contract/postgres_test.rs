use super::*;
use axum::body::to_bytes;
use sqlx::{
    PgPool,
    postgres::{PgConnectOptions, PgPoolOptions},
};
fn target(raw: &str) -> Result<PgConnectOptions, &'static str> {
    let u = url::Url::parse(raw).map_err(|_| "invalid database")?;
    let host = u
        .host_str()
        .ok_or("host")?
        .trim_start_matches('[')
        .trim_end_matches(']');
    let ip: std::net::IpAddr = host.parse().map_err(|_| "loopback IP required")?;
    let name = u.path().strip_prefix('/').ok_or("database")?;
    if !matches!(u.scheme(), "postgres" | "postgresql")
        || !ip.is_loopback()
        || u.query().is_some()
        || u.fragment().is_some()
        || raw.chars().any(char::is_control)
        || !name.starts_with("ohc_")
        || !name.ends_with("_test")
        || name.len() <= 9
        || name.len() > 63
        || !name.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_')
    {
        return Err("owned disposable database required");
    }
    raw.parse().map_err(|_| "invalid options")
}
struct PgFixture {
    admin: PgPool,
    pool: PgPool,
    auth: Arc<server_auth::Store>,
    a: Uuid,
    b: Uuid,
    user: String,
    token: String,
    foreign_token: String,
    ai: Uuid,
    ac: Uuid,
    bi: Uuid,
    bc: Uuid,
    schema: String,
    role: String,
}
impl PgFixture {
    async fn new() -> Self {
        let options = target(
            &std::env::var("OHC_WIDGET_TEST_DATABASE_URL")
                .expect("explicit disposable database required"),
        )
        .expect("unsafe database rejected before connection");
        let schema = format!("widget_case_{}", Uuid::new_v4().simple());
        let role = format!("widget_user_{}", Uuid::new_v4().simple());
        let password = Uuid::new_v4().simple().to_string();
        let admin = PgPoolOptions::new()
            .max_connections(4)
            .connect_with(options.clone().options([("search_path", schema.as_str())]))
            .await
            .unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .unwrap();
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/233_chat_omnichannel.sql"
        ))
        .execute(&admin)
        .await
        .unwrap();
        sqlx::raw_sql(include_str!(
            "../../src/server/migrations/1021_chat_sender_identity_text.sql"
        ))
        .execute(&admin)
        .await
        .unwrap();
        sqlx::raw_sql(&format!("CREATE ROLE {role} LOGIN NOSUPERUSER NOBYPASSRLS NOCREATEDB NOCREATEROLE PASSWORD '{password}';GRANT USAGE ON SCHEMA {schema} TO {role};GRANT SELECT,INSERT,UPDATE,DELETE ON ALL TABLES IN SCHEMA {schema} TO {role};")).execute(&admin).await.unwrap();
        let wrong = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(1))
            .connect_with(
                options
                    .clone()
                    .username(&role)
                    .password("definitely-wrong-fixture-password")
                    .options([("search_path", schema.as_str())]),
            )
            .await;
        assert!(
            wrong.is_err(),
            "private cluster must enforce the fixture's actual password authentication"
        );
        let pool = crate::db::secure_pg_pool_options()
            .max_connections(1)
            .acquire_timeout(Duration::from_secs(3))
            .connect_with(
                options
                    .username(&role)
                    .password(&password)
                    .options([("search_path", schema.as_str())]),
            )
            .await
            .unwrap();
        let identity:(String,String,bool,bool)=sqlx::query_as("SELECT current_user::text,session_user::text,rolsuper,rolbypassrls FROM pg_roles WHERE rolname=current_user").fetch_one(&pool).await.unwrap();
        assert_eq!(identity, (role.clone(), role.clone(), false, false));
        let auth = Arc::new(server_auth::Store::new());
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        let mut tokens = vec![];
        let mut user = String::new();
        for tenant in [a, b] {
            let u = auth
                .create_user(
                    tenant.to_string(),
                    format!("{tenant}@example.test"),
                    "public-synthetic-fixture-password".into(),
                    vec!["ADMIN".into()],
                    tenant.to_string(),
                )
                .await
                .unwrap();
            if tenant == a {
                user = u.id.clone()
            };
            tokens.push(auth.issue_token(&u).unwrap());
        }
        let ai = Uuid::new_v4();
        let ac = Uuid::new_v4();
        let bi = Uuid::new_v4();
        let bc = Uuid::new_v4();
        for (tenant, inbox, contact) in [(a, ai, ac), (b, bi, bc)] {
            sqlx::query("INSERT INTO chat_inboxes(id,tenant_id,name) VALUES($1,$2,'Synthetic configured inbox')").bind(inbox).bind(tenant).execute(&admin).await.unwrap();
            sqlx::query("INSERT INTO chat_contacts(id,tenant_id,name) VALUES($1,$2,'Synthetic configured contact')").bind(contact).bind(tenant).execute(&admin).await.unwrap();
        }
        Self {
            admin,
            pool,
            auth,
            a,
            b,
            user,
            token: tokens.remove(0),
            foreign_token: tokens.remove(0),
            ai,
            ac,
            bi,
            bc,
            schema,
            role,
        }
    }
    fn app(&self, owner_pool: bool) -> axum::Router {
        crate::actual_parent_mount(
            Arc::new(crate::db::DB {
                pool: if owner_pool {
                    self.admin.clone()
                } else {
                    self.pool.clone()
                },
                store: crate::db::DbStore::Postgres,
            }),
            self.auth.clone(),
        )
    }
    async fn request(
        &self,
        owner_pool: bool,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: Value,
    ) -> (StatusCode, Value) {
        let mut req = Request::builder()
            .method(method)
            .uri(path)
            .header("content-type", "application/json")
            .header("x-tenant-id", self.b.to_string());
        if let Some(token) = token {
            req = req.header("authorization", format!("Bearer {token}"));
        }
        let response = self
            .app(owner_pool)
            .oneshot(req.body(Body::from(body.to_string())).unwrap())
            .await
            .unwrap();
        let status = response.status();
        assert_eq!(
            response
                .headers()
                .get("cache-control")
                .and_then(|v| v.to_str().ok()),
            Some("private, no-store")
        );
        let bytes = to_bytes(response.into_body(), 2_097_152).await.unwrap();
        (
            status,
            if bytes.is_empty() {
                Value::Null
            } else {
                match serde_json::from_slice(&bytes) {
                    Ok(value) => value,
                    Err(error) if status.is_success() => {
                        panic!("successful response must be valid JSON: {error}")
                    }
                    Err(_) => Value::String(
                        String::from_utf8(bytes.to_vec()).expect("Axum rejection must be UTF-8"),
                    ),
                }
            },
        )
    }
    fn conversation(&self, foreign: bool) -> Value {
        if foreign {
            json!({"tenant_id":self.b,"inbox_id":self.bi,"contact_id":self.bc})
        } else {
            json!({"tenant_id":self.a,"inbox_id":self.ai,"contact_id":self.ac})
        }
    }
    async fn seed_conversation(&self, foreign: bool) -> Uuid {
        let id = Uuid::new_v4();
        let (t, i, c) = if foreign {
            (self.b, self.bi, self.bc)
        } else {
            (self.a, self.ai, self.ac)
        };
        sqlx::query(
            "INSERT INTO chat_conversations(id,tenant_id,inbox_id,contact_id) VALUES($1,$2,$3,$4)",
        )
        .bind(id)
        .bind(t)
        .bind(i)
        .bind(c)
        .execute(&self.admin)
        .await
        .unwrap();
        id
    }
    async fn counts(&self) -> (i64, i64) {
        sqlx::query_as(
            "SELECT(SELECT count(*) FROM chat_conversations),(SELECT count(*) FROM chat_messages)",
        )
        .fetch_one(&self.admin)
        .await
        .unwrap()
    }
    async fn finish(self) {
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
#[test]
fn unsafe_database_urls_are_rejected_before_any_connection() {
    for u in [
        "postgres://remote.example/ohc_widget_test",
        "postgres://127.0.0.1/production",
        "postgres://127.0.0.1/ohc_widget_test?host=remote",
        "postgres://127.0.0.1/ohc_widget_test#fragment",
    ] {
        assert!(target(u).is_err(), "{u}");
    }
    assert!(target("postgres://127.0.0.1:55439/ohc_widget_test").is_ok());
}
#[tokio::test]
async fn actual_owned_conversation_message_and_history_round_trip() {
    let f = PgFixture::new().await;
    let created = f
        .request(
            false,
            "POST",
            "/api/widget/conversations",
            Some(&f.token),
            f.conversation(false),
        )
        .await;
    let id = created.1["id"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .unwrap_or(Uuid::nil());
    let content = "Actual Unicode 🦀\nrecorded exactly";
    let sent = f
        .request(
            false,
            "POST",
            "/api/widget/messages",
            Some(&f.token),
            json!({"tenant_id":f.a,"conversation_id":id,"content":content}),
        )
        .await;
    let history = f
        .request(
            false,
            "GET",
            &format!("/api/widget/{}/conversations/{id}/messages", f.a),
            Some(&f.token),
            Value::Null,
        )
        .await;
    let stored:Option<(Uuid,Uuid,String,Option<String>,String)>=sqlx::query_as("SELECT tenant_id,conversation_id,sender_type,sender_id::text,content FROM chat_messages WHERE conversation_id=$1").bind(id).fetch_optional(&f.admin).await.unwrap();
    let expected = (
        f.a,
        id,
        "agent".to_string(),
        Some(f.user.clone()),
        content.to_string(),
    );
    let ai = f.ai;
    let ac = f.ac;
    f.finish().await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    assert_eq!(created.1["inbox_id"], ai.to_string());
    assert_eq!(created.1["contact_id"], ac.to_string());
    assert_eq!(created.1["status"], "open");
    assert_eq!(sent.0, StatusCode::OK, "{sent:?}");
    assert_eq!(stored, Some(expected));
    assert_eq!(history.0, StatusCode::OK);
    assert_eq!(history.1["messages"].as_array().unwrap().len(), 1);
    assert_eq!(history.1["messages"][0]["id"], sent.1["id"]);
}
#[tokio::test]
async fn foreign_or_missing_parents_are_refused_even_with_table_owner_pool() {
    let f = PgFixture::new().await;
    let mut outcomes = vec![];
    for owner_pool in [false, true] {
        for (i, c) in [
            (f.bi, f.ac),
            (f.ai, f.bc),
            (Uuid::new_v4(), f.ac),
            (f.ai, Uuid::new_v4()),
        ] {
            outcomes.push(
                f.request(
                    owner_pool,
                    "POST",
                    "/api/widget/conversations",
                    Some(&f.token),
                    json!({"tenant_id":f.a,"inbox_id":i,"contact_id":c}),
                )
                .await
                .0,
            );
        }
    }
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(outcomes, vec![StatusCode::NOT_FOUND; 8]);
    assert_eq!(counts, (0, 0));
}
#[tokio::test]
async fn foreign_conversations_cannot_receive_or_disclose_messages() {
    let f = PgFixture::new().await;
    let foreign = f.seed_conversation(true).await;
    let mut outcomes = vec![];
    for owner_pool in [false, true] {
        for id in [foreign, Uuid::new_v4()] {
            outcomes.push(
                f.request(
                    owner_pool,
                    "POST",
                    "/api/widget/messages",
                    Some(&f.token),
                    json!({"tenant_id":f.a,"conversation_id":id,"content":"must not persist"}),
                )
                .await
                .0,
            );
            outcomes.push(
                f.request(
                    owner_pool,
                    "GET",
                    &format!("/api/widget/{}/conversations/{id}/messages", f.a),
                    Some(&f.token),
                    Value::Null,
                )
                .await
                .0,
            );
        }
    }
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(outcomes, vec![StatusCode::NOT_FOUND; 8]);
    assert_eq!(counts, (1, 0));
}
#[tokio::test]
async fn disabled_revoked_or_nonexistent_identities_cannot_write_real_chat() {
    let f = PgFixture::new().await;
    f.auth
        .update_user(&f.user, None, None, Some(false), &f.a.to_string())
        .await
        .unwrap();
    let disabled = f
        .request(
            false,
            "POST",
            "/api/widget/conversations",
            Some(&f.token),
            f.conversation(false),
        )
        .await
        .0;
    f.auth
        .update_user(&f.user, None, None, Some(true), &f.a.to_string())
        .await
        .unwrap();
    let restored = f.auth.get_user(&f.user, &f.a.to_string()).await.unwrap();
    let fresh_token = f.auth.issue_token(&restored).unwrap();
    let claims = f.auth.validate_token(&fresh_token).await.unwrap();
    f.auth
        .revoke_token(
            claims.jti,
            chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
            &f.a.to_string(),
        )
        .await
        .unwrap();
    let revoked = f
        .request(
            false,
            "POST",
            "/api/widget/conversations",
            Some(&fresh_token),
            f.conversation(false),
        )
        .await
        .0;
    let mut absent = f.auth.get_user(&f.user, &f.a.to_string()).await.unwrap();
    absent.id = Uuid::new_v4().to_string();
    let token = f.auth.issue_token(&absent).unwrap();
    let missing = f
        .request(
            false,
            "POST",
            "/api/widget/conversations",
            Some(&token),
            f.conversation(false),
        )
        .await
        .0;
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(
        (disabled, revoked, missing),
        (
            StatusCode::UNAUTHORIZED,
            StatusCode::UNAUTHORIZED,
            StatusCode::UNAUTHORIZED
        )
    );
    assert_eq!(counts, (0, 0));
}
#[tokio::test]
async fn caller_cannot_forge_sender_or_create_implicit_parents() {
    let f = PgFixture::new().await;
    let id = f.seed_conversation(false).await;
    let spoof=f.request(false,"POST","/api/widget/messages",Some(&f.token),json!({"tenant_id":f.a,"conversation_id":id,"content":"forged","sender_type":"bot","sender_id":Uuid::new_v4(),"direction":"OUTBOUND"})).await.0;
    let missing = f
        .request(
            false,
            "POST",
            "/api/widget/conversations",
            Some(&f.token),
            json!({"tenant_id":f.a}),
        )
        .await
        .0;
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(spoof, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(missing, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(counts, (1, 0));
}
#[tokio::test]
async fn reused_restricted_connection_returns_only_current_tenant_history() {
    let f = PgFixture::new().await;
    let a = f.seed_conversation(false).await;
    let b = f.seed_conversation(true).await;
    for (t, c) in [(f.a, a), (f.b, b)] {
        sqlx::query("INSERT INTO chat_messages(id,tenant_id,conversation_id,sender_type,content) VALUES($1,$2,$3,'contact',$4)").bind(Uuid::new_v4()).bind(t).bind(c).bind(t.to_string()).execute(&f.admin).await.unwrap();
    }
    let mut reads = vec![];
    for (t, c, token) in [
        (f.a, a, &f.token),
        (f.b, b, &f.foreign_token),
        (f.a, a, &f.token),
    ] {
        reads.push(
            f.request(
                false,
                "GET",
                &format!("/api/widget/{t}/conversations/{c}/messages"),
                Some(token),
                Value::Null,
            )
            .await,
        );
    }
    let a = f.a;
    let b = f.b;
    f.finish().await;
    for ((status, body), tenant) in reads.into_iter().zip([a, b, a]) {
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["messages"].as_array().unwrap().len(), 1);
        assert_eq!(body["messages"][0]["content"], tenant.to_string());
    }
}
#[tokio::test]
async fn deferred_commit_failure_does_not_return_a_created_message() {
    let f = PgFixture::new().await;
    let id = f.seed_conversation(false).await;
    sqlx::raw_sql("CREATE FUNCTION reject_widget_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'synthetic deferred commit rejection'; END $$;CREATE CONSTRAINT TRIGGER reject_widget_commit AFTER INSERT ON chat_messages DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_widget_commit();").execute(&f.admin).await.unwrap();
    let result = f
        .request(
            false,
            "POST",
            "/api/widget/messages",
            Some(&f.token),
            json!({"tenant_id":f.a,"conversation_id":id,"content":"must rollback"}),
        )
        .await;
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(result.0, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(counts, (1, 0));
}
#[tokio::test]
async fn an_owned_empty_conversation_is_distinct_from_an_unknown_conversation() {
    let f = PgFixture::new().await;
    let id = f.seed_conversation(false).await;
    let empty = f
        .request(
            false,
            "GET",
            &format!("/api/widget/{}/conversations/{id}/messages", f.a),
            Some(&f.token),
            Value::Null,
        )
        .await;
    let absent = f
        .request(
            false,
            "GET",
            &format!(
                "/api/widget/{}/conversations/{}/messages",
                f.a,
                Uuid::new_v4()
            ),
            Some(&f.token),
            Value::Null,
        )
        .await;
    f.finish().await;
    assert_eq!(
        empty,
        (StatusCode::OK, json!({"messages":[],"next_cursor":null}))
    );
    assert_eq!(absent.0, StatusCode::NOT_FOUND);
}
#[tokio::test]
async fn actual_233_and_canonical_1009_migrate_in_both_installation_orders_without_checksum_changes()
 {
    use sqlx::migrate::{Migration, MigrationType, Migrator};
    use std::borrow::Cow;
    let raw = std::env::var("OHC_WIDGET_TEST_DATABASE_URL").unwrap();
    let options = target(&raw).unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let active = root.join("src/server/migrations/1009_native_omnichannel_chat.sql");
    let historical = include_str!("compatibility/1009_native_omnichannel_chat.sql");
    let sql1009 = if active.is_file() {
        let current = std::fs::read_to_string(active).unwrap();
        assert_eq!(
            current, historical,
            "paired migration bytes changed; review compatibility again"
        );
        current
    } else {
        historical.to_string()
    };
    let m233 = Migration::new(
        233,
        "chat omnichannel".into(),
        MigrationType::Simple,
        include_str!("../../src/server/migrations/233_chat_omnichannel.sql").into(),
        false,
    );
    let m1009 = Migration::new(
        1009,
        "native omnichannel chat".into(),
        MigrationType::Simple,
        sql1009.into(),
        false,
    );
    for first1009 in [false, true] {
        let schema = format!("widget_migration_{}", Uuid::new_v4().simple());
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_with(options.clone().options([("search_path", schema.as_str())]))
            .await
            .unwrap();
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&pool)
            .await
            .unwrap();
        if first1009 {
            Migrator {
                migrations: Cow::Owned(vec![m1009.clone()]),
                ..Migrator::DEFAULT
            }
            .run(&pool)
            .await
            .unwrap();
        }
        let both = Migrator {
            migrations: Cow::Owned(vec![m233.clone(), m1009.clone()]),
            ..Migrator::DEFAULT
        };
        both.run(&pool).await.unwrap();
        both.run(&pool).await.unwrap();
        let tenant = Uuid::new_v4();
        let inbox = Uuid::new_v4();
        let contact = Uuid::new_v4();
        let conversation = Uuid::new_v4();
        let message = Uuid::new_v4();
        sqlx::query("INSERT INTO chat_inboxes(id,tenant_id,name) VALUES($1,$2,'legacy')")
            .bind(inbox)
            .bind(tenant)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO chat_contacts(id,tenant_id,name) VALUES($1,$2,'legacy')")
            .bind(contact)
            .bind(tenant)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "INSERT INTO chat_conversations(id,tenant_id,inbox_id,contact_id) VALUES($1,$2,$3,$4)",
        )
        .bind(conversation)
        .bind(tenant)
        .bind(inbox)
        .bind(contact)
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query("INSERT INTO chat_messages(id,tenant_id,conversation_id,sender_type,sender_id,content) VALUES($1,$2,$3,'contact',$4,'legacy bytes')").bind(message).bind(tenant).bind(conversation).bind(contact).execute(&pool).await.unwrap();
        let bot_message = Uuid::new_v4();
        sqlx::query("INSERT INTO chat_messages(id,tenant_id,conversation_id,sender_type,sender_id,content) VALUES($1,$2,$3,'bot',NULL,'legacy bot bytes')").bind(bot_message).bind(tenant).bind(conversation).execute(&pool).await.unwrap();
        sqlx::query("CREATE INDEX retained_sender_index ON chat_messages(sender_id)")
            .execute(&pool)
            .await
            .unwrap();
        let m1021 = Migration::new(
            1021,
            "chat sender identity text".into(),
            MigrationType::Simple,
            include_str!("../../src/server/migrations/1021_chat_sender_identity_text.sql").into(),
            false,
        );
        let full = Migrator {
            migrations: Cow::Owned(vec![m233.clone(), m1009.clone(), m1021.clone()]),
            ..Migrator::DEFAULT
        };
        full.run(&pool).await.unwrap();
        full.run(&pool).await.unwrap();
        let preserved: (String, String, String) =
            sqlx::query_as("SELECT sender_type,sender_id,content FROM chat_messages WHERE id=$1")
                .bind(message)
                .fetch_one(&pool)
                .await
                .unwrap();
        let preserved_bot: (String, Option<String>, String) =
            sqlx::query_as("SELECT sender_type,sender_id,content FROM chat_messages WHERE id=$1")
                .bind(bot_message)
                .fetch_one(&pool)
                .await
                .unwrap();
        let valid_index:bool=sqlx::query_scalar("SELECT indisvalid FROM pg_index i JOIN pg_class c ON c.oid=i.indexrelid WHERE c.relname='retained_sender_index' AND c.relnamespace=current_schema()::regnamespace").fetch_one(&pool).await.unwrap();
        let data_type:String=sqlx::query_scalar("SELECT data_type FROM information_schema.columns WHERE table_schema=current_schema() AND table_name='chat_messages' AND column_name='sender_id'").fetch_one(&pool).await.unwrap();
        let versions: Vec<(i64, Vec<u8>)> =
            sqlx::query_as("SELECT version,checksum FROM _sqlx_migrations ORDER BY version")
                .fetch_all(&pool)
                .await
                .unwrap();
        let policies: i64 =
            sqlx::query_scalar("SELECT count(*) FROM pg_policies WHERE schemaname=$1")
                .bind(&schema)
                .fetch_one(&pool)
                .await
                .unwrap();
        let legacy:bool=sqlx::query_scalar("SELECT to_regclass('conversations') IS NOT NULL OR to_regclass('messages') IS NOT NULL").fetch_one(&pool).await.unwrap();
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&pool)
            .await
            .unwrap();
        pool.close().await;
        assert_eq!(
            versions,
            vec![
                (233, m233.checksum.to_vec()),
                (1009, m1009.checksum.to_vec()),
                (1021, m1021.checksum.to_vec())
            ]
        );
        assert_eq!(
            preserved,
            ("contact".into(), contact.to_string(), "legacy bytes".into())
        );
        assert_eq!(
            preserved_bot,
            ("bot".into(), None, "legacy bot bytes".into())
        );
        assert!(valid_index);
        assert_eq!(data_type, "text");
        assert_eq!(policies, 10);
        assert!(
            !legacy,
            "canonical migrations do not create parallel legacy tables"
        );
    }
}
#[tokio::test]
async fn legacy_cross_tenant_parent_links_are_not_adopted_by_message_operations() {
    let f = PgFixture::new().await;
    let id = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO chat_conversations(id,tenant_id,inbox_id,contact_id) VALUES($1,$2,$3,$4)",
    )
    .bind(id)
    .bind(f.a)
    .bind(f.bi)
    .bind(f.bc)
    .execute(&f.admin)
    .await
    .unwrap();
    let mut outcomes = vec![];
    for owner_pool in [false, true] {
        outcomes.push(
            f.request(
                owner_pool,
                "POST",
                "/api/widget/messages",
                Some(&f.token),
                json!({"tenant_id":f.a,"conversation_id":id,"content":"must not adopt"}),
            )
            .await
            .0,
        );
        outcomes.push(
            f.request(
                owner_pool,
                "GET",
                &format!("/api/widget/{}/conversations/{id}/messages", f.a),
                Some(&f.token),
                Value::Null,
            )
            .await
            .0,
        );
    }
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(outcomes, vec![StatusCode::NOT_FOUND; 4]);
    assert_eq!(counts, (1, 0));
}
#[tokio::test]
async fn opaque_actor_ids_from_the_actual_stored_identity_are_preserved_without_aliases() {
    use server_auth::user_repository::UserRepository;
    let mut f = PgFixture::new().await;
    let auth_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::raw_sql(include_str!("auth_sqlite.sql"))
        .execute(&auth_pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO tenants(id,name) VALUES($1,'synthetic actor account')")
        .bind(f.a.to_string())
        .execute(&auth_pool)
        .await
        .unwrap();
    let repo = Arc::new(server_auth::sqlite_store::SqliteUserRepository::new(
        auth_pool.clone(),
    ));
    let raw = "principal:user/été_東京_42";
    let now = chrono::Utc::now();
    let user = server_auth::User {
        id: raw.into(),
        username: "stored-opaque-actor".into(),
        email: "opaque@example.test".into(),
        password_hash: String::new(),
        roles: vec!["ADMIN".into()],
        active: true,
        organization_id: Some(f.a.to_string()),
        created_at: now,
        updated_at: now,
        oidc_subject: None,
    };
    repo.create_user(user.clone(), &f.a.to_string())
        .await
        .unwrap();
    f.auth = Arc::new(server_auth::Store::with_repo(repo));
    f.user = raw.into();
    f.token = f.auth.issue_token(&user).unwrap();
    let id = f.seed_conversation(false).await;
    let sent = f
        .request(
            false,
            "POST",
            "/api/widget/messages",
            Some(&f.token),
            json!({"tenant_id":f.a,"conversation_id":id,"content":"exact actor"}),
        )
        .await;
    let stored: Option<(String, String)> =
        sqlx::query_as("SELECT sender_type,sender_id FROM chat_messages WHERE conversation_id=$1")
            .bind(id)
            .fetch_optional(&f.admin)
            .await
            .unwrap();
    f.finish().await;
    auth_pool.close().await;
    assert_eq!(sent.0, StatusCode::OK, "{sent:?}");
    assert_eq!(sent.1["sender_id"], raw);
    assert_eq!(stored, Some(("agent".into(), raw.into())));
}
#[tokio::test]
async fn sender_migration_rejects_an_incompatible_foreign_key_without_dropping_it() {
    let options = target(&std::env::var("OHC_WIDGET_TEST_DATABASE_URL").unwrap()).unwrap();
    let schema = format!("widget_fk_{}", Uuid::new_v4().simple());
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options.options([("search_path", schema.as_str())]))
        .await
        .unwrap();
    sqlx::query(&format!("CREATE SCHEMA {schema}"))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(include_str!(
        "../../src/server/migrations/233_chat_omnichannel.sql"
    ))
    .execute(&pool)
    .await
    .unwrap();
    sqlx::raw_sql("ALTER TABLE chat_messages ADD CONSTRAINT existing_sender_unique UNIQUE(sender_id); CREATE TABLE existing_sender_reference(sender_id UUID REFERENCES chat_messages(sender_id));").execute(&pool).await.unwrap();
    let mut tx = pool.begin().await.unwrap();
    let result = sqlx::raw_sql(include_str!(
        "../../src/server/migrations/1021_chat_sender_identity_text.sql"
    ))
    .execute(&mut *tx)
    .await;
    tx.rollback().await.unwrap();
    let data_type:String=sqlx::query_scalar("SELECT data_type FROM information_schema.columns WHERE table_schema=current_schema() AND table_name='chat_messages' AND column_name='sender_id'").fetch_one(&pool).await.unwrap();
    let constraints:i64=sqlx::query_scalar("SELECT count(*) FROM pg_constraint WHERE conrelid='existing_sender_reference'::regclass AND contype='f'").fetch_one(&pool).await.unwrap();
    sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
        .execute(&pool)
        .await
        .unwrap();
    pool.close().await;
    assert!(
        result.is_err(),
        "do not silently drop or reinterpret a dependent foreign key"
    );
    assert_eq!(data_type, "uuid");
    assert_eq!(constraints, 1);
}
#[tokio::test]
async fn unsupported_sqlite_chat_storage_fails_explicitly_without_using_another_pool() {
    let f = PgFixture::new().await;
    let sqlite = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    let app = crate::actual_parent_mount(
        Arc::new(crate::db::DB {
            pool: f.pool.clone(),
            store: crate::db::DbStore::Sqlite(sqlite.clone()),
        }),
        f.auth.clone(),
    );
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/widget/conversations")
                .header("authorization", format!("Bearer {}", f.token))
                .header("content-type", "application/json")
                .body(Body::from(f.conversation(false).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let counts = f.counts().await;
    let status = response.status();
    f.finish().await;
    sqlite.close().await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(counts, (0, 0));
}
#[tokio::test]
async fn message_content_limit_rejects_before_persistence_and_preserves_valid_bytes() {
    let f = PgFixture::new().await;
    let id = f.seed_conversation(false).await;
    let mut rejected = vec![];
    for content in [
        "x".repeat(16_385),
        "é".repeat(8_193),
        "   ".into(),
        "nul\0byte".into(),
    ] {
        rejected.push(
            f.request(
                false,
                "POST",
                "/api/widget/messages",
                Some(&f.token),
                json!({"tenant_id":f.a,"conversation_id":id,"content":content}),
            )
            .await
            .0,
        );
    }
    let accepted = f
        .request(
            false,
            "POST",
            "/api/widget/messages",
            Some(&f.token),
            json!({"tenant_id":f.a,"conversation_id":id,"content":"x".repeat(16_384)}),
        )
        .await;
    let counts = f.counts().await;
    f.finish().await;
    assert_eq!(rejected, vec![StatusCode::BAD_REQUEST; 4]);
    assert_eq!(accepted.0, StatusCode::OK);
    assert_eq!(accepted.1["content"].as_str().unwrap().len(), 16_384);
    assert_eq!(counts, (1, 1));
}
#[tokio::test]
async fn bounded_history_pages_recover_every_record_without_silent_truncation() {
    let f = PgFixture::new().await;
    let id = f.seed_conversation(false).await;
    for _ in 0..101 {
        sqlx::query("INSERT INTO chat_messages(id,tenant_id,conversation_id,sender_type,content) VALUES($1,$2,$3,'contact',repeat('x',10000))").bind(Uuid::new_v4()).bind(f.a).bind(id).execute(&f.admin).await.unwrap();
    }
    let mut cursor = None::<String>;
    let mut ids = std::collections::BTreeSet::new();
    let mut bounded = true;
    let mut page_shape = true;
    for _ in 0..12 {
        let mut path = format!("/api/widget/{}/conversations/{id}/messages?limit=100", f.a);
        if let Some(c) = cursor.as_ref() {
            path.push_str("&cursor=");
            path.push_str(c);
        }
        let (status, body) = f
            .request(false, "GET", &path, Some(&f.token), Value::Null)
            .await;
        assert_eq!(status, StatusCode::OK);
        bounded &= serde_json::to_vec(&body).unwrap().len() <= 262_144;
        if !body["messages"].is_array() {
            page_shape = false;
            break;
        }
        let rows = body["messages"].as_array().unwrap();
        bounded &= rows.len() <= 100;
        for row in rows {
            assert!(
                ids.insert(row["id"].as_str().unwrap().to_string()),
                "duplicate cursor record"
            );
        }
        cursor = body["next_cursor"].as_str().map(str::to_string);
        if cursor.is_none() {
            break;
        }
    }
    let small = f.seed_conversation(false).await;
    sqlx::query("INSERT INTO chat_messages(id,tenant_id,conversation_id,sender_type,content) SELECT gen_random_uuid(),$1,$2,'contact','small' FROM generate_series(1,101)").bind(f.a).bind(small).execute(&f.admin).await.unwrap();
    let default_page = f
        .request(
            false,
            "GET",
            &format!("/api/widget/{}/conversations/{small}/messages", f.a),
            Some(&f.token),
            Value::Null,
        )
        .await;
    let next = default_page.1["next_cursor"].as_str().unwrap();
    let foreign_cursor = f
        .request(
            false,
            "GET",
            &format!(
                "/api/widget/{}/conversations/{id}/messages?cursor={next}",
                f.a
            ),
            Some(&f.token),
            Value::Null,
        )
        .await;
    f.finish().await;
    assert!(
        bounded,
        "read response must stay within its declared byte and row budget"
    );
    assert!(page_shape, "history must expose explicit page continuation");
    assert!(cursor.is_none());
    assert_eq!(ids.len(), 101);
    assert_eq!(default_page.0, StatusCode::OK);
    assert_eq!(default_page.1["messages"].as_array().unwrap().len(), 50);
    assert_eq!(foreign_cursor.0, StatusCode::BAD_REQUEST);
}
#[tokio::test]
async fn invalid_history_limits_and_oversized_stored_records_fail_honestly() {
    let f = PgFixture::new().await;
    let id = f.seed_conversation(false).await;
    let mut rejected = vec![];
    for query in ["limit=0", "limit=101", "cursor=not-a-valid-cursor"] {
        rejected.push(
            f.request(
                false,
                "GET",
                &format!("/api/widget/{}/conversations/{id}/messages?{query}", f.a),
                Some(&f.token),
                Value::Null,
            )
            .await
            .0,
        );
    }
    sqlx::query("INSERT INTO chat_messages(id,tenant_id,conversation_id,sender_type,content) VALUES($1,$2,$3,'contact',repeat('x',17000))").bind(Uuid::new_v4()).bind(f.a).bind(id).execute(&f.admin).await.unwrap();
    let oversized = f
        .request(
            false,
            "GET",
            &format!("/api/widget/{}/conversations/{id}/messages", f.a),
            Some(&f.token),
            Value::Null,
        )
        .await;
    sqlx::query("UPDATE chat_messages SET content='valid',sender_id=repeat('s',4097) WHERE conversation_id=$1").bind(id).execute(&f.admin).await.unwrap();
    let oversized_sender = f
        .request(
            false,
            "GET",
            &format!("/api/widget/{}/conversations/{id}/messages", f.a),
            Some(&f.token),
            Value::Null,
        )
        .await;
    f.finish().await;
    assert_eq!(rejected, vec![StatusCode::BAD_REQUEST; 3]);
    assert_eq!(
        oversized.0,
        StatusCode::INTERNAL_SERVER_ERROR,
        "stored over-limit content must not become an empty/truncated message"
    );
    assert_eq!(
        oversized_sender.0,
        StatusCode::INTERNAL_SERVER_ERROR,
        "stored over-limit sender must not become an invented anonymous message"
    );
}

#[path = "parent_race_test.rs"]
mod parent_races;
