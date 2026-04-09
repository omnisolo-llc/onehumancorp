package hub

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// RAGSyncServiceImpl implements RAGSyncService interface.
type RAGSyncServiceImpl struct {
}

// NewRAGSyncService creates a new instance of RAGSyncServiceImpl.
func NewRAGSyncService() RAGSyncService {
	return &RAGSyncServiceImpl{}
}

// FetchPendingSyncs retrieves records from the local DB that need syncing.
func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// Implementation placeholder for future db connectivity
	return nil, nil
}

// MarkSynced updates the local DB after a successful sync to the cloud.
func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	// Implementation placeholder for future db connectivity
	telemetry.RecordRagRecordsSynced(ctx, int64(len(ids)))
	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB.
func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Implementation placeholder for future db connectivity
	telemetry.RecordRagRecordsSynced(ctx, int64(len(records)))
	return nil
}
