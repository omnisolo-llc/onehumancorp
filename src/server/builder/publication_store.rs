//! Explicit owner-reviewed publication snapshots and their durable job receipts.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::collections::BTreeSet;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct PublicationActor {
    pub user_id: String,
    pub tenant_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteSnapshot {
    pub domain: Option<String>,
    pub pages: Vec<PublishedPage>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishedPage {
    pub path: String,
    pub title: String,
    pub seo_metadata: Value,
    pub blocks: Vec<PublishedBlock>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishedBlock {
    pub block_type: String,
    pub content: Value,
    pub sort_order: i32,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PublicationStatus {
    Pending,
    Processing,
    Published,
    Failed,
    Revoked,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicationReceipt {
    pub publication_id: Uuid,
    pub operation_id: Uuid,
    pub site_id: Uuid,
    pub version: i64,
    pub status: PublicationStatus,
    pub snapshot_sha256: String,
    pub public_path: Option<String>,
}
#[derive(Debug)]
pub enum PublicationError {
    Unauthorized,
    NotFound,
    Conflict,
    Invalid(&'static str),
    Corrupt,
    Database(sqlx::Error),
}
impl From<sqlx::Error> for PublicationError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}
impl std::fmt::Display for PublicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unauthorized => "Current publication authority is required",
            Self::NotFound => "The owned publication resource was not found",
            Self::Conflict => "The operation is bound to different reviewed content",
            Self::Invalid(reason) => reason,
            Self::Corrupt => "The publication receipt could not be verified",
            Self::Database(_) => "Publication persistence failed",
        })
    }
}
impl std::error::Error for PublicationError {}

