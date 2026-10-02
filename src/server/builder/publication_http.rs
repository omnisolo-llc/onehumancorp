//! Explicit publication actions and the separately mounted public read surface.
use super::publication_json::{MAX_PUBLICATION_BODY_BYTES, decode_publication_json};
use super::publication_public::{PublicPage, read_public_page, read_public_product};
use super::publication_render::render_snapshot;
use super::publication_store::{
    PublicationActor, PublicationError, PublicationReceipt, PublicationStatus, SiteSnapshot,
    read_owned_publication, revoke_owned_publication, submit_publication,
};
use axum::{
    Json, Router,
    body::Bytes,
    extract::{DefaultBodyLimit, Extension, Path, State, rejection::BytesRejection},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

pub const SNAPSHOT_ENCODING: &str = "jcs-rfc8785-v1";
const CSP: &str = "default-src 'none'; script-src 'none'; style-src 'unsafe-inline'; img-src https: http:; base-uri 'none'; form-action 'none'; object-src 'none'; frame-src 'none'; connect-src 'none'";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishRequest {
    pub operation_id: Uuid,
    pub site_id: Option<Uuid>,
    pub snapshot_encoding: String,
    pub snapshot: SiteSnapshot,
}

/// The application and HTTP regression gate share this complete route assembly.
pub fn router(pool: PgPool, auth: std::sync::Arc<server_auth::Store>) -> Router {
    Router::new()
        .nest(
            "/api/v1/builder",
            authenticated_router(pool.clone()).route_layer(axum::middleware::from_fn_with_state(
                auth,
                server_auth::strict_bearer_auth_middleware,
            )),
        )
        .nest("/api/v1/public/sites", public_router(pool))
        .layer(axum::middleware::from_fn(no_store))
}

pub fn authenticated_router(pool: PgPool) -> Router {
    Router::new()
        .route("/publications", post(submit))
        .route("/publications/operations/{operation_id}", get(receipt))
        .route("/publications/{publication_id}", delete(revoke))
        .layer(DefaultBodyLimit::max(MAX_PUBLICATION_BODY_BYTES))
        .layer(axum::middleware::from_fn(no_store))
        .with_state(pool)
}

pub fn public_router(pool: PgPool) -> Router {
    Router::new()
        .route("/{site_id}", get(root_page))
        .route("/{site_id}/pages/{*path}", get(document_page))
        .route("/{site_id}/products/{product_id}", get(product_page))
        .layer(axum::middleware::from_fn(no_store))
        .with_state(pool)
}

async fn no_store(request: axum::extract::Request, next: axum::middleware::Next) -> Response {
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

fn failure(error: PublicationError) -> Response {
    let (status, code, effect) = match &error {
        PublicationError::Unauthorized => (StatusCode::FORBIDDEN, "publication_forbidden", "none"),
        PublicationError::NotFound => (StatusCode::NOT_FOUND, "publication_not_found", "none"),
        PublicationError::Conflict => (StatusCode::CONFLICT, "publication_conflict", "unknown"),
        PublicationError::Invalid("Reviewed site exceeds 1 MiB") => {
            (StatusCode::PAYLOAD_TOO_LARGE, "publication_invalid", "none")
        }
        PublicationError::Invalid(_) => (StatusCode::BAD_REQUEST, "publication_invalid", "none"),
        PublicationError::Corrupt | PublicationError::Database(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "publication_unavailable",
            "unknown",
        ),
    };
    (status, Json(serde_json::json!({"schema_version":1,"error":code,"effect":effect,"message":error.to_string()}))).into_response()
}

fn actor(
    claims: &server_common::Claims,
    headers: &HeaderMap,
) -> Result<PublicationActor, PublicationError> {
    let tenant = server_common::auth_utils::signed_tenant_id(claims)
        .ok_or(PublicationError::Unauthorized)?;
    if claims.organization_id.as_deref() != Some(tenant.as_str()) || claims.sub.trim().is_empty() {
        return Err(PublicationError::Unauthorized);
    }
    let expected_user = headers.get_all("x-ohc-expected-user");
    let expected_tenant = headers.get_all("x-ohc-expected-tenant");
    if expected_user.iter().next().is_some() || expected_tenant.iter().next().is_some() {
        let exact = |values: axum::http::header::GetAll<'_, HeaderValue>, actual: &str| {
            let mut values = values.iter();
            values.next().and_then(|value| value.to_str().ok()) == Some(actual)
                && values.next().is_none()
        };
        if !exact(expected_user, &claims.sub) || !exact(expected_tenant, &tenant) {
            return Err(PublicationError::Conflict);
        }
    }
    Ok(PublicationActor {
        user_id: claims.sub.clone(),
        tenant_id: tenant,
    })
}

fn allowed_fields(value: &Value, allowed: &[&str]) -> Result<(), PublicationError> {
    let object = value
        .as_object()
        .ok_or(PublicationError::Invalid("Invalid reviewed block content"))?;
    if object.keys().any(|key| !allowed.contains(&key.as_str())) {
        return Err(PublicationError::Invalid("Unexpected reviewed block field"));
    }
    Ok(())
}

fn validate_content(snapshot: &SiteSnapshot) -> Result<(), PublicationError> {
    if snapshot.domain.is_some() {
        return Err(PublicationError::Invalid(
            "Application-hosted publication requires a null domain",
        ));
    }
    for page in &snapshot.pages {
        for block in &page.blocks {
            let (fields, collection): (&[&str], Option<(&str, &[&str])>) =
                match block.block_type.as_str() {
                    "HeroBlock" | "Hero" => (&["headline", "subtitle", "copy", "image"], None),
                    "ProductGridBlock" | "Catalog" => (
                        &["items"],
                        Some((
                            "items",
                            &["product_id", "name", "description", "price", "image"],
                        )),
                    ),
                    "ServiceBookingBlock" | "Booking" | "BookingCalendarBlock" => {
                        (&["title", "availability", "booking_url"], None)
                    }
                    "TestimonialBlock" | "Testimonials" => {
                        (&["quotes"], Some(("quotes", &["text", "author"])))
                    }
                    "Contact" | "ContactFormBlock" => (&["email", "phone"], None),
                    "Text" | "TextBlock" => (&["text"], None),
                    "LinkListBlock" => (&["links"], Some(("links", &["label", "url"]))),
                    _ => return Err(PublicationError::Invalid("Unsupported reviewed block type")),
                };
            allowed_fields(&block.content, fields)?;
            if let Some((key, fields)) = collection {
                let items = block
                    .content
                    .get(key)
                    .and_then(Value::as_array)
                    .ok_or(PublicationError::Invalid("Invalid reviewed block list"))?;
                for item in items {
                    allowed_fields(item, fields)?;
                }
            }
        }
    }
    // Rendering is pure and validates every configured URL/text value before
    // persistence. The durable worker renders again under current authority.
    render_snapshot(snapshot, Uuid::nil())?;
    Ok(())
}

#[derive(Serialize)]
struct WireReceipt<'a> {
    schema_version: u8,
    user_id: &'a str,
    organization_id: &'a str,
    snapshot_encoding: &'static str,
    #[serde(flatten)]
    receipt: PublicationReceipt,
}

