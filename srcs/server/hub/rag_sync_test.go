package hub_test

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
)

// Helper to create test provider inline since we can't import db_test
func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := dbConn.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		dbConn.Close()
	})

	return db.NewSqliteProvider(dbConn)
}

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	provider := newTestProvider(t)

	err := hub.InitRAGSyncMetrics()
	if err != nil {
		t.Fatalf("Failed to initialize metrics: %v", err)
	}

	schema := `
	CREATE TABLE swarm_memory_embeddings (
		memory_id        TEXT PRIMARY KEY,
		context          TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin    TEXT,
		created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
		sync_status      VARCHAR(50) DEFAULT 'pending',
		last_sync_at     TIMESTAMPTZ NULL
	);
	`
	if _, err := provider.Exec(ctx, schema); err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	service := hub.NewRAGSyncService(provider)

	records := []hub.RAGSyncRecord{
		{
			ID:         "mem1",
			Context:    "Test context 1",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: hub.SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID:         "mem2",
			Context:    "Test context 2",
			Vector:     []float32{0.4, 0.5, 0.6},
			SyncStatus: hub.SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	if err := service.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT memory_id, sync_status FROM swarm_memory_embeddings")
	if err != nil {
		t.Fatalf("Failed to query inserted records: %v", err)
	}

	count := 0
	for rows.Next() {
		var id string
		var status string
		if err := rows.Scan(&id, &status); err != nil {
			t.Fatalf("Scan failed: %v", err)
		}
		if status != "synced" {
			t.Errorf("Expected status 'synced', got %s for id %s", status, id)
		}
		count++
	}
	rows.Close()
	if count != 2 {
		t.Errorf("Expected 2 records, got %d", count)
	}

	pendingSchema := `
	INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
	VALUES
	('mem3', 'Pending context 3', 'pending'),
	('mem4', 'Pending context 4', 'pending');
	`
	if _, err := provider.Exec(ctx, pendingSchema); err != nil {
		t.Fatalf("Failed to insert pending records: %v", err)
	}

	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("Expected 2 pending records, got %d", len(pending))
	}

	idsToMark := []string{"mem3", "mem4"}
	if err := service.MarkSynced(ctx, idsToMark); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs after mark failed: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Errorf("Expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}
}
