package hub

import (
	"context"
	"database/sql"
	"time"
)

type RAGSyncServiceImpl struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{
		db: db,
	}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	rows, err := s.db.QueryContext(ctx, "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1", limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		if err := rows.Scan(&r.ID, &r.Context, &r.Vector, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			r.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	now := time.Now()
	for _, id := range ids {
		_, err := tx.ExecContext(ctx, "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2", now, id)
		if err != nil {
			return err
		}
	}
	return tx.Commit()
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	for _, r := range records {
		_, err := tx.ExecContext(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status) VALUES ($1, $2, $3, 'synced') ON CONFLICT (memory_id) DO UPDATE SET context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding, sync_status = 'synced'", r.ID, r.Context, r.Vector)
		if err != nil {
			return err
		}
	}
	return tx.Commit()
}
