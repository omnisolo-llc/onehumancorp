package hub

import (
	"context"
	"testing"
	"time"
    "errors"
)

type MockDBProvider struct {
	Records []RAGSyncRecord
    Err error
}

func (m *MockDBProvider) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    if m.Err != nil {
        return nil, m.Err
    }
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

func (m *MockDBProvider) MarkSynced(ctx context.Context, ids []string) error {
    if m.Err != nil {
        return m.Err
    }
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

func (m *MockDBProvider) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if m.Err != nil {
        return m.Err
    }
	m.Records = append(m.Records, records...)
	return nil
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mockDB := &MockDBProvider{}
    service := NewRAGSyncService(mockDB)

	records := []RAGSyncRecord{
		{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
		{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
	}

	err := service.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

    if len(mockDB.Records) != 2 {
        t.Errorf("Expected 2 records to be inserted, got %d", len(mockDB.Records))
    }
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
    mockDB := &MockDBProvider{
        Records: []RAGSyncRecord{
            {ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
            {ID: "2", Context: "test context 2", SyncStatus: SyncStatusSynced},
            {ID: "3", Context: "test context 3", SyncStatus: SyncStatusPending},
        },
    }
    service := NewRAGSyncService(mockDB)

    pending, err := service.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("FetchPendingSyncs failed: %v", err)
    }

    if len(pending) != 2 {
        t.Errorf("Expected 2 pending records, got %d", len(pending))
    }
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
    mockDB := &MockDBProvider{
        Records: []RAGSyncRecord{
            {ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
            {ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
        },
    }
    service := NewRAGSyncService(mockDB)

    err := service.MarkSynced(context.Background(), []string{"1"})
    if err != nil {
        t.Fatalf("MarkSynced failed: %v", err)
    }

    pending, _ := service.FetchPendingSyncs(context.Background(), 10)
    if len(pending) != 1 {
        t.Errorf("Expected 1 pending record after sync, got %d", len(pending))
    }
    if pending[0].ID != "2" {
        t.Errorf("Expected record 2 to be pending, got %s", pending[0].ID)
    }
}

func TestRAGSyncService_Errors(t *testing.T) {
    mockDB := &MockDBProvider{
        Err: errors.New("db error"),
    }
    service := NewRAGSyncService(mockDB)

    _, err := service.FetchPendingSyncs(context.Background(), 10)
    if err == nil {
        t.Errorf("Expected error from FetchPendingSyncs")
    }

    err = service.MarkSynced(context.Background(), []string{"1"})
    if err == nil {
        t.Errorf("Expected error from MarkSynced")
    }

    err = service.ProcessIncomingSync(context.Background(), []RAGSyncRecord{})
    if err == nil {
        t.Errorf("Expected error from ProcessIncomingSync")
    }
}
