use super::*;
use crate::builder::{publication_http, publication_worker};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{HeaderMap, Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

struct HttpFixture {
    data: Fixture,
    auth: Arc<server_auth::Store>,
    app: Router,
    owner: String,
    member: String,
    foreign: String,
}
impl HttpFixture {
    async fn new() -> Self {
        let mut data = Fixture::new(false).await;
        let auth = Arc::new(server_auth::Store::new());
        let mut tokens = Vec::new();
        for (tenant, role) in [
            (&data.a.tenant_id, "ADMIN"),
            (&data.a.tenant_id, "VIEWER"),
            (&data.b.tenant_id, "ADMIN"),
        ] {
            let name = format!("publication-{}", Uuid::new_v4().simple());
            let user = auth
                .create_user(
                    name.clone(),
                    format!("{name}@example.test"),
                    "public-local-fixture-password".into(),
                    vec![role.into()],
                    tenant.clone(),
                )
                .await
                .unwrap();
            sqlx::query("INSERT INTO users(id,username,email,tenant_id,active,roles) VALUES($1,$2,$3,$4,true,ARRAY[$5]::text[])")
                .bind(&user.id).bind(&name).bind(&user.email).bind(tenant).bind(role).execute(&data.admin).await.unwrap();
            sqlx::query("INSERT INTO identity_user_roles(user_id,role_name,tenant_id,position) VALUES($1,$2,$3,0)")
                .bind(&user.id).bind(role).bind(tenant).execute(&data.admin).await.unwrap();
            if tokens.is_empty() {
                data.a.user_id = user.id.clone();
            }
            tokens.push(auth.issue_token(&user).unwrap());
        }
        let app = publication_http::router(data.pool.clone(), auth.clone());
        Self {
            data,
            auth,
            app,
            owner: tokens.remove(0),
            member: tokens.remove(0),
            foreign: tokens.remove(0),
        }
    }
    fn body(&self, id: Uuid) -> serde_json::Value {
        json!({"operation_id":id,"site_id":null,"snapshot_encoding":"jcs-rfc8785-v1","snapshot":self.data.snapshot("Explicit owner review")})
    }
    async fn request(
        &self,
        method: &str,
        path: &str,
        token: Option<&str>,
        body: serde_json::Value,
    ) -> (StatusCode, HeaderMap, Vec<u8>) {
        let mut req = Request::builder()
            .method(method)
            .uri(path)
            .header("x-tenant-id", &self.data.b.tenant_id)
            .header("x-spiffe-id", "spiffe://forged/tenant/not-an-authority");
        if let Some(token) = token {
            req = req.header("authorization", format!("Bearer {token}"));
        }
        let body = if method == "GET" {
            Body::empty()
        } else {
            req = req.header("content-type", "application/json");
            Body::from(serde_json::to_vec(&body).unwrap())
        };
        let response = self
            .app
            .clone()
            .oneshot(req.body(body).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        (
            status,
            headers,
            to_bytes(response.into_body(), 2 * 1024 * 1024)
                .await
                .unwrap()
                .to_vec(),
        )
    }
    async fn publish(&self) -> PublicationReceipt {
        let receipt = submit_publication(
            &self.data.pool,
            &self.data.a,
            Uuid::new_v4(),
            None,
            &self.data.snapshot("Actual committed public document"),
        )
        .await
        .unwrap();
        let work = publication_worker::discover_publication_work(&self.data.pool, 64)
            .await
            .unwrap()
            .into_iter()
            .find(|work| work.publication_id == receipt.publication_id)
            .unwrap();
        let claim = publication_worker::claim_publication(&self.data.pool, &work)
            .await
            .unwrap()
            .unwrap();
        publication_worker::finish_publication(&self.data.pool, &claim)
            .await
            .unwrap()
    }
}

#[tokio::test]
async fn mounted_publication_rejects_unsigned_member_and_disabled_actor_before_effects() {
    let f = HttpFixture::new().await;
    let body = f.body(Uuid::new_v4());
    let unsigned = f
        .request("POST", "/api/v1/builder/publications", None, body.clone())
        .await;
    let member = f
        .request(
            "POST",
            "/api/v1/builder/publications",
            Some(&f.member),
            body.clone(),
        )
        .await;
    f.auth
        .update_user(
            &f.data.a.user_id,
            None,
            None,
            Some(false),
            &f.data.a.tenant_id,
        )
        .await
        .unwrap();
    let disabled = f
        .request("POST", "/api/v1/builder/publications", Some(&f.owner), body)
        .await;
    let counts = f.data.counts().await;
    f.data.finish().await;
    assert_eq!(unsigned.0, StatusCode::UNAUTHORIZED);
    assert_eq!(member.0, StatusCode::FORBIDDEN);
    assert_eq!(disabled.0, StatusCode::UNAUTHORIZED);
    assert_eq!(counts, (0, 0, 0));
}

#[tokio::test]
async fn mounted_publication_returns_the_actual_committed_pending_receipt_and_exact_replay() {
    let f = HttpFixture::new().await;
    let id = Uuid::new_v4();
    let body = f.body(id);
    let first = f
        .request(
            "POST",
            "/api/v1/builder/publications",
            Some(&f.owner),
            body.clone(),
        )
        .await;
    let second = f
        .request("POST", "/api/v1/builder/publications", Some(&f.owner), body)
        .await;
    let read = f
        .request(
            "GET",
            &format!("/api/v1/builder/publications/operations/{id}"),
            Some(&f.owner),
            json!(null),
        )
        .await;
    let counts = f.data.counts().await;
    let actor = f.data.a.clone();
    f.data.finish().await;
    assert_eq!(first.0, StatusCode::ACCEPTED);
    assert_eq!(second.0, StatusCode::ACCEPTED);
    assert_eq!(read.0, StatusCode::OK);
    let receipt: serde_json::Value = serde_json::from_slice(&first.2).unwrap();
    assert_eq!(receipt["schema_version"], 1);
    assert_eq!(receipt["snapshot_encoding"], "jcs-rfc8785-v1");
    assert_eq!(receipt["user_id"], actor.user_id);
    assert_eq!(receipt["organization_id"], actor.tenant_id);
    assert_eq!(receipt["operation_id"], id.to_string());
    assert_eq!(receipt["status"], "pending");
    assert!(receipt["public_path"].is_null());
    assert_eq!(
        receipt,
        serde_json::from_slice::<serde_json::Value>(&second.2).unwrap()
    );
    assert_eq!(
        receipt,
        serde_json::from_slice::<serde_json::Value>(&read.2).unwrap()
    );
    assert_eq!(counts, (1, 1, 1));
}

#[tokio::test]
async fn mounted_receipt_is_private_and_current_canonical_revocation_denies_recovery() {
    let f = HttpFixture::new().await;
    let receipt = f.publish().await;
    let path = format!(
        "/api/v1/builder/publications/operations/{}",
        receipt.operation_id
    );
    let unsigned = f.request("GET", &path, None, json!(null)).await;
    let foreign = f.request("GET", &path, Some(&f.foreign), json!(null)).await;
    let current = f.request("GET", &path, Some(&f.owner), json!(null)).await;
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id=$1")
        .bind(&f.data.a.user_id)
        .execute(&f.data.admin)
        .await
        .unwrap();
    let revoked = f.request("GET", &path, Some(&f.owner), json!(null)).await;
    f.data.finish().await;
    assert_eq!(unsigned.0, StatusCode::UNAUTHORIZED);
    assert_eq!(foreign.0, StatusCode::NOT_FOUND);
    assert_eq!(current.0, StatusCode::OK);
    assert_eq!(revoked.0, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn anonymous_public_document_requires_a_current_receipt_and_safe_response_headers() {
    let f = HttpFixture::new().await;
    let receipt = f.publish().await;
    let path = format!("/api/v1/public/sites/{}", receipt.site_id);
    let response = f.request("GET", &path, None, json!(null)).await;
    let missing = f
        .request("GET", &format!("{path}/pages/private"), None, json!(null))
        .await;
    let mutation = f
        .request("POST", &path, None, json!({"title":"forged"}))
        .await;
    sqlx::query("UPDATE users SET active=false WHERE id=$1")
        .bind(&f.data.a.user_id)
        .execute(&f.data.admin)
        .await
        .unwrap();
    let revoked = f.request("GET", &path, None, json!(null)).await;
    f.data.finish().await;
    assert_eq!(response.0, StatusCode::OK);
    assert_eq!(response.1["cache-control"], "no-store");
    assert!(
        response.1["content-type"]
            .to_str()
            .unwrap()
            .starts_with("text/html")
    );
    assert_eq!(response.1["x-content-type-options"], "nosniff");
    assert!(response.1.contains_key("content-security-policy"));
    assert!(response.1.contains_key("etag"));
    assert!(
        String::from_utf8(response.2)
            .unwrap()
            .contains("Actual committed public document")
    );
    assert_eq!(missing.0, StatusCode::NOT_FOUND);
    assert_eq!(mutation.0, StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(revoked.0, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn pending_legacy_and_malformed_public_references_do_not_expose_private_drafts() {
    let f = HttpFixture::new().await;
    let receipt = submit_publication(
        &f.data.pool,
        &f.data.a,
        Uuid::new_v4(),
        None,
        &f.data.snapshot("Private pending draft"),
    )
    .await
    .unwrap();
    let pending = f
        .request(
            "GET",
            &format!("/api/v1/public/sites/{}", receipt.site_id),
            None,
            json!(null),
        )
        .await;
    let invalid = f
        .request("GET", "/api/v1/public/sites/not-a-site", None, json!(null))
        .await;
    let missing = f
        .request(
            "GET",
            &format!("/api/v1/public/sites/{}", Uuid::new_v4()),
            None,
            json!(null),
        )
        .await;
    f.data.finish().await;
    for response in [pending, invalid, missing] {
        assert_eq!(response.0, StatusCode::NOT_FOUND);
        assert!(
            !String::from_utf8(response.2)
                .unwrap()
                .contains("Private pending draft")
        );
    }
}

#[tokio::test]
async fn version_specific_revoke_preserves_newer_publication_and_private_drafts() {
    let f = HttpFixture::new().await;
    let first = f.publish().await;
    let second = submit_publication(
        &f.data.pool,
        &f.data.a,
        Uuid::new_v4(),
        Some(first.site_id),
        &f.data.snapshot("New explicitly reviewed version"),
    )
    .await
    .unwrap();
    let work = publication_worker::discover_publication_work(&f.data.pool, 64)
        .await
        .unwrap()
        .into_iter()
        .find(|work| work.publication_id == second.publication_id)
        .unwrap();
    let claim = publication_worker::claim_publication(&f.data.pool, &work)
        .await
        .unwrap()
        .unwrap();
    publication_worker::finish_publication(&f.data.pool, &claim)
        .await
        .unwrap();
    let old_path = format!("/api/v1/builder/publications/{}", first.publication_id);
    let new_path = format!("/api/v1/builder/publications/{}", second.publication_id);
    let foreign = f
        .request("DELETE", &old_path, Some(&f.foreign), json!(null))
        .await;
    let retired = f
        .request("DELETE", &old_path, Some(&f.owner), json!(null))
        .await;
    let after_old = f
        .request(
            "GET",
            &format!("/api/v1/public/sites/{}", first.site_id),
            None,
            json!(null),
        )
        .await;
    let old_receipt = f
        .request(
            "GET",
            &format!(
                "/api/v1/builder/publications/operations/{}",
                first.operation_id
            ),
            Some(&f.owner),
            json!(null),
        )
        .await;
    let revoked = f
        .request("DELETE", &new_path, Some(&f.owner), json!(null))
        .await;
    let replay = f
        .request("DELETE", &new_path, Some(&f.owner), json!(null))
        .await;
    let final_public = f
        .request(
            "GET",
            &format!("/api/v1/public/sites/{}", first.site_id),
            None,
            json!(null),
        )
        .await;
    let counts = f.data.counts().await;
    f.data.finish().await;
    assert_eq!(foreign.0, StatusCode::NOT_FOUND);
    assert_eq!(retired.0, StatusCode::OK);
    assert_eq!(after_old.0, StatusCode::OK);
    assert!(
        String::from_utf8(after_old.2)
            .unwrap()
            .contains("New explicitly reviewed version")
    );
    assert_eq!(old_receipt.0, StatusCode::OK);
    let old: serde_json::Value = serde_json::from_slice(&old_receipt.2).unwrap();
    assert_eq!(old["operation_id"], first.operation_id.to_string());
    assert_eq!(old["publication_id"], first.publication_id.to_string());
    assert_eq!(old["status"], "revoked");
    assert!(old["public_path"].is_null());
    assert_eq!(revoked.0, StatusCode::OK);
    assert_eq!(replay.0, StatusCode::OK);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&revoked.2).unwrap(),
        serde_json::from_slice::<serde_json::Value>(&replay.2).unwrap()
    );
    assert_eq!(final_public.0, StatusCode::NOT_FOUND);
    assert_eq!(counts, (1, 1, 2));
}

#[tokio::test]
async fn publication_http_rejects_custom_domains_and_unsupported_nested_content_before_writes() {
    let f = HttpFixture::new().await;
    let mut domains = f.body(Uuid::new_v4());
    domains["snapshot"]["domain"] = json!("unverified.example.test");
    let mut content = f.body(Uuid::new_v4());
    content["snapshot"]["pages"][0]["blocks"][0]["content"]["onclick"] =
        json!("unreviewed control");
    let domain = f
        .request(
            "POST",
            "/api/v1/builder/publications",
            Some(&f.owner),
            domains,
        )
        .await;
    let nested = f
        .request(
            "POST",
            "/api/v1/builder/publications",
            Some(&f.owner),
            content,
        )
        .await;
    let counts = f.data.counts().await;
    f.data.finish().await;
    assert_eq!(domain.0, StatusCode::BAD_REQUEST);
    assert_eq!(nested.0, StatusCode::BAD_REQUEST);
    assert_eq!(counts, (0, 0, 0));
}

#[tokio::test]
async fn publication_http_rejects_oversized_snapshots_without_persistence() {
    let f = HttpFixture::new().await;
    let mut body = f.body(Uuid::new_v4());
    body["snapshot"]["pages"][0]["blocks"][0]["content"]["items"][0]["description"] =
        json!("x".repeat(1_048_577));
    let response = f
        .request("POST", "/api/v1/builder/publications", Some(&f.owner), body)
        .await;
    let counts = f.data.counts().await;
    f.data.finish().await;
    assert_eq!(response.0, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(counts, (0, 0, 0));
}

#[tokio::test]
async fn postgres_receipt_digest_matches_independent_javascript_jcs() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let f = HttpFixture::new().await;
    let mut input = f.body(Uuid::new_v4());
    input["snapshot"]["pages"][0]["seo_metadata"] = json!({
        "name":"Unicode 雪", "\u{e000}":1e-7, "😀":-0.0, "a":1.0,
        "fractions":[0.000001, 0.1, 333333333.3333333, 9007199254740991_u64, 5e-324]
    });
    let response = f
        .request(
            "POST",
            "/api/v1/builder/publications",
            Some(&f.owner),
            input.clone(),
        )
        .await;
    let rows: Vec<serde_json::Value> =
        sqlx::query_scalar("SELECT snapshot FROM builder_publications")
            .fetch_all(&f.data.admin)
            .await
            .unwrap();
    f.data.finish().await;
    assert_eq!(response.0, StatusCode::ACCEPTED);
    assert_eq!(rows.len(), 1);
    let receipt: serde_json::Value = serde_json::from_slice(&response.2).unwrap();
    for snapshot in [&input["snapshot"], &rows[0]] {
        let mut child = Command::new("node")
            .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/jcs-proof.cjs"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(&serde_json::to_vec(snapshot).unwrap())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let proof: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(receipt["snapshot_sha256"], proof["sha256"]);
    }
}

#[tokio::test]
async fn publication_http_requires_explicit_nullable_structural_fields() {
    let f = HttpFixture::new().await;
    for field in ["site_id", "domain"] {
        let mut body = f.body(Uuid::new_v4());
        let object = if field == "domain" {
            &mut body["snapshot"]
        } else {
            &mut body
        };
        object.as_object_mut().unwrap().remove(field);
        let response = f
            .request("POST", "/api/v1/builder/publications", Some(&f.owner), body)
            .await;
        assert_eq!(response.0, StatusCode::BAD_REQUEST, "missing {field}");
    }
    assert_eq!(f.data.counts().await, (0, 0, 0));
    f.data.finish().await;
}

#[tokio::test]
async fn actual_worker_advances_http_receipt_and_serves_reviewed_nested_document() {
    let f = HttpFixture::new().await;
    // A neighboring application route shares the same prefix in the real server.
    let _combined = Router::new()
        .nest(
            "/api/v1/builder",
            Router::new().route(
                "/draft",
                axum::routing::get(|| async { StatusCode::NO_CONTENT }),
            ),
        )
        .merge(f.app.clone());
    let id = Uuid::new_v4();
    let mut body = f.body(id);
    let mut other = body["snapshot"]["pages"][0].clone();
    other["path"] = json!("/review/about");
    other["title"] = json!("Nested reviewed document");
    body["snapshot"]["pages"]
        .as_array_mut()
        .unwrap()
        .push(other);
    let submitted = f
        .request("POST", "/api/v1/builder/publications", Some(&f.owner), body)
        .await;
    assert_eq!(submitted.0, StatusCode::ACCEPTED);
    let (shutdown, receiver) = tokio::sync::watch::channel(false);
    let worker = tokio::spawn(publication_worker::run_publication_worker(
        f.data.pool.clone(),
        receiver,
    ));
    let receipt = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let response = f
                .request(
                    "GET",
                    &format!("/api/v1/builder/publications/operations/{id}"),
                    Some(&f.owner),
                    json!(null),
                )
                .await;
            assert_eq!(response.0, StatusCode::OK);
            let receipt: serde_json::Value = serde_json::from_slice(&response.2).unwrap();
            if receipt["status"] == "published" {
                break receipt;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    })
    .await;
    shutdown.send(true).unwrap();
    worker.await.unwrap();
    let receipt = receipt.unwrap();
    let public = receipt["public_path"].as_str().unwrap();
    let page = f
        .request(
            "GET",
            &format!("{public}/pages/review/about"),
            None,
            json!(null),
        )
        .await;
    assert_eq!(page.0, StatusCode::OK);
    assert!(
        String::from_utf8(page.2)
            .unwrap()
            .contains("Nested reviewed document")
    );
    sqlx::query("DELETE FROM products WHERE id=$1")
        .bind(f.data.product_a.to_string())
        .execute(&f.data.admin)
        .await
        .unwrap();
    let revoked = f
        .request(
            "GET",
            &format!("/api/v1/builder/publications/operations/{id}"),
            Some(&f.owner),
            json!(null),
        )
        .await;
    let revoked: serde_json::Value = serde_json::from_slice(&revoked.2).unwrap();
    assert_eq!(revoked["status"], "published");
    assert!(revoked["public_path"].is_null());
    assert_eq!(
        f.request("GET", public, None, json!(null)).await.0,
        StatusCode::NOT_FOUND
    );
    f.data.finish().await;
}

#[tokio::test]
async fn raw_http_admission_rejects_lossy_json_before_any_write() {
    let f = HttpFixture::new().await;
    let body = f.body(Uuid::new_v4());
    let encoded = serde_json::to_string(&body).unwrap();
    let raw = [
        encoded.replace("\"domain\":null", "\"domain\":null,\"domain\":null"),
        encoded.replace(
            "\"name\":\"Explicit owner review\"",
            "\"name\":\"Explicit owner review\",\"unsafe\":9007199254740991.1",
        ),
        encoded.replace("Explicit owner review", r"nul\u0000text"),
    ];
    for raw in raw {
        let response = f
            .app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/builder/publications")
                    .header("authorization", format!("Bearer {}", f.owner))
                    .header("content-type", "application/json")
                    .body(Body::from(raw))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(response.headers()["cache-control"], "no-store");
        let error: serde_json::Value =
            serde_json::from_slice(&to_bytes(response.into_body(), 65536).await.unwrap()).unwrap();
        assert_eq!(error["error"], "publication_invalid");
        assert_eq!(error["effect"], "none");
    }
    assert_eq!(f.data.counts().await, (0, 0, 0));
    f.data.finish().await;
}

#[tokio::test]
async fn expected_owner_mismatch_and_duplicate_headers_reject_before_mutation() {
    let f = HttpFixture::new().await;
    for duplicate in [false, true] {
        let mut request = Request::builder()
            .method("POST")
            .uri("/api/v1/builder/publications")
            .header("authorization", format!("Bearer {}", f.owner))
            .header("content-type", "application/json")
            .header(
                "x-ohc-expected-user",
                if duplicate {
                    &f.data.a.user_id
                } else {
                    &f.data.b.user_id
                },
            )
            .header("x-ohc-expected-tenant", &f.data.a.tenant_id);
        if duplicate {
            request = request.header("x-ohc-expected-user", &f.data.a.user_id);
        }
        let response = f
            .app
            .clone()
            .oneshot(
                request
                    .body(Body::from(
                        serde_json::to_vec(&f.body(Uuid::new_v4())).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
    assert_eq!(f.data.counts().await, (0, 0, 0));
    f.data.finish().await;
}

#[tokio::test]
async fn selected_product_document_uses_reviewed_snapshot_and_rechecks_public_authority() {
    let f = HttpFixture::new().await;
    let receipt = f.publish().await;
    let path = format!(
        "/api/v1/public/sites/{}/products/{}",
        receipt.site_id, f.data.product_a
    );
    // A subsequent private catalog edit is not an explicit republication.
    sqlx::query("UPDATE products SET title='Unreviewed change',price_cents=9999 WHERE id=$1")
        .bind(f.data.product_a.to_string())
        .execute(&f.data.admin)
        .await
        .unwrap();
    let response = f.request("GET", &path, None, json!(null)).await;
    let foreign = f
        .request(
            "GET",
            &format!(
                "/api/v1/public/sites/{}/products/{}",
                receipt.site_id, f.data.product_b
            ),
            None,
            json!(null),
        )
        .await;
    sqlx::query("UPDATE users SET active=false WHERE id=$1")
        .bind(&f.data.a.user_id)
        .execute(&f.data.admin)
        .await
        .unwrap();
    let revoked = f.request("GET", &path, None, json!(null)).await;
    f.data.finish().await;
    assert_eq!(response.0, StatusCode::OK);
    assert_eq!(response.1["cache-control"], "no-store");
    assert_eq!(response.1["content-type"], "text/html; charset=utf-8");
    let html = String::from_utf8(response.2).unwrap();
    assert!(html.contains("Actual committed public document"));
    assert!(html.contains("12.34"));
    assert!(!html.contains("Unreviewed change"));
    assert!(!html.contains("99.99"));
    assert_eq!(foreign.0, StatusCode::NOT_FOUND);
    assert_eq!(revoked.0, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn publication_routes_preserve_an_independent_nonunit_application_state() {
    #[derive(Clone)]
    struct OuterState {
        marker: String,
    }
    let mut f = HttpFixture::new().await;
    let outer: Router<Arc<OuterState>> = Router::new().route(
        "/independent-state",
        axum::routing::get(
            |axum::extract::State(state): axum::extract::State<Arc<OuterState>>| async move {
                state.marker.clone()
            },
        ),
    );
    // This merge previously failed exactly like the production MeshTransport
    // routes: publication must not force the outer router's state to ().
    f.app = outer
        .merge(publication_http::router(
            f.data.pool.clone(),
            f.auth.clone(),
        ))
        .with_state(Arc::new(OuterState {
            marker: "actual outer state survived".into(),
        }));
    let outer = f
        .request("GET", "/independent-state", None, json!(null))
        .await;
    assert_eq!(outer.0, StatusCode::OK);
    assert_eq!(outer.2, b"actual outer state survived");
    let id = Uuid::new_v4();
    let unsigned = f
        .request("POST", "/api/v1/builder/publications", None, f.body(id))
        .await;
    assert_eq!(unsigned.0, StatusCode::UNAUTHORIZED);
    let submitted = f
        .request(
            "POST",
            "/api/v1/builder/publications",
            Some(&f.owner),
            f.body(id),
        )
        .await;
    assert_eq!(submitted.0, StatusCode::ACCEPTED);
    let receipt: serde_json::Value = serde_json::from_slice(&submitted.2).unwrap();
    assert_eq!(receipt["user_id"], f.data.a.user_id);
    let public = f
        .request(
            "GET",
            &format!(
                "/api/v1/public/sites/{}",
                receipt["site_id"].as_str().unwrap()
            ),
            None,
            json!(null),
        )
        .await;
    assert_eq!(public.0, StatusCode::NOT_FOUND);
    assert_eq!(f.data.counts().await, (1, 1, 1));
    f.data.finish().await;
}
