package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite" // pure Go SQLite driver
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test db: %v", err)
	}

	// Create tables
	queries := []string{
		`CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)`,
		`CREATE TABLE agent_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)`,
	}
	for _, q := range queries {
		if _, err := db.Exec(q); err != nil {
			t.Fatalf("failed to create table: %v", err)
		}
	}

	return db
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewDefaultRAGSyncService(db)

	ctx := context.Background()

	// Insert test data
	db.Exec("INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('1', 'test1', '[0.1, 0.2]', 'pending')")
	db.Exec("INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('2', 'test2', '[0.3, 0.4]', 'synced')")
	db.Exec("INSERT INTO agent_memories (id, content, embedding, sync_status) VALUES ('3', 'test3', '[0.5, 0.6]', 'pending')")

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}

	// Check record 1 (autodream)
	if records[0].ID != "1" || records[0].Type != MemoryTypeAutoDream || records[0].Context != "test1" || len(records[0].Vector) != 2 || records[0].Vector[0] != 0.1 {
		t.Errorf("unexpected record 1: %+v", records[0])
	}

	// Check record 2 (agent)
	if records[1].ID != "3" || records[1].Type != MemoryTypeAgent || records[1].Context != "test3" || len(records[1].Vector) != 2 || records[1].Vector[0] != 0.5 {
		t.Errorf("unexpected record 2: %+v", records[1])
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewDefaultRAGSyncService(db)

	ctx := context.Background()

	db.Exec("INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending')")
	db.Exec("INSERT INTO agent_memories (id, content, sync_status) VALUES ('2', 'test2', 'pending')")

	records := []RAGSyncRecord{
		{ID: "1", Type: MemoryTypeAutoDream},
		{ID: "2", Type: MemoryTypeAgent},
	}

	if err := svc.MarkSynced(ctx, records); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var status string
	var lastSync sql.NullTime
	if err := db.QueryRow("SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'").Scan(&status, &lastSync); err != nil {
		t.Fatalf("failed to query autodream_memories: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected autodream memory to be synced, got %s", status)
	}
	if !lastSync.Valid {
		t.Errorf("expected last_sync_at to be set")
	}

	if err := db.QueryRow("SELECT sync_status, last_sync_at FROM agent_memories WHERE id = '2'").Scan(&status, &lastSync); err != nil {
		t.Fatalf("failed to query agent_memories: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected agent memory to be synced, got %s", status)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewDefaultRAGSyncService(db)

	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Type:       MemoryTypeAutoDream,
			Context:    "new context 1",
			Vector:     []float32{0.7, 0.8},
			LastSyncAt: time.Now(),
		},
		{
			ID:         "2",
			Type:       MemoryTypeAgent,
			Context:    "new context 2",
			Vector:     []float32{0.9, 1.0},
			LastSyncAt: time.Now(),
		},
	}

	if err := svc.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var contextStr, embeddingStr string
	var status string
	if err := db.QueryRow("SELECT content, embedding, sync_status FROM autodream_memories WHERE id = '1'").Scan(&contextStr, &embeddingStr, &status); err != nil {
		t.Fatalf("failed to query autodream_memories: %v", err)
	}
	if contextStr != "new context 1" || embeddingStr != "[0.7,0.8]" || status != "synced" {
		t.Errorf("unexpected autodream memory: %s, %s, %s", contextStr, embeddingStr, status)
	}

	if err := db.QueryRow("SELECT content, embedding, sync_status FROM agent_memories WHERE id = '2'").Scan(&contextStr, &embeddingStr, &status); err != nil {
		t.Fatalf("failed to query agent_memories: %v", err)
	}
	if contextStr != "new context 2" || embeddingStr != "[0.9,1]" || status != "synced" {
		t.Errorf("unexpected agent memory: %s, %s, %s", contextStr, embeddingStr, status)
	}
}
