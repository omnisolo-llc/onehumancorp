package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	t.Helper()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := sqlDB.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	ctx := context.Background()
	provider := db.NewSqliteProvider(sqlDB)

	_, err = provider.Exec(ctx, `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMPTZ NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES
			('1', 'pending context 1', '[0.1, 0.2]', 'pending'),
			('2', 'synced context 2', '[0.3, 0.4]', 'synced'),
			('3', 'pending context 3', '[0.5, 0.6]', 'pending');
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	return provider
}

func TestRAGSyncServiceImpl_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	for _, r := range records {
		if r.ID == "1" {
			if len(r.Vector) != 2 || r.Vector[0] != 0.1 || r.Vector[1] != 0.2 {
				t.Errorf("unexpected vector for record 1: %v", r.Vector)
			}
		}
	}
}

func TestRAGSyncServiceImpl_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	err := svc.MarkSynced(ctx, []string{"1", "3"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(records))
	}
}

func TestRAGSyncServiceImpl_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	svc := NewRAGSyncService(provider)
	ctx := context.Background()

	incoming := []RAGSyncRecord{
		{
			ID:      "4",
			Context: "new incoming sync",
			Vector:  []float32{0.1, 0.2, 0.3},
		},
		{
			ID:      "1", // Update existing
			Context: "updated context 1",
			Vector:  []float32{0.9, 0.8},
		},
	}

	err := svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT content, embedding, sync_status FROM autodream_memories WHERE id = '4'")
	var content, syncStatus, embedding string
	err = row.Scan(&content, &embedding, &syncStatus)
	if err != nil {
		t.Fatalf("failed to verify insert: %v", err)
	}
	if content != "new incoming sync" || syncStatus != "synced" {
		t.Errorf("unexpected values for inserted record: %s, %s", content, syncStatus)
	}
	if embedding != "[0.100000,0.200000,0.300000]" {
		t.Errorf("unexpected vector string for inserted record: %s", embedding)
	}

	row = provider.QueryRow(ctx, "SELECT content, embedding, sync_status FROM autodream_memories WHERE id = '1'")
	err = row.Scan(&content, &embedding, &syncStatus)
	if err != nil {
		t.Fatalf("failed to verify update: %v", err)
	}
	if content != "updated context 1" || syncStatus != "synced" {
		t.Errorf("unexpected values for updated record: %s, %s", content, syncStatus)
	}
	if embedding != "[0.900000,0.800000]" {
		t.Errorf("unexpected vector string for updated record: %s", embedding)
	}
}
