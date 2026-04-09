package hub

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/auth"
    _ "modernc.org/sqlite"
)

func TestDefaultRAGSyncService(t *testing.T) {
    t.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")
    ctx := context.Background()
    dbConn, err := db.New(ctx)
    if err != nil {
        t.Fatalf("failed to connect to db: %v", err)
    }
    defer dbConn.Close()

    // Ensure table exists for test
    _, err = dbConn.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS autodream_memories (
            id TEXT PRIMARY KEY,
            content TEXT,
            sync_status TEXT DEFAULT 'pending',
            last_sync_at TIMESTAMP,
            organization_id TEXT,
            source_type TEXT
        )
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    service := NewDefaultRAGSyncService(dbConn)

    claims := &auth.Claims{OrganizationID: "test-org"}
    authCtx := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

    // Insert a pending record
    _, err = dbConn.Exec(ctx, `
        INSERT INTO autodream_memories (id, content, sync_status, organization_id)
        VALUES ('1', 'test context', 'pending', 'test-org')
    `)
    if err != nil {
        t.Fatalf("failed to insert record: %v", err)
    }

    // Test FetchPendingSyncs
    records, err := service.FetchPendingSyncs(authCtx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(records) != 1 {
        t.Fatalf("expected 1 records, got %d", len(records))
    }
    if records[0].ID != "1" {
        t.Fatalf("expected record ID 1, got %s", records[0].ID)
    }

    // Test MarkSynced
    err = service.MarkSynced(authCtx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    // Verify it was marked synced
    var syncStatus string
    err = dbConn.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '1'").Scan(&syncStatus)
    if err != nil {
        t.Fatalf("failed to query status: %v", err)
    }
    if syncStatus != "synced" {
        t.Fatalf("expected status synced, got %s", syncStatus)
    }

    // Test ProcessIncomingSync
    err = service.ProcessIncomingSync(authCtx, []RAGSyncRecord{{ID: "2", Context: "new context"}})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    var content string
    err = dbConn.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = '2'").Scan(&content)
    if err != nil {
        t.Fatalf("failed to query new record: %v", err)
    }
    if content != "new context" {
        t.Fatalf("expected new context, got %s", content)
    }
}
