package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite in-memory db: %v", err)
	}

	_, err = sqliteDB.Exec(`
        CREATE TABLE consolidated_memory (
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

	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncService_ProcessIncoming(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	service := NewRAGSyncService(provider)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{ID: "3", OrgID: "org2", Context: "incoming 1", Vector: []float32{1.0, 2.0}},
		{ID: "4", OrgID: "org2", Context: "incoming 2", Vector: []float32{3.0, 4.0}},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error processing incoming: %v", err)
	}

	// Verify insertion
	rows, err := provider.Query(ctx, "SELECT id, sync_status FROM consolidated_memory WHERE organization_id = 'org2'")
	if err != nil {
		t.Fatalf("unexpected error querying db: %v", err)
	}
	defer rows.Close()

	count := 0
	for rows.Next() {
		var id string
		var status string
		if err := rows.Scan(&id, &status); err != nil {
			t.Fatalf("unexpected error scanning row: %v", err)
		}
		if status != string(SyncStatusSynced) {
			t.Errorf("expected status synced, got %s", status)
		}
		count++
	}

	if count != 2 {
		t.Errorf("expected 2 inserted records, got %d", count)
	}
}

func TestRAGSyncService_Flow(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	service := NewRAGSyncService(provider)
	ctx := context.Background()

	_, err := provider.Exec(ctx, `INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status) VALUES ($1, $2, $3, $4, $5)`, "1", "org1", "context 1", "test", "pending")
	if err != nil {
		t.Fatalf("failed to insert test record: %v", err)
	}
	_, err = provider.Exec(ctx, `INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status) VALUES ($1, $2, $3, $4, $5)`, "2", "org1", "context 2", "test", "pending")
	if err != nil {
		t.Fatalf("failed to insert test record: %v", err)
	}

	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	err = service.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("unexpected error marking synced: %v", err)
	}

	pending, err = service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending: %v", err)
	}
	if len(pending) != 0 {
		t.Errorf("expected 0 pending records after MarkSynced, got %d", len(pending))
	}
}
