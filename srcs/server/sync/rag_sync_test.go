package sync

import (
	"context"
	"database/sql"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	tmpFile, err := os.CreateTemp("", "testdb-rag-sync-*.sqlite")
	if err != nil {
		t.Fatalf("failed to create temp db: %v", err)
	}
	defer os.Remove(tmpFile.Name())
	defer tmpFile.Close()

	sqlDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)

	// Ensure we set up tables and PRAGMAs required for SQLite vector / structure
	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_memories (
			id TEXT PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := NewRAGSyncService(provider)

	// Insert test data
	_, err = provider.Exec(ctx, `INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status) VALUES (?, ?, ?, ?, ?)`, "id1", "tenant-1", "content1", "[0.1,0.2,0.3]", "pending")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	// Test FetchPendingSyncs
	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "id1" || records[0].SyncStatus != SyncStatusPending || records[0].OrganizationID != "tenant-1" {
		t.Errorf("unexpected record data: %+v", records[0])
	}
	if len(records[0].Vector) != 3 || records[0].Vector[0] != 0.1 {
		t.Errorf("unexpected vector: %+v", records[0].Vector)
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"id1"})
	if err != nil {
		t.Fatalf("expected no error marking synced, got %v", err)
	}

	records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(records))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "id2", OrganizationID: "tenant-2", Context: "content2", Vector: []float32{0.4, 0.5}},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("expected no error processing incoming sync, got %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT content, sync_status, organization_id, embedding FROM agent_memories WHERE id = ?", "id2")
	if err != nil {
		t.Fatalf("expected no error querying id2, got %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatalf("expected to find id2 in db")
	}
	var content string
	var status string
	var orgID string
	var embedding string
	if err := rows.Scan(&content, &status, &orgID, &embedding); err != nil {
		t.Fatalf("failed to scan row: %v", err)
	}
	if content != "content2" || status != "synced" || orgID != "tenant-2" || embedding != "[0.400000,0.500000]" {
		t.Errorf("unexpected data for id2: %s %s %s %s", content, status, orgID, embedding)
	}
}
