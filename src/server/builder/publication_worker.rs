//! Internal durable publication dispatch. Routing metadata never grants content authority.
use super::publication_render::render_snapshot;
use super::publication_store::{
    PublicationActor, PublicationError, PublicationReceipt, SiteSnapshot, builder_tenant_id,
    prepare_snapshot, receipt, require_current_owner,
};
use sqlx::{PgPool, Postgres, Row, Transaction, postgres::PgRow};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub(crate) struct PublicationWorkItem {
    pub publication_id: Uuid,
    pub tenant_id: String,
}
#[derive(Clone, Debug)]
pub(crate) struct PublicationClaim {
    work: PublicationWorkItem,
    lease_token: Uuid,
    snapshot: SiteSnapshot,
    snapshot_sha256: String,
    site_id: Uuid,
    version: i64,
}

pub(crate) async fn discover_publication_work(
    pool: &PgPool,
    limit: i64,
) -> Result<Vec<PublicationWorkItem>, PublicationError> {
    if !(1..=256).contains(&limit) {
        return Err(PublicationError::Invalid(
            "Invalid publication discovery limit",
        ));
    }
    let rows = sqlx::query("SELECT publication_id,tenant_id FROM builder_publication_work ORDER BY publication_id LIMIT $1")
        .bind(limit).fetch_all(pool).await?;
    rows.into_iter()
        .map(|row| {
            Ok(PublicationWorkItem {
                publication_id: row.try_get("publication_id")?,
                tenant_id: row.try_get("tenant_id")?,
            })
        })
        .collect()
}

async fn read_job(
    tx: &mut Transaction<'_, Postgres>,
    work: &PublicationWorkItem,
    lock: bool,
) -> Result<Option<PgRow>, PublicationError> {
    server_common::auth_utils::set_org_context(&mut **tx, &work.tenant_id).await?;
    let query = if lock {
        "SELECT *, lease_until > clock_timestamp() AS lease_valid FROM builder_publications WHERE publication_id=$1 AND tenant_id=$2 FOR UPDATE"
    } else {
        "SELECT *, lease_until > clock_timestamp() AS lease_valid FROM builder_publications WHERE publication_id=$1 AND tenant_id=$2"
    };
    Ok(sqlx::query(query)
        .bind(work.publication_id)
        .bind(&work.tenant_id)
        .fetch_optional(&mut **tx)
        .await?)
}

