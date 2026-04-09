package hub

import (
	"context"
	"database/sql"
	"errors"
	"testing"
	"time"

	_ "modernc.org/sqlite" // modernc sqlite driver for tests
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing.
type MockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
			if limit > 0 && len(pending) == limit {
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

	for i, r := range m.records {
		if idMap[r.ID] {
			m.records[i].SyncStatus = SyncStatusSynced
			m.records[i].LastSyncAt = time.Now()
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return errors.New("no records provided")
	}

	// Just append or upsert for mock
	m.records = append(m.records, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mock := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "Memory 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "Memory 2", SyncStatus: SyncStatusPending},
			{ID: "3", Context: "Memory 3", SyncStatus: SyncStatusSynced},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("Expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfterMark, _ := mock.FetchPendingSyncs(ctx, 10)
	if len(pendingAfterMark) != 1 {
		t.Errorf("Expected 1 pending record after mark, got %d", len(pendingAfterMark))
	}

	if mock.records[0].SyncStatus != SyncStatusSynced {
		t.Errorf("Expected record 1 to be synced")
	}

	// Test ProcessIncomingSync
	newRecords := []RAGSyncRecord{
		{ID: "4", Context: "Memory 4", SyncStatus: SyncStatusSynced},
	}
	err = mock.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	if len(mock.records) != 4 {
		t.Errorf("Expected 4 total records after processing incoming, got %d", len(mock.records))
	}
}

// Full integration test using an in-memory SQLite database
func TestSQLRAGSyncServiceIntegration(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}
	defer db.Close()

	// Create test schema
	_, err = db.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	// Insert some initial data
	_, err = db.Exec(`
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES
			('rec1', 'test context 1', 'pending'),
			('rec2', 'test context 2', 'pending'),
			('rec3', 'test context 3', 'synced')
	`)
	if err != nil {
		t.Fatalf("Failed to insert initial data: %v", err)
	}

	svc := New(db, true) // isSQLite = true
	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"rec1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced worked
	pendingAfter, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs after mark failed: %v", err)
	}
	if len(pendingAfter) != 1 {
		t.Fatalf("Expected 1 pending record after MarkSynced, got %d", len(pendingAfter))
	}
	if pendingAfter[0].ID != "rec2" {
		t.Errorf("Expected rec2 to be pending, got %s", pendingAfter[0].ID)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID:         "rec4",
			Context:    "test context 4",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			// Upsert existing
			ID:         "rec3",
			Context:    "updated context 3",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify ProcessIncomingSync results
	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM autodream_memories").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to count records: %v", err)
	}
	if count != 4 {
		t.Errorf("Expected 4 total records in DB, got %d", count)
	}

	var updatedContext string
	err = db.QueryRow("SELECT content FROM autodream_memories WHERE id = 'rec3'").Scan(&updatedContext)
	if err != nil {
		t.Fatalf("Failed to query updated record: %v", err)
	}
	if updatedContext != "updated context 3" {
		t.Errorf("Expected context to be updated, got %s", updatedContext)
	}
}
