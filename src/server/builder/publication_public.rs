//! Read-only public projection; routing metadata alone grants no content access.
use super::publication_store::{PublicationError, builder_tenant_id, valid_publication_path};
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

pub async fn read_public_page(
    pool: &PgPool,
    site_id: Uuid,
    path: &str,
) -> Result<PublicPage, PublicationError> {
    if !valid_publication_path(path) {
        return Err(PublicationError::NotFound);
    }
    let mut tx = pool.begin().await?;
    // A public reader must fail closed instead of occupying a connection
    // indefinitely while an authority or catalogue writer holds its lock.
    sqlx::query("SET LOCAL statement_timeout = '3000ms'")
        .execute(&mut *tx)
        .await?;
    sqlx::query("SET LOCAL lock_timeout = '1000ms'")
        .execute(&mut *tx)
        .await?;
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
    let html = pages.get(path).cloned().ok_or(PublicationError::NotFound)?;
    tx.commit().await?;
    Ok(PublicPage {
        publication_id,
        rendered_sha256: expected,
        path: path.into(),
        html,
    })
}
