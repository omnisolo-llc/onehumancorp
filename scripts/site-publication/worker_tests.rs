use super::*;
use crate::builder::publication_worker::{
    claim_publication, discover_publication_work, finish_publication,
};

async fn submit(f: &Fixture, title: &str) -> PublicationReceipt {
    submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &f.snapshot(title))
        .await
        .unwrap()
}
async fn current(f: &Fixture, site: Uuid) -> Option<Uuid> {
    sqlx::query_scalar("SELECT current_publication_id FROM builder_sites WHERE id=$1")
        .bind(site)
        .fetch_one(&f.admin)
        .await
        .unwrap()
}

#[tokio::test]
async fn restart_discovers_committed_routing_and_publishes_exact_reviewed_content() {
    let mut f = Fixture::new(true).await;
    let receipt = submit(&f, "Restarted reviewed site").await;
    let options = (*f.pool.connect_options()).clone();
    f.pool.close().await;
    f.pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(8)
        .connect_with(options)
        .await
        .unwrap();
    let work = discover_publication_work(&f.pool, 20).await.unwrap();
    assert_eq!(work.len(), 1);
    assert_eq!(
        (work[0].publication_id, work[0].tenant_id.as_str()),
        (receipt.publication_id, f.a.tenant_id.as_str())
    );
    let claim = claim_publication(&f.pool, &work[0]).await.unwrap().unwrap();
    assert!(current(&f, receipt.site_id).await.is_none());
    let completed = finish_publication(&f.pool, &claim).await.unwrap();
    assert_eq!(completed.status, PublicationStatus::Published);
    assert_eq!(
        current(&f, receipt.site_id).await,
        Some(receipt.publication_id)
    );
    let (pages, digest): (serde_json::Value, String) = sqlx::query_as(
        "SELECT rendered_pages,rendered_sha256 FROM builder_publications WHERE publication_id=$1",
    )
    .bind(receipt.publication_id)
    .fetch_one(&f.admin)
    .await
    .unwrap();
    let expected = crate::builder::publication_render::render_snapshot(
        &f.snapshot("Restarted reviewed site"),
        receipt.site_id,
    )
    .unwrap();
    assert_eq!(pages, serde_json::to_value(expected.pages).unwrap());
    assert_eq!(digest, expected.sha256);
    assert!(
        discover_publication_work(&f.pool, 20)
            .await
            .unwrap()
            .is_empty()
    );
    f.finish().await;
}

#[tokio::test]
async fn concurrent_duplicate_dispatch_has_only_one_current_lease() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Concurrent").await;
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let (a, b) = tokio::join!(
        claim_publication(&f.pool, &work),
        claim_publication(&f.pool, &work)
    );
    let a = a.unwrap();
    let b = b.unwrap();
    assert_ne!(a.is_some(), b.is_some());
    let claim = a.or(b).unwrap();
    finish_publication(&f.pool, &claim).await.unwrap();
    assert_eq!(
        current(&f, receipt.site_id).await,
        Some(receipt.publication_id)
    );
    f.finish().await;
}

#[tokio::test]
async fn expired_lease_recovery_fences_a_late_old_worker() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Lease").await;
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let old = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    sqlx::query("UPDATE builder_publications SET lease_until=clock_timestamp()-INTERVAL '1 second' WHERE publication_id=$1").bind(receipt.publication_id).execute(&f.admin).await.unwrap();
    let new = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    assert!(matches!(
        finish_publication(&f.pool, &old).await,
        Err(PublicationError::Conflict)
    ));
    assert!(current(&f, receipt.site_id).await.is_none());
    finish_publication(&f.pool, &new).await.unwrap();
    assert_eq!(
        current(&f, receipt.site_id).await,
        Some(receipt.publication_id)
    );
    f.finish().await;
}

#[tokio::test]
async fn newer_reviewed_generation_prevents_older_completion() {
    let f = Fixture::new(false).await;
    let first = submit(&f, "First").await;
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let old = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    let second = submit_publication(
        &f.pool,
        &f.a,
        Uuid::new_v4(),
        Some(first.site_id),
        &f.snapshot("Second"),
    )
    .await
    .unwrap();
    assert!(matches!(
        finish_publication(&f.pool, &old).await,
        Err(PublicationError::Conflict)
    ));
    assert!(current(&f, first.site_id).await.is_none());
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .into_iter()
        .find(|w| w.publication_id == second.publication_id)
        .unwrap();
    let new = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    finish_publication(&f.pool, &new).await.unwrap();
    assert_eq!(
        current(&f, first.site_id).await,
        Some(second.publication_id)
    );
    f.finish().await;
}

