package hub

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}

	createTableQuery := `
	CREATE TABLE swarm_memory_embeddings (
		memory_id        TEXT PRIMARY KEY,
		context          TEXT NOT NULL,
		vector_embedding BLOB,
		sync_status      TEXT DEFAULT 'pending',
		last_sync_at     TIMESTAMP NULL
	);
	`
	if _, err := db.Exec(createTableQuery); err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db
}

func TestFetchPendingSyncs(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	service := NewRAGSyncService(db)

	_, err := db.Exec(`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('1', 'test1', '[]', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}
	_, err = db.Exec(`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('2', 'test2', '[]', 'synced')`)
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("expected ID '1', got %s", records[0].ID)
	}
}

func TestMarkSynced(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	service := NewRAGSyncService(db)

	_, err := db.Exec(`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('1', 'test1', '[]', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert mock data: %v", err)
	}

	err = service.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var status string
	err = db.QueryRow(`SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'`).Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got %s", status)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	service := NewRAGSyncService(db)

	records := []RAGSyncRecord{
		{
			ID:         "3",
			Context:    "test3",
			Vector:     []float32{0.1, 0.2},
			SyncStatus: SyncStatusPending,
		},
	}

	err := service.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var count int
	err = db.QueryRow(`SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = '3'`).Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 record, got %d", count)
	}

	// Test update on conflict
	records[0].Context = "updated3"
	err = service.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error on update: %v", err)
	}

	var contextStr string
	var status string
	err = db.QueryRow(`SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = '3'`).Scan(&contextStr, &status)
	if err != nil {
		t.Fatalf("failed to query updated record: %v", err)
	}

	if contextStr != "updated3" {
		t.Errorf("expected context 'updated3', got %s", contextStr)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("expected status 'synced', got %s", status)
	}
}
