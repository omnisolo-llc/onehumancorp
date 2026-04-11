package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	_ "modernc.org/sqlite"
)

// newTestProvider creates a new in-memory SQLite database provider for testing.
func newTestProvider(t *testing.T) db.Provider {
	t.Helper()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqliteDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	t.Cleanup(func() {
		sqliteDB.Close()
	})

	return db.NewSqliteProvider(sqliteDB)
}

func TestDBRAGSyncService(t *testing.T) {
	provider := newTestProvider(t)

	ctx := context.Background()

	// Setup schema
	_, err := provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
		memory_id        TEXT PRIMARY KEY,
		context          TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin    TEXT,
		created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
		sync_status      VARCHAR(50) DEFAULT 'pending',
		last_sync_at     DATETIME NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ('1', 'ctx1', 'pending', NULL)")
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ('2', 'ctx2', 'pending', NULL)")
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ('3', 'ctx3', 'synced', CURRENT_TIMESTAMP)")
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}

	svc, err := NewDBRAGSyncService(provider, otel.Meter("test"))
	if err != nil {
		t.Fatalf("failed to create service: %v", err)
	}

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	pending, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record after marking 1 synced, got %d", len(pending))
	}
	if pending[0].ID != "2" {
		t.Fatalf("expected pending record to be 2, got %s", pending[0].ID)
	}

	// Test ProcessIncomingSync (insert new)
	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "4", Context: "ctx4"},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var syncStatus string
	err = provider.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '4'").Scan(&syncStatus)
	if err != nil {
		t.Fatalf("failed to query new record: %v", err)
	}
	if syncStatus != "synced" {
		t.Fatalf("expected sync status to be synced, got %s", syncStatus)
	}

	// Test ProcessIncomingSync (update existing)
	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "2", Context: "ctx2_updated"},
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var updatedContext string
	err = provider.QueryRow(ctx, "SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = '2'").Scan(&updatedContext, &syncStatus)
	if err != nil {
		t.Fatalf("failed to query updated record: %v", err)
	}
	if updatedContext != "ctx2_updated" {
		t.Fatalf("expected context to be updated, got %s", updatedContext)
	}
	if syncStatus != "synced" {
		t.Fatalf("expected sync status to be synced, got %s", syncStatus)
	}
}
