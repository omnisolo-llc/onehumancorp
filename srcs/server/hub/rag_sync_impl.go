package hub

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type DefaultRAGSyncService struct {
	provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{provider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, content, embedding, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1"
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus)
		if err != nil {
			return nil, err
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// Note: ANY is not supported in sqlite, so we'll need to use an IN clause if we pass multiple ids
	// but for simplicity we will just do one by one or create placeholders
	for _, id := range ids {
		query := "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1"
		_, err := s.provider.Exec(ctx, query, id)
		if err != nil && telemetry.RAGSyncErrorsTotal != nil {
			telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
		}
	}
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		query := "INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at) VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP) ON CONFLICT (id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP"
		_, err := s.provider.Exec(ctx, query, r.ID, r.Context, r.Vector)
		if err != nil {
			if telemetry.RAGSyncErrorsTotal != nil {
				telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}
	if telemetry.RAGRecordsSyncedTotal != nil {
		telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}
