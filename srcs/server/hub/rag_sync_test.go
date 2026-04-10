package hub_test

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	ctx := context.Background()
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()

	provider := db.NewSqliteProvider(sqliteDB)

	// Create consolidated_memory table
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert initial records
	_, err = provider.Exec(ctx, `INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status) VALUES ('1', 'org', 'context 1', '[1.1, 2.2]', 'test', 'pending')`)
	if err != nil { t.Fatalf("failed to insert: %v", err) }
	_, err = provider.Exec(ctx, `INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status) VALUES ('2', 'org', 'context 2', null, 'test', 'pending')`)
	if err != nil { t.Fatalf("failed to insert: %v", err) }
	_, err = provider.Exec(ctx, `INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status) VALUES ('3', 'org', 'context 3', null, 'test', 'synced')`)
	if err != nil { t.Fatalf("failed to insert: %v", err) }

	service := hub.NewRAGSyncService(provider)

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	hasVector := false
	for _, p := range pending {
		if p.ID == "1" {
			if len(p.Vector) != 2 || p.Vector[0] != 1.1 || p.Vector[1] != 2.2 {
				t.Fatalf("vector parsing failed for ID 1: %v", p.Vector)
			}
			hasVector = true
		}
	}
	if !hasVector {
		t.Fatalf("did not find parsed vector")
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pending, _ = service.FetchPendingSyncs(ctx, 10)
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record after sync, got %d", len(pending))
	}
	if pending[0].ID != "2" {
		t.Fatalf("expected remaining pending to be ID 2, got %s", pending[0].ID)
	}

	// Test ProcessIncomingSync
	newRecords := []hub.RAGSyncRecord{
		{
			ID:         "4",
			Context:    "new context from cloud",
			Vector:     []float32{3.3, 4.4},
			SyncStatus: hub.SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID:         "1", // Test upsert (ON CONFLICT)
			Context:    "updated context from cloud",
			Vector:     []float32{5.5, 6.6},
			SyncStatus: hub.SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify "4" was added
	row := provider.QueryRow(ctx, "SELECT content, sync_status FROM consolidated_memory WHERE id = '4'")
	var content, status string
	if err := row.Scan(&content, &status); err != nil {
		t.Fatalf("failed to query new record 4: %v", err)
	}
	if content != "new context from cloud" || status != "synced" {
		t.Fatalf("record 4 has wrong data: %s, %s", content, status)
	}

	// Verify "1" was updated
	row = provider.QueryRow(ctx, "SELECT content, sync_status FROM consolidated_memory WHERE id = '1'")
	if err := row.Scan(&content, &status); err != nil {
		t.Fatalf("failed to query updated record 1: %v", err)
	}
	if content != "updated context from cloud" || status != "synced" {
		t.Fatalf("record 1 has wrong data: %s, %s", content, status)
	}
}
