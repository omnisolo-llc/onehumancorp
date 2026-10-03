use serde_json::json;
use sha2::{Digest, Sha256};

const RECORDS_PER_TENANT: i64 = 100;

// An explicit positive reorder threshold is the stored business rule. Missing
// quantities/rules are unknown, not evidence for an operational alert.
const STOCK_POSTGRES: &str = r#"
SELECT m.id, COALESCE(m.name, '') AS name,
       CAST(m.current_quantity AS BIGINT) AS quantity,
       CAST(m.reorder_threshold AS BIGINT) AS threshold,
       COALESCE(CAST(EXTRACT(EPOCH FROM COALESCE(m.updated_at,m.created_at)) AS TEXT),'') AS version
FROM raw_materials m
WHERE m.tenant_id=$1 AND m.reorder_threshold>0 AND COALESCE(m.updated_at,m.created_at) IS NOT NULL AND m.current_quantity<m.reorder_threshold
AND NOT EXISTS (
  SELECT 1 FROM agent_feed_items f WHERE f.tenant_id=$1 AND f.event_source='operations'
  AND f.context_payload->>'source_table'='raw_materials'
  AND f.context_payload->>'source_id'=m.id
  AND f.context_payload->>'source_version'=COALESCE(CAST(EXTRACT(EPOCH FROM COALESCE(m.updated_at,m.created_at)) AS TEXT),'')
  AND f.context_payload->>'quantity'=CAST(m.current_quantity AS TEXT)
  AND f.context_payload->>'threshold'=CAST(m.reorder_threshold AS TEXT)
)
ORDER BY m.id LIMIT $2
"#;
const STOCK_SQLITE: &str = r#"
SELECT m.id, COALESCE(m.name, '') AS name,
       CAST(m.current_quantity AS BIGINT) AS quantity,
       CAST(m.reorder_threshold AS BIGINT) AS threshold,
       COALESCE(CAST(m.updated_at AS TEXT), CAST(m.created_at AS TEXT), '') AS version
FROM raw_materials m
WHERE m.tenant_id=?1 AND m.reorder_threshold>0 AND COALESCE(m.updated_at,m.created_at) IS NOT NULL AND m.current_quantity<m.reorder_threshold
AND NOT EXISTS (
  SELECT 1 FROM agent_feed_items f WHERE f.tenant_id=?1 AND f.event_source='operations'
  AND json_extract(f.context_payload,'$.source_table')='raw_materials'
  AND json_extract(f.context_payload,'$.source_id')=m.id
  AND json_extract(f.context_payload,'$.source_version')=COALESCE(CAST(m.updated_at AS TEXT), CAST(m.created_at AS TEXT), '')
  AND json_extract(f.context_payload,'$.quantity')=m.current_quantity
  AND json_extract(f.context_payload,'$.threshold')=m.reorder_threshold
)
ORDER BY m.id LIMIT ?2
"#;
const RETIRE_POSTGRES: &str = r#"
UPDATE agent_feed_items SET lifecycle_state='PAUSED', updated_at=CURRENT_TIMESTAMP,
 context_payload=jsonb_set(context_payload,'{observation_status}','"source_changed"'::jsonb)
WHERE id IN (
 SELECT f.id FROM agent_feed_items f
 WHERE f.tenant_id=$1 AND f.event_source='operations' AND f.lifecycle_state='PENDING_APPROVAL'
 AND f.context_payload->>'source_table'='raw_materials'
 AND NOT EXISTS (
  SELECT 1 FROM raw_materials m WHERE m.tenant_id=$1 AND m.id=f.context_payload->>'source_id'
  AND m.reorder_threshold>0 AND COALESCE(m.updated_at,m.created_at) IS NOT NULL AND m.current_quantity<m.reorder_threshold
  AND f.context_payload->>'source_version'=COALESCE(CAST(EXTRACT(EPOCH FROM COALESCE(m.updated_at,m.created_at)) AS TEXT),'')
  AND f.context_payload->>'quantity'=CAST(m.current_quantity AS TEXT)
  AND f.context_payload->>'threshold'=CAST(m.reorder_threshold AS TEXT)
 ) ORDER BY f.id LIMIT $2
)
"#;
const RETIRE_SQLITE: &str = r#"
UPDATE agent_feed_items SET lifecycle_state='PAUSED', updated_at=CURRENT_TIMESTAMP,
 context_payload=json_set(context_payload,'$.observation_status','source_changed')