pub(crate) fn builder_tenant_id(tenant: &str) -> Uuid {
    Uuid::parse_str(tenant)
        .unwrap_or_else(|_| Uuid::new_v5(&Uuid::NAMESPACE_DNS, tenant.as_bytes()))
}
fn canonical_value(value: Value) -> Value {
    match value {
        Value::Object(values) => {
            let mut entries: Vec<_> = values.into_iter().collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            Value::Object(
                entries
                    .into_iter()
                    .map(|(k, v)| (k, canonical_value(v)))
                    .collect(),
            )
        }
        Value::Array(values) => Value::Array(values.into_iter().map(canonical_value).collect()),
        value => value,
    }
}
pub(crate) fn prepare_snapshot(
    snapshot: &SiteSnapshot,
) -> Result<(Value, String, Vec<String>), PublicationError> {
    if snapshot.pages.is_empty() || snapshot.pages.len() > 50 {
        return Err(PublicationError::Invalid(
            "A publication requires between 1 and 50 pages",
        ));
    }
    if snapshot
        .domain
        .as_ref()
        .is_some_and(|d| d.len() > 253 || d.chars().any(char::is_control))
    {
        return Err(PublicationError::Invalid("Invalid requested domain"));
    }
    let mut paths = BTreeSet::new();
    let mut products = BTreeSet::new();
    for page in &snapshot.pages {
        if !page.path.starts_with('/')
            || page.path.starts_with("//")
            || page.path.len() > 512
            || page.path.contains(['?', '#', '\\'])
            || page.path.chars().any(char::is_control)
            || page.path.split('/').any(|s| s == "." || s == "..")
            || !paths.insert(page.path.clone())
        {
            return Err(PublicationError::Invalid(
                "Publication paths must be unique local document paths",
            ));
        }
        if page.title.trim().is_empty()
            || page.title.chars().count() > 200
            || !page.seo_metadata.is_object()
            || page.blocks.len() > 100
        {
            return Err(PublicationError::Invalid("Invalid reviewed page"));
        }
        let mut orders = BTreeSet::new();
        for block in &page.blocks {
            if block.block_type.trim().is_empty()
                || block.block_type.len() > 64
                || !block.content.is_object()
                || !orders.insert(block.sort_order)
            {
                return Err(PublicationError::Invalid("Invalid reviewed block"));
            }
            if matches!(block.block_type.as_str(), "ProductGridBlock" | "Catalog")
                && let Some(items) = block.content.get("items")
            {
                let items = items
                    .as_array()
                    .ok_or(PublicationError::Invalid("Invalid catalog items"))?;
                for item in items {
                    if let Some(id) = item.get("product_id") {
                        let id = id.as_str().and_then(|id| Uuid::parse_str(id).ok()).ok_or(
                            PublicationError::Invalid("Invalid selected product identity"),
                        )?;
                        products.insert(id.to_string());
                    }
                }
            }
        }
    }
    if !paths.contains("/") {
        return Err(PublicationError::Invalid(
            "The reviewed site needs a root page",
        ));
    }
    let value = canonical_value(
        serde_json::to_value(snapshot)
            .map_err(|_| PublicationError::Invalid("Invalid snapshot"))?,
    );
    let bytes =
        serde_json::to_vec(&value).map_err(|_| PublicationError::Invalid("Invalid snapshot"))?;
    if bytes.len() > 1024 * 1024 {
        return Err(PublicationError::Invalid("Reviewed site exceeds 1 MiB"));
    }
    Ok((
        value,
        format!("{:x}", Sha256::digest(bytes)),
        products.into_iter().collect(),
    ))
}
pub(crate) async fn require_current_owner(
    tx: &mut Transaction<'_, Postgres>,
    actor: &PublicationActor,
) -> Result<(), PublicationError> {
    if actor.user_id.trim().is_empty()
        || actor.user_id.len() > 512
        || actor.tenant_id.trim() != actor.tenant_id
        || actor.tenant_id.is_empty()
        || actor.tenant_id.len() > 256
        || actor.tenant_id.eq_ignore_ascii_case("system")
    {
        return Err(PublicationError::Unauthorized);
    }
    server_common::auth_utils::set_org_context(&mut **tx, &actor.tenant_id).await?;
    let user=sqlx::query_scalar::<_,String>("SELECT u.id FROM users u JOIN identity_user_roles r ON r.user_id=u.id AND r.tenant_id=u.tenant_id WHERE u.id=$1 AND u.tenant_id=$2 AND u.active=TRUE AND lower(r.role_name) IN ('admin','owner') FOR SHARE OF u,r")
        .bind(&actor.user_id).bind(&actor.tenant_id).fetch_optional(&mut **tx).await?;
    if user.is_none() {
        return Err(PublicationError::Unauthorized);
    }
    Ok(())
}
fn receipt(row: &sqlx::postgres::PgRow) -> Result<PublicationReceipt, PublicationError> {
    let status = match row.try_get::<String, _>("status")?.as_str() {
        "pending" => PublicationStatus::Pending,
        "processing" => PublicationStatus::Processing,
        "published" => PublicationStatus::Published,
        "failed" => PublicationStatus::Failed,
        "revoked" => PublicationStatus::Revoked,
        _ => return Err(PublicationError::Corrupt),
    };
    Ok(PublicationReceipt {
        publication_id: row.try_get("publication_id")?,
        operation_id: row.try_get("operation_id")?,
        site_id: row.try_get("site_id")?,
        version: row.try_get("site_version")?,
        status,
        snapshot_sha256: row.try_get("snapshot_sha256")?,
        // A later guarded publication read supplies the canonical public URL.
        // Merely decoding a row cannot establish current public eligibility.
        public_path: None,
    })
}

