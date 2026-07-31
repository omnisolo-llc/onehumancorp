use sea_orm::{ConnectionTrait, EntityTrait, Statement, QueryTrait};
use uuid::Uuid;
use sqlx::{Row, Acquire};

#[tokio::test]
async fn test_rls_policies() {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
    let pool_opts = sqlx::postgres::PgPoolOptions::new();
    let pool = match pool_opts.connect(&db_url).await {
        Ok(p) => p,
        Err(_) => {
            println!("Skipping RLS test because Postgres is not available at {}", db_url);
            return;
        }
    };

    let tenant_a = Uuid::new_v4();
    let tenant_b = Uuid::new_v4();

    // Acquire a single connection to ensure SET current_setting applies to the same session
    let mut conn_a = pool.acquire().await.unwrap();

    // Enable RLS for tenant A
    let set_tenant_a = format!("SET app.current_tenant_id = '{}';", tenant_a);
    sqlx::query(&set_tenant_a).execute(&mut *conn_a).await.unwrap();

    let inbox_id_a = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO inboxes (id, tenant_id, name, channel_type, channel_id, is_active) VALUES ($1, $2, 'Inbox A', 'whatsapp', $3, true)"
    )
    .bind(inbox_id_a)
    .bind(tenant_a)
    .bind(Uuid::new_v4())
    .execute(&mut *conn_a)
    .await.unwrap();

    let row = sqlx::query("SELECT count(*) FROM inboxes").fetch_one(&mut *conn_a).await.unwrap();
    let count: i64 = row.get(0);
    assert_eq!(count, 1, "Tenant A should see their inbox");

    // Acquire a DIFFERENT connection for Tenant B to verify isolation properly
    let mut conn_b = pool.acquire().await.unwrap();
    let set_tenant_b = format!("SET app.current_tenant_id = '{}';", tenant_b);
    sqlx::query(&set_tenant_b).execute(&mut *conn_b).await.unwrap();

    let row_b = sqlx::query("SELECT count(*) FROM inboxes").fetch_one(&mut *conn_b).await.unwrap();
    let count_b: i64 = row_b.get(0);
    assert_eq!(count_b, 0, "Tenant B should NOT see Tenant A's inbox");

    let inbox_id_b = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO inboxes (id, tenant_id, name, channel_type, channel_id, is_active) VALUES ($1, $2, 'Inbox B', 'whatsapp', $3, true)"
    )
    .bind(inbox_id_b)
    .bind(tenant_b)
    .bind(Uuid::new_v4())
    .execute(&mut *conn_b)
    .await.unwrap();

    let row_b_after = sqlx::query("SELECT count(*) FROM inboxes").fetch_one(&mut *conn_b).await.unwrap();
    let count_b_after: i64 = row_b_after.get(0);
    assert_eq!(count_b_after, 1, "Tenant B should see their own inbox");

    // Clean up
    sqlx::query("DELETE FROM inboxes WHERE id = $1").bind(inbox_id_b).execute(&mut *conn_b).await.unwrap();
    sqlx::query("DELETE FROM inboxes WHERE id = $1").bind(inbox_id_a).execute(&mut *conn_a).await.unwrap();
}
