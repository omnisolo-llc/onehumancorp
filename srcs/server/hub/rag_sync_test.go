package hub

import (
	"context"
	"database/sql"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	tmpFile, err := os.CreateTemp("", "test-db-*.sqlite")
	if err != nil {
		t.Fatalf("Failed to create temp db file: %v", err)
	}
	defer os.Remove(tmpFile.Name())

	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Setup schema
	_, err = provider.Exec(ctx, `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	service := NewDefaultRAGSyncService(provider)

	// Insert test data
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES (?, ?, ?, ?)", "id1", "context1", []byte{1, 2, 3}, "pending")
	if err != nil {
		t.Fatalf("Failed to insert data: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 1 {
		t.Errorf("Expected 1 pending record, got %d", len(records))
	}
	if records[0].ID != "id1" || records[0].Context != "context1" || records[0].SyncStatus != SyncStatusPending {
		t.Errorf("Record mismatch: %+v", records[0])
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"id1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	records, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(records) != 0 {
		t.Errorf("Expected 0 pending records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:      "id2",
			Context: "context2",
			Vector:  []byte{4, 5, 6},
		},
		{
			ID:      "id1", // test upsert
			Context: "context1_updated",
			Vector:  []byte{1, 2, 3},
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify incoming records
	rows, err := provider.Query(ctx, "SELECT memory_id, context, sync_status FROM swarm_memory_embeddings WHERE memory_id IN (?, ?) ORDER BY memory_id", "id1", "id2")
	if err != nil {
		t.Fatalf("Query failed: %v", err)
	}
	defer rows.Close()

	var result []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var status string
		if err := rows.Scan(&rec.ID, &rec.Context, &status); err != nil {
			t.Fatalf("Scan failed: %v", err)
		}
		rec.SyncStatus = SyncStatus(status)
		result = append(result, rec)
	}

	if len(result) != 2 {
		t.Errorf("Expected 2 records, got %d", len(result))
	}
	if result[0].ID != "id1" || result[0].Context != "context1_updated" || result[0].SyncStatus != SyncStatusSynced {
		t.Errorf("Record mismatch for id1: %+v", result[0])
	}
	if result[1].ID != "id2" || result[1].Context != "context2" || result[1].SyncStatus != SyncStatusSynced {
		t.Errorf("Record mismatch for id2: %+v", result[1])
	}
}
