#[cfg(test)]
mod cuj_e2e_tests {
    use crate::orchestration::departments::memory::layer;
    use crate::orchestration::departments::memory::conflict;
    use crate::orchestration::departments::memory::pruning;
    use crate::workers::memory::MemoryConsolidationWorker;
    use ohc_builtin_agent::memory_store::{VectorRepository, EmbeddingRecord};
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use std::sync::Arc;
    use sqlx::Row;
    use chrono::Utc;

    #[tokio::test]
    async fn test_maya_baker_shop_cuj_e2e() {
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:").unwrap();
        let pool = match SqlitePoolOptions::new().connect_with(conn_opts).await {
            Ok(p) => p,
            Err(_) => return,
        };

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS consolidated_memory (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                agent_id TEXT,
                content TEXT NOT NULL,
                embedding VECTOR(1536),
                source_type TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                last_referenced_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                reference_count INTEGER DEFAULT 0,
                reliability_score INTEGER DEFAULT 50,
                owner_override BOOLEAN DEFAULT FALSE,
                metadata TEXT
            );"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = Arc::new(VectorRepository::new_sqlite(pool.clone()));

        // 1. Cross-Department Context Sharing
        // Maya's baker shop customer success notes
        let cs_record = EmbeddingRecord {
            id: "cs_maya_1".to_string(),
            tenant_id: "maya_baker".to_string(),
            agent_id: "cs_agent".to_string(),
            content: "Customer John loves vegan cakes but hates delays.".to_string(),
            embedding: { let mut v = vec![0.0; 1536]; v[2] = 1.0; v },
            source_type: "CS_TICKET".to_string(),
            created_at: Utc::now() - chrono::Duration::days(1),
            last_referenced_at: Utc::now(),
            reference_count: 2,
            reliability_score: 80,
            owner_override: false,
            metadata: None,
        };

        // Operations notes
        let ops_record = EmbeddingRecord {
            id: "ops_maya_1".to_string(),
            tenant_id: "maya_baker".to_string(),
            agent_id: "ops_agent".to_string(),
            content: "Vegan cakes take 2 days to produce.".to_string(),
            embedding: { let mut v = vec![0.0; 1536]; v[0] = 1.0; v },
            source_type: "OPS_REPORT".to_string(),
            created_at: Utc::now() - chrono::Duration::days(1),
            last_referenced_at: Utc::now(),
            reference_count: 1,
            reliability_score: 75,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&cs_record).await.unwrap();
        repo.upsert(&ops_record).await.unwrap();

        // Check they both exist
        let rows = sqlx::query("SELECT id FROM consolidated_memory WHERE tenant_id = 'maya_baker' AND (id = 'cs_maya_1' OR id = 'ops_maya_1')")
            .fetch_all(&pool)
            .await
            .unwrap();
        assert_eq!(rows.len(), 2, "Cross department context should be fully accessible");

        // 2. Conflict Resolution
        // Marketing thinks price is $50
        let mkt_record = EmbeddingRecord {
            id: "mkt_price_1".to_string(),
            tenant_id: "maya_baker".to_string(),
            agent_id: "mkt_agent".to_string(),
            content: "Vegan cake price is $50".to_string(),
            embedding: vec![0.1; 1536],
            source_type: "NOTES".to_string(),
            created_at: Utc::now() - chrono::Duration::days(5),
            last_referenced_at: Utc::now() - chrono::Duration::days(2),
            reference_count: 1,
            reliability_score: 60,
            owner_override: false,
            metadata: None,
        };

        // Owner override price is $55
        let owner_record = EmbeddingRecord {
            id: "owner_price_1".to_string(),
            tenant_id: "maya_baker".to_string(),
            agent_id: "owner".to_string(),
            content: "Vegan cake price is $55".to_string(),
            embedding: vec![0.1; 1536], // Exact same embedding triggers conflict
            source_type: "NOTES".to_string(),
            created_at: Utc::now() - chrono::Duration::days(2),
            last_referenced_at: Utc::now(),
            reference_count: 1,
            reliability_score: 95,
            owner_override: true, // WINNER
            metadata: None,
        };

        repo.upsert(&mkt_record).await.unwrap();
        repo.upsert(&owner_record).await.unwrap();

        // 3. Stale Context Pruning
        // Discontinued product 6 months ago
        let stale_record = EmbeddingRecord {
            id: "stale_maya_1".to_string(),
            tenant_id: "maya_baker".to_string(),
            agent_id: "mkt_agent".to_string(),
            content: "Discontinued gluten-free muffins 6 months ago.".to_string(),
            embedding: { let mut v = vec![0.0; 1536]; v[1] = 1.0; v },
            source_type: "TASK_SUMMARY".to_string(),
            created_at: Utc::now() - chrono::Duration::days(200),
            last_referenced_at: Utc::now() - chrono::Duration::days(190), // > 180 days ago
            reference_count: 2, // < 5
            reliability_score: 50,
            owner_override: false,
            metadata: None,
        };

        repo.upsert(&stale_record).await.unwrap();

        // Verify pre-worker state
        let count_pre: (i64,) = sqlx::query_as("SELECT count(*) FROM consolidated_memory")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count_pre.0, 5, "Should have 5 records before consolidation");

        // Run the worker
        let mut worker = MemoryConsolidationWorker::new(repo.clone());
        worker.poll_interval = std::time::Duration::from_millis(5);
        worker.start();

        // Give it time to process
        tokio::time::sleep(std::time::Duration::from_millis(150)).await;

        // Verify post-worker state
        let rows = sqlx::query("SELECT id, reference_count FROM consolidated_memory WHERE tenant_id = 'maya_baker' ORDER BY id ASC")
            .fetch_all(&pool)
            .await
            .unwrap();

        // Should have deleted mkt_price_1 and stale_maya_1
        assert_eq!(rows.len(), 3, "Should have exactly 3 records after pruning and conflict resolution");

        let mut remaining_ids: Vec<String> = vec![];
        let mut owner_price_ref_count = 0;

        for row in rows {
            let id: String = row.try_get("id").unwrap();
            let ref_count: i32 = row.try_get("reference_count").unwrap();
            remaining_ids.push(id.clone());

            if id == "owner_price_1" {
                owner_price_ref_count = ref_count;
            }
        }

        remaining_ids.sort();

        assert_eq!(remaining_ids, vec!["cs_maya_1", "ops_maya_1", "owner_price_1"]);
        // mkt_price_1 had ref_count 1, owner_price_1 had 1.
        // winner reference_count += loser reference_count (1) + 1 = 3.
        assert_eq!(owner_price_ref_count, 3, "Winner should have inherited reference counts properly");
    }
}
