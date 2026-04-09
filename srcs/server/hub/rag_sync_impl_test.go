package hub

import (
    "context"
    "database/sql"
    "testing"
    "time"

    _ "modernc.org/sqlite"
)

func TestDatabaseRAGSyncService(t *testing.T) {
    db, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("Failed to open sqlite memory db: %v", err)
    }
    defer db.Close()

    // Setup schema
    _, err = db.Exec(`
        CREATE TABLE consolidated_memory (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            agent_id TEXT,
            content TEXT NOT NULL,
            source_type TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sync_status VARCHAR(50) DEFAULT 'pending',
            last_sync_at TIMESTAMP
        )
    `)
    if err != nil {
        t.Fatalf("Failed to create table: %v", err)
    }

    // Insert test data
    _, err = db.Exec(`
        INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status)
        VALUES
            ('1', 'org1', 'content 1', 'test', 'pending'),
            ('2', 'org1', 'content 2', 'test', 'synced'),
            ('3', 'org1', 'content 3', 'test', 'pending')
    `)
    if err != nil {
        t.Fatalf("Failed to insert data: %v", err)
    }

    svc := NewDatabaseRAGSyncService(db)
    ctx := context.Background()

    // Test FetchPendingSyncs
    records, err := svc.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }
    if len(records) != 2 {
        t.Errorf("Expected 2 pending records, got %d", len(records))
    }

    // Test MarkSynced
    err = svc.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    var syncStatus string
    err = db.QueryRow("SELECT sync_status FROM consolidated_memory WHERE id = '1'").Scan(&syncStatus)
    if err != nil {
        t.Fatalf("Failed to query status: %v", err)
    }
    if syncStatus != "synced" {
        t.Errorf("Expected status synced, got %s", syncStatus)
    }

    // Test ProcessIncomingSync - Update existing
    incomingUpdate := []RAGSyncRecord{
        {ID: "3", Context: "updated content 3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
    }
    err = svc.ProcessIncomingSync(ctx, incomingUpdate)
    if err != nil {
        t.Fatalf("ProcessIncomingSync update failed: %v", err)
    }

    var content string
    err = db.QueryRow("SELECT content FROM consolidated_memory WHERE id = '3'").Scan(&content)
    if err != nil {
        t.Fatalf("Failed to query content: %v", err)
    }
    if content != "updated content 3" {
        t.Errorf("Expected content 'updated content 3', got '%s'", content)
    }

    // Test ProcessIncomingSync - Insert new
    incomingInsert := []RAGSyncRecord{
        {ID: "4", Context: "new content 4", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
    }
    err = svc.ProcessIncomingSync(ctx, incomingInsert)
    if err != nil {
        t.Fatalf("ProcessIncomingSync insert failed: %v", err)
    }

    var newContent string
    err = db.QueryRow("SELECT content FROM consolidated_memory WHERE id = '4'").Scan(&newContent)
    if err != nil {
        t.Fatalf("Failed to query new content: %v", err)
    }
    if newContent != "new content 4" {
        t.Errorf("Expected content 'new content 4', got '%s'", newContent)
    }
}