pub async fn submit_publication(
    pool: &PgPool,
    actor: &PublicationActor,
    operation_id: Uuid,
    site_id: Option<Uuid>,
    snapshot: &SiteSnapshot,
) -> Result<PublicationReceipt, PublicationError> {
    let (snapshot_value, digest, products) = prepare_snapshot(snapshot)?;
    let mut tx = pool.begin().await?;
    require_current_owner(&mut tx, actor).await?;
    let operation_key = serde_json::to_string(&(&actor.tenant_id, &actor.user_id, operation_id))
        .map_err(|_| PublicationError::Invalid("Invalid operation identity"))?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,0))")
        .bind(operation_key)
        .execute(&mut *tx)
        .await?;
    if let Some(row)=sqlx::query("SELECT * FROM builder_publications WHERE tenant_id=$1 AND owner_id=$2 AND operation_id=$3 FOR UPDATE").bind(&actor.tenant_id).bind(&actor.user_id).bind(operation_id).fetch_optional(&mut *tx).await? {
        if row.try_get::<String,_>("snapshot_sha256")?!=digest || row.try_get::<Option<Uuid>,_>("requested_site_id")?!=site_id {return Err(PublicationError::Conflict);}
        let receipt=receipt(&row)?;tx.commit().await?;return Ok(receipt);
    }
    for product in &products {
        if sqlx::query_scalar::<_, String>(
            "SELECT id FROM products WHERE id=$1 AND tenant_id=$2 FOR SHARE",
        )
        .bind(product)
        .bind(&actor.tenant_id)
        .fetch_optional(&mut *tx)
        .await?
        .is_none()
        {
            return Err(PublicationError::NotFound);
        }
    }
    let builder_tenant = builder_tenant_id(&actor.tenant_id);
    server_common::auth_utils::set_org_context(&mut *tx, &builder_tenant.to_string()).await?;
    let (saved_site, version) = if let Some(site_id) = site_id {
        let version=sqlx::query_scalar::<_,i64>("UPDATE builder_sites SET publication_generation=publication_generation+1,domain=$3,updated_at=CURRENT_TIMESTAMP WHERE id=$1 AND tenant_id=$2 AND publication_tenant_id=$4 RETURNING publication_generation").bind(site_id).bind(builder_tenant).bind(&snapshot.domain).bind(&actor.tenant_id).fetch_optional(&mut *tx).await?.ok_or(PublicationError::NotFound)?;
        (site_id, version)
    } else {
        let id = Uuid::new_v4();
        sqlx::query("INSERT INTO builder_sites(id,tenant_id,domain,publication_generation,publication_tenant_id) VALUES($1,$2,$3,1,$4)").bind(id).bind(builder_tenant).bind(&snapshot.domain).bind(&actor.tenant_id).execute(&mut *tx).await?;
        (id, 1)
    };
    sqlx::query("DELETE FROM builder_pages WHERE site_id=$1 AND tenant_id=$2")
        .bind(saved_site)
        .bind(builder_tenant)
        .execute(&mut *tx)
        .await?;
    for page in &snapshot.pages {
        let page_id = Uuid::new_v4();
        sqlx::query("INSERT INTO builder_pages(id,tenant_id,site_id,path,title,seo_metadata) VALUES($1,$2,$3,$4,$5,$6)").bind(page_id).bind(builder_tenant).bind(saved_site).bind(&page.path).bind(&page.title).bind(&page.seo_metadata).execute(&mut *tx).await?;
        for block in &page.blocks {
            sqlx::query("INSERT INTO builder_blocks(id,tenant_id,page_id,block_type,content,sort_order) VALUES($1,$2,$3,$4,$5,$6)").bind(Uuid::new_v4()).bind(builder_tenant).bind(page_id).bind(&block.block_type).bind(&block.content).bind(block.sort_order).execute(&mut *tx).await?;
        }
    }
    server_common::auth_utils::set_org_context(&mut *tx, &actor.tenant_id).await?;
    let row=sqlx::query("INSERT INTO builder_publications(publication_id,tenant_id,owner_id,operation_id,site_id,requested_site_id,site_version,snapshot,snapshot_sha256,product_ids) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) RETURNING *")
        .bind(Uuid::new_v4()).bind(&actor.tenant_id).bind(&actor.user_id).bind(operation_id).bind(saved_site).bind(site_id).bind(version).bind(snapshot_value).bind(digest).bind(products).fetch_one(&mut *tx).await?;
    let receipt = receipt(&row)?;
    tx.commit().await?;
    Ok(receipt)
}
