package hub

import (
	"context"
	"errors"
	"testing"
	"time"

	"go.opentelemetry.io/otel"
)

type MockRAGSyncService struct {
	Records []RAGSyncRecord
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
	idMap := make(map[string]bool)
	for _, id := range ids {
		idMap[id] = true
	}

	count := 0
	for i, r := range m.Records {
		if idMap[r.ID] {
			m.Records[i].SyncStatus = SyncStatusSynced
			m.Records[i].LastSyncAt = time.Now()
			count++
		}
	}

	if count > 0 {
		RagRecordsSyncedTotal.Add(ctx, int64(count))
	} else {
		RagSyncErrorsTotal.Add(ctx, 1)
		return errors.New("no records found to mark synced")
	}

	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}
	// Simulate upsert
	for _, rec := range records {
		found := false
		for i, r := range m.Records {
			if r.ID == rec.ID {
				m.Records[i] = rec
				m.Records[i].SyncStatus = SyncStatusSynced
				m.Records[i].LastSyncAt = time.Now()
				found = true
				break
			}
		}
		if !found {
			rec.SyncStatus = SyncStatusSynced
			rec.LastSyncAt = time.Now()
			m.Records = append(m.Records, rec)
		}
	}
	RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}

func TestFetchPendingSyncs(t *testing.T) {
	mock := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
			{ID: "2", SyncStatus: SyncStatusSynced},
			{ID: "3", SyncStatus: SyncStatusPending},
		},
	}

	pending, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}
}

func TestMarkSynced(t *testing.T) {
	// Re-initialize meter for test isolated context if needed, but globals are fine
	_ = otel.Meter("test")

	mock := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()
	err := mock.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mock.Records[0].SyncStatus != SyncStatusSynced {
		t.Errorf("expected record to be synced, got %s", mock.Records[0].SyncStatus)
	}
	if mock.Records[0].LastSyncAt.IsZero() {
		t.Errorf("expected LastSyncAt to be set")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mock := &MockRAGSyncService{
		Records: []RAGSyncRecord{
			{ID: "1", SyncStatus: SyncStatusSynced, Context: "old"},
		},
	}

	incoming := []RAGSyncRecord{
		{ID: "1", Context: "new"},
		{ID: "2", Context: "new2"},
	}

	ctx := context.Background()
	err := mock.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.Records) != 2 {
		t.Errorf("expected 2 records, got %d", len(mock.Records))
	}
	if mock.Records[0].Context != "new" {
		t.Errorf("expected updated context")
	}
	if mock.Records[1].Context != "new2" {
		t.Errorf("expected inserted context")
	}
}
