#[cfg(test)]
mod tests {
    use crate::db::ScoutDb;
    use crate::agent::ScoutAgent;
    use ohc_builtin_agent_core::pubsub::SubagentBus;
    use sqlx::sqlite::SqlitePoolOptions;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_scout_agent_flow() {
        // Use SQLite in-memory for testing
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create pool");

        // Setup schema
        sqlx::query(r#"
            CREATE TABLE tool_integrations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                api_url TEXT,
                integration_code TEXT,
                status TEXT NOT NULL,
                created_at DATETIME NOT NULL
            );
        "#)
        .execute(&pool)
        .await
        .expect("Failed to create table");

        let db = ScoutDb::new_sqlite(pool);
        let bus = Arc::new(SubagentBus::new());
        let mut rx = bus.subscribe();

        let agent = ScoutAgent::new(db.clone(), bus.clone());

        let id = agent.process_tool_request(
            "tenant-123",
            "TestTool",
            Some("A test tool"),
            Some("https://api.testtool.com")
        ).await.expect("Failed to process tool request");

        // Verify database
        let integration = db.get_integration(&id.to_string(), Some("tenant-123")).await.expect("Failed to get integration")
            .expect("Integration not found");

        assert_eq!(integration.name, "TestTool");
        assert_eq!(integration.tenant_id, "tenant-123");
        assert!(integration.integration_code.unwrap().contains("TestToolClient"));

        // Verify pubsub event
        let evt = rx.recv().await.expect("Failed to receive event");
        assert_eq!(evt.task_id, id.to_string());
    }

    #[tokio::test]
    async fn test_scout_parse_openapi() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create pool");

        sqlx::query(r#"
            CREATE TABLE tool_integrations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                api_url TEXT,
                integration_code TEXT,
                status TEXT NOT NULL,
                created_at DATETIME NOT NULL
            );
        "#)
        .execute(&pool)
        .await
        .expect("Failed to create table");

        let db = ScoutDb::new_sqlite(pool);
        let bus = Arc::new(SubagentBus::new());

        let agent = ScoutAgent::new(db.clone(), bus.clone());

        let id = agent.process_tool_request(
            "tenant-456",
            "DummyAPI",
            Some("Dummy API for testing"),
            Some("https://api.dummy.com/openapi.json")
        ).await.expect("Failed to process tool request");

        let integration = db.get_integration(&id.to_string(), Some("tenant-456")).await.expect("Failed to get integration")
            .expect("Integration not found");

        assert_eq!(integration.name, "DummyAPI");
        assert_eq!(integration.tenant_id, "tenant-456");
        assert!(integration.integration_code.unwrap().contains("DummyAPIClient"));
    }
}
