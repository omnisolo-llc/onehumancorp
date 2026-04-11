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

func TestDefaultRAGSyncService(t *testing.T) {
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	_, err = dbConn.Exec(`CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		embedding BLOB,
		source_type TEXT NOT NULL,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)

	// Insert test data
	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status)
		VALUES ('1', 'org1', 'test content', 'X''010203''', 'test', 'pending')
	`)
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	svc := hub.NewDefaultRAGSyncService(provider)

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Fatalf("expected ID 1, got %s", records[0].ID)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify synced
	var status string
	err = provider.QueryRow(ctx, "SELECT sync_status FROM consolidated_memory WHERE id = '1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != string(hub.SyncStatusSynced) {
		t.Fatalf("expected synced status, got %s", status)
	}

	// Test ProcessIncomingSync
	incoming := []hub.RAGSyncRecord{
		{
			ID:         "2",
			Context:    "incoming context",
			Vector:     []byte{4, 5, 6},
			SyncStatus: hub.SyncStatusSynced,
			LastSyncAt: time.Now(),
			OrganizationID: "org2",
		},
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory WHERE id = '2'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected record 2 to be inserted")
	}
}