async fn receipt_response(
    pool: &PgPool,
    actor: &PublicationActor,
    mut receipt: PublicationReceipt,
    submitted: bool,
) -> Result<Response, PublicationError> {
    if receipt.status == PublicationStatus::Published {
        match read_public_page(pool, receipt.site_id, "/").await {
            Ok(page) if page.publication_id == receipt.publication_id => {
                receipt.public_path = Some(format!("/api/v1/public/sites/{}", receipt.site_id));
            }
            Ok(_) | Err(PublicationError::Unauthorized | PublicationError::NotFound) => {}
            Err(error) => return Err(error),
        }
    }
    let status = if submitted
        && matches!(
            receipt.status,
            PublicationStatus::Pending | PublicationStatus::Processing
        ) {
        StatusCode::ACCEPTED
    } else {
        StatusCode::OK
    };
    Ok((
        status,
        Json(WireReceipt {
            schema_version: 1,
            user_id: &actor.user_id,
            organization_id: &actor.tenant_id,
            snapshot_encoding: SNAPSHOT_ENCODING,
            receipt,
        }),
    )
        .into_response())
}

async fn bounded<F>(work: F) -> Response
where
    F: std::future::Future<Output = Result<Response, PublicationError>>,
{
    match tokio::time::timeout(std::time::Duration::from_secs(15), work).await {
        Ok(Ok(response)) => response,
        Ok(Err(error)) => failure(error),
        Err(_) => failure(PublicationError::Database(sqlx::Error::PoolTimedOut)),
    }
}

