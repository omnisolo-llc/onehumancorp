package hub

import (
    "context"
    "testing"
    "encoding/json"
    "database/sql"

    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
    dbConn, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open test sqlite db: %v", err)
    }
    defer dbConn.Close()

    provider := db.NewSqliteProvider(dbConn)

    // We need to create the swarm_memory_embeddings table in the test database
    ctx := context.Background()
    _, err = provider.Exec(ctx, `
        CREATE TABLE swarm_memory_embeddings (
            memory_id TEXT PRIMARY KEY,
            context TEXT NOT NULL,
            vector_embedding BLOB,
            source_plugin TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        )
    `)
    if err != nil {
        t.Fatalf("failed to create test table: %v", err)
    }

    // Insert some initial data with vector
    vectorBytes, _ := json.Marshal([]float32{1.0, 2.0})
    _, err = provider.Exec(ctx, `
        INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
        VALUES ('test-id-1', 'test context 1', $1, 'pending')
    `, vectorBytes)
    if err != nil {
        t.Fatalf("failed to insert test data: %v", err)
    }

    svc := NewRAGSyncService(provider)

    t.Run("FetchPendingSyncs", func(t *testing.T) {
        records, err := svc.FetchPendingSyncs(ctx, 10)
        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }
        if len(records) != 1 {
            t.Fatalf("expected 1 record, got %d", len(records))
        }
        if records[0].ID != "test-id-1" {
            t.Errorf("expected ID 'test-id-1', got '%s'", records[0].ID)
        }
        if len(records[0].Vector) != 2 || records[0].Vector[0] != 1.0 {
            t.Errorf("expected Vector [1.0, 2.0], got %v", records[0].Vector)
        }
    })

    t.Run("MarkSynced", func(t *testing.T) {
        err := svc.MarkSynced(ctx, []string{"test-id-1"})
        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }

        // Verify status changed
        var status string
        err = provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'test-id-1'").Scan(&status)
        if err != nil {
            t.Fatalf("failed to query status: %v", err)
        }
        if status != "synced" {
            t.Errorf("expected status 'synced', got '%s'", status)
        }
    })

    t.Run("ProcessIncomingSync", func(t *testing.T) {
        newRecord := RAGSyncRecord{
            ID:      "test-id-2",
            Context: "test context 2",
            Vector:  []float32{3.0, 4.0},
        }
        err := svc.ProcessIncomingSync(ctx, []RAGSyncRecord{newRecord})
        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }

        var status string
        var context string
        var vec []byte
        err = provider.QueryRow(ctx, "SELECT sync_status, context, vector_embedding FROM swarm_memory_embeddings WHERE memory_id = 'test-id-2'").Scan(&status, &context, &vec)
        if err != nil {
            t.Fatalf("failed to query new record: %v", err)
        }
        if status != "synced" {
            t.Errorf("expected status 'synced', got '%s'", status)
        }
        if context != "test context 2" {
            t.Errorf("expected context 'test context 2', got '%s'", context)
        }

        var parsedVec []float32
        json.Unmarshal(vec, &parsedVec)
        if len(parsedVec) != 2 || parsedVec[0] != 3.0 {
            t.Errorf("expected Vector [3.0, 4.0], got %v", parsedVec)
        }
    })
}
