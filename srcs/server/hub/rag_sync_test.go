package hub

import (
	"context"
	"database/sql"
	"testing"


	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) (*db.DB, context.Context) {
	ctx := context.Background()
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")

	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}

	_, err = sqliteDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	dbWrapper := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
	return dbWrapper, ctx
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	dbWrapper, ctx := setupTestDB(t)
	service := NewRAGSyncService(dbWrapper)

	provider := dbWrapper.Provider
	tx, _ := provider.Begin(ctx)
	tx.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test content 1', 'pending')`)
	tx.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'test content 2', 'synced')`)
	tx.Commit(ctx)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Fatalf("Expected record ID 1, got %s", records[0].ID)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	dbWrapper, ctx := setupTestDB(t)
	service := NewRAGSyncService(dbWrapper)

	provider := dbWrapper.Provider
	tx, _ := provider.Begin(ctx)
	tx.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test content 1', 'pending')`)
	tx.Commit(ctx)

	err := service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	records, _ := service.FetchPendingSyncs(ctx, 10)
	if len(records) != 0 {
		t.Fatalf("Expected 0 pending records, got %d", len(records))
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	dbWrapper, ctx := setupTestDB(t)
	service := NewRAGSyncService(dbWrapper)

	records := []RAGSyncRecord{
		{ID: "1", Content: "remote content", Vector: []byte("vector"), SyncStatus: SyncStatusSynced},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	provider := dbWrapper.Provider
	var count int
	provider.QueryRow(ctx, `SELECT COUNT(*) FROM autodream_memories WHERE id = '1' AND sync_status = 'synced'`).Scan(&count)
	if count != 1 {
		t.Fatalf("Expected 1 synced record, got %d", count)
	}
}
