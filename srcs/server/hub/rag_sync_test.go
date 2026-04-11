package hub_test

import (
    "context"
    "testing"
    "github.com/onehumancorp/mono/srcs/server/hub"
)

type MockRAGSyncService struct{}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
    return []hub.RAGSyncRecord{}, nil
}
func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    return nil
}
func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
    return nil
}

func TestRAGSyncInterface(t *testing.T) {
    var _ hub.RAGSyncService = &MockRAGSyncService{}
}
