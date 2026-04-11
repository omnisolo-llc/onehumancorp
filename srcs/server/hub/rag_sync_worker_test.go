package hub

import (
	"context"
	"testing"
	"time"
)

// MockRAGSyncService is a mock implementation of RAGSyncService
type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedIDs      []string
	FetchErr       error
	MarkErr        error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchErr != nil {
		return nil, m.FetchErr
	}
	if len(m.PendingRecords) > limit {
		return m.PendingRecords[:limit], nil
	}
	return m.PendingRecords, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkErr != nil {
		return m.MarkErr
	}
	m.MarkedIDs = append(m.MarkedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	return nil
}

func TestSyncDaemon(t *testing.T) {
	mockSvc := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{
			{ID: "record1"},
			{ID: "record2"},
		},
	}
	daemon := NewSyncDaemon(mockSvc, 10*time.Millisecond)

	ctx, cancel := context.WithCancel(context.Background())

	// Start daemon in a goroutine
	go daemon.Start(ctx)

	// Wait for the ticker to fire a few times
	time.Sleep(50 * time.Millisecond)
	cancel() // Stop the daemon

	// Give it a tiny bit of time to gracefully shut down
	time.Sleep(10 * time.Millisecond)

	if len(mockSvc.MarkedIDs) == 0 {
		t.Errorf("Expected marked IDs, got none")
	}

	found1 := false
	found2 := false
	for _, id := range mockSvc.MarkedIDs {
		if id == "record1" {
			found1 = true
		}
		if id == "record2" {
			found2 = true
		}
	}

	if !found1 || !found2 {
		t.Errorf("Expected both records to be marked synced, marked IDs: %v", mockSvc.MarkedIDs)
	}
}
