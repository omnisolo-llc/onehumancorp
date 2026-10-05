//! Encoding compatibility of the real PostgreSQL queue; no alternate queue SQL.
use crate::queue::{Job, PostgresTaskQueue, TaskQueue};
use chrono::{DateTime, Utc};
use futures_util::FutureExt;
use serde_json::{Value, json};
use sqlx::Row;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Enqueue,
    Batch,
    Requeue,
    Dequeue,
}

fn job(tenant: &str) -> Job {
    let now = Utc::now();
    Job {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: tenant.into(),
        parent_task_id: "owned-encoding-parent".into(),
        job_type: "encoding-reviewer".into(),
        payload: json!({"nested":{"unicode":"é 雪 🦀","items":[1,"two",true,null],"decimal":1.25},
            "agent_role":"replaced-on-write","attempts":99,"max_attempts":99})
        .to_string(),
        status: "PENDING".into(),
        retry_count: 2,
        max_retries: 7,
        next_retry_at: now - chrono::Duration::seconds(5),
        locked_until: None,
        created_at: now,
        updated_at: now,
    }
}
fn payload(job: &Job, replace_role: bool) -> Value {
    let mut value: Value = serde_json::from_str(&job.payload).unwrap();
    if replace_role {
        value["agent_role"] = job.job_type.clone().into();
    }
    value["attempts"] = job.retry_count.into();
    value["max_attempts"] = job.max_retries.into();
    value
}
async fn column_type(pool: &sqlx::PgPool) -> String {
    sqlx::query_scalar("SELECT format_type(atttypid,atttypmod) FROM pg_attribute WHERE attrelid='sub_agent_queue'::regclass AND attname='payload'")
        .fetch_one(pool).await.unwrap()
}
async fn row(pool: &sqlx::PgPool, id: &str) -> (String, String, Value, String, DateTime<Utc>) {
    let row = sqlx::query("SELECT tenant_id,parent_task_id,payload::text AS payload,status,scheduled_at FROM sub_agent_queue WHERE id=$1")
        .bind(id).fetch_one(pool).await.unwrap();
    (
        row.get("tenant_id"),
        row.get("parent_task_id"),
        serde_json::from_str(&row.get::<String, _>("payload")).unwrap(),
        row.get("status"),
        row.get("scheduled_at"),
    )
}
async fn assert_row(pool: &sqlx::PgPool, job: &Job, expected: Value, status: &str) {
    let actual = row(pool, &job.id).await;
    assert_eq!(actual.0, job.tenant_id);
    assert_eq!(actual.1, job.parent_task_id);
    assert_eq!(actual.2, expected);
    if column_type(pool).await == "text" {
        let stored: String = sqlx::query_scalar("SELECT payload FROM sub_agent_queue WHERE id=$1")
            .bind(&job.id)
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(
            stored,
            expected.to_string(),
            "legacy TEXT fixture serialization changed"
        );
    }
    assert_eq!(actual.3, status);
    assert_eq!(
        actual.4.timestamp_micros(),
        job.next_retry_at.timestamp_micros()
    );
}
async fn seed(pool: &sqlx::PgPool, job: &Job) {
    // Independently seed a correctly typed row so a broken write cannot hide
    // the actual dequeue codec mismatch. Production schema and queue are intact.
    sqlx::query("INSERT INTO sub_agent_queue(id,tenant_id,parent_task_id,payload,status,scheduled_at) VALUES($1,$2,$3,$4::json,'QUEUED',$5)")
        .bind(&job.id).bind(&job.tenant_id).bind(&job.parent_task_id)
        .bind(payload(job, true).to_string()).bind(job.next_retry_at)
        .execute(pool).await.unwrap();
}

