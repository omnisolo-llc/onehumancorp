package hub

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// RAGSyncServiceImpl provides a concrete implementation of RAGSyncService.
type RAGSyncServiceImpl struct {
	provider db.Provider
}

// NewRAGSyncService creates a new RAGSyncServiceImpl.
func NewRAGSyncService(provider db.Provider) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{
		provider: provider,
	}
}

// FetchPendingSyncs retrieves records from the local DB that need syncing
func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &r.LastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}
		records = append(records, r)
	}

	if len(records) > 0 && telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

// MarkSynced updates the local DB after a successful sync to the cloud
func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Handle UPSERT/UPDATE
	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id = ANY($1)
	`
	if s.provider.IsSQLite() {
		// SQLite doesn't have ANY($1) for arrays, use a transaction and execute iteratively.
		tx, err := s.provider.Begin(ctx)
		if err != nil {
			return err
		}
		defer tx.Rollback(ctx)

		// SQLite has no explicit Prepare on this interface, use Exec
		updateQuery := `
			UPDATE swarm_memory_embeddings
			SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
			WHERE memory_id = $1
		`

		for _, id := range ids {
			_, err := tx.Exec(ctx, updateQuery, id)
			if err != nil {
				return err
			}
		}

		return tx.Commit(ctx)
	}

	_, err := s.provider.Exec(ctx, query, ids)
	if err != nil {
		return fmt.Errorf("failed to update sync status: %w", err)
	}

	return nil
}

// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	upsertQuery := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = CURRENT_TIMESTAMP
	`

	if s.provider.IsSQLite() {
		for _, r := range records {
			_, err := tx.Exec(ctx, upsertQuery, r.ID, r.Context, r.Vector, r.SyncStatus)
			if err != nil {
				return err
			}
		}
	} else {
		// Postgres execution
		for _, r := range records {
			_, err := tx.Exec(ctx, upsertQuery, r.ID, r.Context, r.Vector, r.SyncStatus)
			if err != nil {
				return err
			}
		}
	}

	return tx.Commit(ctx)
}
