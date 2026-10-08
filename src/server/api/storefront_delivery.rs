use crate::builder::edge::{
    get_edge_cache, get_ongoing_generation, inject_dynamic_inventory, regenerate_cache,
};
use crate::utils::cache::HybridCache;
use axum::http::StatusCode;
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Extension, Path, State},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeliveryState {
    pub pool: PgPool,
    auth: Arc<server_auth::Store>,
}

pub fn router<S: Clone + Send + Sync + 'static>(
    pool: PgPool,
    auth: Arc<server_auth::Store>,
) -> Router<S> {
    let state = DeliveryState {
        pool,
        auth: auth.clone(),
    };
    Router::new()
        .route(
            "/{tenant_id}/{product_id}",
            get(get_storefront_product).layer(axum::middleware::from_fn(
                crate::utils::edge_caching_middleware::edge_caching_middleware,
            )),
        )
        .route("/webhook/invalidate", post(invalidate_cache_webhook))
        .route(
            "/resolve_domain",
            get(resolve_domain).layer(axum::middleware::from_fn(
                crate::utils::edge_caching_middleware::edge_caching_middleware,
            )),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            private_owner,
        ))
        .route_layer(axum::middleware::from_fn_with_state(
            auth,
            server_auth::strict_bearer_auth_middleware,
        ))
        .layer(axum::middleware::from_fn(private_headers))
        .layer(DefaultBodyLimit::max(16 * 1024))
        .with_state(state)
}

async fn private_headers(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        axum::http::HeaderValue::from_static("private, no-store"),
    );
    response
}

async fn scoped_read(
    pool: &PgPool,
    tenant: &str,
) -> Result<sqlx::Transaction<'static, sqlx::Postgres>, StatusCode> {
    let mut tx = tokio::time::timeout(std::time::Duration::from_secs(3), pool.begin())
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    server_common::auth_utils::set_org_context(&mut *tx, tenant)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("SET LOCAL statement_timeout='3000ms'")
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    sqlx::query("SET LOCAL lock_timeout='1000ms'")
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(tx)
}

