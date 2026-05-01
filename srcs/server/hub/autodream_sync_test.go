package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

// mockAutoDreamSyncService is a test mock implementation of AutoDreamSyncService.
type mockAutoDreamSyncService struct {
	records []*AutoDreamSyncRecord
}

func (m *mockAutoDreamSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]*AutoDreamSyncRecord, error) {
	var pending []*AutoDreamSyncRecord
	for _, r := range m.records {
		if r.SyncStatus == "pending" {
			pending = append(pending, r)
			if len(pending) >= limit {
				break
			}
		}
	}
	return pending, nil
}

func (m *mockAutoDreamSyncService) ProcessIncomingSync(ctx context.Context, record *AutoDreamSyncRecord) error {
	if record.ID == "" {
		return errors.New("record ID cannot be empty")
	}
	for i, r := range m.records {
		if r.ID == record.ID {
			m.records[i] = record
			return nil
		}
	}
	m.records = append(m.records, record)
	return nil
}

func (m *mockAutoDreamSyncService) MarkRecordSynced(ctx context.Context, id string) error {
	for _, r := range m.records {
		if r.ID == id {
			r.SyncStatus = "synced"
			now := time.Now()
			r.LastSyncAt = &now
			return nil
		}
	}
	return errors.New("record not found")
}

func TestFetchPendingSyncs(t *testing.T) {
	mockService := &mockAutoDreamSyncService{
		records: []*AutoDreamSyncRecord{
			{ID: "1", SyncStatus: "pending"},
			{ID: "2", SyncStatus: "synced"},
			{ID: "3", SyncStatus: "pending"},
		},
	}

	ctx := context.Background()
	results, err := mockService.FetchPendingSyncs(ctx, 2)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(results) != 2 {
		t.Fatalf("expected 2 results, got %d", len(results))
	}
	if results[0].ID != "1" || results[1].ID != "3" {
		t.Errorf("unexpected record IDs returned")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	mockService := &mockAutoDreamSyncService{
		records: []*AutoDreamSyncRecord{},
	}

	ctx := context.Background()

	// Test new record
	record := &AutoDreamSyncRecord{ID: "1", SyncStatus: "pending"}
	err := mockService.ProcessIncomingSync(ctx, record)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(mockService.records) != 1 {
		t.Fatalf("expected 1 record in mock service, got %d", len(mockService.records))
	}

	// Test update existing
	recordUpdated := &AutoDreamSyncRecord{ID: "1", SyncStatus: "synced"}
	err = mockService.ProcessIncomingSync(ctx, recordUpdated)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if mockService.records[0].SyncStatus != "synced" {
		t.Errorf("expected status to be synced, got %s", mockService.records[0].SyncStatus)
	}

	// Test validation error
	err = mockService.ProcessIncomingSync(ctx, &AutoDreamSyncRecord{})
	if err == nil {
		t.Errorf("expected error for empty record ID")
	}
}

func TestMarkRecordSynced(t *testing.T) {
	mockService := &mockAutoDreamSyncService{
		records: []*AutoDreamSyncRecord{
			{ID: "1", SyncStatus: "pending"},
		},
	}

	ctx := context.Background()

	err := mockService.MarkRecordSynced(ctx, "1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if mockService.records[0].SyncStatus != "synced" {
		t.Errorf("expected status to be synced, got %s", mockService.records[0].SyncStatus)
	}
	if mockService.records[0].LastSyncAt == nil {
		t.Errorf("expected LastSyncAt to be set")
	}

	// Test not found
	err = mockService.MarkRecordSynced(ctx, "nonexistent")
	if err == nil {
		t.Errorf("expected error for nonexistent record")
	}
}
