package hub

import (
	"context"
	"time"
	"fmt"
	"strings"
	"database/sql"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []byte
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type ragSyncService struct {
	db db.Provider
}

func NewRAGSyncService(db db.Provider) RAGSyncService {
	return &ragSyncService{db: db}
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
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_pending")))
		}
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSyncAt sql.NullTime
		var vector []byte
		// We scan into NullTime to handle sqlite parsing issues with dates optionally.
		if err := rows.Scan(&r.ID, &r.Context, &vector, &r.SyncStatus, &lastSyncAt); err != nil {
			// If it fails because of string -> time parsing (sqlite), scan to string and parse
			var lastSyncStr sql.NullString
			if scanErr := rows.Scan(&r.ID, &r.Context, &vector, &r.SyncStatus, &lastSyncStr); scanErr == nil {
				if lastSyncStr.Valid {
					t, err := time.Parse(time.RFC3339, lastSyncStr.String)
					if err == nil {
						r.LastSyncAt = t
					}
				}
			} else {
				if telemetry.RagSyncErrorsTotal != nil {
					telemetry.RagSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "fetch_pending_scan")))
				}
				return nil, fmt.Errorf("failed to scan pending sync record: %w", err)
			}
		} else {
			if lastSyncAt.Valid {
				r.LastSyncAt = lastSyncAt.Time
			}
		}
		r.Vector = vector
		records = append(records, r)
	}
	return records, nil
}

func (s *ragSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced_begin")))
		}
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// Build placeholders
	placeholders := make([]string, len(ids))
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = id
	}

	query := fmt.Sprintf(`
		UPDATE swarm_memory_embeddings
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE memory_id IN (%s)
	`, strings.Join(placeholders, ","))

	if _, err := tx.Exec(ctx, query, args...); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced_exec")))
		}
		return fmt.Errorf("failed to execute mark synced: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "mark_synced_commit")))
		}
		return fmt.Errorf("failed to commit mark synced transaction: %w", err)
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(ids)))
	}
	return nil
}

func (s *ragSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	tx, err := s.db.Begin(ctx)
	if err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_begin")))
		}
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
		VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
		ON CONFLICT(memory_id) DO UPDATE SET
			context = excluded.context,
			vector_embedding = excluded.vector_embedding,
			sync_status = excluded.sync_status,
			last_sync_at = excluded.last_sync_at
	`

	for _, r := range records {
		if _, err := tx.Exec(ctx, query, r.ID, r.Context, r.Vector, r.SyncStatus); err != nil {
			if telemetry.RagSyncErrorsTotal != nil {
				telemetry.RagSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_exec")))
			}
			return fmt.Errorf("failed to upsert incoming sync record: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		if telemetry.RagSyncErrorsTotal != nil {
			telemetry.RagSyncErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "process_incoming_commit")))
		}
		return fmt.Errorf("failed to commit process incoming transaction: %w", err)
	}

	if telemetry.RagRecordsSyncedTotal != nil {
		telemetry.RagRecordsSyncedTotal.Add(ctx, int64(len(records)))
	}
	return nil
}