async fn private_owner(
    State(state): State<DeliveryState>,
    mut request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let Some(claims) = request.extensions().get::<server_common::Claims>().cloned() else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let owner =
        match server_auth::commit_authority::verify_owner(&state.auth, &claims, request.headers())
            .await
        {
            Ok(owner) => owner,
            Err(server_auth::commit_authority::AuthorityError::Forbidden) => {
                return StatusCode::FORBIDDEN.into_response();
            }
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
        };
    let path = request
        .uri()
        .path()
        .strip_prefix("/api/v1/storefront")
        .unwrap_or(request.uri().path());
    let segments: Vec<_> = path.trim_start_matches('/').split('/').collect();
    if let [tenant, product] = segments.as_slice()
        && (*tenant != "webhook" || *product != "invalidate")
    {
        if *tenant != owner.tenant_id() {
            return StatusCode::NOT_FOUND.into_response();
        }
        let Ok(product) = Uuid::parse_str(product) else {
            return StatusCode::BAD_REQUEST.into_response();
        };
        let mut tx = match scoped_read(&state.pool, owner.tenant_id()).await {
            Ok(tx) => tx,
            Err(status) => return status.into_response(),
        };
        let exists: Result<bool, _> = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM products WHERE id=$1 AND tenant_id=$2)",
        )
        .bind(product.to_string())
        .bind(owner.tenant_id())
        .fetch_one(&mut *tx)
        .await;
        match exists {
            Ok(false) => return StatusCode::NOT_FOUND.into_response(),
            Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
            Ok(true) => {}
        }
        if tx.commit().await.is_err() {
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    }
    let headers = request.headers().clone();
    request.extensions_mut().insert(owner);
    let response = next.run(request).await;
    match server_auth::commit_authority::verify_owner(&state.auth, &claims, &headers).await {
        Ok(_) => response,
        Err(server_auth::commit_authority::AuthorityError::Forbidden) => {
            StatusCode::FORBIDDEN.into_response()
        }
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

async fn resolve_domain(
    axum::extract::State(state): axum::extract::State<DeliveryState>,
    Extension(owner): Extension<server_auth::commit_authority::VerifiedOwner>,
    req: axum::extract::Request,
) -> Result<impl IntoResponse, StatusCode> {
    let host = req
        .headers()
        .get("X-Forwarded-Host")
        .or_else(|| req.headers().get("Host"))
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    // Remove port if present
    let domain = host.split(':').next().unwrap_or(host).to_string();

    // Domain is a selector, never authority. Do not resolve mutable drafts
    // across tenants, even when another tenant owns the requested hostname.
    let tenant_id =
        Uuid::parse_str(owner.tenant_id()).map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let mut tx = scoped_read(&state.pool, owner.tenant_id()).await?;
    let site_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM builder_sites WHERE domain=$1 AND tenant_id=$2")
            .bind(&domain)
            .bind(tenant_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    let site_id = site_id.ok_or(StatusCode::NOT_FOUND)?;
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let cache = get_edge_cache();
    let cache_key = format!("edge_site_{}_{}_{}", tenant_id, site_id, "en-US");

    if let Some((cached_html, is_stale)) = cache.get_with_swr(&cache_key).await {
        let mut response = Html(cached_html.clone()).into_response();
        set_storefront_headers(&mut response, &cached_html, tenant_id, None);
        if !is_stale {
            return Ok(response);
        } else {
            let pool_clone = state.pool.clone();
            let cache_key_clone = cache_key.clone();
            let cache_clone = cache.clone();
            tokio::spawn(async move {
                let _ =
                    regenerate_cache(pool_clone, tenant_id, site_id, cache_key_clone, cache_clone)
                        .await;
            });
            return Ok(response);
        }
    }

    if let Ok((html, tags)) = regenerate_cache(
        state.pool.clone(),
        tenant_id,
        site_id,
        cache_key.clone(),
        cache.clone(),
    )
    .await
    {
        let mut response = Html(html.clone()).into_response();
        set_storefront_headers(&mut response, &html, tenant_id, Some(tags));
        return Ok(response);
    }

    Err(StatusCode::NOT_FOUND)
}

pub struct CacheInvalidationService {
    cache: Arc<HybridCache<String>>,
}

impl CacheInvalidationService {
    pub fn new(cache: Arc<HybridCache<String>>) -> Self {
        Self { cache }
    }

    pub async fn invalidate(&self, tags: Vec<String>) {
        let futures = tags.into_iter().map(|tag| async move {
            self.cache.invalidate_by_tag(&tag).await;
        });
        futures::future::join_all(futures).await;
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvalidateRequest {
    pub tags: Vec<String>,
}

async fn owned_invalidation_tags(
    pool: &PgPool,
    tenant: &str,
    tags: &[String],
) -> Result<(), StatusCode> {
    if tags.is_empty()
        || tags.len() > 32
        || tags
            .iter()
            .any(|tag| tag.len() > 1024 || tag.chars().any(char::is_control))
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut tx = scoped_read(pool, tenant).await?;
    for tag in tags {
        if tag == &format!("tenant-id:{tenant}") {
            continue;
        }
        let Some(raw_id) = tag.strip_prefix("entity:product:") else {
            return Err(StatusCode::FORBIDDEN);
        };
        let id = Uuid::parse_str(raw_id).map_err(|_| StatusCode::BAD_REQUEST)?;
        if raw_id != id.to_string() {
            return Err(StatusCode::BAD_REQUEST);
        }
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM products WHERE id=$1 AND tenant_id=$2)",
        )
        .bind(raw_id)
        .bind(tenant)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
        if !exists {
            return Err(StatusCode::FORBIDDEN);
        }
    }
    tx.commit()
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(())
}

async fn invalidate_cache_webhook(
    State(state): State<DeliveryState>,
    Extension(owner): Extension<server_auth::commit_authority::VerifiedOwner>,
    Json(payload): Json<InvalidateRequest>,
) -> axum::response::Response {
    match tokio::time::timeout(
        std::time::Duration::from_secs(3),
        owned_invalidation_tags(&state.pool, owner.tenant_id(), &payload.tags),
    )
    .await
    {
        Ok(Ok(())) => {}
        Ok(Err(status)) => return status.into_response(),
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
    let cache = get_edge_cache();
    let service = CacheInvalidationService::new(cache);
    service.invalidate(payload.tags.clone()).await;

    let tags_to_invalidate = payload.tags;
    tokio::spawn(async move {
        let cdn = crate::utils::edge_caching_middleware::get_cdn_cache();
        let client = reqwest::Client::new();
        let futures = tags_to_invalidate.into_iter().map(|tag| {
            let cdn_clone = cdn.clone();
            let client_clone = client.clone();
            async move {
                cdn_clone.invalidate_by_tag(&tag).await;

                // Send purge request to NGINX Edge Cache
                match tokio::time::timeout(
                    std::time::Duration::from_secs(3),
                    client_clone
                        .post("http://edge-cache/purge")
                        .body(tag)
                        .send(),
                )
                .await
                {
                    Ok(Ok(response)) if response.status().is_success() => {
                        tracing::info!("Edge cache acknowledged a tenant-scoped purge")
                    }
                    _ => tracing::warn!("Edge cache purge remains unconfirmed"),
                }
            }
        });
        futures::future::join_all(futures).await;
    });

    (
        StatusCode::ACCEPTED,
        Json(serde_json::json!({"local_cache_invalidated":true,"edge_purge":"unconfirmed"})),
    )
        .into_response()
}

async fn get_storefront_product(
    State(state): State<DeliveryState>,
    Path((tenant_id_str, product_id_str)): Path<(String, String)>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| StatusCode::BAD_REQUEST)?;
    let product_id = Uuid::parse_str(&product_id_str).map_err(|_| StatusCode::BAD_REQUEST)?;

    let cache = get_edge_cache();
    let cache_key = format!("storefront:product:{}:{}", tenant_id, product_id);

    if let Some((cached_html, is_stale)) = cache.get_with_swr(&cache_key).await {
        let html =
            inject_dynamic_inventory(cached_html, tenant_id, &state.pool, cache.clone()).await;
        let mut response = Html(html.clone()).into_response();
        set_storefront_headers(&mut response, &html, tenant_id, None);

        if !is_stale {
            return Ok(response);
        } else {
            let ongoing = get_ongoing_generation();
            let mut guard = ongoing.lock().await;
            if !guard.contains(&cache_key) {
                guard.insert(cache_key.clone());
                let pool_clone = state.pool.clone();
                let cache_key_clone = cache_key.clone();
                let cache_clone = cache.clone();
                tokio::spawn(async move {
                    let _ = crate::builder::edge::regenerate_product_cache(
                        pool_clone,
                        tenant_id,
                        product_id,
                        cache_key_clone.clone(),
                        cache_clone,
                    )
                    .await;
                    let ongoing = get_ongoing_generation();
                    ongoing.lock().await.remove(&cache_key_clone);
                });
            }
            return Ok(response);
        }
    }

    let ongoing = get_ongoing_generation();
    let is_generating = {
        let mut guard = ongoing.lock().await;
        if guard.contains(&cache_key) {
            true
        } else {
            guard.insert(cache_key.clone());
            false
        }
    };

    if is_generating {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        if let Some((cached_html, _)) = cache.get_with_swr(&cache_key).await {
            let html =
                inject_dynamic_inventory(cached_html, tenant_id, &state.pool, cache.clone()).await;
            let mut response = Html(html.clone()).into_response();
            set_storefront_headers(&mut response, &html, tenant_id, None);
            return Ok(response);
        }
    }

    let result = crate::builder::edge::regenerate_product_cache(
        state.pool.clone(),
        tenant_id,
        product_id,
        cache_key.clone(),
        cache.clone(),
    )
    .await;

    {
        let ongoing = get_ongoing_generation();
        ongoing.lock().await.remove(&cache_key);
    }

    let failure = match result {
        Ok((html, tags)) => {
            let final_html =
                inject_dynamic_inventory(html, tenant_id, &state.pool, cache.clone()).await;
            let mut response = Html(final_html.clone()).into_response();
            set_storefront_headers(&mut response, &final_html, tenant_id, Some(tags));
            return Ok(response);
        }
        Err(status) => status,
    };
    Err(failure)
}

fn set_storefront_headers(
    response: &mut axum::response::Response,
    html: &str,
    tenant_id: Uuid,
    custom_tags: Option<Vec<String>>,
) {
    let mut hasher = Sha256::new();
    hasher.update(html.as_bytes());
    let result = hasher.finalize();
    let etag = format!("\"{:x}\"", result);

    let mut tags = vec![format!("tenant-id:{}", tenant_id)];
    if let Some(mut ct) = custom_tags {
        tags.append(&mut ct);
    }

    let cache_tag_comma = tags.join(", ");
    let surrogate_key_space = tags.join(" ");

    if let Ok(val) = cache_tag_comma.parse() {
        response.headers_mut().insert("Cache-Tag", val);
    }

    if let Ok(val) = surrogate_key_space.parse() {
        response.headers_mut().insert("Surrogate-Key", val);
    }

    if let Ok(val) = etag.parse() {
        response.headers_mut().insert("ETag", val);
    }

    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        "private, no-store".parse().unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::header::CACHE_CONTROL;

    #[test]
    fn storefront_headers_bind_cache_tags_to_tenant_and_etag_to_content() {
        let tenant = Uuid::from_u128(1);
        let mut response = axum::response::Response::new(Body::empty());
        set_storefront_headers(
            &mut response,
            "<h1>Store</h1>",
            tenant,
            Some(vec!["catalog:published".into()]),
        );
        assert_eq!(
            response.headers()["Cache-Tag"],
            format!("tenant-id:{tenant}, catalog:published")
        );
        assert_eq!(
            response.headers()["Surrogate-Key"],
            format!("tenant-id:{tenant} catalog:published")
        );
        assert_eq!(response.headers()[CACHE_CONTROL], "private, no-store");
        let expected = format!("\"{:x}\"", Sha256::digest(b"<h1>Store</h1>"));
        assert_eq!(response.headers()["ETag"], expected);
        let previous = response.headers()["ETag"].clone();
        set_storefront_headers(&mut response, "<h1>Updated</h1>", Uuid::from_u128(2), None);
        assert_ne!(response.headers()["ETag"], previous);
        assert_eq!(
            response.headers()["Cache-Tag"],
            format!("tenant-id:{}", Uuid::from_u128(2))
        );
    }

    #[test]
    fn storefront_custom_tags_cannot_inject_http_headers() {
        let mut response = axum::response::Response::new(Body::empty());
        set_storefront_headers(
            &mut response,
            "content",
            Uuid::from_u128(1),
            Some(vec!["tag\r\nSet-Cookie: forged=1".into()]),
        );
        assert!(!response.headers().contains_key("Set-Cookie"));
        assert!(!response.headers().contains_key("Cache-Tag"));
        assert!(!response.headers().contains_key("Surrogate-Key"));
        assert!(response.headers().contains_key("ETag"));
        assert!(response.headers().contains_key(CACHE_CONTROL));
    }
}
