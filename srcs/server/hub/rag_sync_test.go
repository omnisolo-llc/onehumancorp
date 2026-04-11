package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestDefaultRAGSyncService(t *testing.T) {
	tmpFile, err := os.CreateTemp("", "test_hub_rag_sync_*.db")
	if err != nil {
		t.Fatalf("failed to create temp db: %v", err)
	}
	defer os.Remove(tmpFile.Name())

	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec("CREATE TABLE autodream_memories (id TEXT PRIMARY KEY, content TEXT, embedding BLOB, source_mission_id TEXT, created_at DATETIME DEFAULT CURRENT_TIMESTAMP, sync_status VARCHAR(50) DEFAULT 'pending', last_sync_at DATETIME NULL);")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	svc := NewDefaultRAGSyncService(provider)

	ctx := context.Background()

	now := time.Now()
	// Test ProcessIncomingSync
	rec := RAGSyncRecord{
		ID: "mem1",
		Context: "test context",
		Vector: []byte{1, 2, 3, 4},
		SyncStatus: SyncStatusPending,
		LastSyncAt: &now,
	}
	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{rec})
	if err != nil {
		t.Fatalf("failed to ProcessIncomingSync: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to FetchPendingSyncs: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending sync, got %d", len(pending))
	}
	if len(pending[0].Vector) != 4 || pending[0].Vector[0] != 1 {
		t.Fatalf("expected vector data to match, got %v", pending[0].Vector)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"mem1"})
	if err != nil {
		t.Fatalf("failed to MarkSynced: %v", err)
	}

	pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to FetchPendingSyncs after mark: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("expected 0 pending syncs after MarkSynced, got %d", len(pendingAfter))
	}
}
