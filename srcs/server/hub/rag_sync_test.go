package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"
	"time"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	// Initialize in-memory SQLite DB
	sqlDB, err := sql.Open("sqlite", "file::memory:?mode=memory")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)
	defer provider.Close()

	// Manually create the table with the new columns
	createTableQuery := `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`
	if _, err := provider.Exec(ctx, createTableQuery); err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewDatabaseRAGSyncService(provider)

	// Test 1: FetchPendingSyncs (empty)
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Errorf("expected 0 records, got %d", len(records))
	}

	// Insert some dummy pending records
	vecBytes, _ := json.Marshal([]float32{1.1, 2.2})
	insertQuery := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ('mem-1', 'Context 1', ?, 'pending'),
		       ('mem-2', 'Context 2', ?, 'pending'),
		       ('mem-3', 'Context 3', ?, 'synced')
	`
	if _, err := provider.Exec(ctx, insertQuery, vecBytes, vecBytes, vecBytes); err != nil {
		t.Fatalf("failed to insert dummy records: %v", err)
	}

	// Test 2: FetchPendingSyncs
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(records))
	}
	if records[0].Vector == nil || len(records[0].Vector) != 2 {
		t.Errorf("expected vector to be populated, got %v", records[0].Vector)
	}

	// Test 3: MarkSynced
	ids := []string{"mem-1"}
	if err := service.MarkSynced(ctx, ids); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Fetch again, should only be 1 pending now
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("expected 1 pending record after MarkSynced, got %d", len(records))
	}
	if records[0].ID != "mem-2" {
		t.Errorf("expected mem-2 to be pending, got %s", records[0].ID)
	}

	// Test 4: ProcessIncomingSync
	incomingRecords := []RAGSyncRecord{
		{
			ID:         "mem-2", // Exists, will update
			Context:    "Context 2 Updated",
			Vector:     []float32{3.3, 4.4},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID:         "mem-4", // New, will insert
			Context:    "Context 4",
			Vector:     []float32{5.5, 6.6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	if err := service.ProcessIncomingSync(ctx, incomingRecords); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify mem-2 was updated and mem-4 was inserted
	var ctx2, ctx4 string
	var status2, status4 string
	if err := provider.QueryRow(ctx, "SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem-2'").Scan(&ctx2, &status2); err != nil {
		t.Fatalf("failed to query mem-2: %v", err)
	}
	if ctx2 != "Context 2 Updated" || status2 != "synced" {
		t.Errorf("mem-2 not properly updated, got context: %s, status: %s", ctx2, status2)
	}

	if err := provider.QueryRow(ctx, "SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem-4'").Scan(&ctx4, &status4); err != nil {
		t.Fatalf("failed to query mem-4: %v", err)
	}
	if ctx4 != "Context 4" || status4 != "synced" {
		t.Errorf("mem-4 not properly inserted, got context: %s, status: %s", ctx4, status4)
	}
}
