package hub

import (
	"context"
	"database/sql"
)

type RAGSyncServiceImpl struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{db: db}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime
		err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, rec)
	}
	return records, rows.Err()
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// Basic implementation for SQLite/Postgres compatibility in query building
	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id = $1
	`
	for _, id := range ids {
		_, err := s.db.ExecContext(ctx, query, id)
		if err != nil {
			return err
		}
	}
	return nil
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	// Simple upsert logic compatible with both Postgres and SQLite
	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
		ON CONFLICT(memory_id) DO UPDATE SET
			context = excluded.context,
			vector_embedding = excluded.vector_embedding,
			sync_status = excluded.sync_status,
			last_sync_at = CURRENT_TIMESTAMP
	`
	for _, rec := range records {
		_, err := s.db.ExecContext(ctx, query, rec.ID, rec.Context, rec.Vector, rec.SyncStatus)
		if err != nil {
			return err
		}
	}
	return nil
}
