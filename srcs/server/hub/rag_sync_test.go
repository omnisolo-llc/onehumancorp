package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}

	_, err = sqliteDB.Exec(`
		CREATE TABLE agent_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			content TEXT NOT NULL,
			embedding BLOB,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_memories table: %v", err)
	}

	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncService(t *testing.T) {
	database := setupTestDB(t)
	svc := NewRAGSyncService(database)
	ctx := context.Background()

	// 1. ProcessIncomingSync
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})
	recordsToSync := []RAGSyncRecord{
		{
			ID: "1",
			Context: "memory 1",
			Vector: []byte{1, 2, 3},
		},
	}

	err := svc.ProcessIncomingSync(ctxWithClaims, recordsToSync)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// 2. Insert pending manually via Provider Exec
	_, err = database.Exec(ctx, "INSERT INTO agent_memories (id, organization_id, content, sync_status) VALUES ('2', 'local', 'memory 2', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert mock data: %v", err)
	}

	// 3. FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "2" {
		t.Fatalf("Expected pending record ID 2, got %s", pending[0].ID)
	}

	// 4. MarkSynced
	err = svc.MarkSynced(ctx, []string{"2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfterMark, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pendingAfterMark) != 0 {
		t.Fatalf("Expected 0 pending records after MarkSynced, got %d", len(pendingAfterMark))
	}
}
