package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type DefaultRAGSyncService struct {
	provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{
		provider: provider,
	}
}

func float32ArrayToString(floats []float32) (string, error) {
	bytes, err := json.Marshal(floats)
	if err != nil {
		return "", err
	}
	return string(bytes), nil
}

func stringToFloat32Array(s string) ([]float32, error) {
	var floats []float32
	err := json.Unmarshal([]byte(s), &floats)
	if err != nil {
		return nil, err
	}
	return floats, nil
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	if s.provider.IsSQLite() {
		query = `
			SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
			FROM swarm_memory_embeddings
			WHERE sync_status = 'pending'
			LIMIT ?
		`
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var vecStr string
		var lastSyncAt sql.NullTime
		err := rows.Scan(&rec.ID, &rec.Context, &vecStr, &rec.SyncStatus, &lastSyncAt)
		if err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		floats, err := stringToFloat32Array(vecStr)
		if err != nil {
			return nil, fmt.Errorf("failed to parse vector for id %s: %w", rec.ID, err)
		}
		rec.Vector = floats
		if lastSyncAt.Valid {
			rec.LastSyncAt = lastSyncAt.Time
		}
		records = append(records, rec)
	}
	if err := rows.Err(); err != nil {
		return nil, err
	}
	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, id := range ids {
		query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = $1`
		if s.provider.IsSQLite() {
			query = `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE memory_id = ?`
		}
		_, err := tx.Exec(ctx, query, id)
		if err != nil {
			return fmt.Errorf("failed to mark synced for id %s: %w", id, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		return err
	}

	RAGRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		vecStr, err := float32ArrayToString(rec.Vector)
		if err != nil {
			return fmt.Errorf("failed to serialize vector for id %s: %w", rec.ID, err)
		}

		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
		if s.provider.IsSQLite() {
			query = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES (?, ?, ?, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`
		}
		_, err = tx.Exec(ctx, query, rec.ID, rec.Context, vecStr)
		if err != nil {
			RAGSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to upsert incoming sync for id %s: %w", rec.ID, err)
		}
	}

	err = tx.Commit(ctx)
	if err != nil {
		return err
	}

	RAGRecordsSyncedTotal.Add(ctx, int64(len(records)))
	return nil
}
