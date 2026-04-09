package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	ProcessedSyncs []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if len(m.PendingRecords) > limit {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.ProcessedSyncs = append(m.ProcessedSyncs, records...)
	return nil
}

func TestMockRAGSyncService(t *testing.T) {
	mockService := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.MarkedSynced) != 1 || mockService.MarkedSynced[0] != "1" {
		t.Fatalf("expected 1 marked synced with id '1'")
	}

	err = mockService.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "3", Context: "test3"},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(mockService.ProcessedSyncs) != 1 || mockService.ProcessedSyncs[0].ID != "3" {
		t.Fatalf("expected 1 processed sync with id '3'")
	}
}

func TestSqliteRAGSyncService(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE rag_memories (
			id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	service := NewRAGSyncService(db)

	// Insert test data
	_, err = db.Exec(`INSERT INTO rag_memories (id, context, vector) VALUES ('1', 'ctx1', '[0.1, 0.2]')`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	ctx := context.Background()
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "1" || pending[0].Context != "ctx1" || len(pending[0].Vector) != 2 {
		t.Fatalf("invalid record data: %+v", pending[0])
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pendingAfter) != 0 {
		t.Fatalf("expected 0 pending records after MarkSynced, got %d", len(pendingAfter))
	}

	var status string
	var lastSync time.Time
	err = db.QueryRow("SELECT sync_status, last_sync_at FROM rag_memories WHERE id = '1'").Scan(&status, &lastSync)
	if err != nil {
		t.Fatalf("failed to fetch status: %v", err)
	}
	if status != string(SyncStatusSynced) {
		t.Fatalf("expected status synced, got %s", status)
	}
	if lastSync.IsZero() {
		t.Fatalf("expected non-zero last_sync_at")
	}

	incoming := []RAGSyncRecord{
		{ID: "2", Context: "ctx2", Vector: []float32{0.3, 0.4}},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var vecStr string
	err = db.QueryRow("SELECT vector FROM rag_memories WHERE id = '2'").Scan(&vecStr)
	if err != nil {
		t.Fatalf("failed to fetch vector: %v", err)
	}
	if vecStr != "[0.3,0.4]" {
		t.Fatalf("expected [0.3,0.4], got %s", vecStr)
	}
}