WHERE id IN (
 SELECT f.id FROM agent_feed_items f
 WHERE f.tenant_id=?1 AND f.event_source='operations' AND f.lifecycle_state='PENDING_APPROVAL'
 AND json_extract(f.context_payload,'$.source_table')='raw_materials'
 AND NOT EXISTS (
  SELECT 1 FROM raw_materials m WHERE m.tenant_id=?1 AND m.id=json_extract(f.context_payload,'$.source_id')
  AND m.reorder_threshold>0 AND COALESCE(m.updated_at,m.created_at) IS NOT NULL AND m.current_quantity<m.reorder_threshold
  AND json_extract(f.context_payload,'$.source_version')=COALESCE(CAST(m.updated_at AS TEXT),CAST(m.created_at AS TEXT),'')
  AND json_extract(f.context_payload,'$.quantity')=m.current_quantity
  AND json_extract(f.context_payload,'$.threshold')=m.reorder_threshold
 ) ORDER BY f.id LIMIT ?2
)
"#;
const INSERT_POSTGRES: &str = r#"
INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state)
SELECT $1,$2,'operations',$3::jsonb,$4::jsonb,'PENDING_APPROVAL'
FROM raw_materials m WHERE m.id=$5 AND m.tenant_id=$2
 AND m.current_quantity=$6 AND m.reorder_threshold=$7
 AND COALESCE(CAST(EXTRACT(EPOCH FROM COALESCE(m.updated_at,m.created_at)) AS TEXT),'')=$8
 AND m.reorder_threshold>0 AND COALESCE(m.updated_at,m.created_at) IS NOT NULL AND m.current_quantity<m.reorder_threshold
ON CONFLICT(id) DO NOTHING
"#;
const INSERT_SQLITE: &str = r#"
INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state)
SELECT ?1,?2,'operations',?3,?4,'PENDING_APPROVAL'
FROM raw_materials m WHERE m.id=?5 AND m.tenant_id=?2
 AND m.current_quantity=?6 AND m.reorder_threshold=?7
 AND COALESCE(CAST(m.updated_at AS TEXT),CAST(m.created_at AS TEXT),'')=?8
 AND m.reorder_threshold>0 AND COALESCE(m.updated_at,m.created_at) IS NOT NULL AND m.current_quantity<m.reorder_threshold
ON CONFLICT(id) DO NOTHING
"#;

#[derive(sqlx::FromRow)]
struct StockObservation {
    id: String,
    name: String,
    quantity: i64,
    threshold: i64,
    version: String,
}
impl StockObservation {
    fn evidence(&self, tenant: &str) -> (String, String, String) {
        let identity = json!([tenant, self.id, self.version, self.quantity, self.threshold]);
        let key = format!("ops-stock-v1-{:x}", Sha256::digest(identity.to_string()));
        let name = if self.name.trim().is_empty() {
            &self.id
        } else {
            &self.name
        };
        let context = json!({
            "feature_type": "proactive_ops", "source_table": "raw_materials", "source_id": self.id,
            "source_version": self.version, "quantity": self.quantity, "threshold": self.threshold,
            "observation_status": "recorded",
            "description": format!("Inventory record for {name} shows {} units, below its configured reorder threshold of {}.", self.quantity, self.threshold)
        });
        let action = json!({"feature_type":"proactive_ops", "action_type":"acknowledge_recorded_low_stock", "message":"Acknowledge stock alert"});
        (key, context.to_string(), action.to_string())
    }
}

