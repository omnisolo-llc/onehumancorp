package hub_test

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
	_ "modernc.org/sqlite"
)

type MockRAGSyncService struct {
	Records []hub.RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	var pending []hub.RAGSyncRecord
	for _, r := range m.Records {
		if r.SyncStatus == hub.SyncStatusPending {
			pending = append(pending, r)
			if len(pending) == limit {
				break
			}
		}
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
			m.Records[i].SyncStatus = hub.SyncStatusSynced
			m.Records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	m.Records = append(m.Records, records...)
	return nil
}

func TestRAGSyncServiceFlow(t *testing.T) {
	service := &MockRAGSyncService{
		Records: []hub.RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: hub.SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: hub.SyncStatusPending},
			{ID: "3", Context: "test 3", SyncStatus: hub.SyncStatusSynced},
		},
	}

	ctx := context.Background()

	// 1. Fetch pending
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}

	// 2. Mark synced
	ids := []string{"1", "2"}
	err = service.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	// 3. Verify marked
	pendingAfter, _ := service.FetchPendingSyncs(ctx, 10)
	if len(pendingAfter) != 0 {
		t.Fatalf("Expected 0 pending records after sync, got %d", len(pendingAfter))
	}

	// 4. Process incoming
	newRecords := []hub.RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: hub.SyncStatusSynced},
	}
	err = service.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	if len(service.Records) != 4 {
		t.Fatalf("Expected 4 total records, got %d", len(service.Records))
	}
}

func setupSQLiteDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	_, err = db.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}
	return db
}

func TestStandaloneRAGSyncService(t *testing.T) {
	db := setupSQLiteDB(t)
	defer db.Close()
	service := hub.NewStandaloneRAGSyncService(db)
	ctx := context.Background()

	// 1. Setup initial state
	_, err := db.Exec(`
		INSERT INTO autodream_memories (id, content, sync_status) VALUES
		('1', 'test 1', 'pending'),
		('2', 'test 2', 'pending'),
		('3', 'test 3', 'synced');
	`)
	if err != nil {
		t.Fatalf("Failed to insert initial data: %v", err)
	}

	// 2. Fetch pending
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}

	// 3. Mark synced
	ids := []string{"1", "2"}
	err = service.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	// 4. Verify marked
	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("Expected 0 pending records after sync, got %d", len(pendingAfter))
	}

	// 5. Process incoming
	newRecords := []hub.RAGSyncRecord{
		{ID: "4", Context: "test 4", SyncStatus: hub.SyncStatusSynced},
		{ID: "1", Context: "test 1 updated", SyncStatus: hub.SyncStatusSynced},
	}
	err = service.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM autodream_memories").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to count rows: %v", err)
	}
	if count != 4 {
		t.Fatalf("Expected 4 total records, got %d", count)
	}

	var content string
	err = db.QueryRow("SELECT content FROM autodream_memories WHERE id = '1'").Scan(&content)
	if err != nil {
		t.Fatalf("Failed to select row: %v", err)
	}
	if content != "test 1 updated" {
		t.Fatalf("Expected updated content 'test 1 updated', got '%s'", content)
	}
}
