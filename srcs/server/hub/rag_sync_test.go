package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric/noop"
)

func init() {
	// Dummy telemetry init to prevent nil pointer crashes if not already init
	mp := noop.NewMeterProvider()
	m := mp.Meter("test")
	telemetry.RagRecordsSyncedTotal, _ = m.Int64Counter("rag_records_synced_total")
	telemetry.RagSyncErrorsTotal, _ = m.Int64Counter("rag_sync_errors_total")
}

func TestRAGSyncService(t *testing.T) {
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()
	provider := db.NewSqliteProvider(dbConn)

	// Define table as TEXT/BLOB for sqlite testing compat
	_, err = dbConn.Exec(`CREATE TABLE swarm_memory_embeddings (
		memory_id TEXT PRIMARY KEY,
		context TEXT NOT NULL,
		vector_embedding BLOB,
		sync_status TEXT DEFAULT 'pending',
		last_sync_at DATETIME
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "mem-1",
			Context:    "Test context",
			Vector:     []byte("test-vector"),
			SyncStatus: SyncStatusPending,
			LastSyncAt: time.Now(),
		},
	}

	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending sync, got %d", len(pending))
	}

	err = svc.MarkSynced(ctx, []string{"mem-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, _ := svc.FetchPendingSyncs(ctx, 10)
	if len(pendingAfter) != 0 {
		t.Fatalf("Expected 0 pending syncs after MarkSynced, got %d", len(pendingAfter))
	}
}