async fn submit(
    State(pool): State<PgPool>,
    Extension(claims): Extension<server_common::Claims>,
    headers: HeaderMap,
    body: Result<Bytes, BytesRejection>,
) -> Response {
    bounded(async {
        let actor = actor(&claims, &headers)?;
        let bytes = match body {
            Ok(bytes) => bytes,
            Err(error) => {
                return Ok(failure_with_status(
                    error.status(),
                    "Invalid or oversized publication body",
                ));
            }
        };
        if headers
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(';').next())
            .map(str::trim)
            != Some("application/json")
        {
            return Err(PublicationError::Invalid(
                "Publication requires application/json",
            ));
        }
        let value = decode_publication_json(&bytes)?;
        if value.get("site_id").is_none()
            || value
                .get("snapshot")
                .and_then(|snapshot| snapshot.get("domain"))
                .is_none()
        {
            return Err(PublicationError::Invalid(
                "Nullable publication fields must be explicit",
            ));
        }
        let request: PublishRequest = serde_json::from_value(value)
            .map_err(|_| PublicationError::Invalid("Invalid publication request fields"))?;
        if request.snapshot_encoding != SNAPSHOT_ENCODING {
            return Err(PublicationError::Invalid("Unsupported snapshot encoding"));
        }
        validate_content(&request.snapshot)?;
        let receipt = submit_publication(
            &pool,
            &actor,
            request.operation_id,
            request.site_id,
            &request.snapshot,
        )
        .await?;
        receipt_response(&pool, &actor, receipt, true).await
    })
    .await
}
fn failure_with_status(status: StatusCode, message: &'static str) -> Response {
    let mut response = failure(PublicationError::Invalid(message));
    *response.status_mut() = status;
    response
}
async fn receipt(
    State(pool): State<PgPool>,
    Extension(claims): Extension<server_common::Claims>,
    headers: HeaderMap,
    Path(operation_id): Path<Uuid>,
) -> Response {
    bounded(async {
        let actor = actor(&claims, &headers)?;
        let receipt = read_owned_publication(&pool, &actor, operation_id).await?;
        receipt_response(&pool, &actor, receipt, false).await
    })
    .await
}
async fn revoke(
    State(pool): State<PgPool>,
    Extension(claims): Extension<server_common::Claims>,
    headers: HeaderMap,
    Path(publication_id): Path<Uuid>,
) -> Response {
    bounded(async {
        let actor = actor(&claims, &headers)?;
        let receipt = revoke_owned_publication(&pool, &actor, publication_id).await?;
        receipt_response(&pool, &actor, receipt, false).await
    })
    .await
}
async fn page(pool: PgPool, site_id: String, path: String) -> Response {
    bounded(async {
        let site_id = Uuid::parse_str(&site_id).map_err(|_| PublicationError::NotFound)?;
        let page = match read_public_page(&pool, site_id, &path).await {
            Err(PublicationError::Unauthorized) => return Err(PublicationError::NotFound),
            result => result?,
        };
        document_response(page)
    })
    .await
}
async fn root_page(State(pool): State<PgPool>, Path(site_id): Path<String>) -> Response {
    page(pool, site_id, "/".into()).await
}
async fn document_page(
    State(pool): State<PgPool>,
    Path((site_id, path)): Path<(String, String)>,
) -> Response {
    page(pool, site_id, format!("/{path}")).await
}

fn document_response(page: PublicPage) -> Result<Response, PublicationError> {
    if page.html.len() > 8 * 1024 * 1024 {
        return Err(PublicationError::Corrupt);
    }
    Ok((
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8".to_string()),
            (header::X_CONTENT_TYPE_OPTIONS, "nosniff".to_string()),
            (header::REFERRER_POLICY, "no-referrer".to_string()),
            (header::CONTENT_SECURITY_POLICY, CSP.to_string()),
            (header::ETAG, format!("\"{}\"", page.rendered_sha256)),
        ],
        page.html,
    )
        .into_response())
}
async fn product_page(
    State(pool): State<PgPool>,
    Path((site_id, product_id)): Path<(String, String)>,
) -> Response {
    bounded(async {
        let site = Uuid::parse_str(&site_id).map_err(|_| PublicationError::NotFound)?;
        let product = Uuid::parse_str(&product_id).map_err(|_| PublicationError::NotFound)?;
        let page = match read_public_product(&pool, site, product).await {
            Err(PublicationError::Unauthorized) => return Err(PublicationError::NotFound),
            result => result?,
        };
        document_response(page)
    })
    .await
}
