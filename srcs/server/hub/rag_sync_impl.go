package hub

import (
	"context"
	"database/sql"
	"errors"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncService struct {
	provider db.Provider
}

// NewRAGSyncService creates a new RAGSyncService using the provided database provider.
func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncService{
		provider: provider,
	}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if s.provider == nil {
		return nil, errors.New("db provider is nil")
	}

	query := `
		SELECT memory_id, context, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		ORDER BY created_at ASC
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if s.provider == nil {
		return errors.New("db provider is nil")
	}

	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id = $1
	`
	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, id); err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if s.provider == nil {
		return errors.New("db provider is nil")
	}

	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at)
		VALUES ($1, $2, 'synced', CURRENT_TIMESTAMP)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			sync_status = 'synced',
			last_sync_at = CURRENT_TIMESTAMP
	`
	for _, r := range records {
		if _, err := tx.Exec(ctx, query, r.ID, r.Context); err != nil {
			return err
		}
	}

	return tx.Commit(ctx)
}
