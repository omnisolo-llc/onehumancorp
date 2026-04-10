package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

type mockRAGSyncService struct {
	pending []RAGSyncRecord
	synced  []string
	cloud   []RAGSyncRecord
	err     error
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if m.err != nil {
		return nil, m.err
	}
	if limit > len(m.pending) {
		limit = len(m.pending)
	}
	res := m.pending[:limit]
	m.pending = m.pending[limit:]
	return res, nil
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
	m.cloud = append(m.cloud, records...)
	return nil
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	mock := &mockRAGSyncService{
		pending: []RAGSyncRecord{
			{ID: "1", Context: "test 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	records, err := mock.FetchPendingSyncs(ctx, 1)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}
	if records[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", records[0].ID)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	mock := &mockRAGSyncService{}

	ctx := context.Background()
	err := mock.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mock.synced) != 2 {
		t.Fatalf("expected 2 synced IDs, got %d", len(mock.synced))
	}
	if mock.synced[0] != "1" || mock.synced[1] != "2" {
		t.Errorf("expected [1 2], got %v", mock.synced)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mock := &mockRAGSyncService{}

	ctx := context.Background()
	now := time.Now()
	records := []RAGSyncRecord{
		{ID: "1", Context: "cloud 1", SyncStatus: SyncStatusSynced, LastSyncAt: now},
	}

	err := mock.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mock.cloud) != 1 {
		t.Fatalf("expected 1 cloud record, got %d", len(mock.cloud))
	}
	if mock.cloud[0].Context != "cloud 1" {
		t.Errorf("expected context 'cloud 1', got %s", mock.cloud[0].Context)
	}
}

func TestRAGSyncService_Error(t *testing.T) {
	mock := &mockRAGSyncService{
		err: errors.New("simulated error"),
	}

	ctx := context.Background()
	_, err := mock.FetchPendingSyncs(ctx, 10)
	if err == nil {
		t.Fatal("expected error, got nil")
	}

	err = mock.MarkSynced(ctx, []string{"1"})
	if err == nil {
		t.Fatal("expected error, got nil")
	}

	err = mock.ProcessIncomingSync(ctx, []RAGSyncRecord{})
	if err == nil {
		t.Fatal("expected error, got nil")
	}
}
