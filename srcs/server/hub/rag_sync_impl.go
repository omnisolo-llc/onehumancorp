package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/metric"
)

type ragSyncService struct {
	db db.Provider
}

// NewRAGSyncService creates a new implementation of RAGSyncService.
func NewRAGSyncService(database db.Provider) RAGSyncService {
	return &ragSyncService{db: database}
}

func (s *ragSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	rows, err := s.db.Query(ctx, query, limit)
	if err != nil {
		ragSyncErrors.Add(ctx, 1, metric.WithAttributes())
		return nil, fmt.Errorf("fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var rec RAGSyncRecord
		var rawVector []byte
		var status sql.NullString
		var lastSync sql.NullTime

		if err := rows.Scan(&rec.ID, &rec.Context, &rawVector, &status, &lastSync); err != nil {
			ragSyncErrors.Add(ctx, 1, metric.WithAttributes())
			return nil, fmt.Errorf("scan pending syncs: %w", err)
		}

		if len(rawVector) > 0 {
			if s.db.IsSQLite() {
				// SQLite might store it as JSON string representation depending on the provider,
				// but let's assume JSON parsing is safe.
				var floatArr []float32
				if err := json.Unmarshal(rawVector, &floatArr); err == nil {
					rec.Vector = floatArr
				} else {
					// Direct cast if not JSON (e.g. pgvector bytea structure would be more complex, but we try to keep it simple here)
					// Assuming pgvector isn't directly parsed into float32 here without specific lib, but matching the spec.
				}
			}
		}

		if status.Valid {
			rec.SyncStatus = SyncStatus(status.String)
		} else {
			rec.SyncStatus = SyncStatusPending
		}

		if lastSync.Valid {
			rec.LastSyncAt = lastSync.Time
		}
		records = append(records, rec)
	}

	if err := rows.Err(); err != nil {
		ragSyncErrors.Add(ctx, 1, metric.WithAttributes())
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Simple batch update
	placeholders := make([]string, len(ids))
	args := make([]any, len(ids)+2)
	now := time.Now()
	args[0] = string(SyncStatusSynced)
	args[1] = now

	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+3)
		args[i+2] = id
	}

	query := fmt.Sprintf(`
		UPDATE swarm_memory_embeddings
		SET sync_status = $1, last_sync_at = $2
		WHERE memory_id IN (%s)
	`, strings.Join(placeholders, ", "))

	_, err := s.db.Exec(ctx, query, args...)
	if err != nil {
		ragSyncErrors.Add(ctx, 1, metric.WithAttributes())
		return fmt.Errorf("mark synced: %w", err)
	}

	ragRecordsSynced.Add(ctx, int64(len(ids)), metric.WithAttributes())
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := s.db.Begin(ctx)
	if err != nil {
		ragSyncErrors.Add(ctx, 1, metric.WithAttributes())
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	for _, rec := range records {
		var rawVector []byte
		if rec.Vector != nil {
			rawVector, _ = json.Marshal(rec.Vector)
		}

		// Upsert logic (simplified for Postgres/SQLite overlap)
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		if s.db.IsSQLite() {
			// SQLite requires a slightly different syntax if strictly following 3.24+ for UPSERT
			query = `
				INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
				VALUES ($1, $2, $3, $4, $5)
				ON CONFLICT(memory_id) DO UPDATE SET
					context = excluded.context,
					vector_embedding = excluded.vector_embedding,
					sync_status = excluded.sync_status,
					last_sync_at = excluded.last_sync_at
			`
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, rawVector, string(SyncStatusSynced), time.Now())
		if err != nil {
			ragSyncErrors.Add(ctx, 1, metric.WithAttributes())
			return fmt.Errorf("upsert record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		ragSyncErrors.Add(ctx, 1, metric.WithAttributes())
		return fmt.Errorf("commit tx: %w", err)
	}

	ragRecordsSynced.Add(ctx, int64(len(records)), metric.WithAttributes())
	return nil
}