#[derive(Default, Debug, PartialEq)]
pub(crate) struct ScanCounts {
    pub(crate) created: u64,
    pub(crate) retired: u64,
}

pub(crate) async fn scan_postgres(
    connection: &mut sqlx::PgConnection,
    tenant: &str,
) -> Result<ScanCounts, sqlx::Error> {
    let retired = sqlx::query(RETIRE_POSTGRES)
        .bind(tenant)
        .bind(RECORDS_PER_TENANT)
        .execute(&mut *connection)
        .await?
        .rows_affected();
    let records = sqlx::query_as::<_, StockObservation>(STOCK_POSTGRES)
        .bind(tenant)
        .bind(RECORDS_PER_TENANT)
        .fetch_all(&mut *connection)
        .await?;
    let mut created = 0;
    for record in records {
        let (key, context, action) = record.evidence(tenant);
        created += sqlx::query(INSERT_POSTGRES)
            .bind(key)
            .bind(tenant)
            .bind(context)
            .bind(action)
            .bind(&record.id)
            .bind(record.quantity)
            .bind(record.threshold)
            .bind(&record.version)
            .execute(&mut *connection)
            .await?
            .rows_affected();
    }

    Ok(ScanCounts { created, retired })
}

pub(crate) async fn scan_sqlite(
    pool: &sqlx::SqlitePool,
    tenant: &str,
) -> Result<ScanCounts, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let retired = sqlx::query(RETIRE_SQLITE)
        .bind(tenant)
        .bind(RECORDS_PER_TENANT)
        .execute(&mut *tx)
        .await?
        .rows_affected();
    let records = sqlx::query_as::<_, StockObservation>(STOCK_SQLITE)
        .bind(tenant)
        .bind(RECORDS_PER_TENANT)
        .fetch_all(&mut *tx)
        .await?;
    let mut created = 0;
    for record in records {
        let (key, context, action) = record.evidence(tenant);
        created += sqlx::query(INSERT_SQLITE)
            .bind(key)
            .bind(tenant)
            .bind(context)
            .bind(action)
            .bind(&record.id)
            .bind(record.quantity)
            .bind(record.threshold)
            .bind(&record.version)
            .execute(&mut *tx)
            .await?
            .rows_affected();
    }
    tx.commit().await?;
    Ok(ScanCounts { created, retired })
}

