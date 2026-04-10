package hub

import (
    "context"
    "testing"
)

func TestBasicRAGSyncService_FetchPendingSyncs(t *testing.T) {
    service := NewBasicRAGSyncService()

    records, err := service.FetchPendingSyncs(context.Background(), 10)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if len(records) != 0 {
        t.Fatalf("expected 0 record, got %d", len(records))
    }
}

func TestBasicRAGSyncService_MarkSynced(t *testing.T) {
    service := NewBasicRAGSyncService()

    err := service.MarkSynced(context.Background(), []string{"1"})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
}

func TestBasicRAGSyncService_ProcessIncomingSync(t *testing.T) {
    service := NewBasicRAGSyncService()

    err := service.ProcessIncomingSync(context.Background(), []RAGSyncRecord{{ID: "1"}})
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
}
