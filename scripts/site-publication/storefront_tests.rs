use super::*;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use std::{sync::Arc, time::Duration};
use tower::ServiceExt;
struct PrivateFixture {
    data: Fixture,
    app: Router,
    owner: String,
    foreign: String,
    member: String,
}
impl PrivateFixture {
    async fn new() -> Self {
        let data = Fixture::new(false).await;
        let auth = Arc::new(server_auth::Store::new());
        let mut tokens = Vec::new();
        for (tenant, role) in [
            (&data.a.tenant_id, "ADMIN"),
            (&data.b.tenant_id, "ADMIN"),
            (&data.a.tenant_id, "MEMBER"),
        ] {
            let name = format!("storefront-{}", Uuid::new_v4());
            let user = auth
                .create_user(
                    name.clone(),
                    format!("{name}@example.test"),
                    "local-regression-password".into(),
                    vec![role.into()],
                    tenant.clone(),
                )
                .await
                .unwrap();
            tokens.push(auth.issue_token(&user).unwrap());
        }
        let app = crate::application_storefront_routes(data.pool.clone(), auth);
        Self {
            data,
            app,
            owner: tokens.remove(0),
            foreign: tokens.remove(0),
            member: tokens.remove(0),
        }
    }
    async fn get(
        &self,
        path: &str,
        token: Option<&str>,
        host: Option<&str>,
    ) -> (StatusCode, axum::http::HeaderMap, String) {
        let mut request = Request::builder().uri(path);
        if let Some(token) = token {
            request = request.header("authorization", format!("Bearer {token}"));
        }
        if let Some(host) = host {
            request = request.header("x-forwarded-host", host);
        }
        let response = self
            .app
            .clone()
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        let body = String::from_utf8(
            to_bytes(response.into_body(), 1024 * 1024)
                .await
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        (status, headers, body)
    }
}
#[tokio::test]
async fn mutable_product_cache_never_bypasses_current_tenant_or_owner() {
    let f = PrivateFixture::new().await;
    let path = format!(
        "/api/v1/storefront/{}/{}",
        f.data.a.tenant_id, f.data.product_a
    );
    crate::builder::edge::get_edge_cache()
        .set_with_tags(
            &format!(
                "storefront:product:{}:{}",
                f.data.a.tenant_id, f.data.product_a
            ),
            "<h1>Private recorded product</h1>".into(),
            vec![format!("tenant-id:{}", f.data.a.tenant_id)],
            Duration::from_secs(120),
        )
        .await;
    let own = f.get(&path, Some(&f.owner), None).await;
    assert_eq!(own.0, StatusCode::OK);
    assert!(own.2.contains("Private recorded product"));
    let foreign = f.get(&path, Some(&f.foreign), None).await;
    assert_eq!(
        foreign.0,
        StatusCode::NOT_FOUND,
        "A cached response must not bypass tenant selection"
    );
    assert!(!foreign.2.contains("Private recorded product"));
    assert_eq!(
        f.get(&path, Some(&f.member), None).await.0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(f.get(&path, None, None).await.0, StatusCode::UNAUTHORIZED);
    assert_eq!(own.1["cache-control"], "private, no-store");
    f.data.finish().await;
}
#[tokio::test]
async fn mutable_domain_cache_cannot_resolve_another_tenants_site() {
    let f = PrivateFixture::new().await;
    let site = Uuid::new_v4();
    let domain = format!("{}.example.test", Uuid::new_v4());
    sqlx::query("INSERT INTO builder_sites(id,tenant_id,domain) VALUES($1,$2,$3)")
        .bind(site)
        .bind(Uuid::parse_str(&f.data.a.tenant_id).unwrap())
        .bind(&domain)
        .execute(&f.data.admin)
        .await
        .unwrap();
    crate::builder::edge::get_edge_cache()
        .set_with_tags(
            &format!("edge_site_{}_{}_en-US", f.data.a.tenant_id, site),
            "<h1>Private domain draft</h1>".into(),
            vec![format!("tenant-id:{}", f.data.a.tenant_id)],
            Duration::from_secs(120),
        )
        .await;
    // A real shared response-cache entry can survive from an earlier permitted
    // render. The request still must recheck the current owner's domain before
    // serving it; otherwise this cache also bypasses restricted-role RLS.
    crate::utils::edge_caching_middleware::get_cdn_cache()
        .set_with_tags(
            "cdn:/resolve_domain",
            crate::utils::edge_caching_middleware::CachedResponse {
                status: 200,
                headers: vec![("cache-control".into(), "public, s-maxage=60".into())],
                body: b"<h1>Private domain draft</h1>".to_vec(),
            },
            vec![format!("tenant-id:{}", f.data.a.tenant_id)],
            Duration::from_secs(120),
        )
        .await;
    let own = f
        .get(
            "/api/v1/storefront/resolve_domain",
            Some(&f.owner),
            Some(&domain),
        )
        .await;
    assert_eq!(own.0, StatusCode::OK);
    assert!(own.2.contains("Private domain draft"));
    let foreign = f
        .get(
            "/api/v1/storefront/resolve_domain",
            Some(&f.foreign),
            Some(&domain),
        )
        .await;
    assert_eq!(
        foreign.0,
        StatusCode::NOT_FOUND,
        "Same URI/Host cache cannot grant another tenant draft access"
    );
    assert!(!foreign.2.contains("Private domain draft"));
    assert_eq!(own.1["cache-control"], "private, no-store");
    f.data.finish().await;
}

#[tokio::test]
async fn invalidation_cannot_evict_another_tenants_cached_records() {
    let f = PrivateFixture::new().await;
    let cache = crate::builder::edge::get_edge_cache();
    let own_key = format!("invalidation-owned-{}", Uuid::new_v4());
    let foreign_key = format!("invalidation-foreign-{}", Uuid::new_v4());
    let own_tag = format!("tenant-id:{}", f.data.a.tenant_id);
    let foreign_tag = format!("tenant-id:{}", f.data.b.tenant_id);
    cache
        .set_with_tags(
            &own_key,
            "Owner cache".into(),
            vec![own_tag.clone()],
            Duration::from_secs(120),
        )
        .await;
    cache
        .set_with_tags(
            &foreign_key,
            "Foreign cache".into(),
            vec![
                foreign_tag.clone(),
                format!("entity:product:{}", f.data.product_b),
            ],
            Duration::from_secs(120),
        )
        .await;
    for tags in [
        vec![foreign_tag.clone()],
        vec![own_tag.clone(), foreign_tag],
        vec![format!("entity:product:{}", f.data.product_b)],
    ] {
        let response = f
            .app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/storefront/webhook/invalidate")
                    .header("authorization", format!("Bearer {}", f.owner))
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::to_vec(&json!({"tags":tags})).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(response.headers()["cache-control"], "private, no-store");
        assert_eq!(cache.get(&own_key).await.as_deref(), Some("Owner cache"));
        assert_eq!(
            cache.get(&foreign_key).await.as_deref(),
            Some("Foreign cache")
        );
    }
    f.data.finish().await;
}

#[tokio::test]
async fn expired_or_missing_product_cannot_be_resurrected_by_a_private_cache_entry() {
    let f = PrivateFixture::new().await;
    let missing = Uuid::new_v4();
    let path = format!("/api/v1/storefront/{}/{missing}", f.data.a.tenant_id);
    crate::builder::edge::get_edge_cache()
        .set(
            &format!("storefront:product:{}:{missing}", f.data.a.tenant_id),
            "Private deleted product".into(),
            Duration::from_secs(120),
        )
        .await;
    let response = f.get(&path, Some(&f.owner), None).await;
    assert_eq!(response.0, StatusCode::NOT_FOUND);
    assert!(!response.2.contains("Private deleted product"));
    f.data.finish().await;
}
