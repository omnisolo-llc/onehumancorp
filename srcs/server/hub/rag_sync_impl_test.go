package hub

import (
	"context"
	"testing"
    "fmt"
    "path/filepath"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
)

// Helper to create test provider since db.NewTestProvider is in a test file in the db package
// and not exported for other packages.
func createTestDB(t *testing.T) db.Provider {
    tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "test.db")
	dsn := fmt.Sprintf("sqlite://%s?mode=memory&cache=shared", dbPath)
    t.Setenv("DATABASE_URL", dsn)
    database, err := db.New(context.Background())
    if err != nil {
        t.Fatalf("failed to create db: %v", err)
    }
    return database
}

func TestRAGSyncServiceImpl(t *testing.T) {
	// Initialize telemetry to avoid panics on metric recording
	meterProvider := otel.GetMeterProvider()
	otel.SetMeterProvider(meterProvider)

	database := createTestDB(t)
	defer database.Close()
	ctx := context.Background()

	// Ensure the correct schema is present for the test memory database
	_, err := database.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create test schema: %v", err)
	}

	service := NewRAGSyncService(database)

	// Test ProcessIncomingSync (Upsert)
	incoming := []RAGSyncRecord{
		{
			ID:         "test-id-1",
			Context:    "test context info",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: SyncStatusPending,
		},
	}

	if err := service.ProcessIncomingSync(ctx, incoming); err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Insert a pending record manually
	_, err = database.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('test-id-2', 'pending context', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert pending record: %v", err)
	}

	// Fetch pending
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "test-id-2" {
		t.Errorf("expected pending ID 'test-id-2', got '%s'", pending[0].ID)
	}

	// Mark Synced
	if err := service.MarkSynced(ctx, []string{"test-id-2"}); err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Fetch pending again, should be empty
	pending2, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs 2 failed: %v", err)
	}
	if len(pending2) != 0 {
		t.Errorf("expected 0 pending records, got %d", len(pending2))
	}
}
