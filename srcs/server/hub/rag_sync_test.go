package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric/noop"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()

	meter := noop.NewMeterProvider().Meter("test")
	telemetry.InitWithMeter(meter)

	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqliteDB.Close()

	provider := db.NewSqliteProvider(sqliteDB)
	database := &db.DB{Provider: provider}

	// Create table
	_, err = database.Provider.Exec(ctx, `
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status TEXT DEFAULT 'pending',
			last_sync_timestamp DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(database.Provider)

	// 1. Insert some pending records
	_, err = database.Provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES
			('rec1', 'hello world 1', '[0.1, 0.2]', 'pending'),
			('rec2', 'hello world 2', '[0.3, 0.4]', 'synced');
	`)
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}

	// 2. Fetch pending syncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(records))
	}
	if records[0].ID != "rec1" {
		t.Errorf("expected record ID rec1, got %s", records[0].ID)
	}

	// 3. Mark synced
	err = service.MarkSynced(ctx, []string{"rec1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// verify
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(records))
	}

	// 4. Process incoming sync
	incoming := []RAGSyncRecord{
		{
			ID:         "rec3",
			Context:    "hello world 3",
			Vector:     []float32{0.5, 0.6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID:         "rec1", // update
			Context:    "hello world 1 updated",
			Vector:     []float32{0.7, 0.8},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Check rec3
	row := database.Provider.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = 'rec3'")
	var content, status string
	err = row.Scan(&content, &status)
	if err != nil {
		t.Fatalf("failed to query rec3: %v", err)
	}
	if content != "hello world 3" {
		t.Errorf("expected content 'hello world 3', got %s", content)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}

	// Check rec1 update
	row = database.Provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = 'rec1'")
	err = row.Scan(&content)
	if err != nil {
		t.Fatalf("failed to query rec1: %v", err)
	}
	if content != "hello world 1 updated" {
		t.Errorf("expected content 'hello world 1 updated', got %s", content)
	}
}
