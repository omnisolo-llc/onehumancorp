package hub

import (
	"context"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	Records []RAGSyncRecord
	Marked  []string
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, r := range m.Records {
		if r.SyncStatus == SyncStatusPending {
			pending = append(pending, r)
			if len(pending) == limit {
				break
			}
		}
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.Marked = append(m.Marked, ids...)
	for i, r := range m.Records {
		for _, id := range ids {
			if r.ID == id {
				m.Records[i].SyncStatus = SyncStatusSynced
				m.Records[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		found := false
		for i, existing := range m.Records {
			if existing.ID == r.ID {
				m.Records[i] = r
				found = true
				break
			}
		}
		if !found {
			m.Records = append(m.Records, r)
		}
	}
	return nil
}

func TestRAGSyncServiceFlow(t *testing.T) {
	ctx := context.Background()
	mockService := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusSynced},
			{ID: "3", Context: "test 3", SyncStatus: SyncStatusPending},
		},
	}

	// 1. FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// 2. ProcessIncomingSync (simulating pushing to cloud)
	err = mockService.ProcessIncomingSync(ctx, pending)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// 3. MarkSynced (simulating response from cloud back to local)
	idsToMark := []string{}
	for _, r := range pending {
		idsToMark = append(idsToMark, r.ID)
	}
	err = mockService.MarkSynced(ctx, idsToMark)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify they are marked
	if len(mockService.Marked) != 2 {
		t.Fatalf("expected 2 marked records, got %d", len(mockService.Marked))
	}

	// Verify no pending left
	pendingAgain, _ := mockService.FetchPendingSyncs(ctx, 10)
	if len(pendingAgain) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(pendingAgain))
	}
}
