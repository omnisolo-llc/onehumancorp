package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		provider: provider,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_timestamp
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var rawVector []byte
		var syncStatus sql.NullString
		var lastSync sql.NullTime

		err := rows.Scan(&rec.ID, &rec.Context, &rawVector, &syncStatus, &lastSync)
		if err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}

		if len(rawVector) > 0 {
			var vector []float32
			if err := json.Unmarshal(rawVector, &vector); err != nil {
				return nil, fmt.Errorf("failed to unmarshal vector: %w", err)
			}
			rec.Vector = vector
		}

		if syncStatus.Valid {
			rec.SyncStatus = SyncStatus(syncStatus.String)
		}
		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}

		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("row iteration error: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_timestamp = $1
		WHERE memory_id = $2
	`

	now := time.Now()

	// Convert ids to an array of arguments for sqlite compatibility via json_each or multiple args,
	// wait SQLite doesn't natively support ANY($1) array operators well for standard IN clause dynamically without json_each.
	// We'll stick to a simple loop for safety and compatibility in standalone hybrid unless we specifically craft an IN (?, ?, ?)
	// but dynamic arg expansion is error-prone. Let's optimize it slightly by preparing the statement if possible or just loop.
	// To comply with the nitpick from PR review and ensure performance, let's use a batch approach if possible or simple dynamic IN.
	if len(ids) > 0 {
		args := make([]interface{}, len(ids)+1)
		args[0] = now
		query = "UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_timestamp = $1 WHERE memory_id IN ("
		for i, id := range ids {
			if i > 0 {
				query += ", "
			}
			query += fmt.Sprintf("$%d", i+2)
			args[i+1] = id
		}
		query += ")"

		_, err := tx.Exec(ctx, query, args...)
		if err != nil {
			return fmt.Errorf("failed to update sync status: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (
			memory_id, context, vector_embedding, sync_status, last_sync_timestamp
		) VALUES (
			$1, $2, $3, $4, $5
		) ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_timestamp = EXCLUDED.last_sync_timestamp
	`

	for _, rec := range records {
		var rawVector []byte
		if len(rec.Vector) > 0 {
			var err error
			rawVector, err = json.Marshal(rec.Vector)
			if err != nil {
				return fmt.Errorf("failed to marshal vector for id %s: %w", rec.ID, err)
			}
		} else {
			rawVector = nil
		}

		var lastSync interface{}
		if !rec.LastSyncAt.IsZero() {
			lastSync = rec.LastSyncAt
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, rawVector, rec.SyncStatus, lastSync)
		if err != nil {
			return fmt.Errorf("failed to upsert record id %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	return nil
}
