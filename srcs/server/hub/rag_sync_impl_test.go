package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestDefaultRAGSyncService(t *testing.T) {
	ctx := context.Background()
	dbProvider := db.NewTestProvider(t)
	defer dbProvider.Close()

	// Create table for test provider
	_, err := dbProvider.Exec(ctx, "CREATE TABLE swarm_memory_embeddings (memory_id TEXT PRIMARY KEY, context TEXT, sync_status TEXT, last_sync_at TIMESTAMPTZ)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewRAGSyncService(dbProvider)

	// Insert test data
	_, err = dbProvider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'test1', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// FetchPendingSyncs
	recs, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(recs) != 1 {
		t.Errorf("expected 1 record, got %d", len(recs))
	}

	if recs[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", recs[0].ID)
	}

	// MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	recs, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(recs) != 0 {
		t.Errorf("expected 0 records, got %d", len(recs))
	}

	// ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "2", Context: "test2", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	}

	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var count int
	row := dbProvider.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = '2'")
	if err := row.Scan(&count); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 record in db, got %d", count)
	}
}