#[tokio::test]
async fn revocation_after_claim_blocks_public_completion() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Revoked owner").await;
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id=$1")
        .bind(&f.a.user_id)
        .execute(&f.admin)
        .await
        .unwrap();
    assert!(matches!(
        finish_publication(&f.pool, &claim).await,
        Err(PublicationError::Unauthorized)
    ));
    assert!(current(&f, receipt.site_id).await.is_none());
    f.finish().await;
}

#[tokio::test]
async fn reassigned_owner_cannot_finish_the_former_tenants_claim() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Reassigned owner").await;
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    sqlx::query("UPDATE users SET tenant_id=$2 WHERE id=$1")
        .bind(&f.a.user_id)
        .bind(&f.b.tenant_id)
        .execute(&f.admin)
        .await
        .unwrap();
    assert!(matches!(
        finish_publication(&f.pool, &claim).await,
        Err(PublicationError::Unauthorized)
    ));
    assert!(current(&f, receipt.site_id).await.is_none());
    f.finish().await;
}

#[tokio::test]
async fn selected_product_reassignment_blocks_publication() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Moved product").await;
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    sqlx::query("UPDATE products SET tenant_id=$2 WHERE id=$1")
        .bind(f.product_a.to_string())
        .bind(&f.b.tenant_id)
        .execute(&f.admin)
        .await
        .unwrap();
    assert!(matches!(
        finish_publication(&f.pool, &claim).await,
        Err(PublicationError::NotFound)
    ));
    assert!(current(&f, receipt.site_id).await.is_none());
    f.finish().await;
}

#[tokio::test]
async fn explicitly_revoked_receipt_cannot_resume_or_complete() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Revoked receipt").await;
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    sqlx::query("UPDATE builder_publications SET status='revoked',revoked_at=clock_timestamp() WHERE publication_id=$1").bind(receipt.publication_id).execute(&f.admin).await.unwrap();
    assert!(claim_publication(&f.pool, &work).await.unwrap().is_none());
    assert!(matches!(
        finish_publication(&f.pool, &claim).await,
        Err(PublicationError::Conflict)
    ));
    assert!(current(&f, receipt.site_id).await.is_none());
    f.finish().await;
}

#[tokio::test]
async fn deleting_tenant_removes_routing_and_prevents_a_late_completion() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Deleted tenant").await;
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    sqlx::query("DELETE FROM tenants WHERE id=$1")
        .bind(&f.a.tenant_id)
        .execute(&f.admin)
        .await
        .unwrap();
    assert!(
        discover_publication_work(&f.pool, 20)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(finish_publication(&f.pool, &claim).await.is_err());
    assert!(current(&f, receipt.site_id).await.is_none());
    f.finish().await;
}

#[tokio::test]
async fn forged_and_missing_routing_metadata_never_authorize_another_tenant() {
    use crate::builder::publication_worker::PublicationWorkItem;
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Private").await;
    assert_eq!(
        discover_publication_work(&f.pool, 20).await.unwrap().len(),
        1
    );
    let forged = PublicationWorkItem {
        publication_id: receipt.publication_id,
        tenant_id: f.b.tenant_id.clone(),
    };
    assert!(claim_publication(&f.pool, &forged).await.unwrap().is_none());
    let missing = PublicationWorkItem {
        publication_id: Uuid::new_v4(),
        tenant_id: f.a.tenant_id.clone(),
    };
    assert!(
        claim_publication(&f.pool, &missing)
            .await
            .unwrap()
            .is_none()
    );
    let result =
        sqlx::query("INSERT INTO builder_publication_work(publication_id,tenant_id) VALUES($1,$2)")
            .bind(Uuid::new_v4())
            .bind(&f.a.tenant_id)
            .execute(&f.admin)
            .await;
    assert!(result.is_err());
    let result =
        sqlx::query("UPDATE builder_publication_work SET tenant_id=$2 WHERE publication_id=$1")
            .bind(receipt.publication_id)
            .bind(&f.b.tenant_id)
            .execute(&f.admin)
            .await;
    assert!(result.is_err());
    assert!(current(&f, receipt.site_id).await.is_none());
    f.finish().await;
}

