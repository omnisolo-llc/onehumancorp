package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestDBRAGSyncService(t *testing.T) {
	tmpFile, err := os.CreateTemp("", "testdb-*.sqlite")
	if err != nil {
		t.Fatalf("failed to create temp db: %v", err)
	}
	defer os.Remove(tmpFile.Name())

	dbConn, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	_, err = dbConn.Exec(`
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

	provider := db.NewSqliteProvider(dbConn)
	service := NewRAGSyncService(provider)

	ctx := context.Background()

	// Insert test data
	_, err = dbConn.Exec("INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding) VALUES (?, ?, ?)", "1", "ctx1", []byte("v1"))
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Fatalf("expected id 1, got %s", records[0].ID)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:      "2",
			Context: "ctx2",
			Vector:  []byte("v2"),
		},
		{
			ID:      "1",
			Context: "ctx1_updated",
			Vector:  []byte("v1_updated"),
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	var count int
	err = dbConn.QueryRow("SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
	if err != nil {
		t.Fatalf("query count error: %v", err)
	}
	if count != 2 {
		t.Fatalf("expected 2 total records, got %d", count)
	}

	var ctxStr string
	err = dbConn.QueryRow("SELECT context FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&ctxStr)
	if err != nil {
		t.Fatalf("query row error: %v", err)
	}
	if ctxStr != "ctx1_updated" {
		t.Fatalf("expected ctx1_updated, got %s", ctxStr)
	}
}
