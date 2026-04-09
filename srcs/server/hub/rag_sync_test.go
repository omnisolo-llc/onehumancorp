package hub

import (
	"context"
	"errors"
	"testing"
)

// MockRAGSyncService is a mock implementation of RAGSyncService for testing
type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	MarkedSynced   []string
	IncomingSyncs  []RAGSyncRecord
	FetchErr       error
	MarkErr        error
	ProcessErr     error
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.FetchErr != nil {
		RecordSyncError(ctx)
		return nil, m.FetchErr
	}
	res := m.PendingRecords
	if len(res) > limit {
		res = res[:limit]
	}
	return res, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.MarkErr != nil {
		RecordSyncError(ctx)
		return m.MarkErr
	}
	m.MarkedSynced = append(m.MarkedSynced, ids...)
	RecordSyncSuccess(ctx, int64(len(ids)))
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.ProcessErr != nil {
		RecordSyncError(ctx)
		return m.ProcessErr
	}
	m.IncomingSyncs = append(m.IncomingSyncs, records...)
	RecordSyncSuccess(ctx, int64(len(records)))
	return nil
}

func TestRAGSyncServiceFlow(t *testing.T) {
	ctx := context.Background()

	// 1. Standalone client has pending records
	pendingRecord := RAGSyncRecord{
		ID:         "mem_123",
		Context:    "Important context",
		Vector:     []float32{0.1, 0.2, 0.3},
		SyncStatus: SyncStatusPending,
	}

	standaloneMock := &MockRAGSyncService{
		PendingRecords: []RAGSyncRecord{pendingRecord},
	}

	// Fetch pending records on standalone
	records, err := standaloneMock.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending syncs: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "mem_123" {
		t.Errorf("expected record ID mem_123, got %s", records[0].ID)
	}

	// 2. Transmit to Cloud
	cloudMock := &MockRAGSyncService{}

	// Cloud processes incoming records
	err = cloudMock.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error processing incoming syncs: %v", err)
	}
	if len(cloudMock.IncomingSyncs) != 1 {
		t.Fatalf("expected 1 record synced to cloud, got %d", len(cloudMock.IncomingSyncs))
	}

	// 3. Mark as synced on Standalone after successful cloud sync
	ids := []string{records[0].ID}
	err = standaloneMock.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("unexpected error marking synced: %v", err)
	}
	if len(standaloneMock.MarkedSynced) != 1 || standaloneMock.MarkedSynced[0] != "mem_123" {
		t.Errorf("expected mem_123 to be marked synced, got %v", standaloneMock.MarkedSynced)
	}
}

func TestRAGSyncServiceErrors(t *testing.T) {
	ctx := context.Background()

	mock := &MockRAGSyncService{
		FetchErr:   errors.New("fetch error"),
		MarkErr:    errors.New("mark error"),
		ProcessErr: errors.New("process error"),
	}

	_, err := mock.FetchPendingSyncs(ctx, 10)
	if err == nil {
		t.Error("expected fetch error, got nil")
	}

	err = mock.MarkSynced(ctx, []string{"id1"})
	if err == nil {
		t.Error("expected mark error, got nil")
	}

	err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "id2"}})
	if err == nil {
		t.Error("expected process error, got nil")
	}
}
