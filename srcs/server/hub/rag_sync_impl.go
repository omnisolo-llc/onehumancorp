package hub

import (
	"context"
)

// DefaultRAGSyncService is a placeholder implementation of RAGSyncService.
type DefaultRAGSyncService struct{}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// Not implemented
	return []RAGSyncRecord{}, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	// Not implemented
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Not implemented
	return nil
}