#[cfg(test)]
mod tests {
    use super::*;
    async fn fixture() -> sqlx::SqlitePool {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        for sql in [
            "CREATE TABLE tenants(id TEXT PRIMARY KEY)",
            "CREATE TABLE raw_materials(id TEXT PRIMARY KEY,tenant_id TEXT,name TEXT,current_quantity INTEGER,reorder_threshold INTEGER,created_at TEXT,updated_at TEXT)",
            "CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT,event_source TEXT,context_payload TEXT,proposed_action TEXT,lifecycle_state TEXT,created_at TEXT DEFAULT CURRENT_TIMESTAMP,updated_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "INSERT INTO tenants VALUES('a'),('b')",
        ] {
            sqlx::query(sql).execute(&pool).await.unwrap();
        }
        pool
    }
    async fn add(pool: &sqlx::SqlitePool, id: &str, tenant: &str, quantity: i64, threshold: i64) {
        sqlx::query("INSERT INTO raw_materials VALUES(?1,?2,'Flour',?3,?4,'v1','v1')")
            .bind(id)
            .bind(tenant)
            .bind(quantity)
            .bind(threshold)
            .execute(pool)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn empty_or_unconfigured_tenants_never_receive_invented_work() {
        let pool = fixture().await;
        add(&pool, "healthy", "a", 10, 5).await;
        add(&pool, "no-rule", "a", 0, 0).await;
        sqlx::query(
            "INSERT INTO raw_materials VALUES('missing-revision','a','Flour',1,5,NULL,NULL)",
        )
        .execute(&pool)
        .await
        .unwrap();
        assert_eq!(
            scan_sqlite(&pool, "a").await.unwrap(),
            ScanCounts::default()
        );
        assert_eq!(
            scan_sqlite(&pool, "b").await.unwrap(),
            ScanCounts::default()
        );
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_feed_items")
                .fetch_one(&pool)
                .await
                .unwrap(),
            0
        );
    }
    #[tokio::test]
    async fn persisted_threshold_fact_is_tenant_scoped_and_deduped_across_workers_and_restart() {
        let pool = fixture().await;
        add(&pool, "material-a", "a", 2, 5).await;
        add(&pool, "material-b", "b", 1, 9).await;
        let (a, b) = tokio::join!(scan_sqlite(&pool, "a"), scan_sqlite(&pool, "a"));
        assert_eq!(a.unwrap().created + b.unwrap().created, 1);
        assert_eq!(scan_sqlite(&pool.clone(), "a").await.unwrap().created, 0);
        let rows: Vec<(String, String)> =
            sqlx::query_as("SELECT tenant_id,context_payload FROM agent_feed_items")
                .fetch_all(&pool)
                .await
                .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, "a");
        let evidence: serde_json::Value = serde_json::from_str(&rows[0].1).unwrap();
        assert_eq!(evidence["source_id"], "material-a");
        assert_eq!(evidence["quantity"], 2);
        assert_eq!(evidence["threshold"], 5);
        assert_eq!(evidence["source_version"], "v1");
    }
    #[tokio::test]
    async fn changed_facts_retire_old_pending_alerts_without_overwriting_decisions() {
        let pool = fixture().await;
        add(&pool, "material-a", "a", 2, 5).await;
        assert_eq!(scan_sqlite(&pool, "a").await.unwrap().created, 1);
        sqlx::query("UPDATE raw_materials SET current_quantity=8,updated_at='v2'")
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(
            scan_sqlite(&pool, "a").await.unwrap(),
            ScanCounts {
                created: 0,
                retired: 1
            }
        );
        sqlx::query("UPDATE raw_materials SET current_quantity=2,updated_at='v3'")
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(scan_sqlite(&pool, "a").await.unwrap().created, 1);
        sqlx::query("UPDATE agent_feed_items SET lifecycle_state='APPROVED' WHERE lifecycle_state='PENDING_APPROVAL'").execute(&pool).await.unwrap();
        assert_eq!(
            scan_sqlite(&pool, "a").await.unwrap(),
            ScanCounts::default()
        );
        let states: Vec<String> = sqlx::query_scalar(
            "SELECT lifecycle_state FROM agent_feed_items ORDER BY lifecycle_state",
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(states, vec!["APPROVED", "PAUSED"]);
    }
    #[tokio::test]
    async fn bounded_batches_progress_past_already_recorded_materials() {
        let pool = fixture().await;
        for n in 0..101 {
            add(&pool, &format!("material-{n:03}"), "a", 1, 5).await;
        }
        assert_eq!(scan_sqlite(&pool, "a").await.unwrap().created, 100);
        assert_eq!(scan_sqlite(&pool, "a").await.unwrap().created, 1);
        assert_eq!(scan_sqlite(&pool, "a").await.unwrap().created, 0);
    }
    #[tokio::test]
    async fn unreadable_operational_data_is_an_error_not_an_empty_or_fabricated_result() {
        let pool = fixture().await;
        sqlx::query("DROP TABLE raw_materials")
            .execute(&pool)
            .await
            .unwrap();
        assert!(scan_sqlite(&pool.clone(), "a").await.is_err());
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_feed_items")
                .fetch_one(&pool)
                .await
                .unwrap(),
            0
        );
    }
}
