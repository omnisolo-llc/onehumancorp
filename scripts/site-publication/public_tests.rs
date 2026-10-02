use super::*;
use crate::builder::publication_public::read_public_page;
use crate::builder::publication_worker::{
    claim_publication, discover_publication_work, finish_publication,
};

async fn publish(f: &Fixture, title: &str) -> PublicationReceipt {
    let receipt = submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &f.snapshot(title))
        .await
        .unwrap();
    let work = discover_publication_work(&f.pool, 64)
        .await
        .unwrap()
        .into_iter()
        .find(|work| work.publication_id == receipt.publication_id)
        .unwrap();
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    finish_publication(&f.pool, &claim).await.unwrap()
}

#[tokio::test]
async fn public_read_delivers_only_current_committed_bytes_for_uuid_and_legacy_tenants() {
    for legacy in [false, true] {
        let f = Fixture::new(legacy).await;
        let pending = submit_publication(
            &f.pool,
            &f.a,
            Uuid::new_v4(),
            None,
            &f.snapshot("Pending private"),
        )
        .await
        .unwrap();
        assert!(
            read_public_page(&f.pool, pending.site_id, "/")
                .await
                .is_err()
        );
        let published = publish(&f, "Explicit public page").await;
        let result = read_public_page(&f.pool, published.site_id, "/").await;
        let expected:(serde_json::Value,String)=sqlx::query_as("SELECT rendered_pages,rendered_sha256 FROM builder_publications WHERE publication_id=$1").bind(published.publication_id).fetch_one(&f.admin).await.unwrap();
        f.finish().await;
        let page=result.expect("current explicitly committed content must be readable with restricted database authority");
        assert_eq!(page.publication_id, published.publication_id);
        assert_eq!(page.path, "/");
        assert_eq!(page.rendered_sha256, expected.1);
        assert_eq!(page.html, expected.0["/"].as_str().unwrap());
    }
}

#[tokio::test]
async fn every_public_read_rechecks_revoked_owner_role_account_and_selected_product() {
    for kind in ["role", "account", "product", "receipt"] {
        let f = Fixture::new(false).await;
        let published = publish(&f, "Revocable public data").await;
        let first = read_public_page(&f.pool, published.site_id, "/").await;
        match kind {
            "role" => {
                sqlx::query("DELETE FROM identity_user_roles WHERE user_id=$1")
                    .bind(&f.a.user_id)
                    .execute(&f.admin)
                    .await
                    .unwrap();
            }
            "account" => {
                sqlx::query("UPDATE users SET active=false WHERE id=$1")
                    .bind(&f.a.user_id)
                    .execute(&f.admin)
                    .await
                    .unwrap();
            }
            "product" => {
                sqlx::query("UPDATE products SET tenant_id=$2 WHERE id=$1")
                    .bind(f.product_a.to_string())
                    .bind(&f.b.tenant_id)
                    .execute(&f.admin)
                    .await
                    .unwrap();
            }
            _ => {
                sqlx::query("UPDATE builder_publications SET status='revoked',revoked_at=clock_timestamp() WHERE publication_id=$1").bind(published.publication_id).execute(&f.admin).await.unwrap();
            }
        }
        let after = read_public_page(&f.pool, published.site_id, "/").await;
        f.finish().await;
        assert!(first.is_ok(), "precondition: actual first public read");
        assert!(
            after.is_err(),
            "{kind} revocation must deny even previously read bytes"
        );
    }
}

