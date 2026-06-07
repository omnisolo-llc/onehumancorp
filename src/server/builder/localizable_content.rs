use sqlx::{PgPool, Postgres, pool::PoolConnection};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct LocalizableContent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub resource_id: Uuid,
    pub resource_type: String,
    pub field_name: String,
    pub language_code: String,
    pub content: String,
}

async fn acquire_tenant_conn(
    pool: &PgPool,
    tenant_id: Uuid,
) -> Result<PoolConnection<Postgres>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query("SELECT set_config('app.current_tenant', $1, false)")
        .bind(tenant_id.to_string())
        .execute(&mut *conn)
        .await?;
    Ok(conn)
}

pub async fn upsert_localizable_content(
    pool: &PgPool,
    tenant_id: Uuid,
    resource_id: Uuid,
    resource_type: &str,
    field_name: &str,
    language_code: &str,
    content: &str,
) -> Result<LocalizableContent, sqlx::Error> {
    let mut conn = acquire_tenant_conn(pool, tenant_id).await?;
    sqlx::query_as::<_, LocalizableContent>(
        r#"
        INSERT INTO localizable_content (tenant_id, resource_id, resource_type, field_name, language_code, content)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (tenant_id, resource_id, resource_type, field_name, language_code)
        DO UPDATE SET content = EXCLUDED.content, updated_at = NOW()
        RETURNING id, tenant_id, resource_id, resource_type, field_name, language_code, content
        "#,
    )
    .bind(tenant_id)
    .bind(resource_id)
    .bind(resource_type)
    .bind(field_name)
    .bind(language_code)
    .bind(content)
    .fetch_one(&mut *conn)
    .await
}

pub async fn get_localizable_content(
    pool: &PgPool,
    tenant_id: Uuid,
    resource_id: Uuid,
    resource_type: &str,
    field_name: &str,
    language_code: &str,
) -> Result<Option<LocalizableContent>, sqlx::Error> {
    let mut conn = acquire_tenant_conn(pool, tenant_id).await?;
    sqlx::query_as::<_, LocalizableContent>(
        r#"
        SELECT id, tenant_id, resource_id, resource_type, field_name, language_code, content
        FROM localizable_content
        WHERE tenant_id = $1 AND resource_id = $2 AND resource_type = $3 AND field_name = $4 AND language_code = $5
        "#,
    )
    .bind(tenant_id)
    .bind(resource_id)
    .bind(resource_type)
    .bind(field_name)
    .bind(language_code)
    .fetch_optional(&mut *conn)
    .await
}
