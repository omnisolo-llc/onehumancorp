package hub

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric/noop"
)

func setupTestDB(t *testing.T) db.Provider {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite DB: %v", err)
	}

	provider := db.NewSqliteProvider(dbConn)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMPTZ NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider
}

func TestDefaultRAGSyncService(t *testing.T) {
	provider := setupTestDB(t)
	meter := noop.NewMeterProvider().Meter("test")
	InitRAGSyncMetrics(meter)

	service := NewDefaultRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	vec1 := []float32{0.1, 0.2, 0.3}
	vecBytes := encodeVector(vec1)
	_, err := provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding)
		VALUES ('mem-1', 'test context 1', ?)
	`, vecBytes)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "mem-1" || records[0].Context != "test context 1" || len(records[0].Vector) != 3 {
		t.Fatalf("unexpected record content: %+v", records[0])
	}

	// MarkSynced
	err = service.MarkSynced(ctx, []string{"mem-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 records after MarkSynced, got %d", len(records))
	}

	// ProcessIncomingSync
	vec2 := []float32{0.4, 0.5, 0.6}
	incoming := []RAGSyncRecord{
		{
			ID:      "mem-2",
			Context: "incoming context",
			Vector:  vec2,
		},
	}

	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify it was marked synced immediately
	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected incoming record to be marked synced immediately")
	}

	// Verify using plain query
	rows, err := provider.Query(ctx, "SELECT memory_id, sync_status FROM swarm_memory_embeddings WHERE memory_id = 'mem-2'")
	if err != nil {
		t.Fatalf("failed to query after ProcessIncomingSync: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatalf("expected to find mem-2 in db")
	}
	var memID, status string
	if err := rows.Scan(&memID, &status); err != nil {
		t.Fatalf("failed to scan mem-2: %v", err)
	}
	if status != "synced" {
		t.Fatalf("expected mem-2 to have 'synced' status, got %s", status)
	}
}

// Ensure the interface satisfies memory correctly
var _ RAGSyncService = (*DefaultRAGSyncService)(nil)
