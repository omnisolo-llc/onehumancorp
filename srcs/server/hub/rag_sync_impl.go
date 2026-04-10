package hub

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type RAGSyncServiceImpl struct {
	dbProvider db.Provider
}

func NewRAGSyncService(provider db.Provider) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{
		dbProvider: provider,
	}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, sync_status, last_sync_at
		FROM autodream_memories
		WHERE sync_status = 'pending'
		ORDER BY created_at ASC
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &r.LastSyncAt); err != nil {
			return nil, err
		}
		records = append(records, r)
	}

	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	query := `
		UPDATE autodream_memories
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE id = ANY($1)
	`
	_, err := s.dbProvider.Exec(ctx, query, ids)
	if err != nil {
		telemetry.RecordRAGSyncError(ctx)
		return err
	}
	telemetry.RecordRAGRecordSynced(ctx, len(ids))
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		query := `
			INSERT INTO autodream_memories (id, content, sync_status, last_sync_at)
			VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
		// For simplicity, converting vector storage to a string format can be done here.
		// However, in Postgres, it expects a vector type, and in SQLite, it expects a string.
		// For the prompt scope, we are doing foundational interface routing.
		_, err := s.dbProvider.Exec(ctx, query, r.ID, r.Context)
		if err != nil {
			telemetry.RecordRAGSyncError(ctx)
			return err
		}
	}
	telemetry.RecordRAGRecordSynced(ctx, len(records))

	return nil
}
