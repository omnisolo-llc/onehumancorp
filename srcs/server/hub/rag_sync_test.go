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
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	_, err = db.Exec(`
    CREATE TABLE swarm_memory_embeddings (
        memory_id TEXT PRIMARY KEY,
        context TEXT NOT NULL,
        vector_embedding BLOB,
        sync_status VARCHAR(50) DEFAULT 'pending',
        last_sync_at DATETIME NULL
    );
    `)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	return db
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	db.Exec("INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')")
	db.Exec("INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('2', 'ctx2', 'synced')")

	svc := NewSQLRAGSyncService(db)
	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("failed to fetch pending syncs: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", records[0].ID)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	db.Exec("INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')")

	svc := NewSQLRAGSyncService(db)
	err := svc.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("failed to mark synced: %v", err)
	}

	var status string
	err = db.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected synced status, got %s", status)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	svc := NewSQLRAGSyncService(db)
	records := []RAGSyncRecord{
		{ID: "1", Context: "new_ctx"},
	}
	err := svc.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("failed to process incoming sync: %v", err)
	}

	var contextStr string
	var status string
	err = db.QueryRow("SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&contextStr, &status)
	if err != nil {
		t.Fatalf("failed to query memory: %v", err)
	}
	if contextStr != "new_ctx" {
		t.Errorf("expected new_ctx, got %s", contextStr)
	}
	if status != "synced" {
		t.Errorf("expected synced status, got %s", status)
	}
}
