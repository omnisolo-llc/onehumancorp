package hub

import (
	"context"
	"database/sql"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	dbProvider db.Provider
}

func NewRAGSyncService(dbProvider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		dbProvider: dbProvider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT $1"
	if s.dbProvider.IsSQLite() {
		query = "SELECT memory_id, context, vector_embedding, sync_status, last_sync_at FROM swarm_memory_embeddings WHERE sync_status = 'pending' LIMIT ?"
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	rows, err := tx.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var lastSyncAt sql.NullTime // Assuming sql.NullTime or sql.NullTime is needed. We will use sql.NullTime later.
		err := rows.Scan(&rec.ID, &rec.Context, &rec.Vector, &rec.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, err
		}
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, rec)
	}
	if err = rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var successCount int64
	for _, id := range ids {
		query := "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1"
		if s.dbProvider.IsSQLite() {
			query = "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = ?"
		}
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			RAGSyncErrorsTotal.Add(ctx, 1)
			return err
		}
		successCount++
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	RAGRecordsSyncedTotal.Add(ctx, successCount)
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.dbProvider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
		`
		if s.dbProvider.IsSQLite() {
			query = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES (?, ?, ?, ?, ?)
				ON CONFLICT (memory_id) DO UPDATE SET
				context = excluded.context,
				vector_embedding = excluded.vector_embedding,
				sync_status = excluded.sync_status,
				last_sync_at = excluded.last_sync_at
			`
		}
		var lastSyncAt sql.NullTime
		if !rec.LastSyncAt.IsZero() {
			lastSyncAt.Time = rec.LastSyncAt
			lastSyncAt.Valid = true
		}
		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, rec.Vector, rec.SyncStatus, lastSyncAt)
		if err != nil {
			return err
		}
	}
	return tx.Commit(ctx)
}
