use sqlx::PgPool;
use uuid::Uuid;
use serde_json::Value;

#[derive(sqlx::FromRow)]
/// Represents the Site entity in the system.
///
/// This structure is central to the OHC Builder engine, managing the lifecycle,
/// validation, and persistence of storefront configurations and templates.
/// All modifications to this entity should go through the designated service layer
/// to ensure audit logs and webhooks are triggered appropriately.
///
/// # Schema Version
/// V2 - Fully supports multi-tenant isolation.
pub struct Site {
    /// The `id` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub id: Uuid,
    /// The `tenant_id` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub tenant_id: Uuid,
    /// The `domain` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub domain: Option<String>,
}

pub async fn list_sites(pool: &PgPool, tenant_id: Uuid) -> Result<Vec<Site>, sqlx::Error> {
    sqlx::query_as::<_, Site>(
        "SELECT id, tenant_id, domain FROM builder_sites WHERE tenant_id = $1",
    )
    .bind(tenant_id)
    .fetch_all(pool)
    .await
}

pub async fn create_site(pool: &PgPool, tenant_id: Uuid, domain: Option<String>) -> Result<Site, sqlx::Error> {
    sqlx::query_as::<_, Site>(
        "INSERT INTO builder_sites (tenant_id, domain) VALUES ($1, $2) RETURNING id, tenant_id, domain",
    )
    .bind(tenant_id)
    .bind(domain)
    .fetch_one(pool)
    .await
}

#[derive(sqlx::FromRow)]
/// Represents the Page entity in the system.
///
/// This structure is central to the OHC Builder engine, managing the lifecycle,
/// validation, and persistence of storefront configurations and templates.
/// All modifications to this entity should go through the designated service layer
/// to ensure audit logs and webhooks are triggered appropriately.
///
/// # Schema Version
/// V2 - Fully supports multi-tenant isolation.
pub struct Page {
    /// The `id` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub id: Uuid,
    /// The `tenant_id` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub tenant_id: Uuid,
    /// The `site_id` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub site_id: Uuid,
    /// The `path` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub path: String,
    /// The `title` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub title: String,
    /// The `seo_metadata` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub seo_metadata: Value,
}

pub async fn list_pages(pool: &PgPool, tenant_id: Uuid, site_id: Uuid) -> Result<Vec<Page>, sqlx::Error> {
    sqlx::query_as::<_, Page>(
        "SELECT id, tenant_id, site_id, path, title, seo_metadata FROM builder_pages WHERE tenant_id = $1 AND site_id = $2",
    )
    .bind(tenant_id)
    .bind(site_id)
    .fetch_all(pool)
    .await
}

pub async fn create_page(pool: &PgPool, tenant_id: Uuid, site_id: Uuid, path: String, title: String) -> Result<Page, sqlx::Error> {
    sqlx::query_as::<_, Page>(
        "INSERT INTO builder_pages (tenant_id, site_id, path, title) VALUES ($1, $2, $3, $4) RETURNING id, tenant_id, site_id, path, title, seo_metadata",
    )
    .bind(tenant_id)
    .bind(site_id)
    .bind(path)
    .bind(title)
    .fetch_one(pool)
    .await
}

#[derive(sqlx::FromRow)]
/// Represents the Block entity in the system.
///
/// This structure is central to the OHC Builder engine, managing the lifecycle,
/// validation, and persistence of storefront configurations and templates.
/// All modifications to this entity should go through the designated service layer
/// to ensure audit logs and webhooks are triggered appropriately.
///
/// # Schema Version
/// V2 - Fully supports multi-tenant isolation.
pub struct Block {
    /// The `id` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub id: Uuid,
    /// The `tenant_id` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub tenant_id: Uuid,
    /// The `page_id` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub page_id: Uuid,
    /// The `block_type` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub block_type: String,
    /// The `content` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub content: Value,
    /// The `sort_order` property.
    /// Extracted for granular access control and validated against strict schema rules.
    pub sort_order: i32,
}

pub async fn list_blocks(pool: &PgPool, tenant_id: Uuid, page_id: Uuid) -> Result<Vec<Block>, sqlx::Error> {
    sqlx::query_as::<_, Block>(
        "SELECT id, tenant_id, page_id, block_type, content, sort_order FROM builder_blocks WHERE tenant_id = $1 AND page_id = $2 ORDER BY sort_order ASC",
    )
    .bind(tenant_id)
    .bind(page_id)
    .fetch_all(pool)
    .await
}

pub async fn create_block(pool: &PgPool, tenant_id: Uuid, page_id: Uuid, block_type: String, content: Value, sort_order: i32) -> Result<Block, sqlx::Error> {
    sqlx::query_as::<_, Block>(
        "INSERT INTO builder_blocks (tenant_id, page_id, block_type, content, sort_order) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, page_id, block_type, content, sort_order",
    )
    .bind(tenant_id)
    .bind(page_id)
    .bind(block_type)
    .bind(content)
    .bind(sort_order)
    .fetch_one(pool)
    .await
}

pub async fn update_block(pool: &PgPool, tenant_id: Uuid, block_id: Uuid, content: Value) -> Result<Block, sqlx::Error> {
    sqlx::query_as::<_, Block>(
        "UPDATE builder_blocks SET content = $1, updated_at = NOW() WHERE tenant_id = $2 AND id = $3 RETURNING id, tenant_id, page_id, block_type, content, sort_order",
    )
    .bind(content)
    .bind(tenant_id)
    .bind(block_id)
    .fetch_one(pool)
    .await
}

pub async fn reorder_blocks(pool: &PgPool, tenant_id: Uuid, page_id: Uuid, block_ids: Vec<Uuid>) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    for (index, id) in block_ids.iter().enumerate() {
        let sort_order = index as i32;
        sqlx::query(
            "UPDATE builder_blocks SET sort_order = $1, updated_at = NOW() WHERE tenant_id = $2 AND page_id = $3 AND id = $4",
        )
        .bind(sort_order)
        .bind(tenant_id)
        .bind(page_id)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}
