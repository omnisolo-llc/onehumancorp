package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	// db.NewSqliteProvider(sqlDB) as mentioned in memories.
	provider := db.NewSqliteProvider(sqlDB)

	// Create required schema manually since db.New() doesn't run goose migrations automatically in tests.
	// As per instructions, must explicitly execute CREATE TABLE setup statements within the test.
	ctx := context.Background()
	tx, err := provider.Begin(ctx)
	if err != nil {
		t.Fatalf("failed to begin tx: %v", err)
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	if err := tx.Commit(ctx); err != nil {
		t.Fatalf("failed to commit setup tx: %v", err)
	}

	return provider
}

func TestFetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	tx, err := provider.Begin(ctx)
	if err != nil {
		t.Fatalf("failed to begin tx: %v", err)
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES
			('1', 'pending content 1', '[0.1, 0.2]', 'pending', NULL),
			('2', 'synced content', '[0.3, 0.4]', 'synced', CURRENT_TIMESTAMP),
			('3', 'pending content 2', '[0.5, 0.6]', 'pending', NULL)
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	if err := tx.Commit(ctx); err != nil {
		t.Fatalf("failed to commit tx: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	if records[0].ID != "1" && records[1].ID != "1" {
		t.Errorf("expected to find record ID 1")
	}
}

func TestMarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	tx, err := provider.Begin(ctx)
	if err != nil {
		t.Fatalf("failed to begin tx: %v", err)
	}
	defer tx.Rollback(ctx)
	_, err = tx.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
		VALUES
			('1', 'pending content 1', '[0.1, 0.2]', 'pending', NULL)
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	if err := tx.Commit(ctx); err != nil {
		t.Fatalf("failed to commit tx: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify update
	rows, err := provider.Query(ctx, "SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'")
	if err != nil {
		t.Fatalf("failed to query db: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatalf("expected to find record 1")
	}

	var syncStatus string
	var lastSyncAt sql.NullTime
	if err := rows.Scan(&syncStatus, &lastSyncAt); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}

	if syncStatus != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got %s", syncStatus)
	}

	if !lastSyncAt.Valid {
		t.Errorf("expected last_sync_at to be set")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	now := time.Now().Truncate(time.Second)

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "new context from cloud",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: now,
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify insertion
	rows, err := provider.Query(ctx, "SELECT content, sync_status, embedding FROM autodream_memories WHERE id = '1'")
	if err != nil {
		t.Fatalf("failed to query db: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatalf("expected to find record 1")
	}

	var content, syncStatus, embedding string
	if err := rows.Scan(&content, &syncStatus, &embedding); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}

	if content != "new context from cloud" {
		t.Errorf("expected 'new context from cloud', got '%s'", content)
	}
	if syncStatus != string(SyncStatusSynced) {
		t.Errorf("expected 'synced', got '%s'", syncStatus)
	}
	if embedding != "[0.1,0.2,0.3]" {
		t.Errorf("expected '[0.1,0.2,0.3]', got '%s'", embedding)
	}
}