#[tokio::test]
async fn unsupported_rendering_is_terminal_without_a_public_pointer() {
    let f = Fixture::new(false).await;
    let mut snapshot = f.snapshot("Unsupported");
    snapshot.pages[0].blocks[0].block_type = "UnknownExecutable".into();
    let receipt = submit_publication(&f.pool, &f.a, Uuid::new_v4(), None, &snapshot)
        .await
        .unwrap();
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    assert!(matches!(
        finish_publication(&f.pool, &claim).await,
        Err(PublicationError::Invalid(_))
    ));
    let status: String =
        sqlx::query_scalar("SELECT status FROM builder_publications WHERE publication_id=$1")
            .bind(receipt.publication_id)
            .fetch_one(&f.admin)
            .await
            .unwrap();
    assert_eq!(status, "failed");
    assert!(
        discover_publication_work(&f.pool, 20)
            .await
            .unwrap()
            .is_empty()
    );
    assert!(current(&f, receipt.site_id).await.is_none());
    f.finish().await;
}

#[tokio::test]
async fn successful_completion_replay_returns_the_same_committed_receipt() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Replay").await;
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    let first = finish_publication(&f.pool, &claim).await.unwrap();
    let second = finish_publication(&f.pool, &claim).await.unwrap();
    assert_eq!(first.publication_id, second.publication_id);
    assert_eq!(second.status, PublicationStatus::Published);
    assert_eq!(
        current(&f, receipt.site_id).await,
        Some(first.publication_id)
    );
    f.finish().await;
}

#[tokio::test]
async fn deferred_worker_commit_rejection_never_exposes_a_public_pointer() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Rejected final commit").await;
    let work = discover_publication_work(&f.pool, 20)
        .await
        .unwrap()
        .remove(0);
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    sqlx::raw_sql("CREATE SEQUENCE worker_commit_seen; CREATE FUNCTION reject_worker_commit() RETURNS TRIGGER LANGUAGE plpgsql AS $$ BEGIN IF NEW.status='published' THEN PERFORM nextval('worker_commit_seen'); RAISE EXCEPTION 'synthetic worker commit rejection'; END IF; RETURN NEW; END; $$; CREATE CONSTRAINT TRIGGER reject_worker_commit AFTER UPDATE ON builder_publications DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION reject_worker_commit();").execute(&f.admin).await.unwrap();
    sqlx::query(&format!(
        "GRANT USAGE, SELECT ON SEQUENCE worker_commit_seen TO {}",
        f.role
    ))
    .execute(&f.admin)
    .await
    .unwrap();
    let result = finish_publication(&f.pool, &claim).await;
    assert!(
        matches!(result, Err(PublicationError::Database(_))),
        "{result:?}"
    );
    let observed: bool = sqlx::query_scalar("SELECT is_called FROM worker_commit_seen")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    assert!(
        observed,
        "the actual deferred commit trigger must have executed"
    );
    assert!(current(&f, receipt.site_id).await.is_none());
    let status: String =
        sqlx::query_scalar("SELECT status FROM builder_publications WHERE publication_id=$1")
            .bind(receipt.publication_id)
            .fetch_one(&f.admin)
            .await
            .unwrap();
    assert_eq!(status, "processing");
    sqlx::query("DROP TRIGGER reject_worker_commit ON builder_publications")
        .execute(&f.admin)
        .await
        .unwrap();
    finish_publication(&f.pool, &claim).await.unwrap();
    assert_eq!(
        current(&f, receipt.site_id).await,
        Some(receipt.publication_id)
    );
    f.finish().await;
}

