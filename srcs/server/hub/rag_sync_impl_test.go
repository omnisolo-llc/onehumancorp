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
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_memories (
			id TEXT PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			content TEXT NOT NULL,
			embedding BLOB,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMPTZ NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider
}

func TestRAGSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	_, err := provider.Exec(ctx, `
		INSERT INTO agent_memories (id, organization_id, content, sync_status)
		VALUES ('1', 'org1', 'test content 1', 'pending'),
		       ('2', 'org1', 'test content 2', 'synced'),
		       ('3', 'org1', 'test content 3', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}
}

func TestRAGSyncServiceImpl_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	_, err := provider.Exec(ctx, `
		INSERT INTO agent_memories (id, organization_id, content, sync_status)
		VALUES ('1', 'org1', 'test content 1', 'pending'),
		       ('2', 'org1', 'test content 2', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT sync_status FROM agent_memories")
	if err != nil {
		t.Fatalf("failed to query sync_status: %v", err)
	}
	defer rows.Close()

	for rows.Next() {
		var status string
		if err := rows.Scan(&status); err != nil {
			t.Fatalf("failed to scan sync_status: %v", err)
		}
		if status != "synced" {
			t.Errorf("expected status 'synced', got %s", status)
		}
	}
}

func TestRAGSyncServiceImpl_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "test context 1",
			Vector:     []float32{0.1, 0.2, 0.3},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT content, sync_status FROM agent_memories WHERE id = '1'")
	if err != nil {
		t.Fatalf("failed to query agent_memories: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatalf("expected record to be inserted")
	}

	var content, status string
	if err := rows.Scan(&content, &status); err != nil {
		t.Fatalf("failed to scan record: %v", err)
	}

	if content != "test context 1" {
		t.Errorf("expected content 'test context 1', got %s", content)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}
}