async fn exercise(legacy_text: bool, operation: Operation, escaped_nul: bool) {
    let fixture = super::Fixture::open().await;
    sqlx::raw_sql(include_str!(
        "../../src/server/migrations/104_sub_agent_queue.sql"
    ))
    .execute(&fixture.admin)
    .await
    .unwrap();
    assert_eq!(column_type(&fixture.admin).await, "jsonb");
    if legacy_text {
        // A separately labeled legacy fixture models the existing TEXT queue
        // schema. The JSONB cases always keep migration 104's actual type.
        sqlx::query("ALTER TABLE sub_agent_queue ALTER COLUMN payload DROP DEFAULT")
            .execute(&fixture.admin)
            .await
            .unwrap();
        sqlx::query(
            "ALTER TABLE sub_agent_queue ALTER COLUMN payload TYPE TEXT USING payload::text",
        )
        .execute(&fixture.admin)
        .await
        .unwrap();
    }
    let expected_type = if legacy_text { "text" } else { "jsonb" };
    assert_eq!(column_type(&fixture.admin).await, expected_type);
    let queue = PostgresTaskQueue::new(fixture.admin.clone());
    let mut original = job("receipt-pg-a");
    if escaped_nul {
        let mut value = payload(&original, true);
        value["nested"]["unicode"] = "escaped\0NUL é 雪".into();
        original.payload = value.to_string();
    }
    let result = std::panic::AssertUnwindSafe(async {
        match operation {
            Operation::Enqueue => {
                queue
                    .enqueue(original.clone())
                    .await
                    .expect("actual enqueue must accept the installed column type");
                assert_row(
                    &fixture.admin,
                    &original,
                    payload(&original, true),
                    "QUEUED",
                )
                .await;
            }
            Operation::Batch => {
                let other = job("receipt-pg-b");
                queue
                    .enqueue_batch(vec![original.clone(), other.clone()])
                    .await
                    .expect("actual batch must accept the installed column type");
                assert_row(
                    &fixture.admin,
                    &original,
                    payload(&original, true),
                    "QUEUED",
                )
                .await;
                assert_row(&fixture.admin, &other, payload(&other, true), "QUEUED").await;
                assert_eq!(
                    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM sub_agent_queue")
                        .fetch_one(&fixture.admin)
                        .await
                        .unwrap(),
                    2
                );
            }
            Operation::Requeue => {
                seed(&fixture.admin, &original).await;
                let before = row(&fixture.admin, &original.id).await;
                let mut changed = original.clone();
                changed.tenant_id = "receipt-pg-b".into();
                changed.retry_count = 4;
                changed.max_retries = 9;
                changed.next_retry_at = Utc::now() - chrono::Duration::seconds(1);
                let mut updated = payload(&changed, true);
                updated["nested"]["unicode"] = if escaped_nul {
                    "updated λ\0 🚀"
                } else {
                    "updated λ 🚀"
                }
                .into();
                changed.payload = updated.to_string();
                queue
                    .requeue(changed.clone())
                    .await
                    .expect("actual requeue must accept the installed column type");
                assert_eq!(
                    row(&fixture.admin, &original.id).await,
                    before,
                    "foreign tenant requeue changed the original row"
                );
                changed.tenant_id = original.tenant_id.clone();
                queue.requeue(changed.clone()).await.unwrap();
                assert_row(&fixture.admin, &changed, payload(&changed, false), "QUEUED").await;
            }
            Operation::Dequeue => {
                seed(&fixture.admin, &original).await;
                let mut other = job("receipt-pg-b");
                other.job_type = "unrequested-role".into();
                seed(&fixture.admin, &other).await;
                let received = queue.dequeue(vec![original.job_type.clone()]).await;
                if escaped_nul {
                    // The existing JSON role selector rejects escaped NUL even
                    // for TEXT storage. Preserve that boundary, without claiming
                    // or implementing a new parser/lifecycle capability.
                    let error = received.expect_err("existing role parser rejects escaped NUL");
                    assert!(error.contains("unsupported Unicode escape sequence"));
                    assert_row(
                        &fixture.admin,
                        &original,
                        payload(&original, true),
                        "QUEUED",
                    )
                    .await;
                    assert_row(&fixture.admin, &other, payload(&other, true), "QUEUED").await;
                    return;
                }
                let received = received.unwrap().expect("seeded job is due");
                assert_eq!(received.id, original.id);
                assert_eq!(received.tenant_id, original.tenant_id);
                assert_eq!(received.parent_task_id, original.parent_task_id);
                assert_eq!(received.job_type, original.job_type);
                assert_eq!(received.retry_count, original.retry_count + 1);
                assert_eq!(received.max_retries, original.max_retries);
                assert_eq!(received.status, "RUNNING");
                assert_eq!(
                    received.next_retry_at.timestamp_micros(),
                    original.next_retry_at.timestamp_micros()
                );
                assert_eq!(
                    serde_json::from_str::<Value>(&received.payload).unwrap(),
                    payload(&original, true)
                );
                assert_row(
                    &fixture.admin,
                    &original,
                    payload(&original, true),
                    "RUNNING",
                )
                .await;
                assert_row(&fixture.admin, &other, payload(&other, true), "QUEUED").await;
                assert!(
                    queue
                        .dequeue(vec![original.job_type.clone()])
                        .await
                        .unwrap()
                        .is_none()
                );
            }
        }
    })
    .catch_unwind()
    .await;
    // Assert no runtime migration, then clean the uniquely owned schema/role
    // even when the production operation panics on the old JSONB decoder.
    assert_eq!(column_type(&fixture.admin).await, expected_type);
    fixture.close().await;
    if let Err(panic) = result {
        std::panic::resume_unwind(panic);
    }
}