#[tokio::test]
async fn public_read_waits_and_rechecks_an_actual_owner_revocation_transaction() {
    let f = Fixture::new(false).await;
    let published = publish(&f, "Waiting public read").await;
    let mut revoke = f.admin.begin().await.unwrap();
    sqlx::query("UPDATE users SET active=false WHERE id=$1")
        .bind(&f.a.user_id)
        .execute(&mut *revoke)
        .await
        .unwrap();
    let pool = f.pool.clone();
    let read = tokio::spawn(async move { read_public_page(&pool, published.site_id, "/").await });
    let blocked=tokio::time::timeout(std::time::Duration::from_secs(2),async{loop{let blocked:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE application_name=$1 AND wait_event_type='Lock')").bind(&f.schema).fetch_one(&f.admin).await.unwrap();if blocked {break;}tokio::time::sleep(std::time::Duration::from_millis(10)).await;}}).await;
    revoke.commit().await.unwrap();
    let result = read.await.unwrap();
    f.finish().await;
    assert!(
        blocked.is_ok(),
        "read must wait on the actual authority row"
    );
    assert!(result.is_err());
}

#[tokio::test]
async fn public_read_fails_closed_with_a_database_timeout_for_a_persistently_locked_owner() {
    let f = Fixture::new(false).await;
    let published = publish(&f, "Bounded public read").await;
    let mut revoke = f.admin.begin().await.unwrap();
    sqlx::query("UPDATE users SET active=false WHERE id=$1")
        .bind(&f.a.user_id)
        .execute(&mut *revoke)
        .await
        .unwrap();
    let pool = f.pool.clone();
    let mut read =
        tokio::spawn(async move { read_public_page(&pool, published.site_id, "/").await });
    let observed = tokio::time::timeout(std::time::Duration::from_secs(3), &mut read).await;
    let timed_out_in_database = match observed {
        Ok(Ok(Err(PublicationError::Database(error)))) => error
            .as_database_error()
            .is_some_and(|error| error.code().as_deref() == Some("55P03")),
        Err(_) => {
            read.abort();
            let _ = read.await;
            false
        }
        _ => false,
    };
    revoke.rollback().await.unwrap();
    let recovered = read_public_page(&f.pool, published.site_id, "/").await;
    f.finish().await;
    assert!(
        timed_out_in_database,
        "the database must bound an authority lock wait before the outer test deadline"
    );
    assert!(
        recovered.is_ok(),
        "the aborted read must not poison the reused pool or publication"
    );
}

#[tokio::test]
async fn public_paths_never_escape_the_immutable_document_map() {
    let f = Fixture::new(false).await;
    let published = publish(&f, "Only root").await;
    let root = read_public_page(&f.pool, published.site_id, "/").await;
    for path in [
        "//other",
        "/../private",
        "/%2e%2e/private",
        "/missing",
        "/\\private",
        "/\u{0000}",
    ] {
        assert!(
            read_public_page(&f.pool, published.site_id, path)
                .await
                .is_err(),
            "invalid path {path:?}"
        );
    }
    f.finish().await;
    assert!(root.is_ok());
}

#[tokio::test]
async fn missing_routing_and_tenant_cleanup_never_reuse_public_content() {
    let f = Fixture::new(false).await;
    let published = publish(&f, "Routed content").await;
    let first = read_public_page(&f.pool, published.site_id, "/").await;
    sqlx::query("DELETE FROM builder_publication_work WHERE publication_id=$1")
        .bind(published.publication_id)
        .execute(&f.admin)
        .await
        .unwrap();
    let missing = read_public_page(&f.pool, published.site_id, "/").await;
    sqlx::query("DELETE FROM tenants WHERE id=$1")
        .bind(&f.a.tenant_id)
        .execute(&f.admin)
        .await
        .unwrap();
    let deleted = read_public_page(&f.pool, published.site_id, "/").await;
    let sites: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM builder_sites WHERE id=$1")
        .bind(published.site_id)
        .fetch_one(&f.admin)
        .await
        .unwrap();
    f.finish().await;
    assert_eq!(
        sites, 0,
        "tenant cleanup must also remove the bound private draft"
    );
    assert!(first.is_ok());
    assert!(missing.is_err());
    assert!(deleted.is_err());
}

#[tokio::test]
async fn a_raw_tenant_uuid_mapping_collision_cannot_lend_another_owners_public_authority() {
    let mut f = Fixture::new(true).await;
    let first = publish(&f, "Legacy raw tenant").await;
    let mapped = Uuid::new_v5(&Uuid::NAMESPACE_DNS, f.a.tenant_id.as_bytes()).to_string();
    sqlx::query("INSERT INTO tenants(id,name) VALUES($1,'Distinct raw UUID tenant')")
        .bind(&mapped)
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query("UPDATE users SET tenant_id=$2 WHERE id=$1")
        .bind(&f.b.user_id)
        .bind(&mapped)
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query("UPDATE identity_user_roles SET tenant_id=$2 WHERE user_id=$1")
        .bind(&f.b.user_id)
        .bind(&mapped)
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query("UPDATE products SET tenant_id=$2 WHERE id=$1")
        .bind(f.product_b.to_string())
        .bind(&mapped)
        .execute(&f.admin)
        .await
        .unwrap();
    f.b.tenant_id = mapped;
    let mut second_snapshot = f.snapshot("Distinct UUID tenant");
    second_snapshot.pages[0].blocks[0].content["items"][0]["product_id"] =
        json!(f.product_b.to_string());
    let second = submit_publication(&f.pool, &f.b, Uuid::new_v4(), None, &second_snapshot)
        .await
        .unwrap();
    let work = discover_publication_work(&f.pool, 64)
        .await
        .unwrap()
        .into_iter()
        .find(|work| work.publication_id == second.publication_id)
        .unwrap();
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    finish_publication(&f.pool, &claim).await.unwrap();
    let before = read_public_page(&f.pool, first.site_id, "/").await;
    sqlx::query("UPDATE users SET active=false WHERE id=$1")
        .bind(&f.a.user_id)
        .execute(&f.admin)
        .await
        .unwrap();
    let retired = read_public_page(&f.pool, first.site_id, "/").await;
    let other = read_public_page(&f.pool, second.site_id, "/").await;
    f.finish().await;
    assert!(before.is_ok());
    assert!(retired.is_err());
    assert!(
        other.is_ok(),
        "distinct raw tenant remains independently authorized"
    );
    assert_eq!(other.unwrap().publication_id, second.publication_id);
}

#[tokio::test]
async fn routing_identity_and_public_pointer_cannot_be_repointed_to_another_site() {
    let f = Fixture::new(false).await;
    let first = publish(&f, "First public version").await;
    let mut foreign = f.snapshot("Foreign public site");
    foreign.pages[0].blocks[0].content["items"][0]["product_id"] = json!(f.product_b.to_string());
    let second = submit_publication(&f.pool, &f.b, Uuid::new_v4(), None, &foreign)
        .await
        .unwrap();
    let work = discover_publication_work(&f.pool, 64)
        .await
        .unwrap()
        .into_iter()
        .find(|work| work.publication_id == second.publication_id)
        .unwrap();
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    finish_publication(&f.pool, &claim).await.unwrap();
    let before = read_public_page(&f.pool, first.site_id, "/").await;
    let route_change =
        sqlx::query("UPDATE builder_publication_work SET site_id=$2 WHERE publication_id=$1")
            .bind(first.publication_id)
            .bind(second.site_id)
            .execute(&f.admin)
            .await;
    let pointer_change =
        sqlx::query("UPDATE builder_sites SET current_publication_id=$2 WHERE id=$1")
            .bind(first.site_id)
            .bind(second.publication_id)
            .execute(&f.admin)
            .await;
    let after = read_public_page(&f.pool, first.site_id, "/").await;
    f.finish().await;
    assert!(route_change.is_err());
    assert!(pointer_change.is_err());
    assert_eq!(before.unwrap().publication_id, first.publication_id);
    assert_eq!(after.unwrap().publication_id, first.publication_id);
}

#[tokio::test]
async fn an_ambiguous_encoded_document_path_cannot_commit_an_unreachable_publication() {
    let f = Fixture::new(false).await;
    let mut snapshot = f.snapshot("Unreachable document");
    let mut other = snapshot.pages[0].clone();
    other.path = "/%2e%2e/private".into();
    snapshot.pages.push(other);
    let result = submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &snapshot).await;
    let counts = f.counts().await;
    f.finish().await;
    assert!(matches!(result, Err(PublicationError::Invalid(_))));
    assert_eq!(counts, (0, 0, 0));
}
