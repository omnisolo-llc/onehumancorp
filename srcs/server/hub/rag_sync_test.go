package hub

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
)

func TestRAGSyncProvider(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
    CREATE TABLE IF NOT EXISTS consolidated_memory (
        id TEXT PRIMARY KEY,
        organization_id TEXT NOT NULL,
        agent_id TEXT,
        content TEXT NOT NULL,
        embedding BLOB,
        source_type TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        sync_status VARCHAR(50) DEFAULT 'pending',
        last_sync_at DATETIME NULL
    );
    `)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := NewRAGSyncProvider(db)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{ID: "r1", OrgID: "org1", Context: "test 1", SyncStatus: SyncStatusPending},
		{ID: "r2", OrgID: "org2", Context: "test 2", SyncStatus: SyncStatusPending},
	}

	err = provider.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	pending, err := provider.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	err = provider.MarkSynced(ctx, []string{"r1", "r2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := provider.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pendingAfter) != 0 {
		t.Errorf("expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}
}
