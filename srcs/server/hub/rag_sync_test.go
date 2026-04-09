package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

type mockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
		}
	}
	if len(pending) > limit {
		return pending[:limit], nil
	}
	return pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
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

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.records = append(m.records, records...)
	return nil
}

func TestRAGSyncService(t *testing.T) {
	mockService := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test3", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 2 {
		t.Errorf("Expected 2 pending records, got %d", len(pending))
	}

	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pendingAfter) != 1 {
		t.Errorf("Expected 1 pending record after marking one synced, got %d", len(pendingAfter))
	}
}

func TestNewRAGSyncMetrics(t *testing.T) {
	metrics, err := NewRAGSyncMetrics()
	if err != nil {
		t.Fatalf("NewRAGSyncMetrics failed: %v", err)
	}

	if metrics == nil {
		t.Fatal("Expected metrics object, got nil")
	}

	if metrics.RecordsSyncedTotal == nil {
		t.Error("Expected RecordsSyncedTotal to be initialized")
	}

	if metrics.SyncErrorsTotal == nil {
		t.Error("Expected SyncErrorsTotal to be initialized")
	}
}

func TestDefaultRAGSyncService(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open database: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	service := NewDefaultRAGSyncService(db)
	ctx := context.Background()

	_, err = db.Exec("INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'ctx1', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}
	_, err = db.Exec("INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('2', 'ctx2', 'synced')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pending) != 1 {
		t.Errorf("Expected 1 pending records, got %d", len(pending))
	}

	if pending[0].ID != "1" {
		t.Errorf("Expected pending ID 1, got %s", pending[0].ID)
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(pendingAfter) != 0 {
		t.Errorf("Expected 0 pending records after sync, got %d", len(pendingAfter))
	}

	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "3", Context: "ctx3", SyncStatus: SyncStatusSynced, LastSyncAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = '3'").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("Expected 1 record for ID 3, got %d", count)
	}
}