#[tokio::test]
async fn final_worker_authority_and_product_checks_recheck_after_actual_lock_wait() {
    for kind in ["account", "role", "product"] {
        let f = Fixture::new(false).await;
        let receipt = submit(&f, "Racing final authority").await;
        let work = discover_publication_work(&f.pool, 20)
            .await
            .unwrap()
            .remove(0);
        let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
        let mut change = f.admin.begin().await.unwrap();
        match kind {
            "product" => {
                sqlx::query("UPDATE products SET tenant_id=$2 WHERE id=$1")
                    .bind(f.product_a.to_string())
                    .bind(&f.b.tenant_id)
                    .execute(&mut *change)
                    .await
                    .unwrap();
            }
            "role" => {
                sqlx::query("DELETE FROM identity_user_roles WHERE user_id=$1")
                    .bind(&f.a.user_id)
                    .execute(&mut *change)
                    .await
                    .unwrap();
            }
            _ => {
                sqlx::query("UPDATE users SET active=false WHERE id=$1")
                    .bind(&f.a.user_id)
                    .execute(&mut *change)
                    .await
                    .unwrap();
            }
        }
        let pool = f.pool.clone();
        let task = tokio::spawn(async move { finish_publication(&pool, &claim).await });
        let waited=tokio::time::timeout(std::time::Duration::from_secs(5),async {
            loop {
                let blocked:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE application_name=$1 AND wait_event_type='Lock')").bind(&f.schema).fetch_one(&f.admin).await.unwrap();
                if blocked {break;}
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        }).await;
        change.commit().await.unwrap();
        let result = task.await.unwrap();
        assert!(
            waited.is_ok(),
            "worker must have waited for the actual {kind} row"
        );
        if kind == "product" {
            assert!(matches!(result, Err(PublicationError::NotFound)));
        } else {
            assert!(matches!(result, Err(PublicationError::Unauthorized)));
        }
        assert!(current(&f, receipt.site_id).await.is_none());
        f.finish().await;
    }
}

