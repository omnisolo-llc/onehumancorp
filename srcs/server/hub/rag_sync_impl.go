package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type DefaultRAGSyncService struct {
	provider db.Provider
}

func NewDefaultRAGSyncService(provider db.Provider) *DefaultRAGSyncService {
	return &DefaultRAGSyncService{provider: provider}
}

func (s *DefaultRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	q := `SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
	      FROM swarm_memory_embeddings
	      WHERE sync_status = 'pending'
	      LIMIT $1`

	rows, err := s.provider.Query(ctx, q, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		var vec []byte

		if err := rows.Scan(&r.ID, &r.Context, &vec, &r.SyncStatus, &lastSync); err != nil {
			return nil, fmt.Errorf("failed to scan sync record: %w", err)
		}

		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}

		if vec != nil {
			var floatVec []float32
			if s.provider.IsSQLite() {
				// SQLite: deserialize JSON array string from blob/text
				if err := json.Unmarshal(vec, &floatVec); err != nil {
					return nil, fmt.Errorf("failed to decode sqlite vector: %w", err)
				}
			} else {
				// PostgreSQL pgvector: parsing is complex, skipping for simplicity here, assuming JSON format locally
				if err := json.Unmarshal(vec, &floatVec); err != nil {
					return nil, fmt.Errorf("failed to decode pgvector vector: %w", err)
				}
			}
			r.Vector = floatVec
		}

		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return records, nil
}

func (s *DefaultRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now().UTC()

	for _, id := range ids {
		q := `UPDATE swarm_memory_embeddings
		      SET sync_status = 'synced', last_sync_at = $1
		      WHERE memory_id = $2`

		if _, err := tx.Exec(ctx, q, now, id); err != nil {
			ragSyncErrorsTotal.Add(ctx, 1)
			return fmt.Errorf("failed to update sync status for id %s: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit sync status update: %w", err)
	}

	ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))

	return nil
}

func (s *DefaultRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	now := time.Now().UTC()

	// Handle UPSERT with ON CONFLICT
	for _, r := range records {
		var vecJSON []byte
		if len(r.Vector) > 0 {
			var err error
			vecJSON, err = json.Marshal(r.Vector)
			if err != nil {
				return fmt.Errorf("failed to marshal vector: %w", err)
			}
		}

		if s.provider.IsSQLite() {
			q := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			      VALUES ($1, $2, $3, 'synced', $4)
			      ON CONFLICT(memory_id) DO UPDATE SET
			      context = excluded.context,
			      vector_embedding = excluded.vector_embedding,
			      sync_status = 'synced',
			      last_sync_at = excluded.last_sync_at`
			if _, err := tx.Exec(ctx, q, r.ID, r.Context, string(vecJSON), now); err != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("sqlite upsert failed for id %s: %w", r.ID, err)
			}
		} else {
			q := `INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			      VALUES ($1, $2, $3, 'synced', $4)
			      ON CONFLICT(memory_id) DO UPDATE SET
			      context = EXCLUDED.context,
			      vector_embedding = EXCLUDED.vector_embedding,
			      sync_status = 'synced',
			      last_sync_at = EXCLUDED.last_sync_at`
			if _, err := tx.Exec(ctx, q, r.ID, r.Context, string(vecJSON), now); err != nil {
				ragSyncErrorsTotal.Add(ctx, 1)
				return fmt.Errorf("pg upsert failed for id %s: %w", r.ID, err)
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit incoming sync: %w", err)
	}

	return nil
}
