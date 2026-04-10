package hub

import (
	"go.opentelemetry.io/otel/metric"
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/attribute"
)

type DatabaseRAGSyncService struct {
	provider db.Provider
}

func NewDatabaseRAGSyncService(provider db.Provider) *DatabaseRAGSyncService {
	return &DatabaseRAGSyncService{provider: provider}
}

func (s *DatabaseRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
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
		var lastSyncAt sql.NullString
		var vectorBytes []byte

		if err := rows.Scan(&rec.ID, &rec.Context, &vectorBytes, &rec.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
		}

		if vectorBytes != nil {
			if err := json.Unmarshal(vectorBytes, &rec.Vector); err != nil {
				// Handle byte-encoded arrays if needed. For now assuming json representation or raw bytes.
				// This might need specific handling depending on if it's bytea vs blob vs json
				// Let's assume standard JSON unmarshal if stored as JSON/string. If it's pure pgvector it could be different,
				// but for basic sqlite to postgres sync we use stringified JSON.
				return nil, fmt.Errorf("failed to unmarshal vector: %w", err)
			}
		}

		if lastSyncAt.Valid {
			// Try to parse the time, SQLite stores as string usually
			parsedTime, parseErr := time.Parse(time.RFC3339Nano, lastSyncAt.String)
			if parseErr != nil {
				parsedTime, _ = time.Parse(time.RFC3339, lastSyncAt.String)
			}
			rec.LastSyncAt = parsedTime
		}

		records = append(records, rec)
	}

	return records, rows.Err()
}

func (s *DatabaseRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// This is an oversimplified bulk update for demo/testing. For production Postgres/SQLite
	// we'd typically use unnest or build a dynamic query. For SQLite compatibility we can iterate.
	// Since we are in a method expected to handle both we'll do simple queries.

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `UPDATE swarm_memory_embeddings SET sync_status = 'synced', last_sync_at = $1 WHERE memory_id = $2`
	now := time.Now().UTC()

	for _, id := range ids {
		if _, err := tx.Exec(ctx, query, now, id); err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1, metricWithOp("mark_synced"))
			}
			return fmt.Errorf("failed to mark record %s as synced: %w", id, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}

	return nil
}

func (s *DatabaseRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, $5)
		ON CONFLICT (memory_id) DO UPDATE SET
			context = EXCLUDED.context,
			vector_embedding = EXCLUDED.vector_embedding,
			sync_status = EXCLUDED.sync_status,
			last_sync_at = EXCLUDED.last_sync_at
	`

	for _, rec := range records {
		var vectorBytes []byte
		if rec.Vector != nil {
			vectorBytes, err = json.Marshal(rec.Vector)
			if err != nil {
				return fmt.Errorf("failed to marshal vector: %w", err)
			}
		}

		_, err := tx.Exec(ctx, query, rec.ID, rec.Context, vectorBytes, rec.SyncStatus, rec.LastSyncAt)
		if err != nil {
			if ragSyncErrorsTotal != nil {
				ragSyncErrorsTotal.Add(ctx, 1, metricWithOp("process_incoming"))
			}
			return fmt.Errorf("failed to upsert incoming record %s: %w", rec.ID, err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if ragRecordsSyncedTotal != nil {
		ragRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}

	return nil
}

func metricWithOp(op string) metric.MeasurementOption {
	return metric.WithAttributes(attribute.String("operation", op))
}
