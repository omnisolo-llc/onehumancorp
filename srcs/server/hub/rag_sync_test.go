package hub_test

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}
	defer sqliteDB.Close()

	provider := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}

	ctx := context.Background()

	// Setup schema
	createTableQuery := `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding TEXT,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMPTZ NULL
		);
	`
	_, err = provider.Exec(ctx, createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	service, err := hub.NewRAGSyncService(provider)
	if err != nil {
		t.Fatalf("Failed to create sync service: %v", err)
	}

	// Insert test data
	insertQuery := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ($1, $2, $3, $4)
	`
	_, err = provider.Exec(ctx, insertQuery, "id1", "context 1", "[0.1, 0.2]", "pending")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}
	_, err = provider.Exec(ctx, insertQuery, "id2", "context 2", "[0.3, 0.4]", "synced")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(records))
	}
	if records[0].ID != "id1" || records[0].Context != "context 1" {
		t.Errorf("Unexpected record content: %+v", records[0])
	}
	if len(records[0].Vector) != 2 || records[0].Vector[0] != 0.1 {
		t.Errorf("Unexpected vector content: %+v", records[0].Vector)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"id1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced
	var status string
	var lastSyncAtStr sql.NullString
	err = sqliteDB.QueryRow("SELECT sync_status, last_sync_at FROM swarm_memory_embeddings WHERE memory_id = 'id1'").Scan(&status, &lastSyncAtStr)
	if err != nil {
		t.Fatalf("QueryRow failed: %v", err)
	}
	if status != "synced" {
		t.Errorf("Expected status synced, got %s", status)
	}
	if !lastSyncAtStr.Valid || lastSyncAtStr.String == "" {
		t.Errorf("Expected last_sync_at to be set, got empty")
	}

	// Test ProcessIncomingSync
	incomingRecords := []hub.RAGSyncRecord{
		{
			ID:      "id3",
			Context: "context 3",
			Vector:  []float32{0.5, 0.6},
		},
		{
			ID:      "id1", // test upsert
			Context: "context 1 updated",
			Vector:  []float32{0.9, 0.9},
		},
	}

	err = service.ProcessIncomingSync(ctx, incomingRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify ProcessIncomingSync
	var contextStr, vectorStr string
	err = sqliteDB.QueryRow("SELECT context, vector_embedding, sync_status FROM swarm_memory_embeddings WHERE memory_id = 'id3'").Scan(&contextStr, &vectorStr, &status)
	if err != nil {
		t.Fatalf("QueryRow failed for id3: %v", err)
	}
	if status != "synced" {
		t.Errorf("Expected status synced for id3, got %s", status)
	}
	if contextStr != "context 3" || vectorStr != "[0.5,0.6]" {
		t.Errorf("Unexpected content for id3: %s, %s", contextStr, vectorStr)
	}

	err = sqliteDB.QueryRow("SELECT context, vector_embedding, sync_status FROM swarm_memory_embeddings WHERE memory_id = 'id1'").Scan(&contextStr, &vectorStr, &status)
	if err != nil {
		t.Fatalf("QueryRow failed for id1: %v", err)
	}
	if status != "synced" {
		t.Errorf("Expected status synced for id1, got %s", status)
	}
	if contextStr != "context 1 updated" || vectorStr != "[0.9,0.9]" {
		t.Errorf("Unexpected content for id1: %s, %s", contextStr, vectorStr)
	}
}