async fn remove_routing(
    tx: &mut Transaction<'_, Postgres>,
    work: &PublicationWorkItem,
) -> Result<(), PublicationError> {
    server_common::auth_utils::set_org_context(&mut **tx, &work.tenant_id).await?;
    sqlx::query("DELETE FROM builder_publication_work WHERE publication_id=$1 AND tenant_id=$2")
        .bind(work.publication_id)
        .bind(&work.tenant_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

async fn fail_job(
    mut tx: Transaction<'_, Postgres>,
    work: &PublicationWorkItem,
    lease: Option<Uuid>,
    code: &'static str,
) -> Result<(), PublicationError> {
    server_common::auth_utils::set_org_context(&mut *tx, &work.tenant_id).await?;
    let changed = sqlx::query("UPDATE builder_publications SET status='failed',error_code=$3,lease_until=NULL,updated_at=clock_timestamp() WHERE publication_id=$1 AND tenant_id=$2 AND status IN ('pending','processing') AND ($4::uuid IS NULL OR (lease_token=$4 AND lease_until>clock_timestamp()))")
        .bind(work.publication_id).bind(&work.tenant_id).bind(code).bind(lease)
        .execute(&mut *tx).await?.rows_affected();
    if changed == 1 {
        remove_routing(&mut tx, work).await?;
    }
    tx.commit().await?;
    Ok(())
}

fn snapshot(row: &PgRow) -> Result<(SiteSnapshot, String, Vec<String>), PublicationError> {
    let snapshot: SiteSnapshot =
        serde_json::from_value(row.try_get("snapshot")?).map_err(|_| PublicationError::Corrupt)?;
    let (_, digest, products) = prepare_snapshot(&snapshot)?;
    if digest != row.try_get::<String, _>("snapshot_sha256")?
        || products != row.try_get::<Vec<String>, _>("product_ids")?
    {
        return Err(PublicationError::Corrupt);
    }
    Ok((snapshot, digest, products))
}

async fn lock_authority_and_site(
    tx: &mut Transaction<'_, Postgres>,
    work: &PublicationWorkItem,
    row: &PgRow,
    products: &[String],
) -> Result<i64, PublicationError> {
    let actor = PublicationActor {
        user_id: row.try_get("owner_id")?,
        tenant_id: work.tenant_id.clone(),
    };
    require_current_owner(tx, &actor).await?;
    for product in products {
        if sqlx::query_scalar::<_, String>(
            "SELECT id FROM products WHERE id=$1 AND tenant_id=$2 FOR SHARE",
        )
        .bind(product)
        .bind(&actor.tenant_id)
        .fetch_optional(&mut **tx)
        .await?
        .is_none()
        {
            return Err(PublicationError::NotFound);
        }
    }
    let mapped = builder_tenant_id(&actor.tenant_id);
    server_common::auth_utils::set_org_context(&mut **tx, &mapped.to_string()).await?;
    let version = sqlx::query_scalar("SELECT publication_generation FROM builder_sites WHERE id=$1 AND tenant_id=$2 AND publication_tenant_id=$3 FOR UPDATE")
        .bind(row.try_get::<Uuid, _>("site_id")?).bind(mapped).bind(&actor.tenant_id)
        .fetch_optional(&mut **tx).await?;
    server_common::auth_utils::set_org_context(&mut **tx, &actor.tenant_id).await?;
    version.ok_or(PublicationError::NotFound)
}

pub(crate) async fn claim_publication(
    pool: &PgPool,
    work: &PublicationWorkItem,
) -> Result<Option<PublicationClaim>, PublicationError> {
    let mut tx = pool.begin().await?;
    let Some(initial) = read_job(&mut tx, work, false).await? else {
        return Ok(None);
    };
    // Terminal receipts need no rendering or authority grant. Clear stale
    // routing without changing their immutable content or public eligibility.
    let initial_status: String = initial.try_get("status")?;
    if !matches!(initial_status.as_str(), "pending" | "processing") {
        remove_routing(&mut tx, work).await?;
        tx.commit().await?;
        return Ok(None);
    }
    let (snapshot, digest, products) = match snapshot(&initial) {
        Ok(verified) => verified,
        Err(error) => {
            // Database failures remain retryable; permanently invalid stored
            // input must not monopolize the bounded discovery window forever.
            if !matches!(error, PublicationError::Database(_)) {
                fail_job(tx, work, None, "stored_snapshot_invalid").await?;
            }
            return Err(error);
        }
    };
    let generation = match lock_authority_and_site(&mut tx, work, &initial, &products).await {
        Ok(version) => version,
        Err(error) => {
            if !matches!(error, PublicationError::Database(_)) {
                fail_job(tx, work, None, "publication_authority_unavailable").await?;
            }
            return Err(error);
        }
    };
    let Some(row) = read_job(&mut tx, work, true).await? else {
        return Ok(None);
    };
    let status: String = row.try_get("status")?;
    if !matches!(status.as_str(), "pending" | "processing") {
        remove_routing(&mut tx, work).await?;
        tx.commit().await?;
        return Ok(None);
    }
    if status == "processing" && row.try_get::<Option<bool>, _>("lease_valid")? == Some(true) {
        return Ok(None);
    }
    let version: i64 = row.try_get("site_version")?;
    if version != generation {
        fail_job(tx, work, None, "newer_review_supersedes_publication").await?;
        return Ok(None);
    }
    let token = Uuid::new_v4();
    sqlx::query("UPDATE builder_publications SET status='processing',lease_token=$3,lease_until=clock_timestamp()+INTERVAL '30 seconds',updated_at=clock_timestamp(),error_code=NULL WHERE publication_id=$1 AND tenant_id=$2")
        .bind(work.publication_id).bind(&work.tenant_id).bind(token).execute(&mut *tx).await?;
    let claim = PublicationClaim {
        work: work.clone(),
        lease_token: token,
        snapshot,
        snapshot_sha256: digest,
        site_id: row.try_get("site_id")?,
        version,
    };
    tx.commit().await?;
    Ok(Some(claim))
}

pub(crate) async fn finish_publication(
    pool: &PgPool,
    claim: &PublicationClaim,
) -> Result<PublicationReceipt, PublicationError> {
    // Rendering performs no external effects. A retry after a lost reply is safe,
    // but the final database lease/version/authority checks remain mandatory.
    let rendered = render_snapshot(&claim.snapshot, claim.site_id);
    let mut tx = pool.begin().await?;
    let initial = read_job(&mut tx, &claim.work, false)
        .await?
        .ok_or(PublicationError::NotFound)?;
    let (_, digest, products) = snapshot(&initial)?;
    if digest != claim.snapshot_sha256
        || initial.try_get::<Uuid, _>("site_id")? != claim.site_id
        || initial.try_get::<i64, _>("site_version")? != claim.version
    {
        return Err(PublicationError::Conflict);
    }
    let generation = match lock_authority_and_site(&mut tx, &claim.work, &initial, &products).await
    {
        Ok(version) => version,
        Err(error) => {
            if !matches!(error, PublicationError::Database(_)) {
                fail_job(
                    tx,
                    &claim.work,
                    Some(claim.lease_token),
                    "publication_authority_unavailable",
                )
                .await?;
            }
            return Err(error);
        }
    };
    let row = read_job(&mut tx, &claim.work, true)
        .await?
        .ok_or(PublicationError::NotFound)?;
    if row.try_get::<Option<Uuid>, _>("lease_token")? != Some(claim.lease_token) {
        return Err(PublicationError::Conflict);
    }
    let status: String = row.try_get("status")?;
    if status == "published" {
        let value = receipt(&row)?;
        tx.commit().await?;
        return Ok(value);
    }
    if status != "processing" || row.try_get::<Option<bool>, _>("lease_valid")? != Some(true) {
        return Err(PublicationError::Conflict);
    }
    if generation != claim.version {
        fail_job(
            tx,
            &claim.work,
            Some(claim.lease_token),
            "newer_review_supersedes_publication",
        )
        .await?;
        return Err(PublicationError::Conflict);
    }
    let rendered = match rendered {
        Ok(rendered) => rendered,
        Err(error) => {
            fail_job(
                tx,
                &claim.work,
                Some(claim.lease_token),
                "reviewed_content_not_renderable",
            )
            .await?;
            return Err(error);
        }
    };
    let pages = serde_json::to_value(&rendered.pages).map_err(|_| PublicationError::Corrupt)?;
    let row = sqlx::query("UPDATE builder_publications SET status='published',rendered_pages=$4,rendered_sha256=$5,lease_until=NULL,error_code=NULL,updated_at=clock_timestamp() WHERE publication_id=$1 AND tenant_id=$2 AND lease_token=$3 AND status='processing' AND lease_until>clock_timestamp() RETURNING *")
        .bind(claim.work.publication_id).bind(&claim.work.tenant_id).bind(claim.lease_token)
        .bind(pages).bind(rendered.sha256).fetch_optional(&mut *tx).await?.ok_or(PublicationError::Conflict)?;
    let mapped = builder_tenant_id(&claim.work.tenant_id);
    server_common::auth_utils::set_org_context(&mut *tx, &mapped.to_string()).await?;
    let changed = sqlx::query("UPDATE builder_sites SET current_publication_id=$4,updated_at=clock_timestamp() WHERE id=$1 AND tenant_id=$2 AND publication_tenant_id=$3 AND publication_generation=$5")
        .bind(claim.site_id).bind(mapped).bind(&claim.work.tenant_id).bind(claim.work.publication_id).bind(claim.version)
        .execute(&mut *tx).await?.rows_affected();
    if changed != 1 {
        return Err(PublicationError::Conflict);
    }
    remove_routing(&mut tx, &claim.work).await?;
    let value = receipt(&row)?;
    tx.commit().await?;
    Ok(value)
}

pub async fn run_publication_worker(
    pool: PgPool,
    mut shutdown: tokio::sync::watch::Receiver<bool>,
) {
    loop {
        if *shutdown.borrow() {
            return;
        }
        match discover_publication_work(&pool, 64).await {
            Ok(work) => {
                for item in work {
                    if *shutdown.borrow() {
                        return;
                    }
                    match claim_publication(&pool, &item).await {
                        Ok(Some(claim)) => {
                            if let Err(error) = finish_publication(&pool, &claim).await {
                                tracing::warn!(publication_id=%item.publication_id, error=%error, "Publication completion was not acknowledged");
                            }
                        }
                        Ok(None) => {}
                        Err(error) => {
                            tracing::warn!(publication_id=%item.publication_id, error=%error, "Publication claim was not acknowledged")
                        }
                    }
                }
            }
            Err(error) => tracing::warn!(error=%error, "Publication routing is unavailable"),
        }
        tokio::select! {
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() { return; }
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {}
        }
    }
}
