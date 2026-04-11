package hub

import (
	"context"
	"testing"
)

type dummyRAGSyncService struct{}

func (d *dummyRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	return nil, nil
}

func (d *dummyRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	return nil
}

func (d *dummyRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	return nil
}

func TestRAGSyncService_InterfaceCompliance(t *testing.T) {
	var _ RAGSyncService = (*dummyRAGSyncService)(nil)
}
