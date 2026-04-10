package hub

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
	"database/sql"
)

type MockRAGSyncService struct {
	Records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.Records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
		}
	}
	if len(pending) > limit {
		pending = pending[:limit]
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}
	for i, r := range m.Records {
		if idMap[r.ID] {
			m.Records[i].SyncStatus = SyncStatusSynced
			m.Records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Records = append(m.Records, records...)
	return nil
}

func TestRAGSyncService_Mock(t *testing.T) {
	svc := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pendingAfter, _ := svc.FetchPendingSyncs(ctx, 10)
	if len(pendingAfter) != 0 {
		t.Fatalf("expected 0 pending records after marking synced, got %d", len(pendingAfter))
	}

	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "2", Context: "test2", SyncStatus: SyncStatusSynced},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(svc.Records) != 2 {
		t.Fatalf("expected 2 records after ProcessIncomingSync, got %d", len(svc.Records))
	}
}

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	provider := db.NewSqliteProvider(sqliteDB)

	// Create the schema manually for test
	_, err = provider.Exec(context.Background(), `
		CREATE TABLE consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding BLOB,
			source_type TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}
	return provider
}

func TestDBBridgeSyncService(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewDBBridgeSyncService(provider)
	ctx := context.Background()

	// 1. Process Incoming Sync
	records := []RAGSyncRecord{
		{ID: "incoming-1", Context: "incoming test", Vector: []byte{1, 2, 3}},
	}
	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify insertion
	var count int
	row := provider.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory")
	err = row.Scan(&count)
	if err != nil {
		t.Fatalf("failed to count: %v", err)
	}
	if count != 1 {
		t.Fatalf("expected 1 record, got %d", count)
	}

	// 2. Fetch pending (we just inserted with synced status)
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending, got %d", len(pending))
	}

	// 3. Manually insert pending
	_, err = provider.Exec(ctx, "INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status) VALUES ('pending-1', 'test_org', 'content', 'test', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert pending: %v", err)
	}

	pending2, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending2) != 1 {
		t.Fatalf("expected 1 pending, got %d", len(pending2))
	}

	// 4. Mark synced
	err = svc.MarkSynced(ctx, []string{"pending-1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	pending3, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending3) != 0 {
		t.Fatalf("expected 0 pending, got %d", len(pending3))
	}
}
