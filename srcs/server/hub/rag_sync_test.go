package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	cleanup, err := telemetry.InitTelemetry()
	if err != nil {
		t.Fatalf("failed to initialize telemetry: %v", err)
	}
	defer cleanup()

	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_timestamp TIMESTAMP NULL,
			organization_id TEXT DEFAULT 'system'
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES
			('m1', 'context1', X'010203', 'pending'),
			('m2', 'context2', X'040506', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}
	service := NewRAGSyncService(dbWrapper)

	ctx := context.Background()

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	if records[0].SyncStatus != SyncStatusPending || records[1].SyncStatus != SyncStatusPending {
		t.Fatalf("expected sync status pending")
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"m1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record after MarkSynced, got %d", len(records))
	}
	if records[0].ID != "m2" {
		t.Fatalf("expected m2 to be pending")
	}

	// Test ProcessIncomingSync
	incomingRecords := []RAGSyncRecord{
		{
			ID:      "m3",
			Context: "context3",
			Vector:  []byte{7, 8, 9},
		},
	}
	err = service.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var syncStatus string
	err = sqlDB.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'm3'").Scan(&syncStatus)
	if err != nil {
		t.Fatalf("failed to query m3: %v", err)
	}
	if syncStatus != "synced" {
		t.Fatalf("expected m3 to be synced, got %s", syncStatus)
	}
}
