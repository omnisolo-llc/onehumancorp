package hub

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"time"
)

type ragSyncServiceImpl struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{dbProvider: dbProvider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	// For standalone SQLite
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.dbProvider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt *time.Time
		err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, err
		}
		if lastSyncAt != nil {
			rec.LastSyncAt = *lastSyncAt
		}
		records = append(records, rec)
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE memory_id = $1
		`
		_, err := s.dbProvider.Exec(ctx, query, id)
		if err != nil {
			if telemetry.RAGSyncErrorsTotal != nil {
				telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}

	if telemetry.RAGRecordsSyncedTotal != nil {
		telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, rec := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
		_, err := s.dbProvider.Exec(ctx, query, rec.ID, rec.Context, rec.Vector)
		if err != nil {
			if telemetry.RAGSyncErrorsTotal != nil {
				telemetry.RAGSyncErrorsTotal.Add(ctx, 1)
			}
			return err
		}
	}
	if telemetry.RAGRecordsSyncedTotal != nil && len(records) > 0 {
		telemetry.RAGRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}
