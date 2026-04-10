package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric/noop"
)

func TestRAGSyncServiceImpl(t *testing.T) {
	// Initialize noop telemetry
	mp := noop.NewMeterProvider()
	m := mp.Meter("test")
	var err error
	telemetry.RagRecordsSyncedTotal, err = m.Int64Counter("rag_synced")
	if err != nil {
		t.Fatalf("failed to init metric: %v", err)
	}
	telemetry.RagSyncErrorsTotal, err = m.Int64Counter("rag_errors")
	if err != nil {
		t.Fatalf("failed to init metric: %v", err)
	}

	// Setup temporary SQLite database
	tmpfile, err := os.CreateTemp("", "test_db_*.sqlite")
	if err != nil {
		t.Fatalf("failed to create temp db file: %v", err)
	}
	defer os.Remove(tmpfile.Name())

	sqlDB, err := sql.Open("sqlite", tmpfile.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Initialize table schema
	initQuery := `
	CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`
	_, err = provider.Exec(ctx, initQuery)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(provider)

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{
			ID:         "test-id-1",
			Context:    "test context 1",
			Vector:     []float32{0.1, 0.2},
			SyncStatus: SyncStatusPending,
		},
		{
			ID:         "test-id-2",
			Context:    "test context 2",
			Vector:     []float32{0.3, 0.4},
			SyncStatus: SyncStatusPending,
		},
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("failed to process incoming sync: %v", err)
	}


	// Test FetchPendingSyncs
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES (?, ?, ?, ?)", "pending-1", "pending content", "[0.5, 0.6]", "pending")
	if err != nil {
		t.Fatalf("failed to insert pending record: %v", err)
	}

	pendingRecords, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}
	if len(pendingRecords) != 1 {
		t.Errorf("expected 1 pending record, got %d", len(pendingRecords))
	}
	if len(pendingRecords[0].Vector) != 2 || pendingRecords[0].Vector[0] != 0.5 {
		t.Errorf("expected vector to be parsed correctly, got %v", pendingRecords[0].Vector)
	}


	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"pending-1"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	// Verify it was marked synced
	var status string
	err = provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = 'pending-1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to verify sync status: %v", err)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got %s", status)
	}
}
