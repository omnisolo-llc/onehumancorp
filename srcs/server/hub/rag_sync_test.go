package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	_, err = sqliteDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	dbProvider := db.NewSqliteProvider(sqliteDB)
	service := NewRAGSyncService(dbProvider)

	ctx := context.Background()

	vec1, _ := json.Marshal([]float32{1.0, 2.0})
	vec2, _ := json.Marshal([]float32{3.0, 4.0})
	_, err = dbProvider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('1', 'test context 1', $1, 'pending')", vec1)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}
	_, err = dbProvider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ('2', 'test context 2', $1, 'pending')", vec2)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(records))
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 || records[0].ID != "2" {
		t.Fatalf("expected 1 pending record with ID 2, got %v", records)
	}

	incoming := []RAGSyncRecord{
		{ID: "3", Context: "test context 3", Vector: []float32{5.0, 6.0}},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var count int
	err = sqliteDB.QueryRow("SELECT COUNT(*) FROM swarm_memory_embeddings WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 2 {
		t.Fatalf("expected 2 synced records (1 marked, 1 incoming), got %d", count)
	}
}
