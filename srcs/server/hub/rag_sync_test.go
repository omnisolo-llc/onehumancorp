package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"reflect"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) (db.Provider, *sql.DB) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite DB: %v", err)
	}

	_, err = sqlDB.Exec(`
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	return provider, sqlDB
}

func TestFetchPendingSyncs(t *testing.T) {
	provider, sqlDB := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()

	// Insert mock data
	sqlDB.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'content1', 'pending')")
	sqlDB.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('2', 'content2', 'synced')")
	sqlDB.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('3', 'content3', 'pending')")

	service := NewHybridRAGSyncService(provider)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}

	// Test limit
	records, err = service.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Errorf("expected 1 record, got %d", len(records))
	}
}

func TestMarkSynced(t *testing.T) {
	provider, sqlDB := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()

	sqlDB.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'content1', 'pending')")

	service := NewHybridRAGSyncService(provider)

	err := service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var status string
	err = sqlDB.QueryRow("SELECT sync_status FROM autodream_memories WHERE id = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status synced, got %s", status)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	provider, sqlDB := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()

	// Existing record to be updated
	sqlDB.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'old_content', 'synced')")

	service := NewHybridRAGSyncService(provider)

	now := time.Now()

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Content:    "new_content",
			Vector:     []float32{1.1, 2.2},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: &now,
		},
		{
			ID:         "2",
			Content:    "content2",
			Vector:     nil,
			SyncStatus: SyncStatusPending,
			LastSyncAt: nil,
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify record 1 was updated
	var content1 string
	var vectorStr1 *string
	err = sqlDB.QueryRow("SELECT content, embedding FROM autodream_memories WHERE id = '1'").Scan(&content1, &vectorStr1)
	if err != nil {
		t.Fatalf("failed to query record 1: %v", err)
	}
	if content1 != "new_content" {
		t.Errorf("expected new_content, got %s", content1)
	}

	var vec1 []float32
	if err := json.Unmarshal([]byte(*vectorStr1), &vec1); err != nil {
		t.Fatalf("failed to unmarshal vector: %v", err)
	}
	if !reflect.DeepEqual(vec1, []float32{1.1, 2.2}) {
		t.Errorf("expected vector [1.1, 2.2], got %v", vec1)
	}

	// Verify record 2 was inserted
	var content2 string
	var vectorStr2 *string
	var status2 string
	err = sqlDB.QueryRow("SELECT content, embedding, sync_status FROM autodream_memories WHERE id = '2'").Scan(&content2, &vectorStr2, &status2)
	if err != nil {
		t.Fatalf("failed to query record 2: %v", err)
	}
	if content2 != "content2" {
		t.Errorf("expected content2, got %s", content2)
	}
	if vectorStr2 != nil {
		t.Errorf("expected nil vector, got %v", vectorStr2)
	}
	if status2 != "pending" {
		t.Errorf("expected pending, got %s", status2)
	}
}
