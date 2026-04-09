package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *db.DB {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	// Create table matching the schema for tests
	_, err = sqliteDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	return &db.DB{Provider: provider}
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	database := setupTestDB(t)
	service := NewRAGSyncService(database)
	ctx := context.Background()

	// Insert test data
	_, err := database.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('1', 'Test Context 1', 'pending'),
		       ('2', 'Test Context 2', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" && records[0].ID != "2" {
		t.Errorf("unexpected ID '%s'", records[0].ID)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	database := setupTestDB(t)
	service := NewRAGSyncService(database)
	ctx := context.Background()

	// Insert test data
	_, err := database.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('1', 'Test Context 1', 'pending'),
		       ('2', 'Test Context 2', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify update
	var status string
	err = database.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected 'synced', got '%s'", status)
	}

	err = database.QueryRow(ctx, "SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = '2'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != "pending" {
		t.Errorf("expected 'pending', got '%s'", status)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	database := setupTestDB(t)
	service := NewRAGSyncService(database)
	ctx := context.Background()

	recordsToProcess := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "Processed Context 1",
			Vector:     []float32{0.1, 0.2},
			SyncStatus: SyncStatusPending,
			LastSyncAt: time.Now(),
		},
	}

	err := service.ProcessIncomingSync(ctx, recordsToProcess)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify insert
	var contextStr string
	err = database.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = '1'").Scan(&contextStr)
	if err != nil {
		t.Fatalf("failed to query context: %v", err)
	}
	if contextStr != "Processed Context 1" {
		t.Errorf("expected 'Processed Context 1', got '%s'", contextStr)
	}
}
