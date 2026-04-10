package hub

import (
	"context"
	"database/sql"
	"reflect"
	"testing"
	"time"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric/noop"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open SQLite db: %v", err)
	}
	t.Cleanup(func() { sqliteDB.Close() })
	provider := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_timestamp TIMESTAMPTZ NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create test table: %v", err)
	}

	return provider
}

func TestInitRAGSyncMetrics(t *testing.T) {
	meter := noop.NewMeterProvider().Meter("test")
	syncedTotal, errorsTotal, err := InitRAGSyncMetrics(meter)
	if err != nil {
		t.Fatalf("InitRAGSyncMetrics failed: %v", err)
	}
	if syncedTotal == nil {
		t.Error("Expected syncedTotal counter, got nil")
	}
	if errorsTotal == nil {
		t.Error("Expected errorsTotal counter, got nil")
	}
}

func TestProcessAndFetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	testTime := time.Now().Truncate(time.Millisecond)

	records := []RAGSyncRecord{
		{
			ID:         "mem-1",
			Context:    "Context 1",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: SyncStatusPending,
			LastSyncAt: time.Time{},
		},
		{
			ID:         "mem-2",
			Context:    "Context 2",
			Vector:     nil,
			SyncStatus: SyncStatusSynced,
			LastSyncAt: testTime,
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending sync, got %d", len(pending))
	}

	if pending[0].ID != "mem-1" {
		t.Errorf("Expected ID mem-1, got %s", pending[0].ID)
	}
	if !reflect.DeepEqual(pending[0].Vector, []float32{0.1, 0.2, 0.3}) {
		t.Errorf("Expected Vector [0.1, 0.2, 0.3], got %v", pending[0].Vector)
	}
}

func TestMarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "mem-3",
			Context:    "Context 3",
			Vector:     []float32{0.4, 0.5},
			SyncStatus: SyncStatusPending,
		},
	}
	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"mem-3"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 0 {
		t.Fatalf("Expected 0 pending syncs after MarkSynced, got %d", len(pending))
	}
}
