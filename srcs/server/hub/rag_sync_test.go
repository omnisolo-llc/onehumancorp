package hub

import (
	"context"
	"testing"
)

func TestMockRAGSyncService_FetchPendingSyncs(t *testing.T) {
	mock := NewMockRAGSyncService()
	mock.Records["1"] = &RAGSyncRecord{ID: "1", SyncStatus: SyncStatusPending}
	mock.Records["2"] = &RAGSyncRecord{ID: "2", SyncStatus: SyncStatusSynced}
	mock.Records["3"] = &RAGSyncRecord{ID: "3", SyncStatus: SyncStatusPending}

	pending, err := mock.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}
}

func TestMockRAGSyncService_MarkSynced(t *testing.T) {
	mock := NewMockRAGSyncService()
	mock.Records["1"] = &RAGSyncRecord{ID: "1", SyncStatus: SyncStatusPending}

	err := mock.MarkSynced(context.Background(), []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	rec := mock.Records["1"]
	if rec.SyncStatus != SyncStatusSynced {
		t.Errorf("expected status synced, got %s", rec.SyncStatus)
	}
	if rec.LastSyncAt.IsZero() {
		t.Error("expected LastSyncAt to be set")
	}
}

func TestMockRAGSyncService_ProcessIncomingSync(t *testing.T) {
	mock := NewMockRAGSyncService()
	records := []RAGSyncRecord{
		{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
		{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
	}

	err := mock.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mock.Records) != 2 {
		t.Errorf("expected 2 records, got %d", len(mock.Records))
	}

	for _, id := range []string{"1", "2"} {
		rec, ok := mock.Records[id]
		if !ok {
			t.Errorf("expected record %s to exist", id)
			continue
		}
		if rec.SyncStatus != SyncStatusSynced {
			t.Errorf("expected status synced for record %s, got %s", id, rec.SyncStatus)
		}
	}
}
