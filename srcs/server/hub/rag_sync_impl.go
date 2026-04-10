package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric"
)

type ragSyncServiceImpl struct {
	db db.Provider
}

// NewRAGSyncService creates a new RAGSyncService instance.
func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{
		db: db,
	}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_timestamp
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1`

	var rows db.Rows
	var err error
	if s.db.IsSQLite() {
		// SQLite driver uses '?'
		query = `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_timestamp
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT ?`
		rows, err = s.db.Query(ctx, query, limit)
	} else {
		rows, err = s.db.Query(ctx, query, limit)
	}

	if err != nil {
		syncErrors.Add(ctx, 1)
		return nil, fmt.Errorf("failed to query pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		var vectorData []byte

		if err := rows.Scan(&r.ID, &r.Context, &vectorData, &r.SyncStatus, &lastSync); err != nil {
			syncErrors.Add(ctx, 1)
			return nil, fmt.Errorf("failed to scan pending sync row: %w", err)
		}

		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}

		if len(vectorData) > 0 {
			if s.db.IsSQLite() {
				// Parse JSON string array
				var v []float32
				if err := json.Unmarshal(vectorData, &v); err != nil {
					syncErrors.Add(ctx, 1)
					return nil, fmt.Errorf("failed to unmarshal sqlite vector data: %w", err)
				}
				r.Vector = v
			} else {
				// Parse Postgres pgvector string or binary format
				// Based on standard db abstraction vector_embedding is stored as string/[]byte Representation
				// In an actual scenario pgvector returns a string like "[1.1,2.2,3.3]"
				var v []float32
				if err := json.Unmarshal(vectorData, &v); err != nil {
					// pgvector format might be [1,2,3] which is valid JSON
					syncErrors.Add(ctx, 1)
					return nil, fmt.Errorf("failed to unmarshal pgvector vector data: %w", err)
				}
				r.Vector = v
			}
		}

		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		syncErrors.Add(ctx, 1)
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer func() {
		_ = tx.Rollback(ctx)
	}()

	if s.db.IsSQLite() {
		// SQLite fallback since arrays are not supported
		for _, id := range ids {
			_, err := tx.Exec(ctx, `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP WHERE memory_id = ?`, id)
			if err != nil {
				syncErrors.Add(ctx, 1)
				return fmt.Errorf("failed to mark synced for id %s in sqlite: %w", id, err)
			}
		}
	} else {
		// Postgres
		_, err := tx.Exec(ctx, `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_timestamp = CURRENT_TIMESTAMP WHERE memory_id = ANY($1)`, ids)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to mark synced in postgres: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	recordsSynced.Add(ctx, int64(len(ids)))
	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer func() {
		_ = tx.Rollback(ctx)
	}()

	for _, r := range records {
		vectorData, err := json.Marshal(r.Vector)
		if err != nil {
			syncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to marshal vector data: %w", err)
		}

		lastSync := sql.NullTime{Time: r.LastSyncAt, Valid: !r.LastSyncAt.IsZero()}

		if s.db.IsSQLite() {
			_, err = tx.Exec(ctx, `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_timestamp)
				VALUES (?, ?, ?, 'synced', ?)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = 'synced',
					last_sync_timestamp = excluded.last_sync_timestamp
			`, r.ID, r.Context, vectorData, lastSync)
		} else {
			// Ensure formatting for PGVector if needed
			// Let's pass it as a JSON array string and PG can cast to vector
			vectorStr := string(vectorData)

			_, err = tx.Exec(ctx, `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_timestamp)
				VALUES ($1, $2, $3::vector, 'synced', $4)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = 'synced',
					last_sync_timestamp = excluded.last_sync_timestamp
			`, r.ID, r.Context, vectorStr, lastSync)
		}
		if err != nil {
			syncErrors.Add(ctx, 1)
			return fmt.Errorf("failed to upsert record %s: %w", r.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		syncErrors.Add(ctx, 1)
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	recordsSynced.Add(ctx, int64(len(records)), metric.WithAttributes())
	return nil
}
