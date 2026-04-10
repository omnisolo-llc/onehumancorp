package hub

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) *db.DB {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	_, err = sqliteDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	dbProvider := setupTestDB(t)
	svc := NewDefaultRAGSyncService(dbProvider)
	ctx := context.Background()

	// Insert test data
	tx, _ := dbProvider.Begin(ctx)
	tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "1", "ctx1", "pending")
	tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "2", "ctx2", "synced")
	tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "3", "ctx3", "pending")
	tx.Commit(ctx)

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(records))
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	dbProvider := setupTestDB(t)
	svc := NewDefaultRAGSyncService(dbProvider)
	ctx := context.Background()

	// Insert test data
	tx, _ := dbProvider.Begin(ctx)
	tx.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "1", "ctx1", "pending")
	tx.Commit(ctx)

	err := svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify
	tx, _ = dbProvider.Begin(ctx)
	rows, _ := tx.Query(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = ?", "1")
	defer rows.Close()
	if rows.Next() {
		var status string
		rows.Scan(&status)
		if status != "synced" {
			t.Errorf("expected status 'synced', got %s", status)
		}
	}
	tx.Commit(ctx)
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	dbProvider := setupTestDB(t)
	svc := NewDefaultRAGSyncService(dbProvider)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{ID: "1", Context: "new_ctx", Vector: []byte("vec"), SyncStatus: SyncStatusPending},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify
	tx, _ := dbProvider.Begin(ctx)
	rows, _ := tx.Query(ctx, "SELECT sync_status, context FROM swarm_memory_embeddings WHERE memory_id = ?", "1")
	defer rows.Close()
	if rows.Next() {
		var status, context string
		rows.Scan(&status, &context)
		if status != "synced" {
			t.Errorf("expected status 'synced', got %s", status)
		}
		if context != "new_ctx" {
			t.Errorf("expected context 'new_ctx', got %s", context)
		}
	}
	tx.Commit(ctx)
}
