//! Read-only public projection; routing metadata alone grants no content access.
use super::publication_store::{
    PublicationError, PublishedBlock, PublishedPage, SiteSnapshot, builder_tenant_id,
    limit_publication_read, valid_publication_path,
};
use super::publication_worker::{PublicationWorkItem, lock_authority_and_site, snapshot};
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct PublicPage {
    pub publication_id: Uuid,
    pub rendered_sha256: String,
    pub path: String,
    pub html: String,
}

async fn current_pointer(
    tx: &mut Transaction<'_, Postgres>,
    tenant: &str,
    site: Uuid,
) -> Result<Uuid, PublicationError> {
    let mapped = builder_tenant_id(tenant);
    server_common::auth_utils::set_org_context(&mut **tx, &mapped.to_string()).await?;
    let pointer:Option<Option<Uuid>>=sqlx::query_scalar("SELECT current_publication_id FROM builder_sites WHERE id=$1 AND tenant_id=$2 AND publication_tenant_id=$3")
        .bind(site).bind(mapped).bind(tenant).fetch_optional(&mut **tx).await?;
    server_common::auth_utils::set_org_context(&mut **tx, tenant).await?;
    pointer.flatten().ok_or(PublicationError::NotFound)
}

struct VerifiedPublication {
    publication_id: Uuid,
    rendered_sha256: String,
    reviewed: SiteSnapshot,
    pages: BTreeMap<String, String>,
}

async fn read_verified_publication(
    pool: &PgPool,
    site_id: Uuid,
) -> Result<VerifiedPublication, PublicationError> {
    let mut tx = pool.begin().await?;
    // A public reader must fail closed instead of occupying a connection
    // indefinitely while an authority or catalogue writer holds its lock.
    limit_publication_read(&mut tx).await?;
    // This projection contains only immutable internal routing metadata. A
    // valid mapping does not establish status, ownership or product eligibility.
    let tenants: Vec<String> = sqlx::query_scalar(
        "SELECT DISTINCT tenant_id FROM builder_publication_work WHERE site_id=$1 LIMIT 2",
    )
    .bind(site_id)
    .fetch_all(&mut *tx)
    .await?;
    let [tenant] = tenants.as_slice() else {
        return Err(PublicationError::NotFound);
    };
    let publication_id = current_pointer(&mut tx, tenant, site_id).await?;
    let initial=sqlx::query("SELECT * FROM builder_publications WHERE publication_id=$1 AND tenant_id=$2 AND site_id=$3 AND status='published'")
        .bind(publication_id).bind(tenant).bind(site_id).fetch_optional(&mut *tx).await?.ok_or(PublicationError::NotFound)?;
    let (reviewed, _, products) = snapshot(&initial)?;
    let work = PublicationWorkItem {
        publication_id,
        tenant_id: tenant.clone(),
    };
    // Match the worker's lock order: current owner, products, site, receipt.
    // A revocation already writing any row must complete before revalidation.
    lock_authority_and_site(&mut tx, &work, &initial, &products, false).await?;
    if current_pointer(&mut tx, tenant, site_id).await? != publication_id {
        return Err(PublicationError::NotFound);
    }
    let row=sqlx::query("SELECT * FROM builder_publications WHERE publication_id=$1 AND tenant_id=$2 AND site_id=$3 AND status='published' FOR SHARE")
        .bind(publication_id).bind(tenant).bind(site_id).fetch_optional(&mut *tx).await?.ok_or(PublicationError::NotFound)?;
    let pages: BTreeMap<String, String> =
        serde_json::from_value(row.try_get::<Value, _>("rendered_pages")?)
            .map_err(|_| PublicationError::Corrupt)?;
    let bytes = serde_json::to_vec(&pages).map_err(|_| PublicationError::Corrupt)?;
    let expected: String = row.try_get("rendered_sha256")?;
    if format!("{:x}", Sha256::digest(bytes)) != expected
        || pages.len() != reviewed.pages.len()
        || reviewed
            .pages
            .iter()
            .any(|page| !pages.contains_key(&page.path))
    {
        return Err(PublicationError::Corrupt);
    }
    tx.commit().await?;
    Ok(VerifiedPublication {
        publication_id,
        rendered_sha256: expected,
        reviewed,
        pages,
    })
}

pub async fn read_public_page(
    pool: &PgPool,
    site_id: Uuid,
    path: &str,
) -> Result<PublicPage, PublicationError> {
    if !valid_publication_path(path) {
        return Err(PublicationError::NotFound);
    }
    let verified = read_verified_publication(pool, site_id).await?;
    let html = verified
        .pages
        .get(path)
        .cloned()
        .ok_or(PublicationError::NotFound)?;
    Ok(PublicPage {
        publication_id: verified.publication_id,
        rendered_sha256: verified.rendered_sha256,
        path: path.into(),
        html,
    })
}

/// Product links expose only their explicitly reviewed immutable catalog entry.
/// Mutable catalog edits cannot silently change an already reviewed public offer.
pub async fn read_public_product(
    pool: &PgPool,
    site_id: Uuid,
    product_id: Uuid,
) -> Result<PublicPage, PublicationError> {
    let verified = read_verified_publication(pool, site_id).await?;
    let mut selected: Option<&Value> = None;
    for page in &verified.reviewed.pages {
        for block in &page.blocks {
            if !matches!(block.block_type.as_str(), "Catalog" | "ProductGridBlock") {
                continue;
            }
            for item in block
                .content
                .get("items")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if item
                    .get("product_id")
                    .and_then(Value::as_str)
                    .and_then(|value| Uuid::parse_str(value).ok())
                    != Some(product_id)
                {
                    continue;
                }
                if selected.is_some_and(|prior| prior != item) {
                    return Err(PublicationError::NotFound);
                }
                selected = Some(item);
            }
        }
    }
    let mut item = selected.cloned().ok_or(PublicationError::NotFound)?;
    let name = item
        .get("name")
        .and_then(Value::as_str)
        .ok_or(PublicationError::Corrupt)?
        .to_string();
    let description = item
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    item.as_object_mut()
        .ok_or(PublicationError::Corrupt)?
        .remove("product_id");
    let product = SiteSnapshot {
        domain: None,
        pages: vec![PublishedPage {
            path: "/".into(),
            title: name.chars().take(200).collect(),
            seo_metadata: serde_json::json!({"@context":"https://schema.org","@type":"Product","name":name,"description":description}),
            blocks: vec![PublishedBlock {
                block_type: "ProductGridBlock".into(),
                sort_order: 0,
                content: serde_json::json!({"items":[item]}),
            }],
        }],
    };
    let mut rendered = super::publication_render::render_snapshot(&product, site_id)?;
    Ok(PublicPage {
        publication_id: verified.publication_id,
        rendered_sha256: rendered.sha256,
        path: format!("/products/{product_id}"),
        html: rendered
            .pages
            .remove("/")
            .ok_or(PublicationError::Corrupt)?,
    })
}