#[tokio::test]
async fn jsonb_enqueue_preserves_structured_payload_and_identity() {
    exercise(false, Operation::Enqueue, false).await;
}
#[tokio::test]
async fn jsonb_batch_preserves_each_tenant_and_structured_payload() {
    exercise(false, Operation::Batch, false).await;
}
#[tokio::test]
async fn jsonb_requeue_preserves_tenant_predicate_and_retry_fields() {
    exercise(false, Operation::Requeue, false).await;
}
#[tokio::test]
async fn jsonb_dequeue_reads_an_independently_seeded_row_and_increments_retry() {
    exercise(false, Operation::Dequeue, false).await;
}
#[tokio::test]
async fn legacy_text_enqueue_preserves_structured_payload_and_identity() {
    exercise(true, Operation::Enqueue, false).await;
}
#[tokio::test]
async fn legacy_text_batch_preserves_each_tenant_and_structured_payload() {
    exercise(true, Operation::Batch, false).await;
}
#[tokio::test]
async fn legacy_text_requeue_preserves_tenant_predicate_and_retry_fields() {
    exercise(true, Operation::Requeue, false).await;
}
#[tokio::test]
async fn legacy_text_dequeue_reads_an_independently_seeded_row_and_increments_retry() {
    exercise(true, Operation::Dequeue, false).await;
}

#[tokio::test]
async fn legacy_text_enqueue_keeps_valid_json_with_escaped_nul() {
    exercise(true, Operation::Enqueue, true).await;
}
#[tokio::test]
async fn legacy_text_batch_keeps_valid_json_with_escaped_nul() {
    exercise(true, Operation::Batch, true).await;
}
#[tokio::test]
async fn legacy_text_requeue_keeps_valid_json_with_escaped_nul() {
    exercise(true, Operation::Requeue, true).await;
}
#[tokio::test]
async fn legacy_text_dequeue_retains_existing_escaped_nul_role_parser_rejection() {
    exercise(true, Operation::Dequeue, true).await;
}

#[tokio::test]
async fn jsonb_rejects_escaped_nul_without_committing_part_of_the_batch() {
    let fixture = super::Fixture::open().await;
    sqlx::raw_sql(include_str!(
        "../../src/server/migrations/104_sub_agent_queue.sql"
    ))
    .execute(&fixture.admin)
    .await
    .unwrap();
    let queue = PostgresTaskQueue::new(fixture.admin.clone());
    let ordinary = job("receipt-pg-a");
    let mut invalid_for_jsonb = job("receipt-pg-b");
    let mut value = payload(&invalid_for_jsonb, true);
    value["nested"]["unicode"] = "escaped\0NUL".into();
    invalid_for_jsonb.payload = value.to_string();
    let rejected = queue.enqueue_batch(vec![ordinary, invalid_for_jsonb]).await;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sub_agent_queue")
        .fetch_one(&fixture.admin)
        .await
        .unwrap();
    let data_type = column_type(&fixture.admin).await;
    fixture.close().await;
    let error = rejected.expect_err("JSONB must keep its existing escaped-NUL restriction");
    assert!(
        error.contains("unsupported Unicode escape sequence"),
        "{error}"
    );
    assert_eq!(count, 0);
    assert_eq!(data_type, "jsonb");
}
