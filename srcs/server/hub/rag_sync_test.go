package hub

import (
    "context"
    "testing"
)

func TestRAGSyncService_Interface(t *testing.T) {
    // Verify that the interface can be implemented by a mock
    var _ RAGSyncService = &mockRAGSyncService{}
}

type mockRAGSyncService struct{}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    return nil, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    return nil
}
