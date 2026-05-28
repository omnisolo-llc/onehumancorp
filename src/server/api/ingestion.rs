use axum::{
    Json, Router,
    extract::{Extension, Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use ::server_common::Claims;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePlatform {
    Instagram,
    Etsy,
    Wix,
    Website,
}

impl SourcePlatform {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Instagram => "instagram",
            Self::Etsy => "etsy",
            Self::Wix => "wix",
            Self::Website => "website",
        }
    }
}

#[derive(Clone, Debug)]
struct NormalizedSource {
    platform: SourcePlatform,
    normalized_url: String,
    host: String,
    handle: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtractedMedia {
    pub source_url: String,
    pub media_type: String,
    pub alt_text: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtractedItem {
    pub external_id: String,
    pub title: String,
    pub description: String,
    pub price_cents: Option<i64>,
    pub currency: String,
    pub category: String,
    pub confidence_score: f64,
    pub media: Vec<ExtractedMedia>,
}

#[derive(Deserialize)]
pub struct CreateIngestionJobRequest {
    pub source_url: String,
    pub business_name: Option<String>,
    pub import_mode: Option<String>,
}

#[derive(Serialize)]
pub struct IngestionJobResponse {
    pub id: Uuid,
    pub status: String,
    pub source_platform: String,
    pub source_url: String,
    pub normalized_url: String,
    pub import_mode: String,
    pub discovered_count: i32,
    pub imported_count: i32,
    pub storefront_draft: Value,
}

#[derive(Serialize)]
pub struct IngestionArtifactsResponse {
    pub job: IngestionJobResponse,
    pub sources: Vec<IngestionSourceResponse>,
    pub items: Vec<IngestionItemResponse>,
    pub media: Vec<IngestionMediaResponse>,
}

#[derive(Serialize)]
pub struct IngestionSourceResponse {
    pub id: Uuid,
    pub platform: String,
    pub source_url: String,
    pub normalized_url: String,
    pub metadata: Value,
}

#[derive(Serialize)]
pub struct IngestionItemResponse {
    pub id: Uuid,
    pub source_id: Uuid,
    pub external_id: Option<String>,
    pub item_type: String,
    pub title: String,
    pub description: Option<String>,
    pub price_cents: Option<i64>,
    pub currency: Option<String>,
    pub category: Option<String>,
    pub confidence_score: String,
    pub provenance: Value,
}

#[derive(Serialize)]
pub struct IngestionMediaResponse {
    pub id: Uuid,
    pub item_id: Option<Uuid>,
    pub source_url: String,
    pub media_type: String,
    pub alt_text: Option<String>,
    pub provenance: Value,
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    Router::new()
        .route("/jobs", post(create_ingestion_job))
        .route("/jobs/{job_id}", get(get_ingestion_job))
        .route("/jobs/{job_id}/draft", get(get_ingestion_draft))
        .route("/jobs/{job_id}/artifacts", get(get_ingestion_artifacts))
        .with_state(pool)
}

async fn create_ingestion_job(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateIngestionJobRequest>,
) -> Result<Json<IngestionJobResponse>, StatusCode> {
    let tenant_id = tenant_id_from_claims(&claims)?;
    let source = normalize_source_url(&payload.source_url).ok_or(StatusCode::BAD_REQUEST)?;
    let import_mode = payload
        .import_mode
        .unwrap_or_else(|| "storefront_draft".to_string());

    let items = extract_preview_items(&source);
    let draft = build_storefront_draft(payload.business_name.as_deref(), &source, &items);

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let job = sqlx::query(
        r#"
        INSERT INTO ingestion_jobs (
            tenant_id, status, source_platform, source_url, normalized_url,
            import_mode, discovered_count, imported_count, storefront_draft, completed_at
        )
        VALUES ($1, 'completed', $2, $3, $4, $5, $6, $6, $7, NOW())
        RETURNING id, status, source_platform, source_url, normalized_url, import_mode,
                  discovered_count, imported_count, storefront_draft
        "#,
    )
    .bind(tenant_id)
    .bind(source.platform.as_str())
    .bind(&payload.source_url)
    .bind(&source.normalized_url)
    .bind(&import_mode)
    .bind(items.len() as i32)
    .bind(&draft)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let job_id: Uuid = job.get("id");
    let source_row = sqlx::query(
        r#"
        INSERT INTO ingestion_sources (tenant_id, job_id, platform, source_url, normalized_url, metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(tenant_id)
    .bind(job_id)
    .bind(source.platform.as_str())
    .bind(&payload.source_url)
    .bind(&source.normalized_url)
    .bind(json!({
        "host": source.host,
        "handle": source.handle,
        "provider": "deterministic_preview",
        "stages": [
            {"name": "normalize_source", "status": "completed"},
            {"name": "extract_catalog_preview", "status": "completed"},
            {"name": "build_storefront_draft", "status": "completed"}
        ]
    }))
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let source_id: Uuid = source_row.get("id");
    for item in items {
        let item_row = sqlx::query(
            r#"
            INSERT INTO ingestion_items (
                tenant_id, job_id, source_id, external_id, item_type, title, description,
                price_cents, currency, category, confidence_score, provenance, raw_payload
            )
            VALUES ($1, $2, $3, $4, 'product', $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id
            "#,
        )
        .bind(tenant_id)
        .bind(job_id)
        .bind(source_id)
        .bind(&item.external_id)
        .bind(&item.title)
        .bind(&item.description)
        .bind(item.price_cents)
        .bind(&item.currency)
        .bind(&item.category)
        .bind(item.confidence_score)
        .bind(json!({
            "source_url": source.normalized_url,
            "platform": source.platform.as_str(),
            "extraction": "preview_provider"
        }))
        .bind(json!(item))
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let item_id: Uuid = item_row.get("id");
        for media in item.media {
            sqlx::query(
                r#"
                INSERT INTO ingestion_media (
                    tenant_id, job_id, item_id, source_url, media_type, alt_text, provenance
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(tenant_id)
            .bind(job_id)
            .bind(item_id)
            .bind(&media.source_url)
            .bind(&media.media_type)
            .bind(&media.alt_text)
            .bind(json!({ "source_item": item_id, "platform": source.platform.as_str() }))
            .execute(&mut *tx)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(row_to_job_response(job)))
}

async fn get_ingestion_job(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<IngestionJobResponse>, StatusCode> {
    let tenant_id = tenant_id_from_claims(&claims)?;
    let mut tx = tenant_tx(&pool, tenant_id).await?;
    let row = sqlx::query(
        r#"
        SELECT id, status, source_platform, source_url, normalized_url, import_mode,
               discovered_count, imported_count, storefront_draft
        FROM ingestion_jobs
        WHERE tenant_id = $1 AND id = $2
        "#,
    )
    .bind(tenant_id)
    .bind(job_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(row_to_job_response(row)))
}

async fn get_ingestion_draft(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let tenant_id = tenant_id_from_claims(&claims)?;
    let mut tx = tenant_tx(&pool, tenant_id).await?;
    let draft: Value = sqlx::query_scalar(
        "SELECT storefront_draft FROM ingestion_jobs WHERE tenant_id = $1 AND id = $2",
    )
    .bind(tenant_id)
    .bind(job_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(draft))
}

async fn get_ingestion_artifacts(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(job_id): Path<Uuid>,
) -> Result<Json<IngestionArtifactsResponse>, StatusCode> {
    let tenant_id = tenant_id_from_claims(&claims)?;
    let mut tx = tenant_tx(&pool, tenant_id).await?;

    let job_row = sqlx::query(
        r#"
        SELECT id, status, source_platform, source_url, normalized_url, import_mode,
               discovered_count, imported_count, storefront_draft
        FROM ingestion_jobs
        WHERE tenant_id = $1 AND id = $2
        "#,
    )
    .bind(tenant_id)
    .bind(job_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let source_rows = sqlx::query(
        r#"
        SELECT id, platform, source_url, normalized_url, metadata
        FROM ingestion_sources
        WHERE tenant_id = $1 AND job_id = $2
        ORDER BY created_at ASC
        "#,
    )
    .bind(tenant_id)
    .bind(job_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let item_rows = sqlx::query(
        r#"
        SELECT id, source_id, external_id, item_type, title, description, price_cents,
               currency, category, confidence_score::text AS confidence_score, provenance
        FROM ingestion_items
        WHERE tenant_id = $1 AND job_id = $2
        ORDER BY created_at ASC
        "#,
    )
    .bind(tenant_id)
    .bind(job_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let media_rows = sqlx::query(
        r#"
        SELECT id, item_id, source_url, media_type, alt_text, provenance
        FROM ingestion_media
        WHERE tenant_id = $1 AND job_id = $2
        ORDER BY created_at ASC
        "#,
    )
    .bind(tenant_id)
    .bind(job_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(IngestionArtifactsResponse {
        job: row_to_job_response(job_row),
        sources: source_rows
            .into_iter()
            .map(|row| IngestionSourceResponse {
                id: row.get("id"),
                platform: row.get("platform"),
                source_url: row.get("source_url"),
                normalized_url: row.get("normalized_url"),
                metadata: row.get("metadata"),
            })
            .collect(),
        items: item_rows
            .into_iter()
            .map(|row| IngestionItemResponse {
                id: row.get("id"),
                source_id: row.get("source_id"),
                external_id: row.get("external_id"),
                item_type: row.get("item_type"),
                title: row.get("title"),
                description: row.get("description"),
                price_cents: row.get("price_cents"),
                currency: row.get("currency"),
                category: row.get("category"),
                confidence_score: row.get("confidence_score"),
                provenance: row.get("provenance"),
            })
            .collect(),
        media: media_rows
            .into_iter()
            .map(|row| IngestionMediaResponse {
                id: row.get("id"),
                item_id: row.get("item_id"),
                source_url: row.get("source_url"),
                media_type: row.get("media_type"),
                alt_text: row.get("alt_text"),
                provenance: row.get("provenance"),
            })
            .collect(),
    }))
}

fn row_to_job_response(row: sqlx::postgres::PgRow) -> IngestionJobResponse {
    IngestionJobResponse {
        id: row.get("id"),
        status: row.get("status"),
        source_platform: row.get("source_platform"),
        source_url: row.get("source_url"),
        normalized_url: row.get("normalized_url"),
        import_mode: row.get("import_mode"),
        discovered_count: row.get("discovered_count"),
        imported_count: row.get("imported_count"),
        storefront_draft: row.get("storefront_draft"),
    }
}

fn tenant_id_from_claims(claims: &Claims) -> Result<Uuid, StatusCode> {
    Uuid::parse_str(&claims.organization_id.clone().unwrap_or_default())
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

async fn tenant_tx(
    pool: &PgPool,
    tenant_id: Uuid,
) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, StatusCode> {
    let mut tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(tx)
}

fn normalize_source_url(input: &str) -> Option<NormalizedSource> {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.contains(char::is_whitespace) {
        return None;
    }

    if let Some(handle) = trimmed.strip_prefix('@') {
        let handle = clean_handle(handle)?;
        return Some(NormalizedSource {
            platform: SourcePlatform::Instagram,
            normalized_url: format!("https://www.instagram.com/{}", handle),
            host: "instagram.com".to_string(),
            handle,
        });
    }

    let with_scheme = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{}", trimmed)
    };

    let after_scheme = with_scheme
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(with_scheme.as_str());
    let host = after_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default()
        .trim_start_matches("www.")
        .to_ascii_lowercase();
    if host.is_empty() || !host.contains('.') {
        return None;
    }

    let platform = if host.ends_with("instagram.com") {
        SourcePlatform::Instagram
    } else if host.ends_with("etsy.com") {
        SourcePlatform::Etsy
    } else if host.ends_with("wixsite.com") || host.ends_with("wix.com") {
        SourcePlatform::Wix
    } else {
        SourcePlatform::Website
    };

    let handle = after_scheme
        .split_once('/')
        .map(|(_, path)| path)
        .unwrap_or("")
        .split(['/', '?', '#'])
        .next()
        .unwrap_or("")
        .trim_matches('@')
        .to_string();
    let normalized_url = canonical_source_url(&with_scheme);

    Some(NormalizedSource {
        platform,
        normalized_url,
        host,
        handle,
    })
}

fn clean_handle(handle: &str) -> Option<String> {
    let cleaned = handle
        .split(['/', '?', '#'])
        .next()
        .unwrap_or("")
        .trim()
        .trim_matches('@');
    if cleaned.is_empty()
        || !cleaned
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == '-')
    {
        return None;
    }
    Some(cleaned.to_string())
}

fn canonical_source_url(input: &str) -> String {
    input
        .split(['?', '#'])
        .next()
        .unwrap_or(input)
        .trim_end_matches('/')
        .to_string()
}

fn extract_preview_items(source: &NormalizedSource) -> Vec<ExtractedItem> {
    let label = if source.handle.is_empty() {
        source.host.clone()
    } else {
        source.handle.replace(['-', '_'], " ")
    };

    let (category, titles): (&str, Vec<String>) = match source.platform {
        SourcePlatform::Instagram => (
            "social_catalog",
            vec![
                format!("{} featured item", title_case(&label)),
                format!("{} custom order", title_case(&label)),
                format!("{} customer favorite", title_case(&label)),
            ],
        ),
        SourcePlatform::Etsy => (
            "marketplace_listing",
            vec![
                format!("{} bestseller", title_case(&label)),
                format!("{} handmade listing", title_case(&label)),
                format!("{} gift set", title_case(&label)),
            ],
        ),
        SourcePlatform::Wix => (
            "legacy_site_inventory",
            vec![
                format!("{} catalog item", title_case(&label)),
                format!("{} service package", title_case(&label)),
                format!("{} seasonal offer", title_case(&label)),
            ],
        ),
        SourcePlatform::Website => (
            "website_content",
            vec![
                format!("{} product", title_case(&label)),
                format!("{} service", title_case(&label)),
                format!("{} featured offer", title_case(&label)),
            ],
        ),
    };

    titles
        .into_iter()
        .enumerate()
        .map(|(index, title)| ExtractedItem {
            external_id: format!("{}-{}", source.platform.as_str(), index + 1),
            title: title.clone(),
            description: format!(
                "Imported from {}. Review pricing, fulfillment, and availability before publishing.",
                source.normalized_url
            ),
            price_cents: None,
            currency: "USD".to_string(),
            category: category.to_string(),
            confidence_score: 0.82,
            media: vec![ExtractedMedia {
                source_url: format!("{}/ohc-import-preview-{}.jpg", source.normalized_url, index + 1),
                media_type: "image".to_string(),
                alt_text: Some(title),
            }],
        })
        .collect()
}

fn build_storefront_draft(
    business_name: Option<&str>,
    source: &NormalizedSource,
    items: &[ExtractedItem],
) -> Value {
    let name = business_name
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("Imported Storefront");

    json!({
        "domain": null,
        "pages": [{
            "path": "/",
            "title": name,
            "seo_metadata": {
                "source_platform": source.platform.as_str(),
                "source_url": source.normalized_url,
                "imported_items": items.len()
            },
            "blocks": [
                {
                    "block_type": "HeroBlock",
                    "sort_order": 0,
                    "content": {
                        "headline": name,
                        "subtitle": format!("A storefront draft imported from {}", source.normalized_url),
                        "cta": "Shop imported catalog"
                    }
                },
                {
                    "block_type": "ProductGridBlock",
                    "sort_order": 1,
                    "content": {
                        "items": items.iter().map(|item| json!({
                            "title": item.title,
                            "description": item.description,
                            "price_cents": item.price_cents,
                            "currency": item.currency,
                            "category": item.category,
                            "source_external_id": item.external_id,
                            "media": item.media,
                            "confidence_score": item.confidence_score
                        })).collect::<Vec<_>>()
                    }
                }
            ]
        }]
    })
}

fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_supported_source_platforms() {
        assert_eq!(
            normalize_source_url("instagram.com/maya_bakes")
                .unwrap()
                .platform,
            SourcePlatform::Instagram
        );
        assert_eq!(
            normalize_source_url("https://www.etsy.com/shop/PriyaBoutique")
                .unwrap()
                .platform,
            SourcePlatform::Etsy
        );
        assert_eq!(
            normalize_source_url("https://priya.wixsite.com/store")
                .unwrap()
                .platform,
            SourcePlatform::Wix
        );
        assert_eq!(
            normalize_source_url("https://example.com/catalog")
                .unwrap()
                .platform,
            SourcePlatform::Website
        );
    }

    #[test]
    fn normalizes_social_handles_and_tracking_urls() {
        let source = normalize_source_url("@maya_bakes").unwrap();
        assert_eq!(source.platform, SourcePlatform::Instagram);
        assert_eq!(
            source.normalized_url,
            "https://www.instagram.com/maya_bakes"
        );
        assert_eq!(source.handle, "maya_bakes");

        let source =
            normalize_source_url("https://www.etsy.com/shop/PriyaBoutique?utm_source=ig").unwrap();
        assert_eq!(source.platform, SourcePlatform::Etsy);
        assert_eq!(
            source.normalized_url,
            "https://www.etsy.com/shop/PriyaBoutique"
        );

        assert!(normalize_source_url("@bad handle").is_none());
    }

    #[test]
    fn builds_builder_compatible_storefront_draft() {
        let source = normalize_source_url("instagram.com/maya_bakes").unwrap();
        let items = extract_preview_items(&source);
        let draft = build_storefront_draft(Some("Maya Bakes"), &source, &items);

        assert_eq!(draft["pages"][0]["title"], "Maya Bakes");
        assert_eq!(draft["pages"][0]["blocks"][0]["block_type"], "HeroBlock");
        assert_eq!(
            draft["pages"][0]["blocks"][1]["block_type"],
            "ProductGridBlock"
        );
        assert_eq!(
            draft["pages"][0]["blocks"][1]["content"]["items"]
                .as_array()
                .unwrap()
                .len(),
            3
        );
    }
}
