package hub_test

import (
    "context"
    "database/sql"
    "testing"
    "time"

    _ "modernc.org/sqlite"

    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/hub"
)

func TestSQLRAGSyncService(t *testing.T) {
    db, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open sqlite memory db: %v", err)
    }
    defer db.Close()

    // Create table schema
    _, err = db.Exec(`
        CREATE TABLE consolidated_memory (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            content TEXT NOT NULL,
            source_type TEXT NOT NULL,
            sync_status TEXT DEFAULT 'pending',
            last_sync_at TIMESTAMP NULL
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    service := hub.NewSQLRAGSyncService(db)

    // Create context with claims
    claims := &auth.Claims{
        OrganizationID: "org-1",
        Roles:          []string{"agent"},
        Subject:        "user-1",
    }
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    // Insert test data for org-1
    _, err = db.ExecContext(ctx, `
        INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status)
        VALUES ('1', 'org-1', 'memory 1', 'test', 'pending'),
               ('2', 'org-1', 'memory 2', 'test', 'pending'),
               ('3', 'org-1', 'memory 3', 'test', 'synced'),
               ('99', 'org-2', 'memory 99', 'test', 'pending')
    `)
    if err != nil {
        t.Fatalf("failed to insert data: %v", err)
    }

    // Test FetchPendingSyncs
    pending, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(pending) != 2 {
        t.Fatalf("expected 2 pending records, got %d", len(pending))
    }

    for _, p := range pending {
        if p.ID == "99" {
            t.Fatalf("fetched record belonging to another org")
        }
    }

    // Test MarkSynced
    err = service.MarkSynced(ctx, []string{"1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if len(pendingAfter) != 1 {
        t.Fatalf("expected 1 pending record, got %d", len(pendingAfter))
    }
    if pendingAfter[0].ID != "2" {
        t.Fatalf("expected record 2 to be pending")
    }

    // Test ProcessIncomingSync
    now := time.Now()
    err = service.ProcessIncomingSync(ctx, []hub.RAGSyncRecord{
        {ID: "4", Context: "memory 4", SyncStatus: hub.SyncStatusPending},
    })
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    // ProcessIncomingSync marks as synced internally, check if it's there
    var count int
    var orgID string
    err = db.QueryRowContext(ctx, "SELECT organization_id FROM consolidated_memory WHERE id = '4' AND sync_status = 'synced'").Scan(&orgID)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if orgID != "org-1" {
        t.Fatalf("expected org-1, got %s", orgID)
    }

    // ProcessIncomingSync update existing
    err = service.ProcessIncomingSync(ctx, []hub.RAGSyncRecord{
        {ID: "2", Context: "memory 2 updated", SyncStatus: hub.SyncStatusPending, LastSyncAt: now},
    })
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    var updatedContent string
    err = db.QueryRowContext(ctx, "SELECT content FROM consolidated_memory WHERE id = '2'").Scan(&updatedContent)
    if err != nil {
         t.Fatalf("unexpected error: %v", err)
    }
    if updatedContent != "memory 2 updated" {
         t.Fatalf("expected memory 2 updated, got %s", updatedContent)
    }

    // Test fallback (no claims)
    ctxNoAuth := context.Background()
    err = service.ProcessIncomingSync(ctxNoAuth, []hub.RAGSyncRecord{
        {ID: "5", Context: "memory 5", SyncStatus: hub.SyncStatusPending},
    })
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = db.QueryRowContext(ctxNoAuth, "SELECT COUNT(*) FROM consolidated_memory WHERE id = '5' AND organization_id = 'default_org'").Scan(&count)
    if err != nil {
         t.Fatalf("unexpected error: %v", err)
    }
    if count != 1 {
         t.Fatalf("expected 1 record with default_org, got %d", count)
    }

    // Coverage edge cases
    err = service.ProcessIncomingSync(ctx, []hub.RAGSyncRecord{})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }

    err = service.MarkSynced(ctx, []string{})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}
