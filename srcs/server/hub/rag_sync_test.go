package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"reflect"
	"testing"
	"time"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) (*sql.DB, db.Provider) {
	// Use in-memory SQLite database for testing
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite database: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	// Setup schema
	ctx := context.Background()
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
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return sqliteDB, provider
}

func TestFetchPendingSyncs(t *testing.T) {
	sqliteDB, provider := setupTestDB(t)
	defer sqliteDB.Close()

	ctx := context.Background()
	service := NewDefaultRAGSyncService(provider)

	// Insert dummy records
	vector := []float32{1.1, 2.2, 3.3}
	vectorJSON, _ := json.Marshal(vector)

	_, err := provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('req-1', 'hello', $1, 'pending')`, string(vectorJSON))
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	_, err = provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('req-2', 'world', $1, 'synced')`, string(vectorJSON))
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	// Fetch pending
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "req-1" {
		t.Errorf("expected ID req-1, got %s", records[0].ID)
	}
	if records[0].Context != "hello" {
		t.Errorf("expected Context hello, got %s", records[0].Context)
	}
	if records[0].SyncStatus != SyncStatusPending {
		t.Errorf("expected SyncStatus pending, got %s", records[0].SyncStatus)
	}
	if !reflect.DeepEqual(records[0].Vector, vector) {
		t.Errorf("expected vector %v, got %v", vector, records[0].Vector)
	}
}

func TestMarkSynced(t *testing.T) {
	sqliteDB, provider := setupTestDB(t)
	defer sqliteDB.Close()

	ctx := context.Background()
	service := NewDefaultRAGSyncService(provider)

	// Insert dummy record
	_, err := provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('req-1', 'hello', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	// Mark as synced
	err = service.MarkSynced(ctx, []string{"req-1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify in DB
	row := provider.QueryRow(ctx, `SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = 'req-1'`)
	var status string
	var lastSyncAt *time.Time
	if err := row.Scan(&status, &lastSyncAt); err != nil {
		t.Fatalf("failed to fetch updated record: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected sync_status synced, got %s", status)
	}
	if lastSyncAt == nil {
		t.Errorf("expected last_sync_at to be set, but it was nil")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	sqliteDB, provider := setupTestDB(t)
	defer sqliteDB.Close()

	ctx := context.Background()
	service := NewDefaultRAGSyncService(provider)

	now := time.Now().UTC()
	vector := []float32{4.4, 5.5, 6.6}
	record := RAGSyncRecord{
		ID:         "req-incoming",
		Context:    "remote context",
		Vector:     vector,
		SyncStatus: SyncStatusSynced,
		LastSyncAt: now,
	}

	// Process incoming
	err := service.ProcessIncomingSync(ctx, []RAGSyncRecord{record})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify in DB
	row := provider.QueryRow(ctx, `SELECT content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE id = 'req-incoming'`)
	var content string
	var embeddingStr *string
	var status string
	var lastSyncAt time.Time

	if err := row.Scan(&content, &embeddingStr, &status, &lastSyncAt); err != nil {
		t.Fatalf("failed to fetch incoming record: %v", err)
	}

	if content != "remote context" {
		t.Errorf("expected context remote context, got %s", content)
	}
	if status != "synced" {
		t.Errorf("expected sync_status synced, got %s", status)
	}
	// Note: sqlite may format the timestamp differently, just check it is not zero
	if lastSyncAt.IsZero() {
		t.Errorf("expected last_sync_at to be valid")
	}

	var savedVector []float32
	if err := json.Unmarshal([]byte(*embeddingStr), &savedVector); err != nil {
		t.Fatalf("failed to unmarshal saved vector: %v", err)
	}

	if !reflect.DeepEqual(savedVector, vector) {
		t.Errorf("expected vector %v, got %v", vector, savedVector)
	}
}
