package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	records []RAGSyncRecord
	synced  []string
	err     error
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.err != nil {
		return nil, m.err
	}
	if len(m.records) > limit {
		return m.records[:limit], nil
	}
	return m.records, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if m.err != nil {
		return m.err
	}
	m.synced = append(m.synced, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if m.err != nil {
		return m.err
	}
	m.records = append(m.records, records...)
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := &mockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}
}

func TestMarkSynced(t *testing.T) {
	mockService := &mockRAGSyncService{}

	ctx := context.Background()
	err := mockService.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(mockService.synced) != 2 {
		t.Fatalf("expected 2 synced records, got %d", len(mockService.synced))
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mockService := &mockRAGSyncService{}

	ctx := context.Background()
	records := []RAGSyncRecord{
		{ID: "1", Context: "test context", LastSyncAt: time.Now()},
	}

	err := mockService.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(mockService.records) != 1 {
		t.Fatalf("expected 1 processed record, got %d", len(mockService.records))
	}
}

func TestRAGSyncServiceError(t *testing.T) {
	expectedErr := errors.New("mock error")
	mockService := &mockRAGSyncService{err: expectedErr}

	ctx := context.Background()

	_, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != expectedErr {
		t.Fatalf("expected error %v, got %v", expectedErr, err)
	}

	err = mockService.MarkSynced(ctx, []string{"1"})
	if err != expectedErr {
		t.Fatalf("expected error %v, got %v", expectedErr, err)
	}

	err = mockService.ProcessIncomingSync(ctx, []RAGSyncRecord{})
	if err != expectedErr {
		t.Fatalf("expected error %v, got %v", expectedErr, err)
	}
}

func TestDefaultRAGSyncService(t *testing.T) {
	service := NewDefaultRAGSyncService()
	ctx := context.Background()

	_, err := service.FetchPendingSyncs(ctx, 10)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{})
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}