#[tokio::test]
async fn background_worker_resumes_committed_work_and_stops_on_shutdown() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Actual background worker").await;
    let (shutdown, signal) = tokio::sync::watch::channel(false);
    let task = tokio::spawn(crate::builder::publication_worker::run_publication_worker(
        f.pool.clone(),
        signal,
    ));
    let published = tokio::time::timeout(std::time::Duration::from_secs(3), async {
        loop {
            if current(&f, receipt.site_id).await == Some(receipt.publication_id) {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await;
    let _ = shutdown.send(true);
    let stopped = tokio::time::timeout(std::time::Duration::from_secs(2), task).await;
    f.finish().await;
    assert!(
        published.is_ok(),
        "durable work must be consumed without another owner request"
    );
    assert!(stopped.is_ok(), "the worker must observe shutdown");
    stopped.unwrap().unwrap();
}

#[tokio::test]
async fn stored_jsonb_normalization_preserves_snapshot_replay_and_worker_progress() {
    let f = Fixture::new(false).await;
    let mut snapshot = f.snapshot("Numeric metadata");
    snapshot.pages[0].seo_metadata = json!({"zero": -0.0, "nested": [1.0, -0.0, 1e30]});
    let operation = Uuid::new_v4();
    let receipt = submit_publication(&f.pool, &f.a, operation, None, &snapshot)
        .await
        .unwrap();
    let replay = submit_publication(&f.pool, &f.a, operation, None, &snapshot)
        .await
        .unwrap();
    assert_eq!(receipt.publication_id, replay.publication_id);
    let work = discover_publication_work(&f.pool, 64)
        .await
        .unwrap()
        .remove(0);
    let claim = claim_publication(&f.pool, &work).await;
    let result = match claim {
        Ok(Some(claim)) => finish_publication(&f.pool, &claim).await,
        Ok(None) => Err(PublicationError::NotFound),
        Err(error) => Err(error),
    };
    f.finish().await;
    assert!(
        result.is_ok(),
        "accepted JSONB snapshot must remain processable: {result:?}"
    );
}

#[tokio::test]
async fn invalid_stored_snapshots_are_quarantined_without_starving_later_work() {
    let f = Fixture::new(false).await;
    let valid = submit(&f, "Valid after invalid work").await;
    for i in 1..=64u128 {
        let id = Uuid::from_u128(i);
        assert!(id < valid.publication_id);
        let corrupt = match i % 4 {
            0 => json!({"pages": []}),
            1 => json!({"unknown": true}),
            _ => serde_json::to_value(f.snapshot("Incorrect digest or products")).unwrap(),
        };
        let digest = if i % 4 == 3 {
            prepare_snapshot(&f.snapshot("Incorrect digest or products"))
                .unwrap()
                .1
        } else {
            "0".repeat(64)
        };
        sqlx::query("INSERT INTO builder_publications(publication_id,tenant_id,owner_id,operation_id,site_id,site_version,snapshot,snapshot_sha256,product_ids) VALUES($1,$2,$3,$1,$4,$5,$6,$7,'{}')")
            .bind(id).bind(&f.a.tenant_id).bind(&f.a.user_id).bind(valid.site_id).bind(i as i64+1).bind(corrupt).bind(digest).execute(&f.admin).await.unwrap();
        sqlx::query("INSERT INTO builder_publication_work(publication_id,tenant_id) VALUES($1,$2)")
            .bind(id)
            .bind(&f.a.tenant_id)
            .execute(&f.admin)
            .await
            .unwrap();
    }
    let first = discover_publication_work(&f.pool, 64).await.unwrap();
    assert_eq!(first.len(), 64);
    for work in first {
        assert!(claim_publication(&f.pool, &work).await.is_err());
    }
    let remaining = discover_publication_work(&f.pool, 64).await.unwrap();
    let quarantined:i64=sqlx::query_scalar("SELECT COUNT(*) FROM builder_publications WHERE status='failed' AND error_code='stored_snapshot_invalid' AND rendered_pages IS NULL").fetch_one(&f.admin).await.unwrap();
    let preserved: String =
        sqlx::query_scalar("SELECT status FROM builder_publications WHERE publication_id=$1")
            .bind(valid.publication_id)
            .fetch_one(&f.admin)
            .await
            .unwrap();
    let progressed = if remaining.len() == 1 && remaining[0].publication_id == valid.publication_id
    {
        let claim = claim_publication(&f.pool, &remaining[0])
            .await
            .unwrap()
            .unwrap();
        finish_publication(&f.pool, &claim).await.unwrap();
        current(&f, valid.site_id).await == Some(valid.publication_id)
    } else {
        false
    };
    f.finish().await;
    assert!(
        progressed,
        "the next valid job must actually publish after quarantine"
    );
    assert_eq!(
        quarantined, 64,
        "permanent corruption must have visible terminal receipts"
    );
    assert_eq!(
        remaining.len(),
        1,
        "invalid first64 jobs must not starve valid work"
    );
    assert_eq!(remaining[0].publication_id, valid.publication_id);
    assert_eq!(preserved, "pending");
}

#[tokio::test]
async fn terminal_routing_cleanup_preserves_published_content_after_authority_retirement() {
    let f = Fixture::new(false).await;
    let receipt = submit(&f, "Published before retirement").await;
    let work = discover_publication_work(&f.pool, 64)
        .await
        .unwrap()
        .remove(0);
    let claim = claim_publication(&f.pool, &work).await.unwrap().unwrap();
    finish_publication(&f.pool, &claim).await.unwrap();
    let before: (serde_json::Value, String) = sqlx::query_as(
        "SELECT rendered_pages,rendered_sha256 FROM builder_publications WHERE publication_id=$1",
    )
    .bind(receipt.publication_id)
    .fetch_one(&f.admin)
    .await
    .unwrap();
    sqlx::query("INSERT INTO builder_publication_work(publication_id,tenant_id) VALUES($1,$2)")
        .bind(receipt.publication_id)
        .bind(&f.a.tenant_id)
        .execute(&f.admin)
        .await
        .unwrap();
    sqlx::query("DELETE FROM identity_user_roles WHERE user_id=$1")
        .bind(&f.a.user_id)
        .execute(&f.admin)
        .await
        .unwrap();
    let result = claim_publication(&f.pool, &work).await;
    let remaining = discover_publication_work(&f.pool, 64).await.unwrap();
    let after: (serde_json::Value, String) = sqlx::query_as(
        "SELECT rendered_pages,rendered_sha256 FROM builder_publications WHERE publication_id=$1",
    )
    .bind(receipt.publication_id)
    .fetch_one(&f.admin)
    .await
    .unwrap();
    f.finish().await;
    assert!(matches!(result, Ok(None)));
    assert!(remaining.is_empty());
    assert_eq!(before, after);
}

#[tokio::test]
async fn non_ascii_role_lookalikes_do_not_grant_publication_authority() {
    let f = Fixture::new(false).await;
    sqlx::query("UPDATE identity_user_roles SET role_name='ADMİN' WHERE user_id=$1")
        .bind(&f.a.user_id)
        .execute(&f.admin)
        .await
        .unwrap();
    let folded: String = sqlx::query_scalar("SELECT lower('ADMİN')")
        .fetch_one(&f.admin)
        .await
        .unwrap();
    let result = submit_publication(
        &f.pool,
        &f.a,
        Uuid::new_v4(),
        None,
        &f.snapshot("Not authorized"),
    )
    .await;
    f.finish().await;
    assert!(
        matches!(result, Err(PublicationError::Unauthorized)),
        "canonical ASCII roles only; database fold={folded}, result={result:?}"
    );
}
