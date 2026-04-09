package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"

	_ "modernc.org/sqlite"
)

func NewTestDB(t *testing.T) *sql.DB {
	t.Helper()
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := db.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		db.Close()
	})

	return db
}

func TestRAGSyncService(t *testing.T) {
	cleanup, err := telemetry.InitTelemetry()
	if err != nil {
		t.Fatalf("failed to init telemetry: %v", err)
	}
	defer cleanup()

	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	if err := InitMetrics(meter); err != nil {
		t.Fatalf("failed to init metrics: %v", err)
	}

	sqlDB := NewTestDB(t)

	// Create table for sqlite
	_, err = sqlDB.Exec(`
		CREATE TABLE IF NOT EXISTS swarm_memory (
			id TEXT PRIMARY KEY,
			context TEXT,
			vector_embedding BLOB,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewRAGSyncService(sqlDB)
	ctx := context.Background()

	// Insert test data with vector bytes and a genuine NULL last_sync_at
	vec := []float32{0.1, 0.2, 0.3}
	vecBytes, _ := json.Marshal(vec)
	_, err = sqlDB.ExecContext(ctx, "INSERT INTO swarm_memory (id, context, vector_embedding, sync_status, last_sync_at) VALUES ('1', 'ctx1', ?, 'pending', NULL), ('2', 'ctx2', NULL, 'pending', NULL)", string(vecBytes))
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// 1. FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}
	if len(pending[0].Vector) != 3 {
		t.Fatalf("expected vector length 3, got %d", len(pending[0].Vector))
	}

	// 2. MarkSynced
	ids := []string{"1"}
	if err := svc.MarkSynced(ctx, ids); err != nil {
		t.Fatalf("unexpected error marking synced: %v", err)
	}

	pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending after sync: %v", err)
	}
	if len(pendingAfter) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pendingAfter))
	}

	// 3. ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "ctx3", Vector: []float32{0.9, 0.8}},
	}
	if err := svc.ProcessIncomingSync(ctx, incoming); err != nil {
		t.Fatalf("unexpected error processing incoming: %v", err)
	}
}
