package hub

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type RAGSyncServiceImpl struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{
		dbProvider: dbProvider,
	}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// Not fully implemented yet. Assuming standard DB access logic here.
	return []RAGSyncRecord{}, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Not fully implemented yet.
	return nil
}
