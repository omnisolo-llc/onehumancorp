package hub

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
)

func TestHybridRAGSyncService(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = db.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('1', 'test context 1', 'pending'),
		       ('2', 'test context 2', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}

	service := NewHybridRAGSyncService(db)

	records, err := service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}

	err = service.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	records, err = service.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 remaining record, got %d", len(records))
	}
	if records[0].ID != "2" {
		t.Fatalf("expected record '2' to be remaining")
	}

	incoming := []RAGSyncRecord{
		{ID: "3", Context: "new context 3", Vector: []float32{0.1, 0.2}},
		{ID: "1", Context: "updated context 1", Vector: []float32{0.3, 0.4}},
	}
	err = service.ProcessIncomingSync(context.Background(), incoming)
	if err != nil {
		t.Fatalf("expected no error processing sync, got %v", err)
	}

	// Verify the data in the database
	var count int
	err = db.QueryRow("SELECT count(*) FROM swarm_memory_embeddings WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("expected no error checking db, got %v", err)
	}
	if count != 2 {
		t.Fatalf("expected 2 synced records, got %d", count)
	}
}
