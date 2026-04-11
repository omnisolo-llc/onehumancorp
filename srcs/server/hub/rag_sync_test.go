package hub_test

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) (*sql.DB, db.Provider) {
	// Use an in-memory SQLite database with shared cache for testing
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite test db: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)

	ctx := context.Background()

	// Ensure the table doesn't already exist from another test in the same process
	_, err = provider.Exec(ctx, `DROP TABLE IF EXISTS autodream_memories`)
	if err != nil {
		t.Fatalf("failed to drop table: %v", err)
	}

	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	return sqlDB, provider
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	sqlDB, provider := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()
	service := hub.NewRAGSyncService(provider)

	// Insert some test data
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES
			('msg1', 'test content 1', '[0.1, 0.2]', 'pending'),
			('msg2', 'test content 2', NULL, 'synced'),
			('msg3', 'test content 3', '[0.3, 0.4]', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(records))
	}

	if records[0].ID != "msg1" || records[1].ID != "msg3" {
		t.Errorf("unexpected record IDs: %+v", records)
	}

	if *records[0].Vector != "[0.1, 0.2]" {
		t.Errorf("expected vector '[0.1, 0.2]', got '%v'", *records[0].Vector)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	sqlDB, provider := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()
	service := hub.NewRAGSyncService(provider)

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('msg1', 'test content', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"msg1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify update
	row := provider.QueryRow(ctx, `SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = 'msg1'`)
	var status string
	var lastSyncAt *time.Time
	err = row.Scan(&status, &lastSyncAt)
	if err != nil {
		t.Fatalf("failed to query updated record: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
	if lastSyncAt == nil {
		t.Error("expected last_sync_at to be set, got nil")
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	sqlDB, provider := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()
	service := hub.NewRAGSyncService(provider)

	vec := "[0.5, 0.6]"
	records := []hub.RAGSyncRecord{
		{
			ID:         "msg_new",
			Context:    "new context",
			Vector:     &vec,
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify insertion
	row := provider.QueryRow(ctx, `SELECT content, embedding, sync_status FROM autodream_memories WHERE id = 'msg_new'`)
	var content, status string
	var embedding *string
	err = row.Scan(&content, &embedding, &status)
	if err != nil {
		t.Fatalf("failed to query inserted record: %v", err)
	}

	if content != "new context" {
		t.Errorf("expected content 'new context', got '%s'", content)
	}
	if embedding == nil || *embedding != vec {
		t.Errorf("expected embedding '%s', got '%v'", vec, embedding)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}

	// Verify update on conflict
	vec2 := "[0.7, 0.8]"
	records[0].Context = "updated context"
	records[0].Vector = &vec2

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync on conflict failed: %v", err)
	}

	row = provider.QueryRow(ctx, `SELECT content, embedding FROM autodream_memories WHERE id = 'msg_new'`)
	err = row.Scan(&content, &embedding)
	if err != nil {
		t.Fatalf("failed to query updated record: %v", err)
	}

	if content != "updated context" {
		t.Errorf("expected content 'updated context', got '%s'", content)
	}
	if embedding == nil || *embedding != vec2 {
		t.Errorf("expected embedding '%s', got '%v'", vec2, embedding)
	}
}
